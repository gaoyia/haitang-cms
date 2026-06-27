use rocket::serde::json::Json;
use rocket::State;

use crate::guards::AdminAuth;
use crate::models::{
    group_has_banners, ApiResponse, Banner, BannerGroup, BannerGroupView, CreateBannerGroup,
    UpdateBannerGroup,
};

/// 获取所有轮播图组
#[get("/api/admin/banner-groups")]
pub async fn list(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
) -> Json<ApiResponse<Vec<BannerGroupView>>> {
    let mut db = db.inner().clone();

    match BannerGroup::all().exec(&mut db).await {
        Ok(mut groups) => {
            groups.sort_by_key(|g| g.sort);
            let views: Vec<BannerGroupView> = groups.into_iter().map(|g| g.to_view()).collect();
            Json(ApiResponse::success(views))
        }
        Err(e) => Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    }
}

/// 获取单个轮播图组
#[get("/api/admin/banner-groups/<id>")]
pub async fn get(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<BannerGroupView>> {
    let mut db = db.inner().clone();

    match BannerGroup::get_by_id(&mut db, &id).await {
        Ok(group) => Json(ApiResponse::success(group.to_view())),
        Err(_) => Json(ApiResponse::error(404, "轮播图组不存在")),
    }
}

/// 创建轮播图组
#[post("/api/admin/banner-groups", data = "<input>")]
pub async fn create(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    input: Json<CreateBannerGroup>,
) -> Json<ApiResponse<BannerGroupView>> {
    let mut db = db.inner().clone();
    let description = input.description.as_deref().unwrap_or("");
    let sort = input.sort.unwrap_or(0);
    let status = input.status.unwrap_or(1);

    match BannerGroup::create()
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

/// 更新轮播图组
#[put("/api/admin/banner-groups/<id>", data = "<input>")]
pub async fn update(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<UpdateBannerGroup>,
) -> Json<ApiResponse<BannerGroupView>> {
    let mut db = db.inner().clone();

    let mut group = match BannerGroup::get_by_id(&mut db, &id).await {
        Ok(g) => g,
        Err(_) => return Json(ApiResponse::error(404, "轮播图组不存在")),
    };

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
        Ok(_) => match BannerGroup::get_by_id(&mut db, &id).await {
            Ok(updated) => Json(ApiResponse::success(updated.to_view())),
            Err(e) => Json(ApiResponse::error(500, format!("更新成功但查询失败: {e}"))),
        },
        Err(e) => Json(ApiResponse::error(500, format!("更新失败: {e}"))),
    }
}

/// 删除轮播图组
#[delete("/api/admin/banner-groups/<id>")]
pub async fn delete(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    let group = match BannerGroup::get_by_id(&mut db, &id).await {
        Ok(g) => g,
        Err(_) => return Json(ApiResponse::error(404, "轮播图组不存在")),
    };

    let banners = match Banner::all().exec(&mut db).await {
        Ok(b) => b,
        Err(e) => return Json(ApiResponse::error(500, format!("查询轮播图失败: {e}"))),
    };

    if group_has_banners(&banners, id) {
        return Json(ApiResponse::error(400, "轮播图组下仍有轮播图，无法删除"));
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
