use rocket::serde::json::Json;
use rocket::State;

use crate::guards::AdminAuth;
use crate::models::{
    categories_to_views, category_detail_view, category_to_view, count_posts_by_category,
    create_category, delete_category, get_site_default_locale, paginate_vec, upsert_category_i18n,
    ApiResponse, CategoryDetailView, CategoryMeta, CategoryView, CreateCategory, PageResult,
    UpdateCategory,
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

    let mut meta = match CategoryMeta::get_by_id(&mut db, &id).await {
        Ok(c) => c,
        Err(_) => return Json(ApiResponse::error(404, "分类不存在")),
    };

    if let Some(sort) = input.sort {
        meta.update()
            .sort(sort)
            .exec(&mut db)
            .await
            .map_err(|e| format!("{e}"))
            .ok();
    }

    meta = CategoryMeta::get_by_id(&mut db, &id)
        .await
        .expect("分类应存在");

    if input.name.is_some() || input.description.is_some() {
        let lang = crate::models::locale::resolve_locale(input.lang.as_deref(), &default);
        let name = input.name.as_deref().unwrap_or("");
        let description = input.description.as_deref().unwrap_or("");
        if let Err(e) = upsert_category_i18n(&mut db, id, &lang, name, description).await {
            return Json(ApiResponse::error(500, e));
        }
    }

    match category_to_view(&mut db, &meta, &default, &default).await {
        Ok(view) => Json(ApiResponse::success(view)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 删除分类
#[delete("/api/admin/categories/<id>")]
pub async fn delete(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    match count_posts_by_category(&mut db, id).await {
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
