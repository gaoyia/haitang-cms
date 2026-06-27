/// 字典值（按 code + lang 存储）
#[derive(Debug, Clone, toasty::Model)]
#[key(code, lang)]
pub struct DictValue {
    pub code: String,

    /// 多语言项为 zh-cn / en-us；全局项为 ""
    pub lang: String,

    pub value: String,
}
