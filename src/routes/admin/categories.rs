use rocket::State;
use rocket::serde::json::Json;

use crate::guards::AdminAuth;
use crate::models::{
    ApiResponse, CategoryDetailView, CategoryView, CreateCategory, PageResult,
    UpdateCategory, categories_to_views, category_detail_view, category_to_view, create_category,
    delete_category, get_site_default_locale, paginate_vec, update_category,
};
use crate::routes::page::LangPageQuery;

/// 获取所有分类列表
#[get("/api/admin/categories?<query..>")]
pub async fn list(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    query: LangPageQuery,
) -> Json<ApiResponse<PageResult<CategoryView>>> {
    let mut db = db.inner().clone();

    match categories_to_views(&mut db, query.lang.as_deref()).await {
        Ok(views) => {
            let (p, ps) = query.resolve_page();
            Json(ApiResponse::success(paginate_vec(views, p, ps)))
        }
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 获取单个分类（含 translations）
#[get("/api/admin/categories/<id>")]
pub async fn get(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<CategoryDetailView>> {
    let mut db = db.inner().clone();

    match category_detail_view(&mut db, id).await {
        Ok(detail) => Json(ApiResponse::success(detail)),
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 创建分类
#[post("/api/admin/categories", data = "<input>")]
pub async fn create(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    input: Json<CreateCategory>,
) -> Json<ApiResponse<CategoryView>> {
    let mut db = db.inner().clone();
    let default = get_site_default_locale(&mut db).await;

    match create_category(&mut db, &input, &default).await {
        Ok(meta) => {
            let lang = crate::models::locale::resolve_locale(input.lang.as_deref(), &default);
            match category_to_view(&mut db, &meta, &lang, &default).await {
                Ok(view) => Json(ApiResponse::success(view)),
                Err(e) => Json(ApiResponse::error(500, e)),
            }
        }
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 更新分类
#[put("/api/admin/categories/<id>", data = "<input>")]
pub async fn update(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<UpdateCategory>,
) -> Json<ApiResponse<CategoryView>> {
    let mut db = db.inner().clone();
    let default = get_site_default_locale(&mut db).await;

    match update_category(&mut db, id, &input, &default).await {
        Ok(meta) => {
            let lang = crate::models::locale::resolve_locale(input.lang.as_deref(), &default);
            match category_to_view(&mut db, &meta, &lang, &default).await {
                Ok(view) => Json(ApiResponse::success(view)),
                Err(e) => Json(ApiResponse::error(500, e)),
            }
        }
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 删除分类
#[delete("/api/admin/categories/<id>")]
pub async fn delete(_auth: AdminAuth, db: &State<toasty::Db>, id: i64) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    match crate::models::count_posts_by_category(&mut db, id).await {
        Ok(count) if count > 0 => {
            return Json(ApiResponse::error(400, "该分类下仍有文章，无法删除"));
        }
        Err(e) => return Json(ApiResponse::error(500, e)),
        _ => {}
    }

    match delete_category(&mut db, id).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "删除成功".to_string(),
            data: None,
        }),
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}
