use rocket::State;
use rocket::serde::json::Json;

use crate::models::{
    ApiResponse, PostMeta, PostView, is_post_publicly_visible, post_to_view_with_storage,
    sort_post_metas_for_list,
};
use crate::models::asset::now_unix;
use crate::routes::lang::LangQuery;
use crate::storage::StorageService;

/// 公开文章列表查询参数
#[derive(Debug, FromForm)]
pub struct PostListQuery {
    pub lang: Option<String>,
    pub category_id: Option<i64>,
}

/// 获取文章列表（公开）
#[get("/api/posts?<query..>")]
pub async fn list(
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    query: PostListQuery,
) -> Json<ApiResponse<Vec<PostView>>> {
    let mut db = db.inner().clone();

    match PostMeta::all().exec(&mut db).await {
        Ok(posts) => {
            let now = now_unix();
            let visible: Vec<PostMeta> = posts
                .into_iter()
                .filter(|meta| is_post_publicly_visible(meta, now))
                .filter(|meta| {
                    query
                        .category_id
                        .is_none_or(|cid| meta.category_id == cid)
                })
                .collect();
            let mut visible = visible;
            sort_post_metas_for_list(&mut visible);
            let mut views = Vec::new();
            for meta in visible {
                match post_to_view_with_storage(
                    &mut db,
                    &meta,
                    query.lang.as_deref(),
                    storage.inner(),
                )
                .await
                {
                    Ok(view) => views.push(view),
                    Err(e) => return Json(ApiResponse::error(500, e)),
                }
            }
            Json(ApiResponse::success(views))
        }
        Err(e) => Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    }
}

/// 根据 ID 获取单篇文章（公开）
#[get("/api/posts/<id>?<lang..>")]
pub async fn get(
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    id: i64,
    lang: LangQuery,
) -> Json<ApiResponse<PostView>> {
    let mut db = db.inner().clone();

    match PostMeta::get_by_id(&mut db, &id).await {
        Ok(meta) => {
            if !is_post_publicly_visible(&meta, now_unix()) {
                return Json(ApiResponse::error(404, "文章不存在或未发布"));
            }
            match post_to_view_with_storage(
                &mut db,
                &meta,
                lang.lang.as_deref(),
                storage.inner(),
            )
            .await
            {
                Ok(view) => Json(ApiResponse::success(view)),
                Err(e) => Json(ApiResponse::error(500, e)),
            }
        }
        Err(_) => Json(ApiResponse::error(404, "文章不存在")),
    }
}
