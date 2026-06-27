#[macro_use]
extern crate rocket;

mod config;
mod db_migrate;
mod guards;
mod models;
mod routes;

use rocket::fs::{self, FileServer};
use rocket_dyn_templates::Template;
use std::path::Path;

use config::AdminWebConfig;
use guards::auth::JwtConfig;
use routes::admin::auth::seed_admin;

#[launch]
async fn rocket() -> _ {
    // 确保 db 目录存在
    let db_dir = Path::new("db");
    if !db_dir.exists() {
        std::fs::create_dir_all(db_dir).expect("无法创建 db 目录");
    }

    // 初始化 Toasty 数据库（SQLite）
    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
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

    if let Err(e) = db_migrate::run() {
        eprintln!("[错误] 数据库迁移失败: {e}");
    }

    // 种子数据：确保默认管理员账户存在
    {
        let mut db_mut = db.clone();
        seed_admin(&mut db_mut).await;
    }

    // JWT 密钥配置
    let jwt_config = JwtConfig {
        secret: std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "haitang-cms-dev-secret".to_string()),
    };

    let admin_cfg = AdminWebConfig::from_env();
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
