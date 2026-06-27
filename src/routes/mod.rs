pub mod admin;
pub mod admin_web;
pub mod api;
pub mod lang;
pub mod page;
pub mod pages;

use rocket::Route;

/// 汇总所有路由
pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(pages::routes());
    routes.extend(api::routes());
    routes.extend(admin::routes());
    routes
}
