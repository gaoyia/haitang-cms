use serde::{Deserialize, Serialize};

/// JWT 令牌声明
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// 用户标识（用户名）
    pub sub: String,
    /// 用户 ID
    pub user_id: i64,
    /// 角色名称列表
    pub roles: Vec<String>,
    /// 权限代码列表
    pub permissions: Vec<String>,
    /// 过期时间 (Unix timestamp)
    pub exp: usize,
}

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: LoginUserInfo,
}

/// 登录用户信息
#[derive(Debug, Serialize)]
pub struct LoginUserInfo {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
