use serde::{Deserialize, Serialize};

use super::locale::{pick_i18n_row, resolve_locale};

pub const CATEGORY_TEMPLATE_DEFAULT: &str = "default";
pub const CATEGORY_TEMPLATE_GALLERY: &str = "gallery";
pub const CATEGORY_TEMPLATE_RECRUITMENT: &str = "recruitment";
pub const CATEGORY_TEMPLATE_ABOUT: &str = "about";
pub const CATEGORY_TEMPLATE_NONE: &str = "none";

/// 分类结构（不分语言）
#[derive(Debug, Clone, toasty::Model)]
pub struct CategoryMeta {
    #[key]
    #[auto]
    pub id: i64,

    pub sort: i64,

    /// 列表页模板：none | default | gallery | recruitment | about
    pub list_template: String,

    /// 详情页模板：default | gallery | recruitment | about
    pub detail_template: String,
}

/// 分类文案（按语言）
#[derive(Debug, Clone, toasty::Model)]
#[key(category_id, lang)]
pub struct CategoryI18n {
    pub category_id: i64,

    pub lang: String,

    pub name: String,

    pub description: String,

    /// 完整路径，如 /zh-cn/categories/travel
    pub route_path: String,
}

/// 创建分类
#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub lang: Option<String>,
    pub list_template: Option<String>,
    pub detail_template: Option<String>,
    pub route_path: Option<String>,
}

/// 更新分类
#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub lang: Option<String>,
    pub list_template: Option<String>,
    pub detail_template: Option<String>,
    pub route_path: Option<String>,
}

/// 分类视图（已 merge 当前语言）
#[derive(Debug, Serialize)]
pub struct CategoryView {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub sort: i64,
    pub route_path: String,
    pub list_template: String,
    pub detail_template: String,
}

/// 管理端分类详情（含各语言）
#[derive(Debug, Serialize)]
pub struct CategoryDetailView {
    pub id: i64,
    pub sort: i64,
    pub list_template: String,
    pub detail_template: String,
    pub translations: std::collections::HashMap<String, CategoryI18nPayload>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryI18nPayload {
    pub name: String,
    pub description: String,
    pub route_path: String,
}

/// 校验并规范化分类详情模板名
pub fn normalize_category_detail_template(raw: &str) -> Result<String, String> {
    match raw.trim() {
        "" | CATEGORY_TEMPLATE_DEFAULT => Ok(CATEGORY_TEMPLATE_DEFAULT.to_string()),
        CATEGORY_TEMPLATE_GALLERY => Ok(CATEGORY_TEMPLATE_GALLERY.to_string()),
        CATEGORY_TEMPLATE_RECRUITMENT => Ok(CATEGORY_TEMPLATE_RECRUITMENT.to_string()),
        CATEGORY_TEMPLATE_ABOUT => Ok(CATEGORY_TEMPLATE_ABOUT.to_string()),
        CATEGORY_TEMPLATE_NONE => Err("详情模板不能为「无」".to_string()),
        other => Err(format!("不支持的模板类型: {other}")),
    }
}

/// 校验并规范化分类列表模板名（含 none）
pub fn normalize_category_list_template(raw: &str) -> Result<String, String> {
    if raw.trim() == CATEGORY_TEMPLATE_NONE {
        return Ok(CATEGORY_TEMPLATE_NONE.to_string());
    }
    normalize_category_detail_template(raw)
}

/// 是否开放分类归档列表页
pub fn category_list_archive_enabled(list_template: &str) -> bool {
    list_template != CATEGORY_TEMPLATE_NONE
}

/// 列表模板名 → Tera 文件名（白名单；none 不应调用）
pub fn category_list_tera_template(list_template: &str) -> &'static str {
    match list_template {
        CATEGORY_TEMPLATE_GALLERY => "gallery-list",
        CATEGORY_TEMPLATE_RECRUITMENT => "recruitment-list",
        CATEGORY_TEMPLATE_ABOUT => "about-list",
        _ => "category-list",
    }
}

/// 详情模板名 → Tera 文件名（白名单）
pub fn category_detail_tera_template(detail_template: &str) -> &'static str {
    match detail_template {
        CATEGORY_TEMPLATE_GALLERY => "gallery-detail",
        CATEGORY_TEMPLATE_RECRUITMENT => "recruitment-detail",
        CATEGORY_TEMPLATE_ABOUT => "about-detail",
        _ => "post-detail",
    }
}

/// 指定语言下分类 SEO 路径的固定前缀
pub fn category_route_path_prefix(lang: &str) -> String {
    format!("/{}/categories/", super::locale::normalize_lang(lang))
}

/// 校验并规范化分类 SEO 路径；空串合法
pub fn normalize_category_route_path(lang: &str, raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }

    let prefix = category_route_path_prefix(lang);
    let path = if trimmed.starts_with(&prefix) {
        trimmed.to_string()
    } else if !trimmed.contains('/') {
        format!("{prefix}{trimmed}")
    } else if let Some(idx) = trimmed.find("/categories/") {
        let slug = trimmed[idx + "/categories/".len()..].trim();
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

pub fn category_route_path_taken(lang: &str, route_path: &str, other_category_id: i64) -> String {
    format!("该语言（{lang}）下 SEO 路径「{route_path}」已被分类 #{other_category_id} 使用")
}

pub fn category_route_path_ambiguous(route_path: &str, category_ids: &[i64]) -> String {
    let ids = category_ids
        .iter()
        .map(|id| format!("#{id}"))
        .collect::<Vec<_>>()
        .join("、");
    format!("SEO 路径「{route_path}」对应多个分类（{ids}），请修改冲突分类的 SEO 路径")
}

fn collect_category_ids_for_public_key(rows: &[CategoryI18n], lang: &str, key: &str) -> Vec<i64> {
    let lang = super::locale::normalize_lang(lang);
    let key = key.trim();
    if key.is_empty() {
        return vec![];
    }

    let expected = format!("/{lang}/categories/{key}");
    let prefix = category_route_path_prefix(&lang);
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
        if matched && !ids.contains(&row.category_id) {
            ids.push(row.category_id);
        }
    }

    ids
}

/// 非空 `route_path` 在同一语言下不得与其他分类重复
pub async fn ensure_unique_category_route_path(
    db: &mut toasty::Db,
    lang: &str,
    route_path: &str,
    exclude_category_id: Option<i64>,
) -> Result<(), String> {
    if route_path.is_empty() {
        return Ok(());
    }

    let lang = super::locale::normalize_lang(lang);
    let rows = CategoryI18n::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询分类翻译失败: {e}"))?;

    for row in rows {
        if row.lang != lang || row.route_path.is_empty() {
            continue;
        }
        if row.route_path == route_path && exclude_category_id != Some(row.category_id) {
            return Err(category_route_path_taken(&lang, route_path, row.category_id));
        }
    }

    Ok(())
}

/// 从公开 URL 最后一段解析分类 ID
pub async fn resolve_category_id_from_public_key(
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
        if CategoryMeta::get_by_id(db, &id).await.is_ok() {
            return Ok(Some(id));
        }
        return Ok(None);
    }

    let expected = format!("/{lang}/categories/{key}");
    let rows = CategoryI18n::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询分类翻译失败: {e}"))?;

    match collect_category_ids_for_public_key(&rows, &lang, key).as_slice() {
        [] => Ok(None),
        [id] => Ok(Some(*id)),
        ids => Err(category_route_path_ambiguous(&expected, ids)),
    }
}

pub async fn category_templates(
    db: &mut toasty::Db,
    category_id: i64,
) -> Result<(String, String), String> {
    if category_id == 0 {
        return Ok((
            CATEGORY_TEMPLATE_DEFAULT.to_string(),
            CATEGORY_TEMPLATE_DEFAULT.to_string(),
        ));
    }
    let meta = CategoryMeta::get_by_id(db, &category_id)
        .await
        .map_err(|_| "分类不存在".to_string())?;
    Ok((meta.list_template.clone(), meta.detail_template.clone()))
}

pub async fn create_category(
    db: &mut toasty::Db,
    input: &CreateCategory,
    default_lang: &str,
) -> Result<CategoryMeta, String> {
    let lang = resolve_locale(input.lang.as_deref(), default_lang);
    let description = input.description.as_deref().unwrap_or("");
    let list_template = normalize_category_list_template(
        input
            .list_template
            .as_deref()
            .unwrap_or(CATEGORY_TEMPLATE_DEFAULT),
    )?;
    let detail_template = normalize_category_detail_template(
        input
            .detail_template
            .as_deref()
            .unwrap_or(CATEGORY_TEMPLATE_DEFAULT),
    )?;
    let route_path =
        normalize_category_route_path(&lang, input.route_path.as_deref().unwrap_or(""))?;
    ensure_unique_category_route_path(db, &lang, &route_path, None).await?;

    let meta = CategoryMeta::create()
        .sort(input.sort.unwrap_or(0))
        .list_template(&list_template)
        .detail_template(&detail_template)
        .exec(db)
        .await
        .map_err(|e| format!("创建分类失败: {e}"))?;

    CategoryI18n::create()
        .category_id(meta.id)
        .lang(&lang)
        .name(&input.name)
        .description(description)
        .route_path(&route_path)
        .exec(db)
        .await
        .map_err(|e| format!("创建分类翻译失败: {e}"))?;

    Ok(meta)
}

pub async fn category_to_view(
    db: &mut toasty::Db,
    meta: &CategoryMeta,
    lang: &str,
    default_lang: &str,
) -> Result<CategoryView, String> {
    let rows = category_i18n_rows(db, meta.id).await?;
    let i18n = pick_i18n_row(&rows, lang, default_lang, |r| r.lang.as_str())
        .ok_or_else(|| "分类翻译不存在".to_string())?;
    Ok(CategoryView {
        id: meta.id,
        name: i18n.name.clone(),
        description: i18n.description.clone(),
        sort: meta.sort,
        route_path: i18n.route_path.clone(),
        list_template: meta.list_template.clone(),
        detail_template: meta.detail_template.clone(),
    })
}

/// 分类公开页 URL（优先 SEO 路径，否则回退数字 ID）
pub async fn category_public_url(
    db: &mut toasty::Db,
    category_id: i64,
    lang: &str,
    default_lang: &str,
) -> String {
    if category_id == 0 {
        return String::new();
    }
    let lang = super::locale::normalize_lang(lang);
    match CategoryMeta::get_by_id(db, &category_id).await {
        Ok(meta) => match category_to_view(db, &meta, &lang, default_lang).await {
            Ok(view) if !view.route_path.is_empty() => view.route_path,
            Ok(_) | Err(_) => format!("/{lang}/categories/{category_id}"),
        },
        Err(_) => String::new(),
    }
}

pub async fn categories_to_views(
    db: &mut toasty::Db,
    lang: Option<&str>,
) -> Result<Vec<CategoryView>, String> {
    let default = super::dict::get_site_default_locale(db).await;
    let resolved = resolve_locale(lang, &default);

    let mut metas = CategoryMeta::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询分类失败: {e}"))?;
    metas.sort_by_key(|m| m.sort);

    let mut views = Vec::new();
    for meta in metas {
        views.push(category_to_view(db, &meta, &resolved, &default).await?);
    }
    Ok(views)
}

pub async fn category_i18n_rows(
    db: &mut toasty::Db,
    category_id: i64,
) -> Result<Vec<CategoryI18n>, String> {
    let all = CategoryI18n::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询分类翻译失败: {e}"))?;
    Ok(all
        .into_iter()
        .filter(|r| r.category_id == category_id)
        .collect())
}

pub async fn load_category_map(
    db: &mut toasty::Db,
    lang: Option<&str>,
) -> Result<std::collections::HashMap<i64, String>, String> {
    let views = categories_to_views(db, lang).await?;
    Ok(views.into_iter().map(|v| (v.id, v.name)).collect())
}

pub async fn validate_category_id(db: &mut toasty::Db, category_id: i64) -> Result<(), String> {
    if category_id == 0 {
        return Ok(());
    }
    CategoryMeta::get_by_id(db, &category_id)
        .await
        .map_err(|_| "分类不存在".to_string())?;
    Ok(())
}

/// 文章分类必填且须存在
pub async fn validate_post_category_id(db: &mut toasty::Db, category_id: i64) -> Result<(), String> {
    if category_id == 0 {
        return Err("请选择分类".to_string());
    }
    validate_category_id(db, category_id).await
}

pub async fn delete_category(db: &mut toasty::Db, id: i64) -> Result<(), String> {
    let meta = CategoryMeta::get_by_id(db, &id)
        .await
        .map_err(|_| "分类不存在".to_string())?;
    let rows = category_i18n_rows(db, id).await?;
    for row in rows {
        row.delete()
            .exec(db)
            .await
            .map_err(|e| format!("删除分类翻译失败: {e}"))?;
    }
    meta.delete()
        .exec(db)
        .await
        .map_err(|e| format!("删除分类失败: {e}"))?;
    Ok(())
}

pub async fn category_detail_view(
    db: &mut toasty::Db,
    id: i64,
) -> Result<CategoryDetailView, String> {
    let meta = CategoryMeta::get_by_id(db, &id)
        .await
        .map_err(|_| "分类不存在".to_string())?;
    let rows = category_i18n_rows(db, id).await?;
    let translations = rows
        .into_iter()
        .map(|r| {
            (
                r.lang.clone(),
                CategoryI18nPayload {
                    name: r.name,
                    description: r.description,
                    route_path: r.route_path,
                },
            )
        })
        .collect();
    Ok(CategoryDetailView {
        id: meta.id,
        sort: meta.sort,
        list_template: meta.list_template,
        detail_template: meta.detail_template,
        translations,
    })
}

/// 更新或插入某语言的分类翻译
pub async fn upsert_category_i18n(
    db: &mut toasty::Db,
    category_id: i64,
    lang: &str,
    name: &str,
    description: &str,
    route_path: &str,
) -> Result<(), String> {
    let lang = super::locale::normalize_lang(lang);
    let route_path = normalize_category_route_path(&lang, route_path)?;
    ensure_unique_category_route_path(db, &lang, &route_path, Some(category_id)).await?;
    match CategoryI18n::get_by_category_id_and_lang(db, &category_id, &lang).await {
        Ok(mut row) => {
            row.update()
                .name(name)
                .description(description)
                .route_path(&route_path)
                .exec(db)
                .await
                .map_err(|e| format!("更新分类翻译失败: {e}"))?;
        }
        Err(_) => {
            CategoryI18n::create()
                .category_id(category_id)
                .lang(&lang)
                .name(name)
                .description(description)
                .route_path(&route_path)
                .exec(db)
                .await
                .map_err(|e| format!("创建分类翻译失败: {e}"))?;
        }
    }
    Ok(())
}

/// 更新分类元数据与翻译
pub async fn update_category(
    db: &mut toasty::Db,
    id: i64,
    input: &UpdateCategory,
    default_lang: &str,
) -> Result<CategoryMeta, String> {
    let mut meta = CategoryMeta::get_by_id(db, &id)
        .await
        .map_err(|_| "分类不存在".to_string())?;

    let old_list_template = meta.list_template.clone();
    let old_detail_template = meta.detail_template.clone();

    let mut builder = meta.update();
    let mut meta_changed = false;

    if let Some(sort) = input.sort {
        builder = builder.sort(sort);
        meta_changed = true;
    }
    if let Some(ref tpl) = input.list_template {
        let list_template = normalize_category_list_template(tpl)?;
        if old_list_template != list_template {
            builder = builder.list_template(&list_template);
            meta_changed = true;
        }
    }
    if let Some(ref tpl) = input.detail_template {
        let detail_template = normalize_category_detail_template(tpl)?;
        if old_detail_template != detail_template {
            builder = builder.detail_template(&detail_template);
            meta_changed = true;
        }
    }

    if meta_changed {
        builder
            .exec(db)
            .await
            .map_err(|e| format!("更新分类失败: {e}"))?;
        meta = CategoryMeta::get_by_id(db, &id)
            .await
            .map_err(|_| "分类不存在".to_string())?;
    }

    let lang = input
        .lang
        .as_deref()
        .map(|l| super::locale::resolve_locale(Some(l), default_lang));
    if input.name.is_some() || input.description.is_some() || input.route_path.is_some() {
        let resolved_lang = lang.clone().unwrap_or_else(|| default_lang.to_string());
        let rows = category_i18n_rows(db, id).await?;
        let existing = rows.iter().find(|r| r.lang == resolved_lang);

        let name = input
            .name
            .as_deref()
            .or_else(|| existing.map(|e| e.name.as_str()))
            .unwrap_or("");
        let description = input
            .description
            .as_deref()
            .or_else(|| existing.map(|e| e.description.as_str()))
            .unwrap_or("");
        let route_path = input
            .route_path
            .as_deref()
            .or_else(|| existing.map(|e| e.route_path.as_str()))
            .unwrap_or("");

        upsert_category_i18n(db, id, &resolved_lang, name, description, route_path).await?;
        meta = CategoryMeta::get_by_id(db, &id)
            .await
            .map_err(|_| "分类不存在".to_string())?;
    }

    Ok(meta)
}

/// 默认分类种子
struct CategorySeedEntry {
    sort: i64,
    slug: &'static str,
    zh_name: &'static str,
    zh_desc: &'static str,
    en_name: &'static str,
    en_desc: &'static str,
    list_template: &'static str,
    detail_template: &'static str,
}

fn default_category_seed() -> &'static [CategorySeedEntry] {
    &[
        CategorySeedEntry {
            sort: 0,
            slug: "news",
            zh_name: "新闻",
            zh_desc: "站点新闻与动态资讯",
            en_name: "News",
            en_desc: "Site news and updates",
            list_template: CATEGORY_TEMPLATE_DEFAULT,
            detail_template: CATEGORY_TEMPLATE_DEFAULT,
        },
        CategorySeedEntry {
            sort: 1,
            slug: "gallery",
            zh_name: "画廊",
            zh_desc: "图片为主的相册栏目",
            en_name: "Gallery",
            en_desc: "Photo galleries and albums",
            list_template: CATEGORY_TEMPLATE_GALLERY,
            detail_template: CATEGORY_TEMPLATE_GALLERY,
        },
        CategorySeedEntry {
            sort: 2,
            slug: "join",
            zh_name: "加入我们",
            zh_desc: "招聘岗位与团队加入",
            en_name: "Join Us",
            en_desc: "Job openings and careers",
            list_template: CATEGORY_TEMPLATE_RECRUITMENT,
            detail_template: CATEGORY_TEMPLATE_RECRUITMENT,
        },
        CategorySeedEntry {
            sort: 3,
            slug: "about",
            zh_name: "关于我们",
            zh_desc: "站点与团队介绍",
            en_name: "About Us",
            en_desc: "About the site and team",
            list_template: CATEGORY_TEMPLATE_NONE,
            detail_template: CATEGORY_TEMPLATE_ABOUT,
        },
    ]
}

/// 按 slug 生成分类公开页路径（用于种子菜单等）
pub fn category_public_path_by_slug(lang: &str, slug: &str) -> String {
    normalize_category_route_path(lang, slug)
        .unwrap_or_else(|e| panic!("种子分类 slug 无效（{lang}/{slug}）: {e}"))
}

/// 写入默认文章分类（仅当尚无分类时执行）
pub async fn seed_default_categories(db: &mut toasty::Db) {
    let existing = match CategoryMeta::all().exec(db).await {
        Ok(m) => m,
        Err(_) => return,
    };
    if !existing.is_empty() {
        return;
    }

    println!("[种子] 创建默认文章分类...");
    let mut created = 0;

    for entry in default_category_seed() {
        let meta = match CategoryMeta::create()
            .sort(entry.sort)
            .list_template(entry.list_template)
            .detail_template(entry.detail_template)
            .exec(db)
            .await
        {
            Ok(m) => m,
            Err(e) => {
                eprintln!("[种子] 创建分类 meta 失败: {e}");
                continue;
            }
        };

        for (lang, name, desc) in [
            ("zh-cn", entry.zh_name, entry.zh_desc),
            ("en-us", entry.en_name, entry.en_desc),
        ] {
            let route_path = category_public_path_by_slug(lang, entry.slug);
            if let Err(e) = upsert_category_i18n(db, meta.id, lang, name, desc, &route_path).await {
                eprintln!("[种子] 创建分类 i18n {lang} 失败: {e}");
            }
        }
        created += 1;
    }

    if created > 0 {
        println!("[种子] 已创建 {created} 个默认文章分类");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_category_route_path_ok() {
        assert_eq!(normalize_category_route_path("zh-cn", "").unwrap(), "");
    }

    #[test]
    fn full_category_route_path_ok() {
        assert_eq!(
            normalize_category_route_path("zh-cn", "/zh-cn/categories/travel").unwrap(),
            "/zh-cn/categories/travel"
        );
    }

    #[test]
    fn category_slug_only_expanded() {
        assert_eq!(
            normalize_category_route_path("en-us", "gallery").unwrap(),
            "/en-us/categories/gallery"
        );
    }

    #[test]
    fn invalid_category_slug_rejected() {
        assert!(normalize_category_route_path("zh-cn", "/zh-cn/categories/a b").is_err());
    }

    #[test]
    fn normalize_category_template_whitelist() {
        assert_eq!(
            normalize_category_detail_template("default").unwrap(),
            "default"
        );
        assert_eq!(normalize_category_detail_template("gallery").unwrap(), "gallery");
        assert_eq!(
            normalize_category_detail_template("recruitment").unwrap(),
            "recruitment"
        );
        assert_eq!(normalize_category_detail_template("about").unwrap(), "about");
        assert_eq!(
            normalize_category_list_template("none").unwrap(),
            "none"
        );
        assert!(normalize_category_detail_template("none").is_err());
        assert!(!category_list_archive_enabled("none"));
        assert!(category_list_archive_enabled("default"));
        assert!(normalize_category_detail_template("evil").is_err());
    }

    fn test_row(category_id: i64, lang: &str, route_path: &str) -> CategoryI18n {
        CategoryI18n {
            category_id,
            lang: lang.to_string(),
            name: String::new(),
            description: String::new(),
            route_path: route_path.to_string(),
        }
    }

    #[test]
    fn collect_category_ids_single_slug_match() {
        let rows = vec![test_row(3, "zh-cn", "/zh-cn/categories/gallery")];
        assert_eq!(
            collect_category_ids_for_public_key(&rows, "zh-cn", "gallery"),
            vec![3]
        );
    }

    #[test]
    fn collect_category_ids_duplicate_route_path() {
        let rows = vec![
            test_row(1, "zh-cn", "/zh-cn/categories/gallery"),
            test_row(2, "zh-cn", "/zh-cn/categories/gallery"),
        ];
        assert_eq!(
            collect_category_ids_for_public_key(&rows, "zh-cn", "gallery"),
            vec![1, 2]
        );
    }
}
