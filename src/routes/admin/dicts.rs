use rocket::State;
use rocket::serde::json::Json;

use crate::guards::AdminAuth;
use crate::models::locale::LANG_GLOBAL;
use crate::models::{
    ApiResponse, CreateDictMeta, DictDetailView, DictMeta, DictMetaListView, DictValue, UpdateDictMeta,
    UpsertDictValues, delete_dict_by_code, dict_detail_view, dict_meta_list_views,
    find_dict_meta_by_code, upsert_dict_values,
};
use crate::models::{PageResult, paginate_vec};
use crate::routes::page::LangPageQuery;

/// 获取所有字典 meta
#[get("/api/admin/dicts?<query..>")]
pub async fn list(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    query: LangPageQuery,
) -> Json<ApiResponse<PageResult<DictMetaListView>>> {
    let mut db = db.inner().clone();

    match dict_meta_list_views(&mut db, query.lang.as_deref()).await {
        Ok(views) => {
            let (p, ps) = query.resolve_page();
            Json(ApiResponse::success(paginate_vec(views, p, ps)))
        }
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 获取单个字典详情（meta + values）
#[get("/api/admin/dicts/<code>")]
pub async fn get(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    code: &str,
) -> Json<ApiResponse<DictDetailView>> {
    let mut db = db.inner().clone();

    match dict_detail_view(&mut db, code).await {
        Ok(detail) => Json(ApiResponse::success(detail)),
        Err(e) => Json(ApiResponse::error(404, e)),
    }
}

/// 创建字典 meta 及初始 value
#[post("/api/admin/dicts", data = "<input>")]
pub async fn create(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    input: Json<CreateDictMeta>,
) -> Json<ApiResponse<DictDetailView>> {
    let mut db = db.inner().clone();

    if find_dict_meta_by_code(&mut db, &input.code).await.is_ok() {
        return Json(ApiResponse::error(400, "标识码已存在"));
    }

    let description = input.description.as_deref().unwrap_or("");
    let sort = input.sort.unwrap_or(0);
    let translatable = input.translatable.unwrap_or(false);
    let value = input.value.as_deref().unwrap_or("");

    if DictMeta::create()
        .code(&input.code)
        .label(&input.label)
        .description(description)
        .translatable(translatable)
        .sort(sort)
        .exec(&mut db)
        .await
        .is_err()
    {
        return Json(ApiResponse::error(500, "创建失败"));
    }

    let lang = if translatable {
        if let Some(l) = input.lang.as_deref() {
            crate::models::locale::normalize_lang(l)
        } else {
            crate::models::get_site_default_locale(&mut db).await
        }
    } else {
        LANG_GLOBAL.to_string()
    };

    if DictValue::create()
        .code(&input.code)
        .lang(&lang)
        .value(value)
        .exec(&mut db)
        .await
        .is_err()
    {
        return Json(ApiResponse::error(500, "创建字典值失败"));
    }

    match dict_detail_view(&mut db, &input.code).await {
        Ok(detail) => Json(ApiResponse::success(detail)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 更新字典 meta
#[put("/api/admin/dicts/<code>", data = "<input>")]
pub async fn update(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    code: &str,
    input: Json<UpdateDictMeta>,
) -> Json<ApiResponse<DictDetailView>> {
    let mut db = db.inner().clone();

    let mut meta = match find_dict_meta_by_code(&mut db, code).await {
        Ok(d) => d,
        Err(e) => return Json(ApiResponse::error(404, e)),
    };

    let mut builder = meta.update();
    if let Some(ref label) = input.label {
        builder = builder.label(label.as_str());
    }
    if let Some(ref description) = input.description {
        builder = builder.description(description.as_str());
    }
    if let Some(translatable) = input.translatable {
        builder = builder.translatable(translatable);
    }
    if let Some(sort) = input.sort {
        builder = builder.sort(sort);
    }

    if let Err(e) = builder.exec(&mut db).await {
        return Json(ApiResponse::error(500, format!("更新失败: {e}")));
    }

    match dict_detail_view(&mut db, code).await {
        Ok(detail) => Json(ApiResponse::success(detail)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 批量更新字典 values
#[put("/api/admin/dicts/<code>/values", data = "<input>")]
pub async fn update_values(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    code: &str,
    input: Json<UpsertDictValues>,
) -> Json<ApiResponse<DictDetailView>> {
    let mut db = db.inner().clone();

    if let Err(e) = upsert_dict_values(&mut db, code, input.values.clone()).await {
        return Json(ApiResponse::error(400, e));
    }

    match dict_detail_view(&mut db, code).await {
        Ok(detail) => Json(ApiResponse::success(detail)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 删除字典项
#[delete("/api/admin/dicts/<code>")]
pub async fn delete(_auth: AdminAuth, db: &State<toasty::Db>, code: &str) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    match delete_dict_by_code(&mut db, code).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "删除成功".to_string(),
            data: None,
        }),
        Err(e) => Json(ApiResponse::error(404, e)),
    }
}
