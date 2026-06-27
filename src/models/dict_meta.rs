/// 字典元数据（是否多语言、后台标签等，不含具体 value）
#[derive(Debug, Clone, toasty::Model)]
#[key(code)]
pub struct DictMeta {
    pub code: String,

    pub label: String,

    pub description: String,

    /// true = 各语言独立 value；false = 仅 (code, "") 一行
    pub translatable: bool,

    pub sort: i64,
}
