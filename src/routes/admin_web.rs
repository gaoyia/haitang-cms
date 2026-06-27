use rocket::Route;
use rocket::State;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::{self, Redirect, Responder};
use std::convert::Infallible;
use std::path::PathBuf;

use crate::config::{AdminWebConfig, AppConfig};

/// 原始 query 字符串（不含 `?`）
struct RawQuery(Option<String>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RawQuery {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RawQuery(
            req.uri()
                .query()
                .as_ref()
                .map(|q| q.as_str().to_string()),
        ))
    }
}

/// 管理后台 SPA 路由（挂载在 `/{ADMIN_WEB_PATH}`）
pub fn spa_routes() -> Vec<Route> {
    routes![admin_spa_index, admin_spa]
}

/// 开发重定向 / 生产静态文件 / 404
enum AdminSpaResponse {
    Redirect(Redirect),
    File(NamedFile),
    NotFound,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for AdminSpaResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self {
            Self::Redirect(r) => r.respond_to(req),
            Self::File(f) => f.respond_to(req),
            Self::NotFound => Status::NotFound.respond_to(req),
        }
    }
}

#[get("/")]
async fn admin_spa_index(
    cfg: &State<AdminWebConfig>,
    app: &State<AppConfig>,
    query: RawQuery,
) -> AdminSpaResponse {
    resolve_admin_spa(PathBuf::new(), cfg.inner(), app.inner(), query.0).await
}

#[get("/<path..>")]
async fn admin_spa(
    path: PathBuf,
    cfg: &State<AdminWebConfig>,
    app: &State<AppConfig>,
    query: RawQuery,
) -> AdminSpaResponse {
    resolve_admin_spa(path, cfg.inner(), app.inner(), query.0).await
}

async fn resolve_admin_spa(
    path: PathBuf,
    cfg: &AdminWebConfig,
    app: &AppConfig,
    query: Option<String>,
) -> AdminSpaResponse {
    // 开发：无静态构建时由 Rocket 302 到 Vite dev server；生产不走此分支
    if app.is_development() {
        let url = cfg.dev_redirect_url(&path, query.as_deref());
        return AdminSpaResponse::Redirect(Redirect::to(url));
    }

    match serve_admin_spa_static(path, cfg).await {
        Some(file) => AdminSpaResponse::File(file),
        None => AdminSpaResponse::NotFound,
    }
}

/// 生产环境：托管 `static/haitang-cms-admin` 构建产物
async fn serve_admin_spa_static(path: PathBuf, cfg: &AdminWebConfig) -> Option<NamedFile> {
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
