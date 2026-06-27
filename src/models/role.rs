use serde::{Deserialize, Serialize};

/// 角色模型 — Toasty ORM
#[derive(Debug, toasty::Model)]
pub struct Role {
    #[key]
    #[auto]
    pub id: i64,

    #[unique]
    pub name: String,

    pub description: String,

    /// 逗号分隔的权限代码，如 "post:create,post:update,user:read"
    pub permissions: String,
}

/// 创建角色请求
#[derive(Debug, Deserialize)]
pub struct CreateRole {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<String>>,
}

/// 更新角色请求
#[derive(Debug, Deserialize)]
pub struct UpdateRole {
    pub name: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<Vec<String>>,
}

/// 角色序列化视图
#[derive(Debug, Serialize)]
pub struct RoleView {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
}

impl Role {
    pub fn to_view(&self) -> RoleView {
        let permissions: Vec<String> = if self.permissions.is_empty() {
            Vec::new()
        } else {
            self.permissions.split(',').map(|s| s.to_string()).collect()
        };
        RoleView {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            permissions,
        }
    }

    pub fn permissions_vec(&self) -> Vec<String> {
        if self.permissions.is_empty() {
            Vec::new()
        } else {
            self.permissions.split(',').map(|s| s.to_string()).collect()
        }
    }
}
