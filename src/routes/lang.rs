use rocket::FromForm;

/// 公开/管理 API 共用的 lang 查询参数
#[derive(Debug, FromForm)]
pub struct LangQuery {
    pub lang: Option<String>,
}
