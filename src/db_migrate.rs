//! 开发库结构迁移（toasty push_schema 不会删除旧列）

use rusqlite::{Connection, Result};

const DB_PATH: &str = "db/haitang.sqlite";

/// 启动时执行一次性 SQLite 结构迁移
pub fn run() -> Result<()> {
    if !std::path::Path::new(DB_PATH).exists() {
        return Ok(());
    }

    let conn = Connection::open(DB_PATH)?;
    migrate_post_tags_to_i18n(&conn)?;
    Ok(())
}

/// 将 post_metas.tags 迁入 post_i18ns，并删除 meta 上的 tags 列
fn migrate_post_tags_to_i18n(conn: &Connection) -> Result<()> {
    if !table_exists(conn, "post_metas")? {
        return Ok(());
    }

    ensure_post_i18n_tags_column(conn)?;

    if !column_exists(conn, "post_metas", "tags")? {
        return Ok(());
    }

    println!("[迁移] 将 post_metas.tags 迁入 post_i18ns…");

    let default_locale = read_site_default_locale(conn);
    conn.execute(
        "UPDATE post_i18ns
         SET tags = (
             SELECT TRIM(m.tags) FROM post_metas m WHERE m.id = post_i18ns.post_id
         )
         WHERE lang = ?1
           AND TRIM(tags) = ''
           AND post_id IN (SELECT id FROM post_metas WHERE TRIM(tags) != '')",
        [default_locale.as_str()],
    )?;

    conn.execute("ALTER TABLE post_metas DROP COLUMN tags", [])?;
    println!("[迁移] 已移除 post_metas.tags 列");
    Ok(())
}

fn ensure_post_i18n_tags_column(conn: &Connection) -> Result<()> {
    if !table_exists(conn, "post_i18ns")? {
        return Ok(());
    }
    if column_exists(conn, "post_i18ns", "tags")? {
        return Ok(());
    }
    conn.execute(
        "ALTER TABLE post_i18ns ADD COLUMN tags TEXT NOT NULL DEFAULT ''",
        [],
    )?;
    Ok(())
}

fn table_exists(conn: &Connection, table: &str) -> Result<bool> {
    let mut stmt = conn.prepare(
        "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1 LIMIT 1",
    )?;
    Ok(stmt.exists([table])?)
}

fn column_exists(conn: &Connection, table: &str, column: &str) -> Result<bool> {
    if !table_exists(conn, table)? {
        return Ok(false);
    }
    let sql = format!("PRAGMA table_info({table})");
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        if name == column {
            return Ok(true);
        }
    }
    Ok(false)
}

fn read_site_default_locale(conn: &Connection) -> String {
    conn.query_row(
        "SELECT value FROM dict_values
         WHERE code = 'site_default_locale' AND lang = ''
         LIMIT 1",
        [],
        |row| row.get::<_, String>(0),
    )
    .ok()
    .map(|s| s.trim().to_lowercase())
    .filter(|s| !s.is_empty())
    .unwrap_or_else(|| crate::models::locale::DEFAULT_LOCALE.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn legacy_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("memory db");
        conn.execute_batch(
            "CREATE TABLE post_metas (
                id INTEGER PRIMARY KEY,
                category_id INTEGER NOT NULL DEFAULT 0,
                tags TEXT NOT NULL DEFAULT '',
                status INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE post_i18ns (
                post_id INTEGER NOT NULL,
                lang TEXT NOT NULL,
                title TEXT NOT NULL DEFAULT '',
                description TEXT NOT NULL DEFAULT '',
                content TEXT NOT NULL DEFAULT '',
                route_path TEXT NOT NULL DEFAULT '',
                PRIMARY KEY (post_id, lang)
            );
            CREATE TABLE dict_values (code TEXT, lang TEXT, value TEXT);
            INSERT INTO dict_values VALUES ('site_default_locale', '', 'zh-cn');
            INSERT INTO post_metas (id, category_id, tags, status) VALUES (1, 0, 'rust, cms', 1);
            INSERT INTO post_i18ns (post_id, lang, title) VALUES (1, 'zh-cn', 't');",
        )
        .unwrap();
        conn
    }

    #[test]
    fn migrates_tags_and_drops_meta_column() {
        let conn = legacy_conn();
        ensure_post_i18n_tags_column(&conn).unwrap();
        migrate_post_tags_to_i18n(&conn).unwrap();

        assert!(!column_exists(&conn, "post_metas", "tags").unwrap());
        let tags: String = conn
            .query_row(
                "SELECT tags FROM post_i18ns WHERE post_id = 1 AND lang = 'zh-cn'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(tags, "rust, cms");
    }

    #[test]
    fn empty_tags_allowed_on_i18n() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE post_metas (
                id INTEGER PRIMARY KEY,
                category_id INTEGER NOT NULL DEFAULT 0,
                status INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE post_i18ns (
                post_id INTEGER NOT NULL,
                lang TEXT NOT NULL,
                title TEXT NOT NULL DEFAULT '',
                description TEXT NOT NULL DEFAULT '',
                content TEXT NOT NULL DEFAULT '',
                route_path TEXT NOT NULL DEFAULT '',
                tags TEXT NOT NULL DEFAULT '',
                PRIMARY KEY (post_id, lang)
            );",
        )
        .unwrap();
        conn.execute_batch(
            "INSERT INTO post_metas (id, status) VALUES (1, 1);
             INSERT INTO post_i18ns (post_id, lang, title, tags) VALUES (1, 'zh-cn', 't', '');",
        )
        .unwrap();
        let tags: String = conn
            .query_row(
                "SELECT tags FROM post_i18ns WHERE post_id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(tags, "");
    }
}
