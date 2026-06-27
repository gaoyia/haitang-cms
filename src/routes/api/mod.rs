pub mod banners;
pub mod dicts;
pub mod posts;

use rocket::Route;

/// 汇总公开 API 路由
pub fn routes() -> Vec<Route> {
    routes![
        posts::list,
        posts::get,
        posts::list_categories,
        banners::list_by_code,
        dicts::list,
        dicts::map,
        dicts::get_by_code,
    ]
}
