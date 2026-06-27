use rocket::State;
use rocket::serde::json::Json;

use crate::guards::AdminAuth;
use crate::models::{
    ApiResponse, CreatePost, PageResult, PostDetailView, PostI18n, PostMeta, PostView, UpdatePost,
    create_post, delete_post, get_site_default_locale, paginate_vec, post_detail_view,
    post_to_view, posts_to_views, upsert_post_i18n, validate_category_id,
};
use crate::routes::page::LangPageQuery;

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

    let mut meta = match PostMeta::get_by_id(&mut db, &id).await {
        Ok(post) => post,
        Err(_) => return Json(ApiResponse::error(404, "文章不存在")),
    };

    let mut builder = meta.update();
    let mut meta_changed = false;
    if let Some(category_id) = input.category_id {
        if let Err(msg) = validate_category_id(&mut db, category_id).await {
            return Json(ApiResponse::error(400, msg));
        }
        builder = builder.category_id(category_id);
        meta_changed = true;
    }
    if let Some(status) = input.status {
        builder = builder.status(status);
        meta_changed = true;
    }

    if meta_changed {
        if let Err(e) = builder.exec(&mut db).await {
            return Json(ApiResponse::error(500, format!("更新失败: {e}")));
        }
        meta = PostMeta::get_by_id(&mut db, &id).await.expect("文章应存在");
    }

    let lang = input
        .lang
        .as_deref()
        .map(|l| crate::models::locale::resolve_locale(Some(l), &default));
    if input.title.is_some()
        || input.description.is_some()
        || input.content.is_some()
        || input.route_path.is_some()
        || input.tags.is_some()
    {
        let resolved_lang = lang.clone().unwrap_or(default.clone());
        let rows = PostI18n::all().exec(&mut db).await.ok().unwrap_or_default();
        let existing = rows
            .iter()
            .find(|r| r.post_id == id && r.lang == resolved_lang);

        let title = input
            .title
            .as_deref()
            .or_else(|| existing.map(|e| e.title.as_str()))
            .unwrap_or("");
        let description = input
            .description
            .as_deref()
            .or_else(|| existing.map(|e| e.description.as_str()))
            .unwrap_or("");
        let content = input
            .content
            .as_deref()
            .or_else(|| existing.map(|e| e.content.as_str()))
            .unwrap_or("");
        let route_path = input
            .route_path
            .as_deref()
            .or_else(|| existing.map(|e| e.route_path.as_str()))
            .unwrap_or("");
        let tags = input
            .tags
            .as_deref()
            .or_else(|| existing.map(|e| e.tags.as_str()))
            .unwrap_or("");

        if let Err(e) = upsert_post_i18n(
            &mut db,
            id,
            &resolved_lang,
            title,
            description,
            content,
            route_path,
            tags,
        )
        .await
        {
            let code = if e.starts_with("SEO 路径") { 400 } else { 500 };
            return Json(ApiResponse::error(code, e));
        }
    }

    match post_to_view(&mut db, &meta, lang.as_deref()).await {
        Ok(view) => Json(ApiResponse::success(view)),
        Err(e) => Json(ApiResponse::error(500, e)),
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

/// 获取单篇文章（管理端，含全部 translations）
#[get("/api/admin/posts/<id>")]
pub async fn get(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<PostDetailView>> {
    let mut db = db.inner().clone();

    match post_detail_view(&mut db, id).await {
        Ok(detail) => Json(ApiResponse::success(detail)),
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}
