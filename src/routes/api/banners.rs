use rocket::State;
use rocket::serde::json::Json;

use crate::models::{
    ApiResponse, PublicBannerView, dict::get_site_default_locale, load_public_banners_by_code,
};
use crate::models::locale::normalize_lang;

/// 按轮播图组 code 获取启用的轮播图列表（公开）
#[get("/api/banners?<code>&<lang>")]
pub async fn list_by_code(
    db: &State<toasty::Db>,
    code: &str,
    lang: Option<&str>,
) -> Json<ApiResponse<Vec<PublicBannerView>>> {
    let mut db = db.inner().clone();
    let default_lang = get_site_default_locale(&mut db).await;
    let resolved = lang.map(normalize_lang).unwrap_or_else(|| default_lang.clone());

    match load_public_banners_by_code(&mut db, code, &resolved, &default_lang).await {
        Ok(views) => Json(ApiResponse::success(views)),
        Err(_) => Json(ApiResponse::error(404, "轮播图组不存在")),
    }
}
