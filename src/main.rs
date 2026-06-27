#[macro_use]
extern crate rocket;

mod config;
mod db_patch;
mod guards;
mod models;
mod routes;
mod storage;

use rocket::fs::{self, FileServer};
use rocket_dyn_templates::Template;
use std::path::Path;

use config::{AdminWebConfig, StorageConfig};
use guards::auth::JwtConfig;
use routes::admin::auth::{seed_admin, seed_default_banner_data};
use storage::StorageService;

#[launch]
async fn rocket() -> _ {
    // 确保 db 目录存在
    let db_dir = Path::new("db");
    if !db_dir.exists() {
        std::fs::create_dir_all(db_dir).expect("无法创建 db 目录");
    }

    // 初始化 Toasty 数据库（SQLite）
    // Asset / PostAsset 须显式注册：仅 models!(crate::*) 时 inventory 可能未链接到 schema
    let db = toasty::Db::builder()
        .models(toasty::models!(
            crate::*,
            crate::models::Asset,
            crate::models::PostAsset,
            crate::models::BannerAsset,
        ))
        .connect("sqlite:db/haitang.sqlite")
        .await
        .expect("数据库连接失败");

    // 自动建表（表已存在时静默跳过）
    if let Err(e) = db.push_schema().await {
        let msg = e.to_string();
        if !msg.contains("already exists") {
            eprintln!("[错误] 建表失败: {e}");
        }
    }

    // 为已有库补全新增列（push_schema 不会 ALTER 已有表）
    {
        let mut db_mut = db.clone();
        db_patch::apply_schema_patches(&mut db_mut).await;
    }

    let admin_cfg = AdminWebConfig::from_env();
    let storage_cfg = StorageConfig::from_env();
    let storage = StorageService::from_config(storage_cfg.clone()).unwrap_or_else(|e| {
        panic!("[错误] 存储配置无效: {e}");
    });
    if let Err(e) = storage.ensure_local_dir() {
        eprintln!("[错误] 创建上传目录失败: {e}");
    } else if storage_cfg.backend == "local" {
        println!(
            "[资源存储] 本地模式：{} → {}",
            storage_cfg.local_dir.display(),
            storage_cfg.public_url_prefix
        );
    }

    // 种子数据：默认管理员、菜单、轮播图与资源等
    {
        let mut db_mut = db.clone();
        seed_admin(&mut db_mut).await;
        seed_default_banner_data(&mut db_mut, &storage).await;
    }

    // JWT 密钥配置
    let jwt_config = JwtConfig {
        secret: std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "haitang-cms-dev-secret".to_string()),
    };

    if admin_cfg.static_dir.join("index.html").is_file() {
        println!(
            "[管理后台] 静态 SPA：/{}/ → {}",
            admin_cfg.path_segment,
            admin_cfg.static_dir.display()
        );
    } else {
        eprintln!(
            "[管理后台] 未找到 {}，请先执行 admin-web 生产构建（pnpm build）",
            admin_cfg.static_dir.join("index.html").display()
        );
    }

    rocket::build()
        .manage(db)
        .manage(jwt_config)
        .manage(admin_cfg.clone())
        .manage(storage)
        // 挂载静态资源（jQuery 等）
        .mount("/static", FileServer::new(fs::relative!("static")))
        // 管理后台 SPA（history 模式回退 index.html）
        .mount(
            admin_cfg.mount_path.as_str(),
            routes::admin_web::spa_routes(),
        )
        // 挂载所有路由
        .mount("/", routes::routes())
        // 附加模板引擎
        .attach(Template::fairing())
}
