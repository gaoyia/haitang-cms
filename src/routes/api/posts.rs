use rocket::State;
use rocket::serde::json::Json;

use crate::models::{
    ApiResponse, CategoryView, PostMeta, PostView, categories_to_views, is_post_publicly_visible,
    post_to_view, posts_to_views,
};
use crate::models::asset::now_unix;
use crate::routes::lang::LangQuery;

/// 获取所有文章列表（公开）
#[get("/api/posts?<lang..>")]
pub async fn list(db: &State<toasty::Db>, lang: LangQuery) -> Json<ApiResponse<Vec<PostView>>> {
    let mut db = db.inner().clone();

    match PostMeta::all().exec(&mut db).await {
        Ok(posts) => {
            let now = now_unix();
            let visible: Vec<PostMeta> = posts
                .into_iter()
                .filter(|meta| is_post_publicly_visible(meta, now))
                .collect();
            match posts_to_views(&mut db, visible, lang.lang.as_deref()).await {
                Ok(views) => Json(ApiResponse::success(views)),
                Err(e) => Json(ApiResponse::error(500, e)),
            }
        }
        Err(e) => Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    }
}

/// 根据 ID 获取单篇文章（公开）
#[get("/api/posts/<id>?<lang..>")]
pub async fn get(db: &State<toasty::Db>, id: i64, lang: LangQuery) -> Json<ApiResponse<PostView>> {
    let mut db = db.inner().clone();

    match PostMeta::get_by_id(&mut db, &id).await {
        Ok(meta) => {
            if !is_post_publicly_visible(&meta, now_unix()) {
                return Json(ApiResponse::error(404, "文章不存在或未发布"));
            }
            match post_to_view(&mut db, &meta, lang.lang.as_deref()).await {
                Ok(view) => Json(ApiResponse::success(view)),
                Err(e) => Json(ApiResponse::error(500, e)),
            }
        }
        Err(_) => Json(ApiResponse::error(404, "文章不存在")),
    }
}

/// 获取所有分类列表（公开）
#[get("/api/categories?<lang..>")]
pub async fn list_categories(
    db: &State<toasty::Db>,
    lang: LangQuery,
) -> Json<ApiResponse<Vec<CategoryView>>> {
    let mut db = db.inner().clone();

    match categories_to_views(&mut db, lang.lang.as_deref()).await {
        Ok(views) => Json(ApiResponse::success(views)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}
