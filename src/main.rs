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

use config::{AdminWebConfig, AppConfig, StorageConfig};
use guards::auth::JwtConfig;
use routes::admin::auth::run_startup_seeds;
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

    let app_cfg = AppConfig::from_env();
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

    if app_cfg.is_development() {
        println!("[环境] development（HAITANG_ENV 未设时默认）");
    } else {
        println!("[环境] production");
    }

    // 种子数据：development 幂等补全；production 仅首次安装（无用户时）
    {
        let mut db_mut = db.clone();
        run_startup_seeds(&mut db_mut, &storage, &app_cfg).await;
    }

    // JWT 密钥配置
    let jwt_config = JwtConfig {
        secret: std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "haitang-cms-dev-secret".to_string()),
    };

    if app_cfg.is_development() {
        println!(
            "[管理后台] 开发模式：/{}/ → {}（Vite dev，无静态构建亦可访问）",
            admin_cfg.path_segment, admin_cfg.dev_server_url
        );
    } else if admin_cfg.static_dir.join("index.html").is_file() {
        println!(
            "[管理后台] 生产静态：/{}/ → {}",
            admin_cfg.path_segment,
            admin_cfg.static_dir.display()
        );
    } else {
        eprintln!(
            "[管理后台] 生产环境未找到 {}，请执行 admin-web 生产构建（pnpm build）",
            admin_cfg.static_dir.join("index.html").display()
        );
    }

    rocket::build()
        .manage(db)
        .manage(jwt_config)
        .manage(app_cfg)
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
