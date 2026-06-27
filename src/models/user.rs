use serde::{Deserialize, Serialize};

/// 用户模型 — Toasty ORM
#[derive(Debug, toasty::Model)]
pub struct User {
    #[key]
    #[auto]
    pub id: i64,

    #[unique]
    pub username: String,

    pub password_hash: String,

    pub nickname: String,

    pub email: String,

    /// 0 = 禁用, 1 = 启用
    pub status: i64,
}

/// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
}

/// 更新用户请求
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub status: Option<i64>,
    pub password: Option<String>,
}

/// 分配角色请求
#[derive(Debug, Deserialize)]
pub struct AssignRoles {
    pub role_ids: Vec<i64>,
}

/// 用户序列化视图
#[derive(Debug, Serialize)]
pub struct UserView {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub status: i64,
    pub role_ids: Vec<i64>,
}

impl User {
    pub fn to_view(&self, role_ids: Vec<i64>) -> UserView {
        UserView {
            id: self.id,
            username: self.username.clone(),
            nickname: self.nickname.clone(),
            email: self.email.clone(),
            status: self.status,
            role_ids,
        }
    }
}
