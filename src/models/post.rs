use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::category::{load_category_map, validate_category_id};
use super::locale::{pick_i18n_row, resolve_locale};

/// 文章结构（不分语言）
#[derive(Debug, Clone, toasty::Model)]
pub struct PostMeta {
    #[key]
    #[auto]
    pub id: i64,

    pub category_id: i64,

    /// 0 = 草稿, 1 = 已发布
    pub status: i64,
}

/// 文章文案与 SEO 路径（按语言）
#[derive(Debug, Clone, toasty::Model)]
#[key(post_id, lang)]
pub struct PostI18n {
    pub post_id: i64,

    pub lang: String,

    pub title: String,

    pub description: String,

    pub content: String,

    /// 完整路径，如 /zh-cn/posts/hello
    pub route_path: String,

    /// 该语言下的标签，逗号分隔（已规范化，便于 SEO keywords）
    pub tags: String,
}

/// 创建文章
#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub tags: Option<String>,
    pub category_id: Option<i64>,
    pub route_path: Option<String>,
    pub lang: Option<String>,
    pub status: Option<i64>,
}

/// 更新文章
#[derive(Debug, Deserialize)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub tags: Option<String>,
    pub category_id: Option<i64>,
    pub route_path: Option<String>,
    pub lang: Option<String>,
    pub status: Option<i64>,
}

/// 文章视图（已 merge 当前语言）
#[derive(Debug, Serialize)]
pub struct PostView {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub content: String,
    pub tags: String,
    pub category_id: i64,
    pub category_name: String,
    pub route_path: String,
    pub status: i64,
    pub lang: String,
}

/// 管理端文章详情
#[derive(Debug, Serialize)]
pub struct PostDetailView {
    pub id: i64,
    pub category_id: i64,
    pub status: i64,
    pub translations: HashMap<String, PostI18nPayload>,
    #[serde(default)]
    pub covers: Vec<super::asset::AssetView>,
    #[serde(default)]
    pub attachments: Vec<super::asset::AssetView>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostI18nPayload {
    pub title: String,
    pub description: String,
    pub content: String,
    pub route_path: String,
    pub tags: String,
}

/// 规范化标签字符串：去空白、统一中英文逗号分隔符
pub fn normalize_tags(raw: &str) -> String {
    raw.split([',', '，'])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

/// 指定语言下文章 SEO 路径的固定前缀
pub fn post_route_path_prefix(lang: &str) -> String {
    format!("/{}/posts/", super::locale::normalize_lang(lang))
}

/// 校验并规范化文章 SEO 路径；空串合法
pub fn normalize_post_route_path(lang: &str, raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }

    let prefix = post_route_path_prefix(lang);
    let path = if trimmed.starts_with(&prefix) {
        trimmed.to_string()
    } else if !trimmed.contains('/') {
        format!("{prefix}{trimmed}")
    } else if let Some(idx) = trimmed.find("/posts/") {
        let slug = trimmed[idx + "/posts/".len()..].trim();
        format!("{prefix}{slug}")
    } else {
        return Err(format!("SEO 路径须以 {prefix} 开头"));
    };

    let slug = path.strip_prefix(&prefix).unwrap_or_default().trim();
    if slug.is_empty()
        || slug
            .chars()
            .any(|c| c.is_whitespace() || matches!(c, '#' | '?' | '/'))
    {
        return Err("SEO 路径 slug 不能为空，且不能包含空格、#、? 或 /".to_string());
    }

    Ok(format!("{prefix}{slug}"))
}

/// 同一语言下 SEO 路径已被其他文章占用
pub fn post_route_path_taken(lang: &str, route_path: &str, other_post_id: i64) -> String {
    format!("该语言（{lang}）下 SEO 路径「{route_path}」已被文章 #{other_post_id} 使用")
}

/// 公开访问时 slug 对应多篇已发布文章（数据异常）
pub fn post_route_path_ambiguous(route_path: &str, post_ids: &[i64]) -> String {
    let ids = post_ids
        .iter()
        .map(|id| format!("#{id}"))
        .collect::<Vec<_>>()
        .join("、");
    format!("SEO 路径「{route_path}」对应多篇文章（{ids}），请修改冲突文章的 SEO 路径")
}

/// 按公开 URL 段收集匹配的文章 ID（去重、保持发现顺序）
fn collect_post_ids_for_public_key(rows: &[PostI18n], lang: &str, key: &str) -> Vec<i64> {
    let lang = super::locale::normalize_lang(lang);
    let key = key.trim();
    if key.is_empty() {
        return vec![];
    }

    let expected = format!("/{lang}/posts/{key}");
    let prefix = post_route_path_prefix(&lang);
    let mut ids = Vec::new();

    for row in rows {
        if row.lang != lang {
            continue;
        }
        let matched = row.route_path == expected
            || (!row.route_path.is_empty()
                && row
                    .route_path
                    .strip_prefix(&prefix)
                    .is_some_and(|slug| slug == key));
        if matched && !ids.contains(&row.post_id) {
            ids.push(row.post_id);
        }
    }

    ids
}

/// 非空 `route_path` 在同一语言下不得与其他文章重复
pub async fn ensure_unique_post_route_path(
    db: &mut toasty::Db,
    lang: &str,
    route_path: &str,
    exclude_post_id: Option<i64>,
) -> Result<(), String> {
    if route_path.is_empty() {
        return Ok(());
    }

    let lang = super::locale::normalize_lang(lang);
    let rows = PostI18n::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询文章翻译失败: {e}"))?;

    for row in rows {
        if row.lang != lang || row.route_path.is_empty() {
            continue;
        }
        if row.route_path == route_path && exclude_post_id != Some(row.post_id) {
            return Err(post_route_path_taken(&lang, route_path, row.post_id));
        }
    }

    Ok(())
}

/// 从公开 URL 最后一段解析文章 ID：纯数字按 ID 查，否则按当前语言 `route_path` 匹配
pub async fn resolve_post_id_from_public_key(
    db: &mut toasty::Db,
    lang: &str,
    key: &str,
) -> Result<Option<i64>, String> {
    let lang = super::locale::normalize_lang(lang);
    let key = key.trim();
    if key.is_empty() {
        return Ok(None);
    }

    if let Ok(id) = key.parse::<i64>() {
        if PostMeta::get_by_id(db, &id).await.is_ok() {
            return Ok(Some(id));
        }
        return Ok(None);
    }

    let expected = format!("/{lang}/posts/{key}");
    let rows = PostI18n::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询文章翻译失败: {e}"))?;

    match collect_post_ids_for_public_key(&rows, &lang, key).as_slice() {
        [] => Ok(None),
        [id] => Ok(Some(*id)),
        ids => Err(post_route_path_ambiguous(&expected, ids)),
    }
}

pub async fn post_i18n_rows(db: &mut toasty::Db, post_id: i64) -> Result<Vec<PostI18n>, String> {
    let all = PostI18n::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询文章翻译失败: {e}"))?;
    Ok(all.into_iter().filter(|r| r.post_id == post_id).collect())
}

pub async fn create_post(
    db: &mut toasty::Db,
    input: &CreatePost,
    default_lang: &str,
) -> Result<PostMeta, String> {
    let lang = resolve_locale(input.lang.as_deref(), default_lang);
    let category_id = input.category_id.unwrap_or(0);
    validate_category_id(db, category_id).await?;

    let description = input.description.as_deref().unwrap_or("");
    let content = input.content.as_deref().unwrap_or("");
    let tags = normalize_tags(input.tags.as_deref().unwrap_or(""));
    let route_path = normalize_post_route_path(&lang, input.route_path.as_deref().unwrap_or(""))?;
    ensure_unique_post_route_path(db, &lang, &route_path, None).await?;
    let status = input.status.unwrap_or(1);

    let meta = PostMeta::create()
        .category_id(category_id)
        .status(status)
        .exec(db)
        .await
        .map_err(|e| format!("创建文章失败: {e}"))?;

    PostI18n::create()
        .post_id(meta.id)
        .lang(&lang)
        .title(&input.title)
        .description(description)
        .content(content)
        .route_path(&route_path)
        .tags(&tags)
        .exec(db)
        .await
        .map_err(|e| format!("创建文章翻译失败: {e}"))?;

    Ok(meta)
}

pub async fn post_to_view(
    db: &mut toasty::Db,
    meta: &PostMeta,
    lang: Option<&str>,
) -> Result<PostView, String> {
    let default = super::dict::get_site_default_locale(db).await;
    let resolved = resolve_locale(lang, &default);
    let rows = post_i18n_rows(db, meta.id).await?;
    let i18n = pick_i18n_row(&rows, &resolved, &default, |r| r.lang.as_str())
        .ok_or_else(|| "文章翻译不存在".to_string())?;

    let cat_map = load_category_map(db, Some(&resolved)).await?;
    let category_name = if meta.category_id == 0 {
        String::new()
    } else {
        cat_map.get(&meta.category_id).cloned().unwrap_or_default()
    };

    Ok(PostView {
        id: meta.id,
        title: i18n.title.clone(),
        description: i18n.description.clone(),
        content: i18n.content.clone(),
        tags: i18n.tags.clone(),
        category_id: meta.category_id,
        category_name,
        route_path: i18n.route_path.clone(),
        status: meta.status,
        lang: i18n.lang.clone(),
    })
}

pub async fn posts_to_views(
    db: &mut toasty::Db,
    metas: Vec<PostMeta>,
    lang: Option<&str>,
) -> Result<Vec<PostView>, String> {
    let mut views = Vec::new();
    for meta in metas {
        views.push(post_to_view(db, &meta, lang).await?);
    }
    Ok(views)
}

pub async fn post_detail_view(db: &mut toasty::Db, id: i64) -> Result<PostDetailView, String> {
    let meta = PostMeta::get_by_id(db, &id)
        .await
        .map_err(|_| "文章不存在".to_string())?;
    let rows = post_i18n_rows(db, id).await?;
    let translations = rows
        .into_iter()
        .map(|r| {
            (
                r.lang.clone(),
                PostI18nPayload {
                    title: r.title,
                    description: r.description,
                    content: r.content,
                    route_path: r.route_path,
                    tags: r.tags,
                },
            )
        })
        .collect();
    Ok(PostDetailView {
        id: meta.id,
        category_id: meta.category_id,
        status: meta.status,
        translations,
        covers: vec![],
        attachments: vec![],
    })
}

/// 文章翻译 upsert 入参
pub struct PostI18nUpsert<'a> {
    pub lang: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub content: &'a str,
    pub route_path: &'a str,
    pub tags: &'a str,
}

pub async fn upsert_post_i18n(
    db: &mut toasty::Db,
    post_id: i64,
    input: PostI18nUpsert<'_>,
) -> Result<(), String> {
    let PostI18nUpsert {
        lang,
        title,
        description,
        content,
        route_path,
        tags,
    } = input;
    let lang = super::locale::normalize_lang(lang);
    let tags = normalize_tags(tags);
    let route_path = normalize_post_route_path(&lang, route_path)?;
    ensure_unique_post_route_path(db, &lang, &route_path, Some(post_id)).await?;
    match PostI18n::get_by_post_id_and_lang(db, &post_id, &lang).await {
        Ok(mut row) => {
            row.update()
                .title(title)
                .description(description)
                .content(content)
                .route_path(&route_path)
                .tags(&tags)
                .exec(db)
                .await
                .map_err(|e| format!("更新文章翻译失败: {e}"))?;
        }
        Err(_) => {
            PostI18n::create()
                .post_id(post_id)
                .lang(&lang)
                .title(title)
                .description(description)
                .content(content)
                .route_path(&route_path)
                .tags(&tags)
                .exec(db)
                .await
                .map_err(|e| format!("创建文章翻译失败: {e}"))?;
        }
    }
    Ok(())
}

pub async fn delete_post(db: &mut toasty::Db, id: i64) -> Result<(), String> {
    let meta = PostMeta::get_by_id(db, &id)
        .await
        .map_err(|_| "文章不存在".to_string())?;

    super::asset::delete_post_asset_links(db, id).await?;

    let rows = post_i18n_rows(db, id).await?;
    for row in rows {
        row.delete()
            .exec(db)
            .await
            .map_err(|e| format!("删除文章翻译失败: {e}"))?;
    }
    meta.delete()
        .exec(db)
        .await
        .map_err(|e| format!("删除文章失败: {e}"))?;
    Ok(())
}

pub async fn count_posts_by_category(
    db: &mut toasty::Db,
    category_id: i64,
) -> Result<usize, String> {
    let posts = PostMeta::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询文章失败: {e}"))?;
    Ok(posts
        .iter()
        .filter(|p| p.category_id == category_id)
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_route_path_ok() {
        assert_eq!(normalize_post_route_path("zh-cn", "").unwrap(), "");
    }

    #[test]
    fn full_route_path_ok() {
        assert_eq!(
            normalize_post_route_path("zh-cn", "/zh-cn/posts/hello").unwrap(),
            "/zh-cn/posts/hello"
        );
    }

    #[test]
    fn slug_only_expanded() {
        assert_eq!(
            normalize_post_route_path("en-us", "my-slug").unwrap(),
            "/en-us/posts/my-slug"
        );
    }

    #[test]
    fn invalid_slug_rejected() {
        assert!(normalize_post_route_path("zh-cn", "/zh-cn/posts/a b").is_err());
    }

    #[test]
    fn foreign_prefix_renormalized_to_lang() {
        assert_eq!(
            normalize_post_route_path("zh-cn", "/en-us/posts/hello").unwrap(),
            "/zh-cn/posts/hello"
        );
    }

    fn test_row(post_id: i64, lang: &str, route_path: &str) -> PostI18n {
        PostI18n {
            post_id,
            lang: lang.to_string(),
            title: String::new(),
            description: String::new(),
            content: String::new(),
            route_path: route_path.to_string(),
            tags: String::new(),
        }
    }

    #[test]
    fn collect_ids_single_slug_match() {
        let rows = vec![test_row(3, "zh-cn", "/zh-cn/posts/hello")];
        assert_eq!(
            collect_post_ids_for_public_key(&rows, "zh-cn", "hello"),
            vec![3]
        );
    }

    #[test]
    fn collect_ids_duplicate_route_path() {
        let rows = vec![
            test_row(1, "zh-cn", "/zh-cn/posts/hello"),
            test_row(2, "zh-cn", "/zh-cn/posts/hello"),
        ];
        assert_eq!(
            collect_post_ids_for_public_key(&rows, "zh-cn", "hello"),
            vec![1, 2]
        );
    }

    #[test]
    fn collect_ids_respects_lang() {
        let rows = vec![
            test_row(1, "zh-cn", "/zh-cn/posts/hello"),
            test_row(2, "en-us", "/en-us/posts/hello"),
        ];
        assert_eq!(
            collect_post_ids_for_public_key(&rows, "zh-cn", "hello"),
            vec![1]
        );
    }
}
