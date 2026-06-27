use serde::{Deserialize, Serialize};

use super::BannerGroup;
use super::find_banner_group_by_code;

/// 轮播图模型
#[derive(Debug, Clone, toasty::Model)]
pub struct Banner {
    #[key]
    #[auto]
    pub id: i64,

    pub group_id: i64,

    pub title: String,

    /// 图片地址
    pub image_url: String,

    /// 点击跳转链接
    pub link_url: String,

    pub description: String,

    pub sort: i64,

    /// 0 = 禁用, 1 = 启用
    pub status: i64,
}

/// 创建轮播图请求
#[derive(Debug, Deserialize)]
pub struct CreateBanner {
    pub group_id: i64,
    pub title: String,
    pub image_url: Option<String>,
    pub link_url: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
}

/// 更新轮播图请求
#[derive(Debug, Deserialize)]
pub struct UpdateBanner {
    pub group_id: Option<i64>,
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub link_url: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
}

/// 轮播图序列化视图
#[derive(Debug, Clone, Serialize)]
pub struct BannerView {
    pub id: i64,
    pub group_id: i64,
    pub group_name: String,
    pub title: String,
    pub image_url: String,
    pub link_url: String,
    pub description: String,
    pub sort: i64,
    pub status: i64,
}

impl Banner {
    pub fn to_view(&self, group_name: &str) -> BannerView {
        BannerView {
            id: self.id,
            group_id: self.group_id,
            group_name: group_name.to_string(),
            title: self.title.clone(),
            image_url: self.image_url.clone(),
            link_url: self.link_url.clone(),
            description: self.description.clone(),
            sort: self.sort,
            status: self.status,
        }
    }
}

/// 加载轮播图组 ID → 名称映射
pub async fn load_banner_group_map(db: &mut toasty::Db) -> Result<std::collections::HashMap<i64, String>, String> {
    let groups = BannerGroup::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询轮播图组失败: {e}"))?;

    Ok(groups.into_iter().map(|g| (g.id, g.name)).collect())
}

/// 批量转换轮播图视图
pub async fn banners_to_views(db: &mut toasty::Db, banners: Vec<Banner>) -> Result<Vec<BannerView>, String> {
    let map = load_banner_group_map(db).await?;
    Ok(banners
        .into_iter()
        .map(|b| {
            let name = map.get(&b.group_id).cloned().unwrap_or_default();
            b.to_view(&name)
        })
        .collect())
}

/// 单条轮播图转视图
pub async fn banner_to_view(db: &mut toasty::Db, banner: Banner) -> Result<BannerView, String> {
    let map = load_banner_group_map(db).await?;
    let name = map.get(&banner.group_id).cloned().unwrap_or_default();
    Ok(banner.to_view(&name))
}

/// 校验轮播图组是否存在
pub async fn validate_banner_group_id(db: &mut toasty::Db, group_id: i64) -> Result<(), String> {
    match BannerGroup::get_by_id(db, &group_id).await {
        Ok(_) => Ok(()),
        Err(_) => Err("轮播图组不存在".to_string()),
    }
}

/// 判断轮播图组下是否仍有轮播图
pub fn group_has_banners(banners: &[Banner], group_id: i64) -> bool {
    banners.iter().any(|b| b.group_id == group_id)
}

/// 按组 code 加载启用的公开轮播图（已排序）
pub async fn load_public_banners_by_code(
    db: &mut toasty::Db,
    code: &str,
) -> Result<Vec<BannerView>, String> {
    let group = find_banner_group_by_code(db, code).await?;

    if group.status != 1 {
        return Ok(Vec::new());
    }

    let banners = Banner::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询轮播图失败: {e}"))?;

    let filtered: Vec<Banner> = filter_banners_by_group(banners, group.id)
        .into_iter()
        .filter(|b| b.status == 1)
        .collect();

    banners_to_views(db, filtered).await
}

/// 按组 ID 筛选并排序轮播图
pub fn filter_banners_by_group(mut banners: Vec<Banner>, group_id: i64) -> Vec<Banner> {
    banners.retain(|b| b.group_id == group_id);
    banners.sort_by_key(|b| b.sort);
    banners
}
