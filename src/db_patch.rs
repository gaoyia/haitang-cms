//! 增量 schema 补丁（toasty `push_schema` 不会为已有表补列）

/// 为已有 SQLite 库补全新增列等结构变更
pub async fn apply_schema_patches(db: &mut toasty::Db) {
    if !db.capability().sql {
        return;
    }

    ensure_assets_upload_name(db).await;
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
            // 列已存在时 SQLite 报 duplicate column，视为正常
            if !msg.contains("duplicate column") {
                eprintln!("[警告] 补列 assets.upload_name 失败: {e}");
            }
        }
    }
}
