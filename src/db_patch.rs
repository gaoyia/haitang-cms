//! 增量 schema 补丁（toasty `push_schema` 不会为已有表补列）

use crate::models::asset::now_unix;

/// 为已有 SQLite 库补全新增列等结构变更
pub async fn apply_schema_patches(db: &mut toasty::Db) {
    if !db.capability().sql {
        return;
    }

    ensure_assets_upload_name(db).await;
    ensure_post_meta_timestamps(db).await;
}

async fn add_i64_column(db: &mut toasty::Db, table: &str, column: &str) {
    let sql = format!("ALTER TABLE {table} ADD COLUMN {column} INTEGER NOT NULL DEFAULT 0");
    let result = toasty::sql::statement(&sql).exec(db).await;

    match result {
        Ok(_) => println!("[数据库] 已补列 {table}.{column}"),
        Err(e) => {
            let msg = e.to_string();
            if !msg.contains("duplicate column") {
                eprintln!("[警告] 补列 {table}.{column} 失败: {e}");
            }
        }
    }
}

async fn ensure_assets_upload_name(db: &mut toasty::Db) {
    let result = toasty::sql::statement(
        "ALTER TABLE assets ADD COLUMN upload_name TEXT NOT NULL DEFAULT ''",
    )
    .exec(db)
    .await;

    match result {
        Ok(_) => println!("[数据库] 已补列 assets.upload_name"),
        Err(e) => {
            let msg = e.to_string();
            if !msg.contains("duplicate column") {
                eprintln!("[警告] 补列 assets.upload_name 失败: {e}");
            }
        }
    }
}

async fn ensure_post_meta_timestamps(db: &mut toasty::Db) {
    for column in [
        "created_at",
        "updated_at",
        "published_at",
        "display_time",
    ] {
        add_i64_column(db, "post_metas", column).await;
    }

    let now = now_unix();
    let backfill = toasty::sql::statement(
        "UPDATE post_metas SET created_at = ?1, updated_at = ?1, display_time = ?1 WHERE created_at = 0",
    )
    .bind(now)
    .exec(db)
    .await;

    if let Err(e) = backfill {
        eprintln!("[警告] 回填 post_metas 时间字段失败: {e}");
    }

    let publish_backfill = toasty::sql::statement(
        "UPDATE post_metas SET published_at = ?1 WHERE status = 1 AND published_at = 0",
    )
    .bind(now)
    .exec(db)
    .await;

    if let Err(e) = publish_backfill {
        eprintln!("[警告] 回填 post_metas.published_at 失败: {e}");
    }
}
