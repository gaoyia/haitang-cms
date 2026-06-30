//! 轮播图 Hero  overlay 的 meta_json 结构与解析

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 单语言 Hero 文案（存于 meta_json 各语言键下）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BannerHeroLocale {
    #[serde(default)]
    pub badge: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub actions: Vec<BannerHeroAction>,
}

/// Hero 操作按钮（可为站内路径或外链）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BannerHeroAction {
    pub label: String,
    pub url: String,
    #[serde(default = "default_action_variant")]
    pub variant: String,
}

fn default_action_variant() -> String {
    "primary".to_string()
}

/// 校验并规范化轮播图 meta_json（顶层为语言码 → Hero 对象）
pub fn normalize_banner_meta_json(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok("{}".to_string());
    }
    let value: serde_json::Value =
        serde_json::from_str(trimmed).map_err(|e| format!("meta_json 不是合法 JSON: {e}"))?;
    let obj = value
        .as_object()
        .ok_or_else(|| "meta_json 必须是 JSON 对象".to_string())?;

    let mut normalized = serde_json::Map::new();
    for (lang, locale_val) in obj {
        if lang.starts_with('_') {
            normalized.insert(lang.clone(), locale_val.clone());
            continue;
        }
        let locale: BannerHeroLocale = serde_json::from_value(locale_val.clone())
            .map_err(|e| format!("语言「{lang}」Hero 结构无效: {e}"))?;
        normalized.insert(
            lang.clone(),
            serde_json::to_value(sanitize_hero_locale(locale))
                .map_err(|e| format!("序列化语言「{lang}」失败: {e}"))?,
        );
    }

    Ok(serde_json::Value::Object(normalized).to_string())
}

fn sanitize_hero_locale(mut locale: BannerHeroLocale) -> BannerHeroLocale {
    locale.badge = locale.badge.trim().to_string();
    locale.title = locale.title.trim().to_string();
    locale.description = locale.description.trim().to_string();
    locale.tags = locale
        .tags
        .into_iter()
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();
    locale.actions = locale
        .actions
        .into_iter()
        .map(|mut a| {
            a.label = a.label.trim().to_string();
            a.url = a.url.trim().to_string();
            a.variant = match a.variant.trim() {
                "secondary" => "secondary".to_string(),
                _ => "primary".to_string(),
            };
            a
        })
        .filter(|a| !a.label.is_empty() && !a.url.is_empty())
        .collect();
    locale
}

fn parse_meta_map(meta_json: &str) -> HashMap<String, BannerHeroLocale> {
    let trimmed = meta_json.trim();
    if trimmed.is_empty() {
        return HashMap::new();
    }
    let Ok(value) = serde_json::from_str::<serde_json::Value>(trimmed) else {
        return HashMap::new();
    };
    let Some(obj) = value.as_object() else {
        return HashMap::new();
    };

    let mut map = HashMap::new();
    for (lang, val) in obj {
        if lang.starts_with('_') {
            continue;
        }
        if let Ok(locale) = serde_json::from_value::<BannerHeroLocale>(val.clone()) {
            map.insert(lang.clone(), sanitize_hero_locale(locale));
        }
    }
    map
}

/// 按语言解析 Hero；缺失时回退默认语言，再回退 banner 基础字段
pub fn resolve_banner_hero(
    meta_json: &str,
    lang: &str,
    fallback_lang: &str,
    banner_title: &str,
    banner_description: &str,
) -> BannerHeroLocale {
    let map = parse_meta_map(meta_json);
    if let Some(locale) = map.get(lang).cloned().filter(has_hero_content) {
        return locale;
    }
    if lang != fallback_lang {
        if let Some(locale) = map
            .get(fallback_lang)
            .cloned()
            .filter(has_hero_content)
        {
            return locale;
        }
    }
    if let Some((_, locale)) = map.into_iter().find(|(_, v)| has_hero_content(v)) {
        return locale;
    }

    BannerHeroLocale {
        title: banner_title.trim().to_string(),
        description: banner_description.trim().to_string(),
        ..Default::default()
    }
}

fn has_hero_content(locale: &BannerHeroLocale) -> bool {
    !locale.badge.is_empty()
        || !locale.title.is_empty()
        || !locale.description.is_empty()
        || !locale.tags.is_empty()
        || !locale.actions.is_empty()
}

/// 种子：首页默认 Hero 文案（中英）
pub fn default_home_banner_meta_json() -> String {
    serde_json::json!({
        "zh-cn": {
            "badge": "让文字「如风流动」🎐「如星闪耀」✨",
            "title": "让数字内容如<em>「一缕晨光」</em>闪耀荧幕🎉",
            "description": "海棠 CMS 为追求优雅与性能的创作者提供结构化的工作空间。公开站点由 Tera 模板驱动，管理后台由 Vue 3 独立构建。",
            "tags": ["公开 API", "Vue 3", "Tera"],
            "actions": [
                { "label": "浏览新闻", "url": "/zh-cn/categories/news", "variant": "primary" },
                { "label": "了解更多", "url": "/zh-cn/about", "variant": "secondary" }
            ]
        },
        "en-us": {
            "badge": "Words flow like wind 🎐 stars shine bright ✨",
            "title": "Let digital content shine on screen like <em>morning light</em> 🎉",
            "description": "Haitang CMS offers a structured workspace for creators who care about elegance and performance. The public site is powered by Tera; the admin app is built with Vue 3.",
            "tags": ["Public API", "Vue 3", "Tera"],
            "actions": [
                { "label": "Browse news", "url": "/en-us/categories/news", "variant": "primary" },
                { "label": "Learn more", "url": "/en-us/about", "variant": "secondary" }
            ]
        }
    })
    .to_string()
}

/// 种子：首页第二条轮播 Hero 文案（春日白花主题，banner-2.jpg）
pub fn default_home_banner_2_meta_json() -> String {
    serde_json::json!({
        "zh-cn": {
            "badge": "自然 · 清新 · 春季",
            "title": "让创意火花如<em>「枝头新蕊」</em>徐徐绽放🌸",
            "description": "用海棠 CMS 记录每一次生长与发布，让创作像春天一样轻盈而有生命力。",
            "tags": ["自然", "清新", "春季"],
            "actions": [
                { "label": "浏览新闻", "url": "/zh-cn/categories/news", "variant": "primary" },
                { "label": "走进画廊", "url": "/zh-cn/categories/gallery", "variant": "secondary" }
            ]
        },
        "en-us": {
            "badge": "Nature · Fresh · Spring",
            "title": "Let creative sparks unfold slowly like <em>new buds on the branch</em> 🌸",
            "description": "Capture every season of growth and publishing with Haitang CMS—creation that feels as light and alive as spring.",
            "tags": ["Nature", "Fresh", "Spring"],
            "actions": [
                { "label": "Browse news", "url": "/en-us/categories/news", "variant": "primary" },
                { "label": "Visit gallery", "url": "/en-us/categories/gallery", "variant": "secondary" }
            ]
        }
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_banner_meta_json_parses_actions_and_tags() {
        let raw = r#"{
            "zh-cn": {
                "badge": "标签",
                "title": "标题",
                "tags": ["A", " B ", ""],
                "actions": [
                    { "label": "按钮", "url": "/zh-cn/news", "variant": "primary" },
                    { "label": "", "url": "/x" }
                ]
            }
        }"#;
        let normalized = normalize_banner_meta_json(raw).unwrap();
        let map = parse_meta_map(&normalized);
        let zh = map.get("zh-cn").unwrap();
        assert_eq!(zh.tags, vec!["A", "B"]);
        assert_eq!(zh.actions.len(), 1);
        assert_eq!(zh.actions[0].url, "/zh-cn/news");
    }

    #[test]
    fn resolve_banner_hero_falls_back_to_default_lang() {
        let meta = r#"{"en-us":{"badge":"EN"}}"#;
        let hero = resolve_banner_hero(meta, "zh-cn", "en-us", "fallback title", "desc");
        assert_eq!(hero.badge, "EN");
    }
}
