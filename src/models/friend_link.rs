use serde::{Deserialize, Serialize};

/// 友情链接
#[derive(Debug, Clone, toasty::Model)]
pub struct FriendLink {
    #[key]
    #[auto]
    pub id: i64,

    /// 展示名称（alt / 无障碍）
    pub title: String,

    /// 跳转 URL
    pub url: String,

    /// 友链图片地址（通常为 friend_link 资源公开 URL）
    pub image_url: String,

    pub sort: i64,

    /// 0 = 禁用, 1 = 启用
    pub status: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateFriendLink {
    pub title: String,
    pub url: String,
    pub image_url: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFriendLink {
    pub title: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FriendLinkView {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub image_url: String,
    pub sort: i64,
    pub status: i64,
}

/// 公开页友链（Tera / 公开 API）
#[derive(Debug, Clone, Serialize)]
pub struct PublicFriendLink {
    pub title: String,
    pub url: String,
    pub image_url: String,
}

impl FriendLink {
    pub fn to_view(&self) -> FriendLinkView {
        FriendLinkView {
            id: self.id,
            title: self.title.clone(),
            url: self.url.clone(),
            image_url: self.image_url.clone(),
            sort: self.sort,
            status: self.status,
        }
    }

    pub fn to_public(&self) -> PublicFriendLink {
        PublicFriendLink {
            title: self.title.clone(),
            url: self.url.clone(),
            image_url: self.image_url.clone(),
        }
    }
}

pub fn validate_friend_link_url(raw: &str) -> Result<String, String> {
    let url = raw.trim();
    if url.is_empty() {
        return Err("链接不能为空".to_string());
    }
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err("链接须以 http:// 或 https:// 开头".to_string());
    }
    Ok(url.to_string())
}

pub fn validate_friend_link_image_url(raw: &str) -> Result<String, String> {
    let url = raw.trim();
    if url.is_empty() {
        return Err("友链图片不能为空".to_string());
    }
    Ok(url.to_string())
}

/// 获取启用的公开友链（按 sort 升序）
pub async fn get_public_friend_links(db: &mut toasty::Db) -> Vec<PublicFriendLink> {
    let rows = match FriendLink::all().exec(db).await {
        Ok(list) => list,
        Err(_) => return Vec::new(),
    };

    let mut enabled: Vec<_> = rows.into_iter().filter(|r| r.status == 1).collect();
    enabled.sort_by_key(|r| r.sort);
    enabled.into_iter().map(|r| r.to_public()).collect()
}

pub fn friend_links_to_views(links: Vec<FriendLink>) -> Vec<FriendLinkView> {
    let mut sorted = links;
    sorted.sort_by_key(|r| r.sort);
    sorted.into_iter().map(|r| r.to_view()).collect()
}

/// 种子：预制友链条目（依赖 seed/{admin_id}/friend-link-*.jpg 与对应 assets 记录）
pub async fn seed_default_friend_links(
    db: &mut toasty::Db,
    storage: &crate::storage::StorageService,
) -> Result<(), String> {
    use super::asset::seed_default_friend_link_assets;
    use super::user::find_default_admin_user_id;

    let existing = FriendLink::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询友链失败: {e}"))?;
    if !existing.is_empty() {
        return Ok(());
    }

    let admin_user_id = find_default_admin_user_id(db).await?;
    let assets = seed_default_friend_link_assets(db, storage, admin_user_id).await?;

    const SEEDS: &[(&str, &str, &str, i64)] = &[
        ("智谱", "https://www.zhipuai.cn/", "friend-link-zhipu.jpg", 0),
        ("Vite", "https://vite.dev/", "friend-link-vite.jpg", 10),
        ("GitHub", "https://github.com/", "friend-link-github.jpg", 20),
        ("SQLite", "https://www.sqlite.org/", "friend-link-sqlite.jpg", 30),
        ("Markdown", "https://commonmark.org/", "friend-link-markdown.jpg", 40),
        (
            "Element Plus",
            "https://element-plus.org/",
            "friend-link-element-plus.jpg",
            50,
        ),
        ("Rocket", "https://rocket.rs/", "friend-link-rocket.jpg", 60),
        ("Rust", "https://www.rust-lang.org/", "friend-link-rust.jpg", 70),
    ];

    println!("[种子] 创建默认友链...");

    for (title, url, filename, sort) in SEEDS {
        let asset = assets
            .get(*filename)
            .ok_or_else(|| format!("缺少友链种子资源: {filename}"))?;
        let image_url = storage.public_url(&asset.storage_key);

        FriendLink::create()
            .title(*title)
            .url(*url)
            .image_url(&image_url)
            .sort(*sort)
            .status(1)
            .exec(db)
            .await
            .map_err(|e| format!("创建友链「{title}」失败: {e}"))?;
    }

    println!("[种子] 默认友链已创建（{} 条）", SEEDS.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_friend_link_url_requires_http() {
        assert!(validate_friend_link_url("https://a.com").is_ok());
        assert!(validate_friend_link_url("/foo").is_err());
        assert!(validate_friend_link_url("").is_err());
    }
}
