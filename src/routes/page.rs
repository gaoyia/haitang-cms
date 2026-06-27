use rocket::FromForm;

/// 管理端列表分页查询参数
#[derive(Debug, FromForm)]
pub struct PageQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl PageQuery {
    /// 解析分页参数，默认第 1 页、每页 10 条
    pub fn resolve(&self) -> (i64, i64) {
        let page = self.page.unwrap_or(1).max(1);
        let page_size = self.page_size.unwrap_or(10).clamp(1, 100);
        (page, page_size)
    }
}

/// 带 lang 的管理端列表查询参数
#[derive(Debug, FromForm)]
pub struct LangPageQuery {
    pub lang: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl LangPageQuery {
    pub fn resolve_page(&self) -> (i64, i64) {
        PageQuery {
            page: self.page,
            page_size: self.page_size,
        }
        .resolve()
    }
}
