use rocket::State;
use rocket::serde::json::Json;

use crate::models::{
    ApiResponse, CategoryMeta, CategoryView, category_to_view, resolve_category_id_from_public_key,
};
use crate::routes::lang::LangQuery;

/// 按路径查询分类
#[derive(Debug, FromForm)]
pub struct CategoryByPathQuery {
    pub lang: Option<String>,
    pub path: String,
}

/// 获取所有分类列表（公开）
#[get("/api/categories?<lang..>")]
pub async fn list(db: &State<toasty::Db>, lang: LangQuery) -> Json<ApiResponse<Vec<CategoryView>>> {
    let mut db = db.inner().clone();

    match crate::models::categories_to_views(&mut db, lang.lang.as_deref()).await {
        Ok(views) => Json(ApiResponse::success(views)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 获取单个分类详情（公开）
#[get("/api/categories/<id>?<lang..>")]
pub async fn get(
    db: &State<toasty::Db>,
    id: i64,
    lang: LangQuery,
) -> Json<ApiResponse<CategoryView>> {
    let mut db = db.inner().clone();
    let default = crate::models::get_site_default_locale(&mut db).await;
    let resolved = crate::models::locale::resolve_locale(lang.lang.as_deref(), &default);

    match CategoryMeta::get_by_id(&mut db, &id).await {
        Ok(meta) => match category_to_view(&mut db, &meta, &resolved, &default).await {
            Ok(view) => Json(ApiResponse::success(view)),
            Err(e) => Json(ApiResponse::error(500, e)),
        },
        Err(_) => Json(ApiResponse::error(404, "分类不存在")),
    }
}

/// 按 SEO 路径或 slug 获取分类（公开）
#[get("/api/categories/by-path?<query..>")]
pub async fn get_by_path(
    db: &State<toasty::Db>,
    query: CategoryByPathQuery,
) -> Json<ApiResponse<CategoryView>> {
    let mut db = db.inner().clone();
    let default = crate::models::get_site_default_locale(&mut db).await;
    let resolved = crate::models::locale::resolve_locale(query.lang.as_deref(), &default);

    let trimmed = query.path.trim();
    let key = if let Some(idx) = trimmed.rfind("/categories/") {
        trimmed[idx + "/categories/".len()..].trim()
    } else {
        trimmed.trim_start_matches('/')
    };

    if key.is_empty() {
        return Json(ApiResponse::error(400, "path 不能为空"));
    }

    match resolve_category_id_from_public_key(&mut db, &resolved, key).await {
        Ok(Some(id)) => match CategoryMeta::get_by_id(&mut db, &id).await {
            Ok(meta) => match category_to_view(&mut db, &meta, &resolved, &default).await {
                Ok(view) => Json(ApiResponse::success(view)),
                Err(e) => Json(ApiResponse::error(500, e)),
            },
            Err(_) => Json(ApiResponse::error(404, "分类不存在")),
        },
        Ok(None) => Json(ApiResponse::error(404, "分类不存在")),
        Err(e) if e.contains("对应多个分类") => Json(ApiResponse::error(409, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}