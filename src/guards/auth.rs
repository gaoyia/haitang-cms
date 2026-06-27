use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use rocket::State;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::models::Claims;

/// JWT 密钥配置（通过 Rocket manage 注入）
pub struct JwtConfig {
    pub secret: String,
}

/// Admin 授权请求守卫 — 从 Authorization: Bearer <token> 中提取并验证 JWT
pub struct AdminAuth {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminAuth {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jwt_config = match request.guard::<&State<JwtConfig>>().await {
            Outcome::Success(config) => config,
            _ => return Outcome::Error((Status::InternalServerError, "JWT 配置缺失")),
        };

        let token = match request.headers().get_one("Authorization") {
            Some(h) if h.starts_with("Bearer ") => &h[7..],
            _ => return Outcome::Error((Status::Unauthorized, "缺少 Authorization 头")),
        };

        let validation = Validation::new(Algorithm::HS256);
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_config.secret.as_bytes()),
            &validation,
        ) {
            Ok(data) => Outcome::Success(AdminAuth {
                claims: data.claims,
            }),
            Err(_) => Outcome::Error((Status::Unauthorized, "令牌无效或已过期")),
        }
    }
}
