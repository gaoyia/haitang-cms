use rocket::serde::json::Json;
use rocket::State;

use crate::guards::AdminAuth;
use crate::models::{paginate_vec, ApiResponse, CreateRole, PageResult, Role, RoleView, UpdateRole, ALL_PERMISSIONS};
use crate::routes::page::PageQuery;

/// 获取所有角色列表
#[get("/api/admin/roles?<page..>")]
pub async fn list(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    page: PageQuery,
) -> Json<ApiResponse<PageResult<RoleView>>> {
    let mut db = db.inner().clone();

    match Role::all().exec(&mut db).await {
        Ok(roles) => {
            let views: Vec<RoleView> = roles.into_iter().map(|r| r.to_view()).collect();
            let (p, ps) = page.resolve();
            Json(ApiResponse::success(paginate_vec(views, p, ps)))
        }
        Err(e) => Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    }
}

/// 获取单个角色
#[get("/api/admin/roles/<id>")]
pub async fn get(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<RoleView>> {
    let mut db = db.inner().clone();

    match Role::get_by_id(&mut db, &id).await {
        Ok(role) => Json(ApiResponse::success(role.to_view())),
        Err(_) => Json(ApiResponse::error(404, "角色不存在")),
    }
}

/// 创建新角色
#[post("/api/admin/roles", data = "<input>")]
pub async fn create(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    input: Json<CreateRole>,
) -> Json<ApiResponse<RoleView>> {
    let mut db = db.inner().clone();

    let description = input.description.as_deref().unwrap_or("");
    let permissions = input
        .permissions
        .as_ref()
        .map(|p| p.join(","))
        .unwrap_or_default();

    match Role::create()
        .name(&input.name)
        .description(description)
        .permissions(&permissions)
        .exec(&mut db)
        .await
    {
        Ok(role) => Json(ApiResponse::success(role.to_view())),
        Err(e) => Json(ApiResponse::error(500, format!("创建失败: {e}"))),
    }
}

/// 更新角色
#[put("/api/admin/roles/<id>", data = "<input>")]
pub async fn update(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<UpdateRole>,
) -> Json<ApiResponse<RoleView>> {
    let mut db = db.inner().clone();

    let mut role = match Role::get_by_id(&mut db, &id).await {
        Ok(r) => r,
        Err(_) => return Json(ApiResponse::error(404, "角色不存在")),
    };

    let mut builder = role.update();
    if let Some(ref name) = input.name {
        builder = builder.name(name.as_str());
    }
    if let Some(ref description) = input.description {
        builder = builder.description(description.as_str());
    }
    if let Some(ref permissions) = input.permissions {
        builder = builder.permissions(&permissions.join(","));
    }

    match builder.exec(&mut db).await {
        Ok(_) => match Role::get_by_id(&mut db, &id).await {
            Ok(updated) => Json(ApiResponse::success(updated.to_view())),
            Err(e) => Json(ApiResponse::error(500, format!("更新成功但查询失败: {e}"))),
        },
        Err(e) => Json(ApiResponse::error(500, format!("更新失败: {e}"))),
    }
}

/// 删除角色
#[delete("/api/admin/roles/<id>")]
pub async fn delete(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    let role = match Role::get_by_id(&mut db, &id).await {
        Ok(r) => r,
        Err(_) => return Json(ApiResponse::error(404, "角色不存在")),
    };

    match role.delete().exec(&mut db).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "删除成功".to_string(),
            data: None,
        }),
        Err(e) => Json(ApiResponse::error(500, format!("删除失败: {e}"))),
    }
}

/// 获取系统所有可用权限列表
#[get("/api/admin/permissions")]
pub async fn permissions_list(_auth: AdminAuth) -> Json<ApiResponse<serde_json::Value>> {
    // 按分组返回权限列表
    let mut groups: Vec<serde_json::Value> = Vec::new();
    let mut current_group = "";
    let mut current_perms: Vec<serde_json::Value> = Vec::new();

    for perm in ALL_PERMISSIONS {
        if perm.group != current_group {
            if !current_group.is_empty() {
                groups.push(serde_json::json!({
                    "group": current_group,
                    "permissions": current_perms,
                }));
            }
            current_group = perm.group;
            current_perms = Vec::new();
        }
        current_perms.push(serde_json::json!({
            "code": perm.code,
            "label": perm.label,
        }));
    }
    if !current_group.is_empty() {
        groups.push(serde_json::json!({
            "group": current_group,
            "permissions": current_perms,
        }));
    }

    Json(ApiResponse::success(serde_json::json!(groups)))
}
