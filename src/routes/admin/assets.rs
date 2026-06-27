use rocket::State;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use tokio::io::AsyncReadExt;

use crate::guards::AdminAuth;
use crate::models::{
    ApiResponse, Banner, BannerAssetsView, LinkBannerAssetInput, LinkPostAssetInput, PageResult,
    PostAssetsView, PostMeta, SetBannerImageEnabledInput, asset_to_view_by_id, banner_assets_view,
    create_asset_record, delete_asset_record, link_banner_asset, link_post_asset, list_asset_views,
    post_assets_view, set_banner_image_enabled, unlink_banner_asset, unlink_post_asset,
};
use crate::storage::{AssetPurpose, StorageService};

#[derive(FromForm)]
pub(crate) struct UploadForm<'r> {
    file: TempFile<'r>,
}

#[derive(rocket::form::FromForm)]
pub(crate) struct AssetListQuery {
    purpose: Option<String>,
    keyword: Option<String>,
    #[field(default = 1)]
    page: i64,
    #[field(default = 10)]
    page_size: i64,
}

impl AssetListQuery {
    fn resolve_page(&self) -> (i64, i64) {
        let page = if self.page < 1 { 1 } else { self.page };
        let page_size = if self.page_size < 1 {
            10
        } else if self.page_size > 100 {
            100
        } else {
            self.page_size
        };
        (page, page_size)
    }
}

#[derive(rocket::form::FromForm)]
pub(crate) struct UploadQuery {
    purpose: String,
    post_id: Option<i64>,
    role: Option<String>,
    banner_id: Option<i64>,
    banner_role: Option<String>,
}

/// 上传资源
#[post("/api/admin/assets?<query..>", data = "<form>")]
pub async fn upload(
    auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    query: UploadQuery,
    form: Form<UploadForm<'_>>,
) -> Json<ApiResponse<crate::models::AssetView>> {
    let mut db = db.inner().clone();
    let purpose = match AssetPurpose::parse(&query.purpose) {
        Ok(p) => p,
        Err(e) => return Json(ApiResponse::error(400, e)),
    };

    let temp = form.into_inner().file;
    let raw_name = temp
        .name()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "file".to_string());
    let declared_mime = temp
        .content_type()
        .map(|ct| ct.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let mut bytes = Vec::new();
    match temp.open().await {
        Ok(mut reader) => {
            if let Err(e) = reader.read_to_end(&mut bytes).await {
                return Json(ApiResponse::error(500, format!("读取上传文件失败: {e}")));
            }
        }
        Err(e) => return Json(ApiResponse::error(500, format!("打开上传文件失败: {e}"))),
    }

    if bytes.is_empty() {
        return Json(ApiResponse::error(400, "上传文件为空"));
    }
    if bytes.len() as u64 > storage.config.max_bytes {
        return Json(ApiResponse::error(
            400,
            format!("文件大小超过限制（{} 字节）", storage.config.max_bytes),
        ));
    }

    let file_meta = crate::storage::resolve_upload_file_meta(&raw_name, &declared_mime, &bytes);
    let mime = file_meta.mime_type;
    let ext = crate::storage::extension_for(&mime, &bytes);

    if let Err(e) = purpose.validate_file(&mime, &file_meta.validate_name) {
        return Json(ApiResponse::error(400, e));
    }

    let storage_key = storage.new_object_key(auth.claims.user_id, ext.as_deref());
    let original_name = storage_key
        .rsplit('/')
        .next()
        .unwrap_or(&storage_key)
        .to_string();
    let upload_name = crate::storage::filename::ensure_filename_extension(&raw_name, &mime, &bytes);
    if let Err(e) = storage.put(&storage_key, &bytes) {
        return Json(ApiResponse::error(500, format!("写入存储失败: {e}")));
    }

    let asset = match create_asset_record(
        &mut db,
        &storage_key,
        &original_name,
        &upload_name,
        &mime,
        bytes.len() as i64,
        purpose,
    )
    .await
    {
        Ok(a) => a,
        Err(e) => {
            let _ = storage.delete(&storage_key);
            return Json(ApiResponse::error(500, e));
        }
    };

    if let (Some(pid), Some(role_str)) = (query.post_id, query.role.as_deref()) {
        if PostMeta::get_by_id(&mut db, &pid).await.is_err() {
            return Json(ApiResponse::error(404, "文章不存在"));
        }
        let input = LinkPostAssetInput {
            asset_id: asset.id,
            role: role_str.to_string(),
            sort_order: None,
        };
        if let Err(e) = link_post_asset(&mut db, pid, &input).await {
            return Json(ApiResponse::error(400, e));
        }
    }

    if let (Some(bid), Some(role_str)) = (query.banner_id, query.banner_role.as_deref()) {
        if Banner::get_by_id(&mut db, &bid).await.is_err() {
            return Json(ApiResponse::error(404, "轮播图不存在"));
        }
        let input = LinkBannerAssetInput {
            asset_id: asset.id,
            role: role_str.to_string(),
            sort_order: None,
            enabled: None,
        };
        if let Err(e) = link_banner_asset(&mut db, storage, bid, &input).await {
            return Json(ApiResponse::error(400, e));
        }
    }

    match asset_to_view_by_id(&mut db, asset.id, storage).await {
        Ok(view) => Json(ApiResponse::success(view)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 资源列表
#[get("/api/admin/assets?<query..>")]
pub async fn list(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    query: AssetListQuery,
) -> Json<ApiResponse<PageResult<crate::models::AssetView>>> {
    let mut db = db.inner().clone();

    match list_asset_views(
        &mut db,
        storage,
        query.purpose.as_deref(),
        query.keyword.as_deref(),
    )
    .await
    {
        Ok(views) => {
            let (page, page_size) = query.resolve_page();
            let total = views.len() as i64;
            let start = ((page - 1) * page_size) as usize;
            let list = views
                .into_iter()
                .skip(start)
                .take(page_size as usize)
                .collect();
            Json(ApiResponse::success(PageResult {
                list,
                total,
                page,
                page_size,
            }))
        }
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 资源详情
#[get("/api/admin/assets/<id>")]
pub async fn get(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    id: i64,
) -> Json<ApiResponse<crate::models::AssetView>> {
    let mut db = db.inner().clone();

    match asset_to_view_by_id(&mut db, id, storage).await {
        Ok(view) => Json(ApiResponse::success(view)),
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 彻底删除资源（无引用时）
#[delete("/api/admin/assets/<id>")]
pub async fn delete(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    id: i64,
) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    match delete_asset_record(&mut db, storage, id).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "删除成功".to_string(),
            data: None,
        }),
        Err(e) if e.contains("引用") => Json(ApiResponse::error(400, e)),
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 获取文章关联的资源
#[get("/api/admin/posts/<id>/assets")]
pub async fn list_post_assets(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    id: i64,
) -> Json<ApiResponse<PostAssetsView>> {
    let mut db = db.inner().clone();

    if PostMeta::get_by_id(&mut db, &id).await.is_err() {
        return Json(ApiResponse::error(404, "文章不存在"));
    }

    match post_assets_view(&mut db, id, storage).await {
        Ok(view) => Json(ApiResponse::success(view)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 关联资源到文章
#[post("/api/admin/posts/<id>/assets", data = "<input>")]
pub async fn link_post(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<LinkPostAssetInput>,
) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    if PostMeta::get_by_id(&mut db, &id).await.is_err() {
        return Json(ApiResponse::error(404, "文章不存在"));
    }

    match link_post_asset(&mut db, id, &input).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "关联成功".to_string(),
            data: None,
        }),
        Err(e) => Json(ApiResponse::error(400, e)),
    }
}

/// 解除文章与资源的关联；purge=true 且无其他引用时彻底删除
#[delete("/api/admin/posts/<id>/assets/<asset_id>?<purge>")]
pub async fn unlink_post(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    id: i64,
    asset_id: i64,
    purge: Option<bool>,
) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    if PostMeta::get_by_id(&mut db, &id).await.is_err() {
        return Json(ApiResponse::error(404, "文章不存在"));
    }

    match unlink_post_asset(&mut db, storage, id, asset_id, purge.unwrap_or(false)).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "操作成功".to_string(),
            data: None,
        }),
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(400, e)),
    }
}

/// 获取轮播图关联的资源
#[get("/api/admin/banners/<id>/assets")]
pub async fn list_banner_assets(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    id: i64,
) -> Json<ApiResponse<BannerAssetsView>> {
    let mut db = db.inner().clone();

    if Banner::get_by_id(&mut db, &id).await.is_err() {
        return Json(ApiResponse::error(404, "轮播图不存在"));
    }

    match banner_assets_view(&mut db, id, storage).await {
        Ok(view) => Json(ApiResponse::success(view)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}

/// 关联资源到轮播图
#[post("/api/admin/banners/<id>/assets", data = "<input>")]
pub async fn link_banner(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    id: i64,
    input: Json<LinkBannerAssetInput>,
) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    if Banner::get_by_id(&mut db, &id).await.is_err() {
        return Json(ApiResponse::error(404, "轮播图不存在"));
    }

    match link_banner_asset(&mut db, storage, id, &input).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "关联成功".to_string(),
            data: None,
        }),
        Err(e) => Json(ApiResponse::error(400, e)),
    }
}

/// 解除轮播图与资源的关联；purge=true 且无其他引用时彻底删除
#[delete("/api/admin/banners/<id>/assets/<asset_id>?<purge>")]
pub async fn unlink_banner(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    id: i64,
    asset_id: i64,
    purge: Option<bool>,
) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    if Banner::get_by_id(&mut db, &id).await.is_err() {
        return Json(ApiResponse::error(404, "轮播图不存在"));
    }

    match unlink_banner_asset(&mut db, storage, id, asset_id, purge.unwrap_or(false)).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "操作成功".to_string(),
            data: None,
        }),
        Err(e) if e.contains("不存在") => Json(ApiResponse::error(404, e)),
        Err(e) => Json(ApiResponse::error(400, e)),
    }
}

/// 启用/停用轮播图图片展示（保留资源关联）
#[put("/api/admin/banners/<id>/assets/image-enabled", data = "<input>")]
pub async fn set_banner_image_enabled_route(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    id: i64,
    input: Json<SetBannerImageEnabledInput>,
) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    if Banner::get_by_id(&mut db, &id).await.is_err() {
        return Json(ApiResponse::error(404, "轮播图不存在"));
    }

    match set_banner_image_enabled(&mut db, storage, id, input.enabled).await {
        Ok(_) => Json(ApiResponse {
            code: 0,
            message: "操作成功".to_string(),
            data: None,
        }),
        Err(e) if e.contains("尚未") => Json(ApiResponse::error(400, e)),
        Err(e) => Json(ApiResponse::error(500, e)),
    }
}
