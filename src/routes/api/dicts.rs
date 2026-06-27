use rocket::State;
use rocket::serde::json::Json;

use crate::models::{ApiResponse, DictPublicView, dict_public_views, load_dict_map};
use crate::routes::lang::LangQuery;

/// 获取全部字典项（公开，已按 lang 解析 value）
#[get("/api/dicts?<lang..>")]
pub async fn list(
    db: &State<toasty::Db>,
    lang: LangQuery,
) -> Json<ApiResponse<Vec<DictPublicView>>> {
    let mut db = db.inner().clone();

    match dict_public_views(&mut db, lang.lang.as_deref()).await {
        Ok(views) => Json(ApiResponse::success(views)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 按 code 获取字典项（公开）
#[get("/api/dicts/<code>?<lang..>")]
pub async fn get_by_code(
    db: &State<toasty::Db>,
    code: &str,
    lang: LangQuery,
) -> Json<ApiResponse<DictPublicView>> {
    let mut db = db.inner().clone();

    match dict_public_views(&mut db, lang.lang.as_deref()).await {
        Ok(views) => match views.into_iter().find(|v| v.code == code) {
            Some(view) => Json(ApiResponse::success(view)),
            None => Json(ApiResponse::error(404, "字典项不存在")),
        },
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 获取字典键值映射（公开，便于前台一次性加载）
#[get("/api/dicts/map?<lang..>")]
pub async fn map(db: &State<toasty::Db>, lang: LangQuery) -> Json<ApiResponse<serde_json::Value>> {
    let mut db = db.inner().clone();
    let dict_map = load_dict_map(&mut db, lang.lang.as_deref()).await;
    Json(ApiResponse::success(serde_json::json!(dict_map)))
}
