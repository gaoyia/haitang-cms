use serde::{Deserialize, Serialize};

use super::locale::{pick_i18n_row, resolve_locale};

/// 分类结构（不分语言）
#[derive(Debug, Clone, toasty::Model)]
pub struct CategoryMeta {
    #[key]
    #[auto]
    pub id: i64,

    pub sort: i64,
}

/// 分类文案（按语言）
#[derive(Debug, Clone, toasty::Model)]
#[key(category_id, lang)]
pub struct CategoryI18n {
    pub category_id: i64,

    pub lang: String,

    pub name: String,

    pub description: String,
}

/// 创建分类
#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub lang: Option<String>,
}

/// 更新分类
#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub lang: Option<String>,
}

/// 分类视图（已 merge 当前语言）
#[derive(Debug, Serialize)]
pub struct CategoryView {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub sort: i64,
}

/// 管理端分类详情（含各语言）
#[derive(Debug, Serialize)]
pub struct CategoryDetailView {
    pub id: i64,
    pub sort: i64,
    pub translations: std::collections::HashMap<String, CategoryI18nPayload>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryI18nPayload {
    pub name: String,
    pub description: String,
}

pub async fn create_category(
    db: &mut toasty::Db,
    input: &CreateCategory,
    default_lang: &str,
) -> Result<CategoryMeta, String> {
    let lang = resolve_locale(input.lang.as_deref(), default_lang);
    let description = input.description.as_deref().unwrap_or("");

    let meta = CategoryMeta::create()
        .sort(input.sort.unwrap_or(0))
        .exec(db)
        .await
        .map_err(|e| format!("创建分类失败: {e}"))?;

    CategoryI18n::create()
        .category_id(meta.id)
        .lang(&lang)
        .name(&input.name)
        .description(description)
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
    })
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
                },
            )
        })
        .collect();
    Ok(CategoryDetailView {
        id: meta.id,
        sort: meta.sort,
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
) -> Result<(), String> {
    let lang = super::locale::normalize_lang(lang);
    match CategoryI18n::get_by_category_id_and_lang(db, &category_id, &lang).await {
        Ok(mut row) => {
            row.update()
                .name(name)
                .description(description)
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
                .exec(db)
                .await
                .map_err(|e| format!("创建分类翻译失败: {e}"))?;
        }
    }
    Ok(())
}
