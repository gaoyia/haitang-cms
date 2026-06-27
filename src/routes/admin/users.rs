use rocket::State;
use rocket::serde::json::Json;

use crate::guards::AdminAuth;
use crate::models::{
    ApiResponse, AssignRoles, CreateUser, PageResult, UpdateUser, User, UserRole, UserView,
    paginate_vec,
};
use crate::routes::page::PageQuery;

use super::auth::{get_user_role_ids, hash_password};

/// 获取所有用户列表
#[get("/api/admin/users?<page..>")]
pub async fn list(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    page: PageQuery,
) -> Json<ApiResponse<PageResult<UserView>>> {
    let mut db = db.inner().clone();

    let users = match User::all().exec(&mut db).await {
        Ok(u) => u,
        Err(e) => return Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    };

    let mut views = Vec::new();
    for user in users {
        let role_ids = get_user_role_ids(&mut db, user.id).await;
        views.push(user.to_view(role_ids));
    }

    let (p, ps) = page.resolve();
    Json(ApiResponse::success(paginate_vec(views, p, ps)))
}

/// 获取单个用户
#[get("/api/admin/users/<id>")]
pub async fn get(_auth: AdminAuth, db: &State<toasty::Db>, id: i64) -> Json<ApiResponse<UserView>> {
    let mut db = db.inner().clone();

    let user = match User::get_by_id(&mut db, &id).await {
        Ok(u) => u,
        Err(_) => return Json(ApiResponse::error(404, "用户不存在")),
    };

    let role_ids = get_user_role_ids(&mut db, user.id).await;
    Json(ApiResponse::success(user.to_view(role_ids)))
}

/// 创建新用户
#[post("/api/admin/users", data = "<input>")]
pub async fn create(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    input: Json<CreateUser>,
) -> Json<ApiResponse<UserView>> {
    let mut db = db.inner().clone();

    let pw_hash = hash_password(&input.password);
    let nickname = input.nickname.as_deref().unwrap_or(&input.username);
    let email = input.email.as_deref().unwrap_or("");

    match User::create()
        .username(&input.username)
        .password_hash(&pw_hash)
        .nickname(nickname)
        .email(email)
        .status(1i64)
        .exec(&mut db)
        .await
    {
        Ok(user) => Json(ApiResponse::success(user.to_view(Vec::new()))),
        Err(e) => Json(ApiResponse::error(500, format!("创建失败: {e}"))),
    }
}

/// 更新用户
#[put("/api/admin/users/<id>", data = "<input>")]
pub async fn update(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<UpdateUser>,
) -> Json<ApiResponse<UserView>> {
    let mut db = db.inner().clone();

    let mut user = match User::get_by_id(&mut db, &id).await {
        Ok(u) => u,
        Err(_) => return Json(ApiResponse::error(404, "用户不存在")),
    };

    let mut builder = user.update();
    if let Some(ref nickname) = input.nickname {
        builder = builder.nickname(nickname.as_str());
    }
    if let Some(ref email) = input.email {
        builder = builder.email(email.as_str());
    }
    if let Some(status) = input.status {
        builder = builder.status(status);
    }
    if let Some(ref password) = input.password {
        let pw_hash = hash_password(password);
        builder = builder.password_hash(&pw_hash);
    }

    match builder.exec(&mut db).await {
        Ok(_) => match User::get_by_id(&mut db, &id).await {
            Ok(updated) => {
                let role_ids = get_user_role_ids(&mut db, updated.id).await;
                Json(ApiResponse::success(updated.to_view(role_ids)))
            }
            Err(e) => Json(ApiResponse::error(500, format!("更新成功但查询失败: {e}"))),
        },
        Err(e) => Json(ApiResponse::error(500, format!("更新失败: {e}"))),
    }
}

/// 删除用户
#[delete("/api/admin/users/<id>")]
pub async fn delete(_auth: AdminAuth, db: &State<toasty::Db>, id: i64) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    let user = match User::get_by_id(&mut db, &id).await {
        Ok(u) => u,
        Err(_) => return Json(ApiResponse::error(404, "用户不存在")),
    };

    // 删除用户角色关联
    if let Ok(links) = UserRole::all().exec(&mut db).await {
        for link in links {
            if link.user_id == id {
                let _ = link.delete().exec(&mut db).await;
            }
        }
    }

    match user.delete().exec(&mut db).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "删除成功".to_string(),
            data: None,
        }),
        Err(e) => Json(ApiResponse::error(500, format!("删除失败: {e}"))),
    }
}

/// 分配用户角色
#[put("/api/admin/users/<id>/roles", data = "<input>")]
pub async fn assign_roles(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<AssignRoles>,
) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    // 确认用户存在
    if User::get_by_id(&mut db, &id).await.is_err() {
        return Json(ApiResponse::error(404, "用户不存在"));
    }

    // 删除旧的角色关联
    if let Ok(links) = UserRole::all().exec(&mut db).await {
        for link in links {
            if link.user_id == id {
                let _ = link.delete().exec(&mut db).await;
            }
        }
    }

    // 创建新的角色关联
    for &role_id in &input.role_ids {
        if let Err(e) = UserRole::create()
            .user_id(id)
            .role_id(role_id)
            .exec(&mut db)
            .await
        {
            return Json(ApiResponse::error(500, format!("分配角色失败: {e}")));
        }
    }

    Json(ApiResponse {
        code: 0,
        message: "角色分配成功".to_string(),
        data: None,
    })
}
