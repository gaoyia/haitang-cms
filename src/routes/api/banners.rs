use rocket::serde::json::Json;
use rocket::State;

use crate::models::{load_public_banners_by_code, ApiResponse, BannerView};

/// 按轮播图组 code 获取启用的轮播图列表（公开）
#[get("/api/banners?<code>")]
pub async fn list_by_code(
    db: &State<toasty::Db>,
    code: &str,
) -> Json<ApiResponse<Vec<BannerView>>> {
    let mut db = db.inner().clone();

    match load_public_banners_by_code(&mut db, code).await {
        Ok(views) => Json(ApiResponse::success(views)),
        Err(_) => Json(ApiResponse::error(404, "轮播图组不存在")),
    }
}
