use rocket::State;
use rocket::serde::json::Json;

use crate::guards::AdminAuth;
use crate::models::{
    ADMIN_SIDEBAR_CODE, ApiResponse, CreateMenuItem, MenuGroup, MenuGroupTreeView, MenuItemMeta,
    MenuView, UpdateMenuItem, all_menu_group_trees, create_menu_item, delete_menu_item,
    get_admin_sidebar_item, get_admin_sidebar_nav_items, get_admin_sidebar_tree, get_db_menu_tree,
    get_site_default_locale, menu_has_children, merged_menu_item, upsert_menu_i18n,
    validate_parent_id,
};
use crate::routes::admin::auth::{get_roles_info, get_user_role_ids};
use crate::routes::lang::LangQuery;

/// 获取所有菜单组及其菜单树（菜单管理总览）
#[get("/api/admin/menus/overview?<lang..>")]
pub async fn overview(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    lang: LangQuery,
) -> Json<ApiResponse<Vec<MenuGroupTreeView>>> {
    let mut db = db.inner().clone();

    match all_menu_group_trees(&mut db, lang.lang.as_deref()).await {
        Ok(trees) => Json(ApiResponse::success(trees)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 获取指定菜单组的菜单树
#[get("/api/admin/menus?<group_id>&<lang..>")]
pub async fn list_by_group(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    group_id: i64,
    lang: LangQuery,
) -> Json<ApiResponse<Vec<MenuView>>> {
    if group_id == 0 {
        return Json(ApiResponse::success(get_admin_sidebar_tree()));
    }

    let mut db = db.inner().clone();

    match get_db_menu_tree(&mut db, group_id, lang.lang.as_deref()).await {
        Ok(tree) => Json(ApiResponse::success(tree)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 获取当前用户可见的导航菜单（admin-web 动态侧栏）
#[get("/api/admin/nav?<code>")]
pub async fn nav(
    auth: AdminAuth,
    db: &State<toasty::Db>,
    code: &str,
) -> Json<ApiResponse<Vec<crate::models::AdminNavMenuJsonItem>>> {
    if code == ADMIN_SIDEBAR_CODE {
        let mut db = db.inner().clone();
        let role_ids = get_user_role_ids(&mut db, auth.claims.user_id).await;
        let (_, permissions) = get_roles_info(&mut db, &role_ids).await;
        let items = get_admin_sidebar_nav_items(&permissions);
        return Json(ApiResponse::success(items));
    }

    Json(ApiResponse::error(404, "该菜单组不支持导航接口"))
}

/// 获取单个菜单
#[get("/api/admin/menus/item/<id>?<lang..>")]
pub async fn get_item(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    lang: LangQuery,
) -> Json<ApiResponse<MenuView>> {
    if let Some(item) = get_admin_sidebar_item(id) {
        let _ = lang;
        return Json(ApiResponse::success(item));
    }

    let mut db = db.inner().clone();

    match MenuItemMeta::get_by_id(&mut db, &id).await {
        Ok(meta) => match merged_menu_item(&mut db, &meta, lang.lang.as_deref()).await {
            Ok(item) => Json(ApiResponse::success(item.to_flat_view())),
            Err(e) => Json(ApiResponse::error(500, e)),
        },
        Err(_) => Json(ApiResponse::error(404, "菜单不存在")),
    }
}

/// 创建菜单
#[post("/api/admin/menus", data = "<input>")]
pub async fn create(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    input: Json<CreateMenuItem>,
) -> Json<ApiResponse<MenuView>> {
    let mut db = db.inner().clone();

    if MenuGroup::get_by_id(&mut db, &input.group_id)
        .await
        .is_err()
    {
        return Json(ApiResponse::error(404, "菜单组不存在"));
    }

    let default = get_site_default_locale(&mut db).await;

    match create_menu_item(&mut db, &input, &default).await {
        Ok(meta) => match merged_menu_item(&mut db, &meta, input.lang.as_deref()).await {
            Ok(item) => Json(ApiResponse::success(item.to_flat_view())),
            Err(e) => Json(ApiResponse::error(500, e)),
        },
        Err(e) => Json(ApiResponse::error(400, e)),
    }
}

/// 更新菜单
#[put("/api/admin/menus/<id>", data = "<input>")]
pub async fn update(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<UpdateMenuItem>,
) -> Json<ApiResponse<MenuView>> {
    let mut db = db.inner().clone();
    let default = get_site_default_locale(&mut db).await;

    let mut meta = match MenuItemMeta::get_by_id(&mut db, &id).await {
        Ok(m) => m,
        Err(_) => return Json(ApiResponse::error(404, "菜单不存在")),
    };

    let group_id = input.group_id.unwrap_or(meta.group_id);
    let parent_id = input.parent_id.unwrap_or(meta.parent_id);

    let all_menus = match MenuItemMeta::all().exec(&mut db).await {
        Ok(m) => m,
        Err(e) => return Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    };

    if let Err(msg) = validate_parent_id(&all_menus, Some(id), group_id, parent_id) {
        return Json(ApiResponse::error(400, msg));
    }

    let mut builder = meta.update();
    if let Some(gid) = input.group_id {
        builder = builder.group_id(gid);
    }
    if let Some(pid) = input.parent_id {
        builder = builder.parent_id(pid);
    }
    if let Some(ref icon) = input.icon {
        builder = builder.icon(icon.as_str());
    }
    if let Some(ref permission) = input.permission {
        builder = builder.permission(permission.as_str());
    }
    if let Some(sort) = input.sort {
        builder = builder.sort(sort);
    }
    if let Some(status) = input.status {
        builder = builder.status(status);
    }

    if let Err(e) = builder.exec(&mut db).await {
        return Json(ApiResponse::error(500, format!("更新失败: {e}")));
    }

    meta = MenuItemMeta::get_by_id(&mut db, &id)
        .await
        .expect("菜单应存在");

    if input.title.is_some() || input.path.is_some() {
        let lang = crate::models::locale::resolve_locale(input.lang.as_deref(), &default);
        let title = input.title.as_deref().unwrap_or("");
        let path = input.path.as_deref().unwrap_or("");
        if let Err(e) = upsert_menu_i18n(&mut db, id, &lang, title, path).await {
            return Json(ApiResponse::error(500, e));
        }
    }

    match merged_menu_item(&mut db, &meta, input.lang.as_deref()).await {
        Ok(item) => Json(ApiResponse::success(item.to_flat_view())),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 删除菜单
#[delete("/api/admin/menus/<id>")]
pub async fn delete(_auth: AdminAuth, db: &State<toasty::Db>, id: i64) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    let all_menus = match MenuItemMeta::all().exec(&mut db).await {
        Ok(m) => m,
        Err(e) => return Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    };

    if menu_has_children(&all_menus, id) {
        return Json(ApiResponse::error(400, "该菜单下仍有子菜单，无法删除"));
    }

    match delete_menu_item(&mut db, id).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "删除成功".to_string(),
            data: None,
        }),
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}
