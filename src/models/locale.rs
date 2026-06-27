/// 全局字典项使用的 lang 哨兵（不分语言）
pub const LANG_GLOBAL: &str = "";

/// 系统内置默认语言（种子未写入时的兜底）
pub const DEFAULT_LOCALE: &str = "zh-cn";

/// 将前端或请求中的语言码规范为小写 BCP 47 风格（如 zh-cn、en-us）
pub fn normalize_lang(input: &str) -> String {
    let s = input.trim();
    if s.is_empty() {
        return DEFAULT_LOCALE.to_string();
    }
    match s.to_lowercase().as_str() {
        "zh" | "zh-cn" | "zh-hans" => "zh-cn".to_string(),
        "en" | "en-us" | "en-gb" => "en-us".to_string(),
        other => other.to_string(),
    }
}

/// 解析最终使用的 locale：请求语言 → 站点默认 → 内置默认
pub fn resolve_locale(request_lang: Option<&str>, site_default: &str) -> String {
    request_lang
        .map(normalize_lang)
        .filter(|l| !l.is_empty())
        .unwrap_or_else(|| normalize_lang(site_default))
}

/// 解析字典项 `site_locales`（逗号分隔）
pub fn parse_locale_list(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(normalize_lang)
        .filter(|s| !s.is_empty())
        .collect()
}

/// 判断语言是否在站点支持列表中
pub fn is_supported_locale(lang: &str, supported: &[String]) -> bool {
    supported.iter().any(|l| l == lang)
}

/// 公开页 URL 前缀，如 `/zh-cn`
pub fn locale_prefix(lang: &str) -> String {
    format!("/{}", normalize_lang(lang))
}

/// 构建带语言前缀的公开页路径
pub fn locale_path(lang: &str, page_slug: &str) -> String {
    let prefix = locale_prefix(lang);
    if page_slug.is_empty() {
        format!("{prefix}/")
    } else {
        format!("{prefix}/{page_slug}")
    }
}

/// BCP 47 转 HTML lang 属性（如 zh-cn → zh-CN）
pub fn html_lang_attr(lang: &str) -> String {
    match normalize_lang(lang).as_str() {
        "zh-cn" => "zh-CN".to_string(),
        "en-us" => "en-US".to_string(),
        other => other.to_string(),
    }
}

/// 公开页标题（按语言与页面 slug）
pub fn public_page_title(lang: &str, page_slug: &str) -> &'static str {
    let en = lang == "en-us";
    match page_slug {
        "posts" if en => "Posts",
        "posts" => "最新文章",
        "about" if en => "About",
        "about" => "关于我们",
        _ if en => "Home",
        _ => "首页",
    }
}

/// 管理后台入口标题（无语言前缀路由）
pub fn admin_page_title() -> &'static str {
    "管理后台"
}

/// 从多语言行中按 lang 选取，支持 fallback 到 default_lang
pub fn pick_i18n_row<'a, T>(
    rows: &'a [T],
    lang: &str,
    default_lang: &str,
    get_lang: impl Fn(&T) -> &str,
) -> Option<&'a T> {
    rows.iter()
        .find(|r| get_lang(r) == lang)
        .or_else(|| rows.iter().find(|r| get_lang(r) == default_lang))
        .or_else(|| rows.first())
}
