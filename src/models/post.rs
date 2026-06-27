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

    pub tags: String,

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
    pub tags: String,
    pub status: i64,
    pub translations: HashMap<String, PostI18nPayload>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostI18nPayload {
    pub title: String,
    pub description: String,
    pub content: String,
    pub route_path: String,
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
    let tags = input.tags.as_deref().unwrap_or("");
    let route_path = input.route_path.as_deref().unwrap_or("");
    let status = input.status.unwrap_or(1);

    let meta = PostMeta::create()
        .category_id(category_id)
        .tags(tags)
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
        .route_path(route_path)
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
        tags: meta.tags.clone(),
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
                },
            )
        })
        .collect();
    Ok(PostDetailView {
        id: meta.id,
        category_id: meta.category_id,
        tags: meta.tags,
        status: meta.status,
        translations,
    })
}

pub async fn upsert_post_i18n(
    db: &mut toasty::Db,
    post_id: i64,
    lang: &str,
    title: &str,
    description: &str,
    content: &str,
    route_path: &str,
) -> Result<(), String> {
    let lang = super::locale::normalize_lang(lang);
    match PostI18n::get_by_post_id_and_lang(db, &post_id, &lang).await {
        Ok(mut row) => {
            row.update()
                .title(title)
                .description(description)
                .content(content)
                .route_path(route_path)
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
                .route_path(route_path)
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
