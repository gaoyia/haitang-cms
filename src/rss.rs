//! 公开 RSS 2.0 输出

use serde::Serialize;

use crate::models::dict::load_dict_map;
use crate::models::locale::{encode_uri_path, locale_path, normalize_lang};
use crate::models::post::{PostMeta, PostView, is_post_publicly_visible, post_to_view_with_storage};
use crate::models::asset::now_unix;
use crate::storage::StorageService;

const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Debug, Serialize)]
pub struct RssFeedView {
    pub channel_title: String,
    pub channel_description: String,
    pub site_url: String,
    pub feed_url: String,
    pub site_name: String,
    pub lang: String,
    pub last_build_date: String,
    pub items: Vec<RssItemView>,
}

#[derive(Debug, Serialize)]
pub struct RssItemView {
    pub title: String,
    pub link: String,
    pub guid: String,
    pub description: String,
    pub pub_date: String,
}

/// 构建 `/<lang>/rss` 订阅上下文
pub async fn build_posts_rss_feed(
    db: &mut toasty::Db,
    storage: &StorageService,
    lang: &str,
    site_origin: &str,
) -> Result<RssFeedView, String> {
    let lang = normalize_lang(lang);
    let dict = load_dict_map(db, Some(&lang)).await;
    let site_name = dict
        .get("site_name")
        .cloned()
        .unwrap_or_else(|| "Haitang CMS".to_string());

    let origin = site_origin.trim_end_matches('/');
    let site_url = format!("{origin}{}", locale_path(&lang, ""));
    let feed_url = format!("{origin}{}", encode_uri_path(&locale_path(&lang, "rss")));

    let now = now_unix();
    let posts = PostMeta::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询文章失败: {e}"))?;

    let mut items = Vec::new();
    for meta in posts.into_iter().filter(|m| is_post_publicly_visible(m, now)) {
        let view = post_to_view_with_storage(db, &meta, Some(&lang), storage).await?;
        items.push(rss_item_from_post(&view, origin, &lang));
    }
    items.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

    let (channel_title, channel_description) = if lang == "en-us" {
        (
            format!("{site_name} — All Posts"),
            format!("Latest published posts from {site_name}"),
        )
    } else {
        (
            format!("{site_name} — 全部文章"),
            format!("{site_name} 站点已发布文章订阅"),
        )
    };

    Ok(RssFeedView {
        channel_title,
        channel_description,
        site_url,
        feed_url,
        site_name,
        lang,
        last_build_date: format_rss_pub_date(now),
        items,
    })
}

fn rss_item_from_post(view: &PostView, origin: &str, lang: &str) -> RssItemView {
    let link = absolute_post_url(origin, view, lang);
    let ts = if view.display_time > 0 {
        view.display_time
    } else {
        view.publish_time
    };
    RssItemView {
        title: view.title.clone(),
        link: link.clone(),
        guid: link,
        description: rss_item_description(&view.description, &view.content),
        pub_date: format_rss_pub_date(ts),
    }
}

fn absolute_post_url(origin: &str, view: &PostView, lang: &str) -> String {
    if !view.route_path.trim().is_empty() {
        format!("{origin}{}", encode_uri_path(view.route_path.trim()))
    } else {
        format!("{origin}/{lang}/posts/{}", view.id)
    }
}

fn rss_item_description(description: &str, content: &str) -> String {
    let base = if !description.trim().is_empty() {
        description.trim()
    } else {
        content.trim()
    };
    let flat: String = base
        .chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect();
    let trimmed = flat.split_whitespace().collect::<Vec<_>>().join(" ");
    truncate_chars(&trimmed, 500)
}

fn truncate_chars(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    format!("{}…", text.chars().take(max).collect::<String>())
}

/// Unix 秒 → RSS pubDate（UTC，RFC 822）
pub fn format_rss_pub_date(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    let (year, month, day, hour, min, sec, wday) = unix_to_utc(ts as u64);
    format!(
        "{}, {:02} {} {:04} {:02}:{:02}:{:02} +0000",
        WEEKDAYS[wday],
        day,
        MONTHS[(month.saturating_sub(1)) as usize],
        year,
        hour,
        min,
        sec
    )
}

fn unix_to_utc(ts: u64) -> (u32, u32, u32, u32, u32, u32, usize) {
    let days = ts / 86400;
    let sod = ts % 86400;
    let hour = sod / 3600;
    let min = (sod % 3600) / 60;
    let sec = sod % 60;

    let z = days as i64 + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = y + if m <= 2 { 1 } else { 0 };

    let wday = (days as i64 + 3).rem_euclid(7) as usize;
    (
        year as u32,
        m as u32,
        d as u32,
        hour as u32,
        min as u32,
        sec as u32,
        wday,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_rss_pub_date_epoch() {
        assert_eq!(
            format_rss_pub_date(0),
            ""
        );
        assert!(format_rss_pub_date(1_609_459_200).contains("2021"));
    }

    #[test]
    fn truncate_chars_respects_limit() {
        assert_eq!(truncate_chars("hello", 10), "hello");
        assert_eq!(truncate_chars("一二三四五六", 3), "一二三…");
    }
}
