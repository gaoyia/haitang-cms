use rocket::State;
use rocket::serde::json::Json;

use crate::guards::AdminAuth;
use crate::models::{
    ApiResponse, CreatePost, PageResult, PostDetailView, PostMeta, PostView, UpdatePost,
    create_post, delete_post, get_site_default_locale, paginate_vec, post_assets_view,
    post_detail_view, post_to_view, posts_to_views, update_post,
};
use crate::routes::page::LangPageQuery;
use crate::storage::StorageService;

fn is_post_route_path_client_error(message: &str) -> bool {
    message.starts_with("SEO 路径")
        || message.contains("已被文章 #")
        || message.contains("对应多篇文章")
}

/// 创建新文章（需授权）
#[post("/api/admin/posts", data = "<input>")]
pub async fn create(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    input: Json<CreatePost>,
) -> Json<ApiResponse<PostView>> {
    let mut db = db.inner().clone();
    let default = get_site_default_locale(&mut db).await;

    match create_post(&mut db, &input, &default).await {
        Ok(meta) => match post_to_view(&mut db, &meta, input.lang.as_deref()).await {
            Ok(view) => Json(ApiResponse::success(view)),
            Err(e) => Json(ApiResponse::error(500, e)),
        },
        Err(e) => Json(ApiResponse::error(400, e)),
    }
}

/// 更新文章（需授权）
#[put("/api/admin/posts/<id>", data = "<input>")]
pub async fn update(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<UpdatePost>,
) -> Json<ApiResponse<PostView>> {
    let mut db = db.inner().clone();
    let default = get_site_default_locale(&mut db).await;

    match update_post(&mut db, id, &input, &default).await {
        Ok(meta) => {
            let lang = input
                .lang
                .as_deref()
                .map(|l| crate::models::locale::resolve_locale(Some(l), &default));
            match post_to_view(&mut db, &meta, lang.as_deref()).await {
                Ok(view) => Json(ApiResponse::success(view)),
                Err(e) => {
                    let code = if is_post_route_path_client_error(&e) {
                        400
                    } else {
                        500
                    };
                    Json(ApiResponse::error(code, e))
                }
            }
        }
        Err(e) => {
            let code = if e.contains("不存在") {
                404
            } else if is_post_route_path_client_error(&e) {
                400
            } else {
                500
            };
            Json(ApiResponse::error(code, e))
        }
    }
}

/// 删除文章（需授权）
#[delete("/api/admin/posts/<id>")]
pub async fn delete(_auth: AdminAuth, db: &State<toasty::Db>, id: i64) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    match delete_post(&mut db, id).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "删除成功".to_string(),
            data: None,
        }),
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 获取所有文章（管理端）
#[get("/api/admin/posts?<query..>")]
pub async fn list(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    query: LangPageQuery,
) -> Json<ApiResponse<PageResult<PostView>>> {
    let mut db = db.inner().clone();

    match PostMeta::all().exec(&mut db).await {
        Ok(posts) => match posts_to_views(&mut db, posts, query.lang.as_deref()).await {
            Ok(views) => {
                let (p, ps) = query.resolve_page();
                Json(ApiResponse::success(paginate_vec(views, p, ps)))
            }
            Err(e) => Json(ApiResponse::error(500, e)),
        },
        Err(e) => Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    }
}

/// 获取单篇文章（管理端，含全部 translations 与资源）
#[get("/api/admin/posts/<id>")]
pub async fn get(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    id: i64,
) -> Json<ApiResponse<PostDetailView>> {
    let mut db = db.inner().clone();

    match post_detail_view(&mut db, id).await {
        Ok(mut detail) => {
            if let Ok(assets) = post_assets_view(&mut db, id, storage).await {
                detail.covers = assets.covers;
                detail.attachments = assets.attachments;
            }
            Json(ApiResponse::success(detail))
        }
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}
