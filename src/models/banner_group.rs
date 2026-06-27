use serde::{Deserialize, Serialize};

/// 轮播图组模型
#[derive(Debug, toasty::Model)]
pub struct BannerGroup {
    #[key]
    #[auto]
    pub id: i64,

    pub name: String,

    #[unique]
    pub code: String,

    pub description: String,

    pub sort: i64,

    /// 0 = 禁用, 1 = 启用
    pub status: i64,
}

/// 创建轮播图组请求
#[derive(Debug, Deserialize)]
pub struct CreateBannerGroup {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
}

/// 更新轮播图组请求
#[derive(Debug, Deserialize)]
pub struct UpdateBannerGroup {
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
}

/// 轮播图组序列化视图
#[derive(Debug, Clone, Serialize)]
pub struct BannerGroupView {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: String,
    pub sort: i64,
    pub status: i64,
}

impl BannerGroup {
    pub fn to_view(&self) -> BannerGroupView {
        BannerGroupView {
            id: self.id,
            name: self.name.clone(),
            code: self.code.clone(),
            description: self.description.clone(),
            sort: self.sort,
            status: self.status,
        }
    }
}

/// 从数据库按 code 查找轮播图组
pub async fn find_banner_group_by_code(
    db: &mut toasty::Db,
    code: &str,
) -> Result<BannerGroup, String> {
    let groups = BannerGroup::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询轮播图组失败: {e}"))?;

    groups
        .into_iter()
        .find(|g| g.code == code)
        .ok_or_else(|| "轮播图组不存在".to_string())
}
