use serde::{Deserialize, Serialize};

use super::admin_sidebar::{ADMIN_SIDEBAR_CODE, is_admin_sidebar_code};

/// 菜单组模型（可编辑，不含内置 admin_sidebar）
#[derive(Debug, toasty::Model)]
pub struct MenuGroup {
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

/// 创建菜单组请求
#[derive(Debug, Deserialize)]
pub struct CreateMenuGroup {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
}

/// 更新菜单组请求
#[derive(Debug, Deserialize)]
pub struct UpdateMenuGroup {
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
}

/// 菜单组序列化视图
#[derive(Debug, Clone, Serialize)]
pub struct MenuGroupView {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: String,
    pub sort: i64,
    pub status: i64,
    /// 内置菜单组为 true，不可增删改
    pub readonly: bool,
}

impl MenuGroup {
    pub fn to_view(&self) -> MenuGroupView {
        MenuGroupView {
            id: self.id,
            name: self.name.clone(),
            code: self.code.clone(),
            description: self.description.clone(),
            sort: self.sort,
            status: self.status,
            readonly: false,
        }
    }
}

/// 内置后台侧边栏菜单组视图（虚拟，id = 0）
pub fn admin_sidebar_group_view() -> MenuGroupView {
    MenuGroupView {
        id: 0,
        name: "后台侧边栏".to_string(),
        code: ADMIN_SIDEBAR_CODE.to_string(),
        description: "管理后台左侧导航（系统内置，只读）".to_string(),
        sort: -1,
        status: 1,
        readonly: true,
    }
}

/// 校验菜单组 code 是否可用于创建/更新
pub fn validate_menu_group_code(code: &str) -> Result<(), String> {
    if is_admin_sidebar_code(code) {
        return Err(format!("code `{code}` 为系统保留，不可使用"));
    }
    Ok(())
}

/// 从数据库按 code 查找菜单组
pub async fn find_menu_group_by_code(db: &mut toasty::Db, code: &str) -> Result<MenuGroup, String> {
    if is_admin_sidebar_code(code) {
        return Err("后台侧边栏为内置菜单组".to_string());
    }

    let groups = MenuGroup::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询菜单组失败: {e}"))?;

    groups
        .into_iter()
        .find(|g| g.code == code)
        .ok_or_else(|| "菜单组不存在".to_string())
}
