pub mod assets;
pub mod auth;
pub mod banner_groups;
pub mod banners;
pub mod categories;
pub mod dicts;
pub mod menu_groups;
pub mod menus;
pub mod posts;
pub mod roles;
pub mod users;

use rocket::Route;

/// 汇总管理后台路由（/api/admin/*，需要 token 授权）
pub fn routes() -> Vec<Route> {
    routes![
        auth::login,
        auth::me,
        posts::create,
        posts::update,
        posts::delete,
        posts::list,
        posts::get,
        categories::list,
        categories::get,
        categories::create,
        categories::update,
        categories::delete,
        users::list,
        users::get,
        users::create,
        users::update,
        users::delete,
        users::assign_roles,
        roles::list,
        roles::get,
        roles::create,
        roles::update,
        roles::delete,
        roles::permissions_list,
        menu_groups::list,
        menu_groups::get,
        menu_groups::create,
        menu_groups::update,
        menu_groups::delete,
        menus::overview,
        menus::list_by_group,
        menus::nav,
        menus::get_item,
        menus::create,
        menus::update,
        menus::delete,
        banner_groups::list,
        banner_groups::get,
        banner_groups::create,
        banner_groups::update,
        banner_groups::delete,
        banners::list,
        banners::get,
        banners::create,
        banners::update,
        banners::delete,
        dicts::list,
        dicts::get,
        dicts::create,
        dicts::update,
        dicts::update_values,
        dicts::delete,
        assets::upload,
        assets::list,
        assets::get,
        assets::delete,
        assets::list_post_assets,
        assets::link_post,
        assets::unlink_post,
        assets::list_banner_assets,
        assets::link_banner,
        assets::unlink_banner,
        assets::set_banner_image_enabled_route,
    ]
}
