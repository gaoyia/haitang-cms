pub mod banners;
pub mod categories;
pub mod dicts;
pub mod friend_links;
pub mod posts;

use rocket::Route;

/// 汇总公开 API 路由
pub fn routes() -> Vec<Route> {
    routes![
        posts::list,
        posts::get,
        categories::list,
        categories::get,
        categories::get_by_path,
        banners::list_by_code,
        friend_links::list,
        dicts::list,
        dicts::map,
        dicts::get_by_code,
    ]
}
