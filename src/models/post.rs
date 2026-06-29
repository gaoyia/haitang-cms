use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::asset::{
    ensure_post_cover_link, link_post_asset, seed_default_gallery_assets, LinkPostAssetInput,
};
use super::category::{
    category_public_url, category_templates, load_category_map, resolve_category_id_from_public_key,
    validate_post_category_id,
};
use super::dict::get_site_default_locale;
use super::locale::{pick_i18n_row, resolve_locale};
use crate::storage::{PostAssetRole, StorageService};

/// 文章结构（不分语言）
#[derive(Debug, Clone, toasty::Model)]
pub struct PostMeta {
    #[key]
    #[auto]
    pub id: i64,

    pub category_id: i64,

    /// 0 = 草稿, 1 = 已发布
    pub status: i64,

    /// 创建时间（Unix 秒）
    pub created_at: i64,

    /// 最后内容变更时间（不含仅改状态）
    pub updated_at: i64,

    /// 首次实际公开时间；未到计划发布时间或未发布过为 0
    pub published_at: i64,

    /// 计划发布时间（Unix 秒）；已发布且留空时等于保存时刻
    pub publish_time: i64,

    /// 前台展示时间
    pub display_time: i64,

    /// 扩展字段 JSON（如招聘岗位薪资、地点等）
    pub meta_json: String,
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
    /// 前台展示时间（Unix 秒）；缺省或 0 表示使用服务端当前时间
    pub display_time: Option<i64>,
    /// 计划发布时间（Unix 秒）；缺省或 0 时，已发布用当前时间，草稿为 0
    pub publish_time: Option<i64>,
    pub meta_json: Option<String>,
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
    /// 前台展示时间（Unix 秒）；缺省或 0 表示使用服务端当前时间
    pub display_time: Option<i64>,
    /// 计划发布时间（Unix 秒）；缺省或 0 时，已发布用当前时间，草稿为 0
    pub publish_time: Option<i64>,
    pub meta_json: Option<String>,
}

/// 校验并规范化文章 meta_json（须为 JSON 对象）
pub fn normalize_post_meta_json(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok("{}".to_string());
    }
    let value: serde_json::Value =
        serde_json::from_str(trimmed).map_err(|e| format!("meta_json 不是合法 JSON: {e}"))?;
    if !value.is_object() {
        return Err("meta_json 必须是 JSON 对象".to_string());
    }
    Ok(value.to_string())
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
    /// 所属分类公开页 URL（SEO 路径或 /{lang}/categories/{id}）
    pub category_route_path: String,
    pub route_path: String,
    pub status: i64,
    pub lang: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub published_at: i64,
    pub publish_time: i64,
    pub display_time: i64,
    #[serde(default)]
    pub covers: Vec<super::asset::AssetView>,
    #[serde(default)]
    pub attachments: Vec<super::asset::AssetView>,
    pub list_template: String,
    pub detail_template: String,
    pub meta_json: String,
}

/// 管理端文章详情
#[derive(Debug, Serialize)]
pub struct PostDetailView {
    pub id: i64,
    pub category_id: i64,
    pub status: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub published_at: i64,
    pub publish_time: i64,
    pub display_time: i64,
    pub meta_json: String,
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

pub fn normalize_tags(raw: &str) -> String {
    raw.split([',', '，'])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

/// 解析写入用的展示时间：缺省或 0 表示使用服务端当前时间
fn resolve_display_time(raw: Option<i64>, now: i64) -> i64 {
    match raw {
        None | Some(0) => now,
        Some(ts) => ts,
    }
}

/// 解析计划发布时间：缺省或 0 时，已发布用当前时间，草稿为 0
fn resolve_publish_time(raw: Option<i64>, status: i64, now: i64) -> i64 {
    match raw {
        None | Some(0) if status == 1 => now,
        None | Some(0) => 0,
        Some(ts) => ts,
    }
}

/// 文章是否对访客可见（已发布且到达计划发布时间）
pub fn is_post_publicly_visible(meta: &PostMeta, now: i64) -> bool {
    meta.status == 1 && meta.publish_time > 0 && meta.publish_time <= now
}

/// 首次实际公开时间：已到计划发布时刻时写入
fn resolve_published_at(status: i64, publish_time: i64, now: i64, existing: i64) -> i64 {
    if existing > 0 {
        return existing;
    }
    if status == 1 && publish_time > 0 && publish_time <= now {
        now
    } else {
        0
    }
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

async fn touch_post_updated_at(db: &mut toasty::Db, post_id: i64) -> Result<(), String> {
    let mut meta = PostMeta::get_by_id(db, &post_id)
        .await
        .map_err(|_| "文章不存在".to_string())?;
    meta.update()
        .updated_at(super::asset::now_unix())
        .exec(db)
        .await
        .map_err(|e| format!("更新文章时间失败: {e}"))?;
    Ok(())
}

pub async fn create_post(
    db: &mut toasty::Db,
    input: &CreatePost,
    default_lang: &str,
) -> Result<PostMeta, String> {
    let lang = resolve_locale(input.lang.as_deref(), default_lang);
    let category_id = input
        .category_id
        .filter(|&id| id > 0)
        .ok_or_else(|| "请选择分类".to_string())?;
    validate_post_category_id(db, category_id).await?;

    let description = input.description.as_deref().unwrap_or("");
    let content = input.content.as_deref().unwrap_or("");
    let tags = normalize_tags(input.tags.as_deref().unwrap_or(""));
    let route_path = normalize_post_route_path(&lang, input.route_path.as_deref().unwrap_or(""))?;
    ensure_unique_post_route_path(db, &lang, &route_path, None).await?;
    let status = input.status.unwrap_or(1);
    let now = super::asset::now_unix();
    let display_time = resolve_display_time(input.display_time, now);
    let publish_time = resolve_publish_time(input.publish_time, status, now);
    let published_at = resolve_published_at(status, publish_time, now, 0);
    let meta_json = normalize_post_meta_json(input.meta_json.as_deref().unwrap_or("{}"))?;

    let meta = PostMeta::create()
        .category_id(category_id)
        .status(status)
        .created_at(now)
        .updated_at(now)
        .published_at(published_at)
        .publish_time(publish_time)
        .display_time(display_time)
        .meta_json(&meta_json)
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
    post_to_view_inner(db, meta, lang, None).await
}

pub async fn post_to_view_with_storage(
    db: &mut toasty::Db,
    meta: &PostMeta,
    lang: Option<&str>,
    storage: &crate::storage::StorageService,
) -> Result<PostView, String> {
    post_to_view_inner(db, meta, lang, Some(storage)).await
}

async fn post_to_view_inner(
    db: &mut toasty::Db,
    meta: &PostMeta,
    lang: Option<&str>,
    storage: Option<&crate::storage::StorageService>,
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

    let category_route_path =
        category_public_url(db, meta.category_id, &resolved, &default).await;

    let (list_template, detail_template) = category_templates(db, meta.category_id).await?;

    let (covers, attachments) = if let Some(storage) = storage {
        let assets = super::asset::post_assets_view(db, meta.id, storage).await?;
        (assets.covers, assets.attachments)
    } else {
        (vec![], vec![])
    };

    Ok(PostView {
        id: meta.id,
        title: i18n.title.clone(),
        description: i18n.description.clone(),
        content: i18n.content.clone(),
        tags: i18n.tags.clone(),
        category_id: meta.category_id,
        category_name,
        category_route_path,
        route_path: i18n.route_path.clone(),
        status: meta.status,
        lang: i18n.lang.clone(),
        created_at: meta.created_at,
        updated_at: meta.updated_at,
        published_at: meta.published_at,
        publish_time: meta.publish_time,
        display_time: meta.display_time,
        covers,
        attachments,
        list_template,
        detail_template,
        meta_json: meta.meta_json.clone(),
    })
}

pub async fn posts_to_views(
    db: &mut toasty::Db,
    mut metas: Vec<PostMeta>,
    lang: Option<&str>,
) -> Result<Vec<PostView>, String> {
    metas.sort_by_key(|m| std::cmp::Reverse(m.display_time));
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
        created_at: meta.created_at,
        updated_at: meta.updated_at,
        published_at: meta.published_at,
        publish_time: meta.publish_time,
        display_time: meta.display_time,
        meta_json: meta.meta_json.clone(),
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
    touch_post_updated_at(db, post_id).await?;
    Ok(())
}

/// 更新文章元数据与翻译；`updated_at` 在除「仅改状态」外的变更时刷新
pub async fn update_post(
    db: &mut toasty::Db,
    id: i64,
    input: &UpdatePost,
    default_lang: &str,
) -> Result<PostMeta, String> {
    let mut meta = PostMeta::get_by_id(db, &id)
        .await
        .map_err(|_| "文章不存在".to_string())?;
    let old_status = meta.status;
    let old_category_id = meta.category_id;
    let old_display_time = meta.display_time;
    let old_publish_time = meta.publish_time;
    let old_published_at = meta.published_at;
    let now = super::asset::now_unix();
    let mut next_status = meta.status;
    let mut next_publish_time = meta.publish_time;

    let old_meta_json = meta.meta_json.clone();
    let mut builder = meta.update();
    let mut meta_changed = false;
    let mut touch_updated = false;

    if let Some(category_id) = input.category_id {
        validate_post_category_id(db, category_id).await?;
        if old_category_id != category_id {
            builder = builder.category_id(category_id);
            meta_changed = true;
            touch_updated = true;
        }
    }
    if let Some(status) = input.status
        && old_status != status
    {
        builder = builder.status(status);
        meta_changed = true;
        next_status = status;
    }
    if input.publish_time.is_some() || input.status.is_some() {
        let status = input.status.unwrap_or(old_status);
        let raw = input.publish_time.or(if input.status.is_some() && old_publish_time == 0 {
            None
        } else {
            Some(old_publish_time)
        });
        let publish_time = resolve_publish_time(raw, status, now);
        if old_publish_time != publish_time {
            builder = builder.publish_time(publish_time);
            meta_changed = true;
            touch_updated = true;
            next_publish_time = publish_time;
        }
    }
    if let Some(display_time) = input.display_time {
        let display_time = resolve_display_time(Some(display_time), now);
        if old_display_time != display_time {
            builder = builder.display_time(display_time);
            meta_changed = true;
            touch_updated = true;
        }
    }
    let published_at = resolve_published_at(next_status, next_publish_time, now, old_published_at);
    if old_published_at != published_at {
        builder = builder.published_at(published_at);
        meta_changed = true;
    }
    if let Some(ref json) = input.meta_json {
        let meta_json = normalize_post_meta_json(json)?;
        if old_meta_json != meta_json {
            builder = builder.meta_json(meta_json.as_str());
            meta_changed = true;
            touch_updated = true;
        }
    }
    if touch_updated {
        builder = builder.updated_at(now);
    }
    if meta_changed {
        builder
            .exec(db)
            .await
            .map_err(|e| format!("更新失败: {e}"))?;
        meta = PostMeta::get_by_id(db, &id)
            .await
            .map_err(|_| "文章不存在".to_string())?;
    }

    let lang = input
        .lang
        .as_deref()
        .map(|l| super::locale::resolve_locale(Some(l), default_lang));
    if input.title.is_some()
        || input.description.is_some()
        || input.content.is_some()
        || input.route_path.is_some()
        || input.tags.is_some()
    {
        let resolved_lang = lang.clone().unwrap_or_else(|| default_lang.to_string());
        let rows = post_i18n_rows(db, id).await?;
        let existing = rows.iter().find(|r| r.lang == resolved_lang);

        let title = input
            .title
            .as_deref()
            .or_else(|| existing.map(|e| e.title.as_str()))
            .unwrap_or("");
        let description = input
            .description
            .as_deref()
            .or_else(|| existing.map(|e| e.description.as_str()))
            .unwrap_or("");
        let content = input
            .content
            .as_deref()
            .or_else(|| existing.map(|e| e.content.as_str()))
            .unwrap_or("");
        let route_path = input
            .route_path
            .as_deref()
            .or_else(|| existing.map(|e| e.route_path.as_str()))
            .unwrap_or("");
        let tags = input
            .tags
            .as_deref()
            .or_else(|| existing.map(|e| e.tags.as_str()))
            .unwrap_or("");

        upsert_post_i18n(
            db,
            id,
            PostI18nUpsert {
                lang: &resolved_lang,
                title,
                description,
                content,
                route_path,
                tags,
            },
        )
        .await?;
        meta = PostMeta::get_by_id(db, &id)
            .await
            .map_err(|_| "文章不存在".to_string())?;
    }

    Ok(meta)
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
    fn resolve_display_time_defaults_to_now() {
        assert_eq!(super::resolve_display_time(None, 100), 100);
        assert_eq!(super::resolve_display_time(Some(0), 100), 100);
        assert_eq!(super::resolve_display_time(Some(50), 100), 50);
    }

    #[test]
    fn resolve_publish_time_respects_status() {
        assert_eq!(super::resolve_publish_time(None, 1, 100), 100);
        assert_eq!(super::resolve_publish_time(Some(0), 1, 100), 100);
        assert_eq!(super::resolve_publish_time(None, 0, 100), 0);
        assert_eq!(super::resolve_publish_time(Some(200), 1, 100), 200);
    }

    #[test]
    fn normalize_post_meta_json_requires_object() {
        assert_eq!(super::normalize_post_meta_json("").unwrap(), "{}");
        assert_eq!(
            super::normalize_post_meta_json(r#"{"salary":"15K"}"#).unwrap(),
            r#"{"salary":"15K"}"#
        );
        assert!(super::normalize_post_meta_json("[]").is_err());
        assert!(super::normalize_post_meta_json("{bad").is_err());
    }

    #[test]
    fn is_post_publicly_visible_checks_publish_time() {
        let meta = PostMeta {
            id: 1,
            category_id: 1,
            status: 1,
            created_at: 0,
            updated_at: 0,
            published_at: 0,
            publish_time: 100,
            display_time: 100,
            meta_json: "{}".to_string(),
        };
        assert!(is_post_publicly_visible(&meta, 100));
        assert!(is_post_publicly_visible(&meta, 200));
        assert!(!is_post_publicly_visible(&meta, 99));
        assert!(!is_post_publicly_visible(
            &PostMeta { status: 0, ..meta.clone() },
            200
        ));
    }

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

/// 写入预制示例文章（新闻 + 相册；仅当尚无文章时执行）
pub async fn seed_default_sample_posts(
    db: &mut toasty::Db,
    storage: &StorageService,
) -> Result<(), String> {
    let posts = PostMeta::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询文章失败: {e}"))?;
    if !posts.is_empty() {
        return Ok(());
    }

    seed_default_news_post(db).await?;
    seed_default_gallery_post(db, storage).await?;
    seed_default_recruitment_post(db).await?;
    seed_default_about_post(db).await?;
    ensure_seed_sample_covers(db, storage).await?;
    Ok(())
}

/// 为预制示例文章补全封面（幂等；development 每次启动、新库首次写入后均会执行）
pub async fn ensure_seed_sample_covers(
    db: &mut toasty::Db,
    storage: &StorageService,
) -> Result<(), String> {
    let assets = match seed_default_gallery_assets(db, storage).await {
        Ok(a) => a,
        Err(e) => {
            eprintln!("[种子] 示例封面资源不可用: {e}");
            return Ok(());
        }
    };

    let mut linked = 0;
    for slug in ["site-launch", "about-haitang-cms"] {
        let Some(post_id) = find_post_id_by_public_slug(db, "zh-cn", slug).await? else {
            continue;
        };
        if ensure_post_cover_link(db, post_id, assets.cover.id).await? {
            linked += 1;
            println!("[种子] 已为文章 #{post_id}（{slug}）补全封面");
        }
    }

    if linked > 0 {
        println!("[种子] 示例文章封面补全完成（{linked} 篇）");
    }
    Ok(())
}

async fn find_post_id_by_public_slug(
    db: &mut toasty::Db,
    lang: &str,
    slug: &str,
) -> Result<Option<i64>, String> {
    let rows = PostI18n::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询文章翻译失败: {e}"))?;
    let ids = collect_post_ids_for_public_key(&rows, lang, slug);
    Ok(ids.first().copied())
}

async fn seed_default_news_post(db: &mut toasty::Db) -> Result<(), String> {
    let Some(category_id) = resolve_category_id_from_public_key(db, "zh-cn", "news").await? else {
        println!("[种子] 未找到「新闻」分类，跳过预制新闻文章");
        return Ok(());
    };

    let default_lang = get_site_default_locale(db).await;
    println!("[种子] 创建预制新闻文章...");

    let meta = create_post(
        db,
        &CreatePost {
            title: "站点上线公告".to_string(),
            description: Some(
                "海棠 CMS 公开站点已正式上线，欢迎浏览新闻与画廊等栏目。".to_string(),
            ),
            content: Some(
                "感谢访问本站。\n\n\
                我们已完成公开站点的首批内容搭建，当前提供**新闻**与**画廊**两个主要栏目：\
                新闻用于发布站点动态与公告，画廊用于展示以图片为主的内容。\n\n\
                后续将陆续完善「加入我们」「关于我们」等页面。\
                若你在使用过程中有任何建议，欢迎通过管理后台与我们联系。"
                    .to_string(),
            ),
            tags: Some("公告,站点".to_string()),
            category_id: Some(category_id),
            route_path: Some("site-launch".to_string()),
            lang: Some("zh-cn".to_string()),
            status: Some(1),
            display_time: None,
            publish_time: None,
            meta_json: None,
        },
        &default_lang,
    )
    .await?;

    upsert_post_i18n(
        db,
        meta.id,
        PostI18nUpsert {
            lang: "en-us",
            title: "Site Launch Announcement",
            description:
                "The public site is now live. Explore news, gallery, and more.",
            content: "Thank you for visiting.\n\n\
                We have published the first batch of public content. \
                **News** covers announcements and updates; **Gallery** highlights image-focused stories.\n\n\
                Pages such as Join Us and About Us will be expanded soon. \
                We welcome your feedback through the admin console.",
            route_path: "site-launch",
            tags: "announcement,site",
        },
    )
    .await?;

    println!("[种子] 预制新闻文章已创建（post_id={}）", meta.id);
    Ok(())
}

/// 写入预制相册文章（由 `seed_default_sample_posts` 在空库时调用）
async fn seed_default_gallery_post(
    db: &mut toasty::Db,
    storage: &StorageService,
) -> Result<(), String> {
    let Some(category_id) = resolve_category_id_from_public_key(db, "zh-cn", "gallery").await?
    else {
        println!("[种子] 未找到「画廊」分类，跳过预制相册文章");
        return Ok(());
    };

    let assets = match seed_default_gallery_assets(db, storage).await {
        Ok(a) => a,
        Err(e) => {
            eprintln!("[种子] 预制相册资源失败: {e}");
            return Ok(());
        }
    };

    let default_lang = get_site_default_locale(db).await;
    println!("[种子] 创建预制相册文章...");

    let meta = create_post(
        db,
        &CreatePost {
            title: "春日花园".to_string(),
            description: Some(
                "一组春日花卉摄影，记录花开时节的细腻色彩与自然光影。".to_string(),
            ),
            content: Some(
                "本组照片拍摄于早春，涵盖白樱、海棠与各色球根花卉。\
                希望这些画面能为你带来一点关于季节更替的温柔记忆。"
                    .to_string(),
            ),
            tags: Some("摄影,自然,春天".to_string()),
            category_id: Some(category_id),
            route_path: Some("spring-garden".to_string()),
            lang: Some("zh-cn".to_string()),
            status: Some(1),
            display_time: None,
            publish_time: None,
            meta_json: None,
        },
        &default_lang,
    )
    .await?;

    upsert_post_i18n(
        db,
        meta.id,
        PostI18nUpsert {
            lang: "en-us",
            title: "Spring Garden",
            description:
                "A spring floral photography set capturing soft colors and natural light.",
            content: "Shot in early spring, this set features cherry blossoms, crabapples, \
                and seasonal blooms. May these images bring a gentle reminder of the turning seasons.",
            route_path: "spring-garden",
            tags: "photography,nature,spring",
        },
    )
    .await?;

    link_post_asset(
        db,
        meta.id,
        &LinkPostAssetInput {
            asset_id: assets.cover.id,
            role: PostAssetRole::Cover.as_str().to_string(),
            sort_order: Some(0),
        },
    )
    .await?;

    for (sort_order, asset) in assets.attachments.iter().enumerate() {
        link_post_asset(
            db,
            meta.id,
            &LinkPostAssetInput {
                asset_id: asset.id,
                role: PostAssetRole::Attachment.as_str().to_string(),
                sort_order: Some(sort_order as i64),
            },
        )
        .await?;
    }

    println!(
        "[种子] 预制相册文章已创建（post_id={}，封面 1 张、附件 {} 张）",
        meta.id,
        assets.attachments.len()
    );
    Ok(())
}

/// 写入预制招聘岗位（由 `seed_default_sample_posts` 在空库时调用）
async fn seed_default_recruitment_post(db: &mut toasty::Db) -> Result<(), String> {
    let Some(category_id) = resolve_category_id_from_public_key(db, "zh-cn", "join").await? else {
        println!("[种子] 未找到「加入我们」分类，跳过预制招聘文章");
        return Ok(());
    };

    let default_lang = get_site_default_locale(db).await;
    println!("[种子] 创建预制招聘岗位...");

    let meta_json = r#"{"salary":"15K-25K","location":"北京","employment_type":"全职","department":"研发"}"#;

    let meta = create_post(
        db,
        &CreatePost {
            title: "Rust 后端工程师".to_string(),
            description: Some(
                "负责海棠 CMS 公开站与管理端 API 的设计与实现，参与内容模型与多语言能力建设。"
                    .to_string(),
            ),
            content: Some(
                "## 岗位职责\n\n\
                - 使用 Rust（Rocket）设计与实现 REST API\n\
                - 维护 toasty 数据模型与 SQLite 迁移\n\
                - 与前端协作完成管理端与公开站功能\n\n\
                ## 任职要求\n\n\
                - 熟悉 Rust 与 Web 后端开发\n\
                - 了解 SQL 与 REST 设计\n\
                - 有 CMS 或多语言站点经验者优先"
                    .to_string(),
            ),
            tags: Some("Rust,后端,全职".to_string()),
            category_id: Some(category_id),
            route_path: Some("rust-backend-engineer".to_string()),
            lang: Some("zh-cn".to_string()),
            status: Some(1),
            display_time: None,
            publish_time: None,
            meta_json: Some(meta_json.to_string()),
        },
        &default_lang,
    )
    .await?;

    upsert_post_i18n(
        db,
        meta.id,
        PostI18nUpsert {
            lang: "en-us",
            title: "Rust Backend Engineer",
            description:
                "Build and maintain public-site and admin APIs for Haitang CMS, including content models and i18n.",
            content: "## Responsibilities\n\n\
                - Design REST APIs with Rust (Rocket)\n\
                - Maintain toasty models and SQLite patches\n\
                - Collaborate with the admin-web team\n\n\
                ## Requirements\n\n\
                - Solid Rust and backend experience\n\
                - SQL and REST API design\n\
                - CMS or multilingual site experience is a plus",
            route_path: "rust-backend-engineer",
            tags: "Rust,backend,full-time",
        },
    )
    .await?;

    println!("[种子] 预制招聘岗位已创建（post_id={}）", meta.id);
    Ok(())
}

/// 写入预制「关于我们」介绍页（由 `seed_default_sample_posts` 在空库时调用）
async fn seed_default_about_post(db: &mut toasty::Db) -> Result<(), String> {
    let Some(category_id) = resolve_category_id_from_public_key(db, "zh-cn", "about").await? else {
        println!("[种子] 未找到「关于我们」分类，跳过预制介绍文章");
        return Ok(());
    };

    let default_lang = get_site_default_locale(db).await;
    println!("[种子] 创建预制关于我们文章...");

    let meta_json = r#"{"highlight":"结构化优雅","founded":"2024","location":"北京","contact":"hello@example.com"}"#;

    let meta = create_post(
        db,
        &CreatePost {
            title: "关于海棠 CMS".to_string(),
            description: Some(
                "轻量级内容管理系统，Rust + Rocket 后端，Tera 公开站，Vue 3 管理端。".to_string(),
            ),
            content: Some(
                "**海棠 CMS** 是一个轻量级内容管理系统，采用 Rust + Rocket 作为后端，SQLite 作为数据库，公开站点使用 Tera 模板引擎，管理后台使用 Vue 3 + Vite 构建。\n\n\
                ## 技术栈\n\n\
                - Rocket、Toasty ORM、Tera\n\
                - Vue 3 管理端、JWT 认证\n\n\
                ## 设计理念\n\n\
                遵循「结构化优雅」——在高效实用与温暖精致之间取得平衡。"
                    .to_string(),
            ),
            tags: Some("CMS,Rust,介绍".to_string()),
            category_id: Some(category_id),
            route_path: Some("about-haitang-cms".to_string()),
            lang: Some("zh-cn".to_string()),
            status: Some(1),
            display_time: None,
            publish_time: None,
            meta_json: Some(meta_json.to_string()),
        },
        &default_lang,
    )
    .await?;

    upsert_post_i18n(
        db,
        meta.id,
        PostI18nUpsert {
            lang: "en-us",
            title: "About Haitang CMS",
            description:
                "A lightweight CMS with Rust + Rocket, Tera public site, and Vue 3 admin.",
            content: "**Haitang CMS** is a lightweight content management system built with Rust and Rocket, SQLite for storage, Tera templates for the public site, and Vue 3 for the admin panel.\n\n\
                ## Stack\n\n\
                - Rocket, Toasty ORM, Tera\n\
                - Vue 3 admin, JWT auth\n\n\
                ## Design\n\n\
                Structured elegance — balancing efficiency with a warm, refined experience.",
            route_path: "about-haitang-cms",
            tags: "CMS,Rust,intro",
        },
    )
    .await?;

    println!("[种子] 预制关于我们文章已创建（post_id={}）", meta.id);
    Ok(())
}
