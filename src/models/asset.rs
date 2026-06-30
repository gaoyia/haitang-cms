use serde::{Deserialize, Serialize};

use super::get_post_cover_max;
use super::user::find_default_admin_user_id;
use crate::storage::{AssetPurpose, BannerAssetRole, PostAssetRole, StorageService};

/// 资源元数据
#[derive(Debug, Clone, toasty::Model)]
pub struct Asset {
    #[key]
    #[auto]
    pub id: i64,

    /// 存储后端对象键
    pub storage_key: String,

    pub original_name: String,

    /// 用户上传时的原始文件名（经清洗，不含路径）
    pub upload_name: String,

    pub mime_type: String,

    pub size: i64,

    /// cover | content | banner | friend_link | attachment
    pub purpose: String,

    pub created_at: i64,
}

/// 文章与资源关联
#[derive(Debug, Clone, toasty::Model)]
#[key(post_id, asset_id)]
pub struct PostAsset {
    pub post_id: i64,

    pub asset_id: i64,

    /// cover | attachment
    pub role: String,

    pub sort_order: i64,
}

/// 轮播图与资源关联
#[derive(Debug, Clone, toasty::Model)]
#[key(banner_id, asset_id)]
pub struct BannerAsset {
    pub banner_id: i64,

    pub asset_id: i64,

    /// image
    pub role: String,

    pub sort_order: i64,

    /// 1 = 启用展示, 0 = 停用（保留关联）
    pub enabled: i64,
}

#[derive(Debug, Serialize)]
pub struct AssetView {
    pub id: i64,
    pub storage_key: String,
    pub original_name: String,
    pub upload_name: String,
    pub mime_type: String,
    pub size: i64,
    pub purpose: String,
    pub url: String,
    pub created_at: i64,
    pub ref_count: i64,
}

#[derive(Debug, Serialize)]
pub struct PostAssetsView {
    pub covers: Vec<AssetView>,
    pub cover_max: i64,
    pub attachments: Vec<AssetView>,
}

#[derive(Debug, Serialize)]
pub struct BannerAssetsView {
    pub image: Option<AssetView>,
    pub image_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct LinkPostAssetInput {
    pub asset_id: i64,
    pub role: String,
    pub sort_order: Option<i64>,
}

/// 批量更新文章附件排序（须包含当前全部已关联附件 ID，顺序即 sort_order）
#[derive(Debug, Deserialize)]
pub struct ReorderPostAttachmentsInput {
    pub asset_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct LinkBannerAssetInput {
    pub asset_id: i64,
    pub role: String,
    pub sort_order: Option<i64>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SetBannerImageEnabledInput {
    pub enabled: bool,
}

pub fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub async fn count_post_asset_refs(db: &mut toasty::Db, asset_id: i64) -> Result<i64, String> {
    let rows = post_asset_rows(db).await?;
    Ok(rows.iter().filter(|r| r.asset_id == asset_id).count() as i64)
}

pub async fn count_banner_asset_refs(db: &mut toasty::Db, asset_id: i64) -> Result<i64, String> {
    let rows = banner_asset_rows(db).await?;
    Ok(rows.iter().filter(|r| r.asset_id == asset_id).count() as i64)
}

pub async fn count_asset_refs(db: &mut toasty::Db, asset_id: i64) -> Result<i64, String> {
    let post = count_post_asset_refs(db, asset_id).await?;
    let banner = count_banner_asset_refs(db, asset_id).await?;
    Ok(post + banner)
}

async fn post_asset_rows(db: &mut toasty::Db) -> Result<Vec<PostAsset>, String> {
    PostAsset::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询文章资源关联失败: {e}"))
}

async fn banner_asset_rows(db: &mut toasty::Db) -> Result<Vec<BannerAsset>, String> {
    BannerAsset::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询轮播图资源关联失败: {e}"))
}

pub fn asset_to_view(asset: &Asset, storage: &StorageService, ref_count: i64) -> AssetView {
    AssetView {
        id: asset.id,
        storage_key: asset.storage_key.clone(),
        original_name: asset.original_name.clone(),
        upload_name: asset.upload_name.clone(),
        mime_type: asset.mime_type.clone(),
        size: asset.size,
        purpose: asset.purpose.clone(),
        url: storage.public_url(&asset.storage_key),
        created_at: asset.created_at,
        ref_count,
    }
}

pub async fn create_asset_record(
    db: &mut toasty::Db,
    storage_key: &str,
    original_name: &str,
    upload_name: &str,
    mime_type: &str,
    size: i64,
    purpose: AssetPurpose,
) -> Result<Asset, String> {
    Asset::create()
        .storage_key(storage_key)
        .original_name(original_name)
        .upload_name(upload_name)
        .mime_type(mime_type)
        .size(size)
        .purpose(purpose.as_str())
        .created_at(now_unix())
        .exec(db)
        .await
        .map_err(|e| format!("创建资源记录失败: {e}"))
}

pub async fn get_asset(db: &mut toasty::Db, id: i64) -> Result<Asset, String> {
    Asset::get_by_id(db, &id)
        .await
        .map_err(|_| "资源不存在".to_string())
}

pub async fn asset_to_view_by_id(
    db: &mut toasty::Db,
    id: i64,
    storage: &StorageService,
) -> Result<AssetView, String> {
    let asset = get_asset(db, id).await?;
    let ref_count = count_asset_refs(db, id).await?;
    Ok(asset_to_view(&asset, storage, ref_count))
}

pub async fn list_asset_views(
    db: &mut toasty::Db,
    storage: &StorageService,
    purpose: Option<&str>,
    keyword: Option<&str>,
) -> Result<Vec<AssetView>, String> {
    let mut assets = Asset::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询资源失败: {e}"))?;

    if let Some(p) = purpose.filter(|s| !s.is_empty()) {
        assets.retain(|a| a.purpose == p);
    }
    if let Some(kw) = keyword.filter(|s| !s.trim().is_empty()) {
        let kw = kw.trim().to_lowercase();
        assets.retain(|a| {
            a.original_name.to_lowercase().contains(&kw)
                || a.upload_name.to_lowercase().contains(&kw)
        });
    }

    assets.sort_by_key(|a| std::cmp::Reverse(a.created_at));

    let post_refs = post_asset_rows(db).await?;
    let banner_refs = banner_asset_rows(db).await?;
    let mut views = Vec::new();
    for asset in assets {
        let post_count = post_refs.iter().filter(|r| r.asset_id == asset.id).count() as i64;
        let banner_count = banner_refs
            .iter()
            .filter(|r| r.asset_id == asset.id)
            .count() as i64;
        views.push(asset_to_view(&asset, storage, post_count + banner_count));
    }
    Ok(views)
}

pub async fn delete_asset_record(
    db: &mut toasty::Db,
    storage: &StorageService,
    id: i64,
) -> Result<(), String> {
    let asset = get_asset(db, id).await?;
    let ref_count = count_asset_refs(db, id).await?;
    if ref_count > 0 {
        return Err(format!("资源仍被 {ref_count} 处引用，无法删除"));
    }

    storage
        .delete(&asset.storage_key)
        .map_err(|e| format!("删除存储对象失败: {e}"))?;

    asset
        .delete()
        .exec(db)
        .await
        .map_err(|e| format!("删除资源记录失败: {e}"))
}

pub async fn delete_post_asset_links(db: &mut toasty::Db, post_id: i64) -> Result<(), String> {
    let rows = post_asset_rows(db).await?;
    for row in rows.into_iter().filter(|r| r.post_id == post_id) {
        row.delete()
            .exec(db)
            .await
            .map_err(|e| format!("删除文章资源关联失败: {e}"))?;
    }
    Ok(())
}

pub async fn post_assets_view(
    db: &mut toasty::Db,
    post_id: i64,
    storage: &StorageService,
) -> Result<PostAssetsView, String> {
    let cover_max = get_post_cover_max(db).await;
    let links = post_asset_rows(db).await?;
    let post_links: Vec<_> = links.into_iter().filter(|r| r.post_id == post_id).collect();

    let mut covers = Vec::new();
    let mut attachments = Vec::new();

    for link in post_links {
        let asset = get_asset(db, link.asset_id).await?;
        let ref_count = count_asset_refs(db, link.asset_id).await?;
        let view = asset_to_view(&asset, storage, ref_count);
        if link.role == PostAssetRole::Cover.as_str() {
            covers.push((link.sort_order, view));
        } else if link.role == PostAssetRole::Attachment.as_str() {
            attachments.push((link.sort_order, view));
        }
    }

    covers.sort_by_key(|(sort, _)| *sort);
    attachments.sort_by_key(|(sort, _)| *sort);
    Ok(PostAssetsView {
        covers: covers.into_iter().map(|(_, v)| v).collect(),
        cover_max,
        attachments: attachments.into_iter().map(|(_, v)| v).collect(),
    })
}

async fn count_post_covers(db: &mut toasty::Db, post_id: i64) -> Result<i64, String> {
    let rows = post_asset_rows(db).await?;
    Ok(rows
        .iter()
        .filter(|r| r.post_id == post_id && r.role == PostAssetRole::Cover.as_str())
        .count() as i64)
}

async fn next_cover_sort_order(db: &mut toasty::Db, post_id: i64) -> Result<i64, String> {
    let rows = post_asset_rows(db).await?;
    let max = rows
        .iter()
        .filter(|r| r.post_id == post_id && r.role == PostAssetRole::Cover.as_str())
        .map(|r| r.sort_order)
        .max()
        .unwrap_or(-1);
    Ok(max + 1)
}

async fn next_attachment_sort_order(db: &mut toasty::Db, post_id: i64) -> Result<i64, String> {
    let rows = post_asset_rows(db).await?;
    let max = rows
        .iter()
        .filter(|r| r.post_id == post_id && r.role == PostAssetRole::Attachment.as_str())
        .map(|r| r.sort_order)
        .max()
        .unwrap_or(-1);
    Ok(max + 1)
}

pub async fn link_post_asset(
    db: &mut toasty::Db,
    post_id: i64,
    input: &LinkPostAssetInput,
) -> Result<(), String> {
    let role = PostAssetRole::parse(&input.role)?;
    let asset = get_asset(db, input.asset_id).await?;

    if !role.accepts_purpose(&asset.purpose) {
        return Err(format!(
            "资源用途「{}」不能作为{}",
            asset.purpose,
            if role == PostAssetRole::Cover {
                "封面"
            } else {
                "附件"
            }
        ));
    }

    if role == PostAssetRole::Cover {
        let max = get_post_cover_max(db).await;
        let already_linked = PostAsset::get_by_post_id_and_asset_id(db, &post_id, &input.asset_id)
            .await
            .is_ok();
        if !already_linked {
            let count = count_post_covers(db, post_id).await?;
            if count >= max {
                return Err(format!(
                    "封面图最多 {max} 张，可在字典 post_cover_max 中调整"
                ));
            }
        }
    }

    let sort_order = if role == PostAssetRole::Cover {
        if let Some(order) = input.sort_order {
            order
        } else {
            next_cover_sort_order(db, post_id).await?
        }
    } else if role == PostAssetRole::Attachment {
        if let Some(order) = input.sort_order {
            order
        } else {
            next_attachment_sort_order(db, post_id).await?
        }
    } else {
        input.sort_order.unwrap_or(0)
    };
    match PostAsset::get_by_post_id_and_asset_id(db, &post_id, &input.asset_id).await {
        Ok(mut row) => {
            row.update()
                .role(role.as_str())
                .sort_order(sort_order)
                .exec(db)
                .await
                .map_err(|e| format!("更新文章资源关联失败: {e}"))?;
        }
        Err(_) => {
            PostAsset::create()
                .post_id(post_id)
                .asset_id(input.asset_id)
                .role(role.as_str())
                .sort_order(sort_order)
                .exec(db)
                .await
                .map_err(|e| format!("创建文章资源关联失败: {e}"))?;
        }
    }

    Ok(())
}

/// 若文章尚无封面，则关联指定资源为封面（幂等）
pub async fn ensure_post_cover_link(
    db: &mut toasty::Db,
    post_id: i64,
    asset_id: i64,
) -> Result<bool, String> {
    let count = count_post_covers(db, post_id).await?;
    if count > 0 {
        return Ok(false);
    }
    link_post_asset(
        db,
        post_id,
        &LinkPostAssetInput {
            asset_id,
            role: PostAssetRole::Cover.as_str().to_string(),
            sort_order: Some(0),
        },
    )
    .await?;
    Ok(true)
}

/// 按给定顺序全量更新文章附件的 `sort_order`
pub async fn reorder_post_attachments(
    db: &mut toasty::Db,
    post_id: i64,
    asset_ids: &[i64],
) -> Result<(), String> {
    let rows = post_asset_rows(db).await?;
    let linked: Vec<i64> = rows
        .iter()
        .filter(|r| r.post_id == post_id && r.role == PostAssetRole::Attachment.as_str())
        .map(|r| r.asset_id)
        .collect();

    if asset_ids.len() != linked.len() {
        return Err("附件列表须包含当前全部已关联附件".to_string());
    }

    let linked_set: std::collections::HashSet<i64> = linked.iter().copied().collect();
    let mut seen = std::collections::HashSet::new();
    for id in asset_ids {
        if !linked_set.contains(id) {
            return Err(format!("资源 #{id} 不是本文附件"));
        }
        if !seen.insert(*id) {
            return Err("附件 ID 重复".to_string());
        }
    }

    for (sort_order, asset_id) in asset_ids.iter().enumerate() {
        let mut row = PostAsset::get_by_post_id_and_asset_id(db, &post_id, asset_id)
            .await
            .map_err(|_| format!("资源 #{asset_id} 与本文关联不存在"))?;
        row.update()
            .sort_order(sort_order as i64)
            .exec(db)
            .await
            .map_err(|e| format!("更新附件排序失败: {e}"))?;
    }

    Ok(())
}

/// 按给定顺序全量更新文章封面的 `sort_order`
pub async fn reorder_post_covers(
    db: &mut toasty::Db,
    post_id: i64,
    asset_ids: &[i64],
) -> Result<(), String> {
    let rows = post_asset_rows(db).await?;
    let linked: Vec<i64> = rows
        .iter()
        .filter(|r| r.post_id == post_id && r.role == PostAssetRole::Cover.as_str())
        .map(|r| r.asset_id)
        .collect();

    if asset_ids.len() != linked.len() {
        return Err("封面列表须包含当前全部已关联封面".to_string());
    }

    let linked_set: std::collections::HashSet<i64> = linked.iter().copied().collect();
    let mut seen = std::collections::HashSet::new();
    for id in asset_ids {
        if !linked_set.contains(id) {
            return Err(format!("资源 #{id} 不是本文封面"));
        }
        if !seen.insert(*id) {
            return Err("封面 ID 重复".to_string());
        }
    }

    for (sort_order, asset_id) in asset_ids.iter().enumerate() {
        let mut row = PostAsset::get_by_post_id_and_asset_id(db, &post_id, asset_id)
            .await
            .map_err(|_| format!("资源 #{asset_id} 与本文关联不存在"))?;
        row.update()
            .sort_order(sort_order as i64)
            .exec(db)
            .await
            .map_err(|e| format!("更新封面排序失败: {e}"))?;
    }

    Ok(())
}

pub async fn unlink_post_asset(
    db: &mut toasty::Db,
    storage: &StorageService,
    post_id: i64,
    asset_id: i64,
    purge: bool,
) -> Result<(), String> {
    let row = PostAsset::get_by_post_id_and_asset_id(db, &post_id, &asset_id)
        .await
        .map_err(|_| "文章与资源的关联不存在".to_string())?;

    row.delete()
        .exec(db)
        .await
        .map_err(|e| format!("解除关联失败: {e}"))?;

    if purge {
        let ref_count = count_asset_refs(db, asset_id).await?;
        if ref_count == 0 {
            delete_asset_record(db, storage, asset_id).await?;
        }
    }

    Ok(())
}

pub async fn delete_banner_asset_links(db: &mut toasty::Db, banner_id: i64) -> Result<(), String> {
    let rows = banner_asset_rows(db).await?;
    for row in rows.into_iter().filter(|r| r.banner_id == banner_id) {
        row.delete()
            .exec(db)
            .await
            .map_err(|e| format!("删除轮播图资源关联失败: {e}"))?;
    }
    Ok(())
}

pub async fn banner_assets_view(
    db: &mut toasty::Db,
    banner_id: i64,
    storage: &StorageService,
) -> Result<BannerAssetsView, String> {
    let links = banner_asset_rows(db).await?;
    let banner_links: Vec<_> = links
        .into_iter()
        .filter(|r| r.banner_id == banner_id)
        .collect();

    let mut image = None;
    let mut image_enabled = false;
    for link in banner_links {
        if link.role != BannerAssetRole::Image.as_str() {
            continue;
        }
        image_enabled = link.enabled == 1;
        let asset = get_asset(db, link.asset_id).await?;
        let ref_count = count_asset_refs(db, link.asset_id).await?;
        image = Some(asset_to_view(&asset, storage, ref_count));
        break;
    }

    Ok(BannerAssetsView {
        image,
        image_enabled,
    })
}

/// 将轮播图 image_url 与关联资源 URL 同步（公开页与 API 输出）
pub async fn sync_banner_image_url(
    db: &mut toasty::Db,
    storage: &StorageService,
    banner_id: i64,
) -> Result<(), String> {
    use super::Banner;

    let assets = banner_assets_view(db, banner_id, storage).await?;
    let url = if assets.image_enabled {
        assets
            .image
            .as_ref()
            .map(|a| a.url.clone())
            .unwrap_or_default()
    } else {
        String::new()
    };

    let mut banner = Banner::get_by_id(db, &banner_id)
        .await
        .map_err(|_| "轮播图不存在".to_string())?;
    banner
        .update()
        .image_url(url.as_str())
        .exec(db)
        .await
        .map_err(|e| format!("同步轮播图图片地址失败: {e}"))
}

pub async fn link_banner_asset(
    db: &mut toasty::Db,
    storage: &StorageService,
    banner_id: i64,
    input: &LinkBannerAssetInput,
) -> Result<(), String> {
    let role = BannerAssetRole::parse(&input.role)?;
    let asset = get_asset(db, input.asset_id).await?;

    if !role.accepts_purpose(&asset.purpose) {
        return Err(format!("资源用途「{}」不能作为轮播图", asset.purpose));
    }

    let rows = banner_asset_rows(db).await?;
    for row in rows
        .into_iter()
        .filter(|r| r.banner_id == banner_id && r.role == BannerAssetRole::Image.as_str())
    {
        row.delete()
            .exec(db)
            .await
            .map_err(|e| format!("解除旧轮播图关联失败: {e}"))?;
    }

    let sort_order = input.sort_order.unwrap_or(0);
    let enabled = if input.enabled.unwrap_or(true) { 1 } else { 0 };
    match BannerAsset::get_by_banner_id_and_asset_id(db, &banner_id, &input.asset_id).await {
        Ok(mut row) => {
            row.update()
                .role(role.as_str())
                .sort_order(sort_order)
                .enabled(enabled)
                .exec(db)
                .await
                .map_err(|e| format!("更新轮播图资源关联失败: {e}"))?;
        }
        Err(_) => {
            BannerAsset::create()
                .banner_id(banner_id)
                .asset_id(input.asset_id)
                .role(role.as_str())
                .sort_order(sort_order)
                .enabled(enabled)
                .exec(db)
                .await
                .map_err(|e| format!("创建轮播图资源关联失败: {e}"))?;
        }
    }

    sync_banner_image_url(db, storage, banner_id).await
}

pub async fn unlink_banner_asset(
    db: &mut toasty::Db,
    storage: &StorageService,
    banner_id: i64,
    asset_id: i64,
    purge: bool,
) -> Result<(), String> {
    let row = BannerAsset::get_by_banner_id_and_asset_id(db, &banner_id, &asset_id)
        .await
        .map_err(|_| "轮播图与资源的关联不存在".to_string())?;

    row.delete()
        .exec(db)
        .await
        .map_err(|e| format!("解除关联失败: {e}"))?;

    sync_banner_image_url(db, storage, banner_id).await?;

    if purge {
        let ref_count = count_asset_refs(db, asset_id).await?;
        if ref_count == 0 {
            delete_asset_record(db, storage, asset_id).await?;
        }
    }

    Ok(())
}

pub async fn set_banner_image_enabled(
    db: &mut toasty::Db,
    storage: &StorageService,
    banner_id: i64,
    enabled: bool,
) -> Result<(), String> {
    let rows = banner_asset_rows(db).await?;
    let Some(mut row) = rows
        .into_iter()
        .find(|r| r.banner_id == banner_id && r.role == BannerAssetRole::Image.as_str())
    else {
        return Err("轮播图尚未关联图片".to_string());
    };

    row.update()
        .enabled(if enabled { 1 } else { 0 })
        .exec(db)
        .await
        .map_err(|e| format!("更新图片启用状态失败: {e}"))?;

    sync_banner_image_url(db, storage, banner_id).await
}

const SEED_BANNER_FILES: &[(&str, &str)] = &[
    ("banner-1.png", "image/png"),
    ("banner-2.jpg", "image/jpeg"),
];

/// 确保预制轮播图资源已入库（文件须位于 `uploads/seed/{admin_user_id}/` 下）
pub async fn seed_default_banner_assets(
    db: &mut toasty::Db,
    storage: &StorageService,
) -> Result<std::collections::HashMap<String, Asset>, String> {
    use std::collections::HashMap;

    let admin_user_id = find_default_admin_user_id(db).await?;
    let mut map = HashMap::new();

    for (filename, mime) in SEED_BANNER_FILES {
        let storage_key = format!("seed/{admin_user_id}/{filename}");

        let asset = if let Some(existing) = find_asset_by_storage_key(db, &storage_key).await? {
            existing
        } else {
            let file_path = storage.config.local_dir.join(&storage_key);
            let size = std::fs::metadata(&file_path)
                .map_err(|e| format!("轮播图种子文件不存在（{}）: {e}", file_path.display()))?
                .len() as i64;

            let created = create_asset_record(
                db,
                &storage_key,
                filename,
                filename,
                mime,
                size,
                AssetPurpose::Banner,
            )
            .await?;

            println!(
                "[种子] 轮播图资源已入库（{}）",
                storage.public_url(&storage_key)
            );
            created
        };

        map.insert(filename.to_string(), asset);
    }

    Ok(map)
}

/// 确保默认轮播图 banner-1 资源已入库（兼容旧调用）
pub async fn seed_default_banner_asset(
    db: &mut toasty::Db,
    storage: &StorageService,
) -> Result<Asset, String> {
    let map = seed_default_banner_assets(db, storage).await?;
    map.get("banner-1.png")
        .cloned()
        .ok_or_else(|| "缺少轮播图种子资源 banner-1.png".to_string())
}

async fn find_asset_by_storage_key(
    db: &mut toasty::Db,
    storage_key: &str,
) -> Result<Option<Asset>, String> {
    let assets = Asset::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询资源失败: {e}"))?;
    Ok(assets.into_iter().find(|a| a.storage_key == storage_key))
}

/// 若轮播图尚未关联图片，则绑定默认种子资源并同步 image_url
pub async fn ensure_banner_seed_asset_link(
    db: &mut toasty::Db,
    storage: &StorageService,
    banner_id: i64,
    asset_id: i64,
) -> Result<(), String> {
    let links = banner_asset_rows(db).await?;
    if links
        .iter()
        .any(|r| r.banner_id == banner_id && r.role == BannerAssetRole::Image.as_str())
    {
        return Ok(());
    }

    link_banner_asset(
        db,
        storage,
        banner_id,
        &LinkBannerAssetInput {
            asset_id,
            role: BannerAssetRole::Image.as_str().to_string(),
            sort_order: Some(0),
            enabled: Some(true),
        },
    )
    .await?;

    println!("[种子] 默认轮播图已关联资源（banner_id={banner_id}）");
    Ok(())
}

const SEED_FRIEND_LINK_FILES: &[&str] = &[
    "friend-link-zhipu.jpg",
    "friend-link-vite.jpg",
    "friend-link-github.jpg",
    "friend-link-sqlite.jpg",
    "friend-link-markdown.jpg",
    "friend-link-element-plus.jpg",
    "friend-link-rocket.jpg",
    "friend-link-rust.jpg",
];

const SEED_FRIEND_LINK_MIME: &str = "image/jpeg";

/// 确保预制友链图片已入库（文件须位于 `uploads/seed/{admin_user_id}/friend-link-*.jpg`）
pub async fn seed_default_friend_link_assets(
    db: &mut toasty::Db,
    storage: &StorageService,
    admin_user_id: i64,
) -> Result<std::collections::HashMap<String, Asset>, String> {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for filename in SEED_FRIEND_LINK_FILES {
        let storage_key = format!("seed/{admin_user_id}/{filename}");

        let asset = if let Some(existing) = find_asset_by_storage_key(db, &storage_key).await? {
            existing
        } else {
            let file_path = storage.config.local_dir.join(&storage_key);
            let size = std::fs::metadata(&file_path)
                .map_err(|e| format!("友链种子文件不存在（{}）: {e}", file_path.display()))?
                .len() as i64;

            create_asset_record(
                db,
                &storage_key,
                filename,
                filename,
                SEED_FRIEND_LINK_MIME,
                size,
                AssetPurpose::FriendLink,
            )
            .await?
        };

        map.insert(filename.to_string(), asset);
    }

    println!(
        "[种子] 友链种子资源已就绪（{} 个）",
        SEED_FRIEND_LINK_FILES.len()
    );
    Ok(map)
}

const SEED_GALLERY_COUNT: usize = 6;
const SEED_GALLERY_MIME: &str = "image/jpeg";

/// 预制相册文章所需的种子图片资源
pub struct GallerySeedAssets {
    /// 封面图（gallery-1.jpg，purpose=cover）
    pub cover: Asset,
    /// 附件图（gallery-2.jpg … gallery-6.jpg，purpose=attachment）
    pub attachments: Vec<Asset>,
}

fn seed_gallery_storage_key(admin_user_id: i64, index: usize) -> String {
    format!("seed/{admin_user_id}/gallery-{index}.jpg")
}

async fn ensure_seed_gallery_asset(
    db: &mut toasty::Db,
    storage: &StorageService,
    admin_user_id: i64,
    index: usize,
    purpose: AssetPurpose,
) -> Result<Asset, String> {
    let filename = format!("gallery-{index}.jpg");
    let storage_key = seed_gallery_storage_key(admin_user_id, index);

    if let Some(asset) = find_asset_by_storage_key(db, &storage_key).await? {
        return Ok(asset);
    }

    let file_path = storage.config.local_dir.join(&storage_key);
    let size = std::fs::metadata(&file_path)
        .map_err(|e| format!("相册种子文件不存在（{}）: {e}", file_path.display()))?
        .len() as i64;

    let asset = create_asset_record(
        db,
        &storage_key,
        &filename,
        &filename,
        SEED_GALLERY_MIME,
        size,
        purpose,
    )
    .await?;

    println!(
        "[种子] 相册资源已入库（{}）",
        storage.public_url(&storage_key)
    );
    Ok(asset)
}

/// 确保 6 张相册种子图片已入库（文件须位于 `uploads/seed/{admin_user_id}/gallery-*.jpg`）
pub async fn seed_default_gallery_assets(
    db: &mut toasty::Db,
    storage: &StorageService,
) -> Result<GallerySeedAssets, String> {
    let admin_user_id = find_default_admin_user_id(db).await?;

    let cover = ensure_seed_gallery_asset(
        db,
        storage,
        admin_user_id,
        1,
        AssetPurpose::Cover,
    )
    .await?;

    let mut attachments = Vec::with_capacity(SEED_GALLERY_COUNT - 1);
    for index in 2..=SEED_GALLERY_COUNT {
        attachments.push(
            ensure_seed_gallery_asset(
                db,
                storage,
                admin_user_id,
                index,
                AssetPurpose::Attachment,
            )
            .await?,
        );
    }

    Ok(GallerySeedAssets { cover, attachments })
}
