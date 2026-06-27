use rocket::Route;
use rocket::State;
use rocket::fs::NamedFile;
use std::path::PathBuf;

use crate::config::AdminWebConfig;

/// 管理后台 SPA 路由（挂载在 `/{ADMIN_WEB_PATH}`）
pub fn spa_routes() -> Vec<Route> {
    routes![admin_spa_index, admin_spa]
}

#[get("/")]
pub async fn admin_spa_index(cfg: &State<AdminWebConfig>) -> Option<NamedFile> {
    serve_admin_spa(PathBuf::new(), cfg).await
}

#[get("/<path..>")]
pub async fn admin_spa(path: PathBuf, cfg: &State<AdminWebConfig>) -> Option<NamedFile> {
    serve_admin_spa(path, cfg).await
}

async fn serve_admin_spa(path: PathBuf, cfg: &AdminWebConfig) -> Option<NamedFile> {
    let base = &cfg.static_dir;
    if !base.join("index.html").is_file() {
        return None;
    }

    if path.as_os_str().is_empty() {
        return NamedFile::open(base.join("index.html")).await.ok();
    }

    let candidate = base.join(&path);
    if candidate.is_file() {
        return NamedFile::open(candidate).await.ok();
    }

    // history 模式：非静态文件路径回退 index.html
    NamedFile::open(base.join("index.html")).await.ok()
}
