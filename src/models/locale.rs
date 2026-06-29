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

/// 从当前公开页路径提取语言切换后缀（去掉首段 locale）
///
/// 例如 `/en-us/posts/my-slug` → `posts/my-slug`，`/en-us/` → ``。
pub fn locale_switch_suffix(current_path: &str) -> String {
    let trimmed = current_path.trim();
    if trimmed.is_empty() || trimmed == "/" {
        return String::new();
    }
    let without_leading = trimmed.trim_start_matches('/');
    match without_leading.find('/') {
        Some(idx) => without_leading[idx + 1..].to_string(),
        None => String::new(),
    }
}

/// 将站内路径各段做百分号编码，供 Redirect Location 使用（含中文 slug）
pub fn encode_uri_path(path: &str) -> String {
    path.split('/')
        .map(encode_uri_path_segment)
        .collect::<Vec<_>>()
        .join("/")
}

fn encode_uri_path_segment(segment: &str) -> String {
    if segment.is_empty() {
        return String::new();
    }
    segment
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
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
        "post-detail" if en => "Article",
        "post-detail" => "文章详情",
        "category-archive" if en => "Category",
        "category-archive" => "分类归档",
        "about" if en => "About",
        "about" => "关于我们",
        _ if en => "Home",
        _ => "首页",
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_uri_path_keeps_ascii_segments() {
        assert_eq!(encode_uri_path("/en-us/posts/hello"), "/en-us/posts/hello");
    }

    #[test]
    fn encode_uri_path_encodes_unicode_slug() {
        assert_eq!(
            encode_uri_path("/en-us/posts/测试测试"),
            "/en-us/posts/%E6%B5%8B%E8%AF%95%E6%B5%8B%E8%AF%95"
        );
    }

    #[test]
    fn locale_switch_suffix_strips_locale_prefix() {
        assert_eq!(locale_switch_suffix("/en-us/posts/hello"), "posts/hello");
        assert_eq!(locale_switch_suffix("/zh-cn/categories/12"), "categories/12");
        assert_eq!(locale_switch_suffix("/en-us/"), "");
        assert_eq!(locale_switch_suffix("/en-us"), "");
    }

    #[test]
    fn locale_path_with_nested_slug() {
        assert_eq!(
            locale_path("zh-cn", "posts/my-slug"),
            "/zh-cn/posts/my-slug"
        );
    }
}
