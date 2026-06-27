use rocket::serde::json::Json;
use rocket::State;

use crate::guards::AdminAuth;
use crate::models::{
    group_has_menus, is_admin_sidebar_code, validate_menu_group_code, ApiResponse, CreateMenuGroup,
    MenuGroup, MenuGroupView, UpdateMenuGroup,
};
use crate::models::{admin_sidebar_group_view, MenuItemMeta};

/// 获取所有菜单组（含只读后台侧边栏）
#[get("/api/admin/menu-groups")]
pub async fn list(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
) -> Json<ApiResponse<Vec<MenuGroupView>>> {
    let mut db = db.inner().clone();

    let mut views = vec![admin_sidebar_group_view()];

    match MenuGroup::all().exec(&mut db).await {
        Ok(mut groups) => {
            groups.sort_by_key(|g| g.sort);
            views.extend(groups.into_iter().map(|g| g.to_view()));
            Json(ApiResponse::success(views))
        }
        Err(e) => Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    }
}

/// 获取单个菜单组
#[get("/api/admin/menu-groups/<id>")]
pub async fn get(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<MenuGroupView>> {
    if id == 0 {
        return Json(ApiResponse::success(admin_sidebar_group_view()));
    }

    let mut db = db.inner().clone();

    match MenuGroup::get_by_id(&mut db, &id).await {
        Ok(group) => Json(ApiResponse::success(group.to_view())),
        Err(_) => Json(ApiResponse::error(404, "菜单组不存在")),
    }
}

/// 创建菜单组
#[post("/api/admin/menu-groups", data = "<input>")]
pub async fn create(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    input: Json<CreateMenuGroup>,
) -> Json<ApiResponse<MenuGroupView>> {
    if let Err(msg) = validate_menu_group_code(&input.code) {
        return Json(ApiResponse::error(400, msg));
    }

    let mut db = db.inner().clone();
    let description = input.description.as_deref().unwrap_or("");
    let sort = input.sort.unwrap_or(0);
    let status = input.status.unwrap_or(1);

    match MenuGroup::create()
        .name(&input.name)
        .code(&input.code)
        .description(description)
        .sort(sort)
        .status(status)
        .exec(&mut db)
        .await
    {
        Ok(group) => Json(ApiResponse::success(group.to_view())),
        Err(e) => Json(ApiResponse::error(500, format!("创建失败: {e}"))),
    }
}

/// 更新菜单组
#[put("/api/admin/menu-groups/<id>", data = "<input>")]
pub async fn update(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<UpdateMenuGroup>,
) -> Json<ApiResponse<MenuGroupView>> {
    if id == 0 {
        return Json(ApiResponse::error(403, "后台侧边栏为系统内置，不可修改"));
    }

    if let Some(ref code) = input.code {
        if let Err(msg) = validate_menu_group_code(code) {
            return Json(ApiResponse::error(400, msg));
        }
    }

    let mut db = db.inner().clone();

    let mut group = match MenuGroup::get_by_id(&mut db, &id).await {
        Ok(g) => g,
        Err(_) => return Json(ApiResponse::error(404, "菜单组不存在")),
    };

    if is_admin_sidebar_code(&group.code) {
        return Json(ApiResponse::error(403, "后台侧边栏为系统内置，不可修改"));
    }

    let mut builder = group.update();
    if let Some(ref name) = input.name {
        builder = builder.name(name.as_str());
    }
    if let Some(ref code) = input.code {
        builder = builder.code(code.as_str());
    }
    if let Some(ref description) = input.description {
        builder = builder.description(description.as_str());
    }
    if let Some(sort) = input.sort {
        builder = builder.sort(sort);
    }
    if let Some(status) = input.status {
        builder = builder.status(status);
    }

    match builder.exec(&mut db).await {
        Ok(_) => match MenuGroup::get_by_id(&mut db, &id).await {
            Ok(updated) => Json(ApiResponse::success(updated.to_view())),
            Err(e) => Json(ApiResponse::error(500, format!("更新成功但查询失败: {e}"))),
        },
        Err(e) => Json(ApiResponse::error(500, format!("更新失败: {e}"))),
    }
}

/// 删除菜单组
#[delete("/api/admin/menu-groups/<id>")]
pub async fn delete(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<()>> {
    if id == 0 {
        return Json(ApiResponse::error(403, "后台侧边栏为系统内置，不可删除"));
    }

    let mut db = db.inner().clone();

    let group = match MenuGroup::get_by_id(&mut db, &id).await {
        Ok(g) => g,
        Err(_) => return Json(ApiResponse::error(404, "菜单组不存在")),
    };

    if is_admin_sidebar_code(&group.code) {
        return Json(ApiResponse::error(403, "后台侧边栏为系统内置，不可删除"));
    }

    let menus = match MenuItemMeta::all().exec(&mut db).await {
        Ok(m) => m,
        Err(e) => return Json(ApiResponse::error(500, format!("查询菜单失败: {e}"))),
    };

    if group_has_menus(&menus, id) {
        return Json(ApiResponse::error(400, "菜单组下仍有菜单，无法删除"));
    }

    match group.delete().exec(&mut db).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "删除成功".to_string(),
            data: None,
        }),
        Err(e) => Json(ApiResponse::error(500, format!("删除失败: {e}"))),
    }
}
