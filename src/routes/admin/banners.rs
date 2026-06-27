use rocket::State;
use rocket::serde::json::Json;

use crate::guards::AdminAuth;
use crate::models::{
    ApiResponse, Banner, BannerView, CreateBanner, PageResult, UpdateBanner, banner_to_view,
    banners_to_views, delete_banner_asset_links, filter_banners_by_group, paginate_vec,
    validate_banner_group_id,
};
use crate::routes::page::PageQuery;

/// 获取轮播图列表（可按组筛选）
#[get("/api/admin/banners?<group_id>&<page..>")]
pub async fn list(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    group_id: Option<i64>,
    page: PageQuery,
) -> Json<ApiResponse<PageResult<BannerView>>> {
    let mut db = db.inner().clone();

    match Banner::all().exec(&mut db).await {
        Ok(banners) => {
            let filtered = match group_id {
                Some(gid) => filter_banners_by_group(banners, gid),
                None => {
                    let mut all = banners;
                    all.sort_by_key(|b| (b.group_id, b.sort));
                    all
                }
            };
            match banners_to_views(&mut db, filtered).await {
                Ok(views) => {
                    let (p, ps) = page.resolve();
                    Json(ApiResponse::success(paginate_vec(views, p, ps)))
                }
                Err(e) => Json(ApiResponse::error(500, e)),
            }
        }
        Err(e) => Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    }
}

/// 获取单个轮播图
#[get("/api/admin/banners/<id>")]
pub async fn get(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<BannerView>> {
    let mut db = db.inner().clone();

    match Banner::get_by_id(&mut db, &id).await {
        Ok(banner) => match banner_to_view(&mut db, banner).await {
            Ok(view) => Json(ApiResponse::success(view)),
            Err(e) => Json(ApiResponse::error(500, e)),
        },
        Err(_) => Json(ApiResponse::error(404, "轮播图不存在")),
    }
}

/// 创建轮播图
#[post("/api/admin/banners", data = "<input>")]
pub async fn create(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    input: Json<CreateBanner>,
) -> Json<ApiResponse<BannerView>> {
    let mut db = db.inner().clone();

    if let Err(msg) = validate_banner_group_id(&mut db, input.group_id).await {
        return Json(ApiResponse::error(400, msg));
    }

    let image_url = input.image_url.as_deref().unwrap_or("");
    let link_url = input.link_url.as_deref().unwrap_or("");
    let description = input.description.as_deref().unwrap_or("");
    let sort = input.sort.unwrap_or(0);
    let status = input.status.unwrap_or(1);

    match Banner::create()
        .group_id(input.group_id)
        .title(&input.title)
        .image_url(image_url)
        .link_url(link_url)
        .description(description)
        .sort(sort)
        .status(status)
        .exec(&mut db)
        .await
    {
        Ok(banner) => match banner_to_view(&mut db, banner).await {
            Ok(view) => Json(ApiResponse::success(view)),
            Err(e) => Json(ApiResponse::error(500, e)),
        },
        Err(e) => Json(ApiResponse::error(500, format!("创建失败: {e}"))),
    }
}

/// 更新轮播图
#[put("/api/admin/banners/<id>", data = "<input>")]
pub async fn update(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<UpdateBanner>,
) -> Json<ApiResponse<BannerView>> {
    let mut db = db.inner().clone();

    let mut banner = match Banner::get_by_id(&mut db, &id).await {
        Ok(b) => b,
        Err(_) => return Json(ApiResponse::error(404, "轮播图不存在")),
    };

    if let Some(group_id) = input.group_id
        && let Err(msg) = validate_banner_group_id(&mut db, group_id).await
    {
        return Json(ApiResponse::error(400, msg));
    }

    let mut builder = banner.update();
    if let Some(group_id) = input.group_id {
        builder = builder.group_id(group_id);
    }
    if let Some(ref title) = input.title {
        builder = builder.title(title.as_str());
    }
    if let Some(ref image_url) = input.image_url {
        builder = builder.image_url(image_url.as_str());
    }
    if let Some(ref link_url) = input.link_url {
        builder = builder.link_url(link_url.as_str());
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
        Ok(_) => match Banner::get_by_id(&mut db, &id).await {
            Ok(updated) => match banner_to_view(&mut db, updated).await {
                Ok(view) => Json(ApiResponse::success(view)),
                Err(e) => Json(ApiResponse::error(500, e)),
            },
            Err(e) => Json(ApiResponse::error(500, format!("更新成功但查询失败: {e}"))),
        },
        Err(e) => Json(ApiResponse::error(500, format!("更新失败: {e}"))),
    }
}

/// 删除轮播图
#[delete("/api/admin/banners/<id>")]
pub async fn delete(_auth: AdminAuth, db: &State<toasty::Db>, id: i64) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    match Banner::get_by_id(&mut db, &id).await {
        Ok(banner) => {
            let _ = delete_banner_asset_links(&mut db, id).await;
            match banner.delete().exec(&mut db).await {
                Ok(_) => Json(ApiResponse {
                    code: 0,
                    message: "删除成功".to_string(),
                    data: None,
                }),
                Err(e) => Json(ApiResponse::error(500, format!("删除失败: {e}"))),
            }
        }
        Err(_) => Json(ApiResponse::error(404, "轮播图不存在")),
    }
}
