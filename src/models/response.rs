use serde::Serialize;

/// 分页列表响应
#[derive(Debug, Clone, Serialize)]
pub struct PageResult<T: Serialize> {
    pub list: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

/// 对内存列表做分页切片（管理端列表通用）
pub fn paginate_vec<T: Serialize>(items: Vec<T>, page: i64, page_size: i64) -> PageResult<T> {
    let page = page.max(1);
    let page_size = page_size.clamp(1, 100);
    let total = items.len() as i64;
    let start = ((page - 1) * page_size) as usize;

    let list: Vec<T> = items
        .into_iter()
        .skip(start)
        .take(page_size as usize)
        .collect();

    PageResult {
        list,
        total,
        page,
        page_size,
    }
}

/// 通用 API 响应
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "ok".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
}
