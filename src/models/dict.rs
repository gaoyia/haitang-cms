use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::dict_meta::DictMeta;
use super::dict_value::DictValue;
use super::locale::{DEFAULT_LOCALE, LANG_GLOBAL, pick_i18n_row, resolve_locale};

/// 创建字典 meta 请求
#[derive(Debug, Deserialize)]
pub struct CreateDictMeta {
    pub code: String,
    pub label: String,
    pub description: Option<String>,
    pub translatable: Option<bool>,
    pub sort: Option<i64>,
    pub value: Option<String>,
    pub lang: Option<String>,
}

/// 更新字典 meta 请求
#[derive(Debug, Deserialize)]
pub struct UpdateDictMeta {
    pub label: Option<String>,
    pub description: Option<String>,
    pub translatable: Option<bool>,
    pub sort: Option<i64>,
}

/// 批量更新字典 value（lang → value）
#[derive(Debug, Deserialize)]
pub struct UpsertDictValues {
    pub values: HashMap<String, String>,
}

/// 字典 meta 视图
#[derive(Debug, Clone, Serialize)]
pub struct DictMetaView {
    pub code: String,
    pub label: String,
    pub description: String,
    pub translatable: bool,
    pub sort: i64,
}

/// 字典详情（管理端）
#[derive(Debug, Serialize)]
pub struct DictDetailView {
    #[serde(flatten)]
    pub meta: DictMetaView,
    pub values: HashMap<String, String>,
}

/// 公开字典项（扁平，含当前语言解析后的 value）
#[derive(Debug, Clone, Serialize)]
pub struct DictPublicView {
    pub code: String,
    pub label: String,
    pub value: String,
    pub description: String,
    pub sort: i64,
}

impl From<DictMeta> for DictMetaView {
    fn from(m: DictMeta) -> Self {
        Self {
            code: m.code,
            label: m.label,
            description: m.description,
            translatable: m.translatable,
            sort: m.sort,
        }
    }
}

/// 读取站点支持的语言列表（来自字典 site_locales）
pub async fn get_site_locales(db: &mut toasty::Db) -> Vec<String> {
    match DictValue::get_by_code_and_lang(db, "site_locales", LANG_GLOBAL).await {
        Ok(v) if !v.value.trim().is_empty() => super::locale::parse_locale_list(&v.value),
        _ => vec![DEFAULT_LOCALE.to_string(), "en-us".to_string()],
    }
}

/// 读取站点默认语言（来自字典 site_default_locale）
pub async fn get_site_default_locale(db: &mut toasty::Db) -> String {
    match DictValue::get_by_code_and_lang(db, "site_default_locale", LANG_GLOBAL).await {
        Ok(v) if !v.value.trim().is_empty() => super::locale::normalize_lang(&v.value),
        _ => DEFAULT_LOCALE.to_string(),
    }
}

/// 按 code 查找 meta
pub async fn find_dict_meta_by_code(db: &mut toasty::Db, code: &str) -> Result<DictMeta, String> {
    DictMeta::get_by_code(db, code)
        .await
        .map_err(|_| "字典项不存在".to_string())
}

/// 获取某 code 的全部 value 行
pub async fn get_dict_values_for_code(
    db: &mut toasty::Db,
    code: &str,
) -> Result<Vec<DictValue>, String> {
    let all = DictValue::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询字典值失败: {e}"))?;
    Ok(all.into_iter().filter(|v| v.code == code).collect())
}

/// 解析单个字典项在当前语言下的 value
pub async fn resolve_dict_value(
    db: &mut toasty::Db,
    meta: &DictMeta,
    lang: &str,
    default_lang: &str,
) -> String {
    let rows = match get_dict_values_for_code(db, &meta.code).await {
        Ok(r) => r,
        Err(_) => return String::new(),
    };

    if !meta.translatable {
        return rows
            .iter()
            .find(|v| v.lang == LANG_GLOBAL)
            .map(|v| v.value.clone())
            .unwrap_or_default();
    }

    pick_i18n_row(&rows, lang, default_lang, |v| v.lang.as_str())
        .map(|v| v.value.clone())
        .unwrap_or_default()
}

/// 加载全部字典为 code → value（已按 lang fallback）
pub async fn load_dict_map(db: &mut toasty::Db, lang: Option<&str>) -> HashMap<String, String> {
    let default = get_site_default_locale(db).await;
    let resolved = resolve_locale(lang, &default);

    let metas = match DictMeta::all().exec(db).await {
        Ok(mut m) => {
            m.sort_by_key(|d| d.sort);
            m
        }
        Err(_) => return HashMap::new(),
    };

    let mut map = HashMap::new();
    for meta in metas {
        let value = resolve_dict_value(db, &meta, &resolved, &default).await;
        map.insert(meta.code, value);
    }
    map
}

/// 构建公开字典视图列表
pub async fn dict_public_views(
    db: &mut toasty::Db,
    lang: Option<&str>,
) -> Result<Vec<DictPublicView>, String> {
    let default = get_site_default_locale(db).await;
    let resolved = resolve_locale(lang, &default);

    let mut metas = DictMeta::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询字典失败: {e}"))?;
    metas.sort_by_key(|d| d.sort);

    let mut views = Vec::new();
    for meta in metas {
        let value = resolve_dict_value(db, &meta, &resolved, &default).await;
        views.push(DictPublicView {
            code: meta.code.clone(),
            label: meta.label.clone(),
            value,
            description: meta.description.clone(),
            sort: meta.sort,
        });
    }
    Ok(views)
}

/// 构建管理端详情
pub async fn dict_detail_view(db: &mut toasty::Db, code: &str) -> Result<DictDetailView, String> {
    let meta = find_dict_meta_by_code(db, code).await?;
    let rows = get_dict_values_for_code(db, code).await?;
    let values = rows.into_iter().map(|v| (v.lang, v.value)).collect();
    Ok(DictDetailView {
        meta: meta.into(),
        values,
    })
}

/// 默认字典种子条目：(code, label, translatable, description, sort, global_value, [(lang, value)...])
pub type DictSeedEntry = (
    &'static str,
    &'static str,
    bool,
    &'static str,
    i64,
    Option<&'static str>,
    &'static [(&'static str, &'static str)],
);

/// 默认字典种子表
pub fn default_dict_seed() -> &'static [DictSeedEntry] {
    &[
        (
            "site_default_locale",
            "站点默认语言",
            false,
            "全站 fallback 语言码，如 zh-cn",
            0,
            Some("zh-cn"),
            &[],
        ),
        (
            "site_locales",
            "启用语言列表",
            false,
            "逗号分隔，如 zh-cn,en-us",
            1,
            Some("zh-cn,en-us"),
            &[],
        ),
        (
            "site_icp",
            "备案号",
            false,
            "网站底部 ICP 备案号",
            2,
            Some(""),
            &[],
        ),
        (
            "site_logo",
            "站点 Logo",
            false,
            "Logo 地址，相对或绝对 URL",
            3,
            Some("/static/resources/logo.svg"),
            &[],
        ),
        (
            "site_name",
            "站点名称",
            true,
            "页头、标题等",
            10,
            None,
            &[("zh-cn", "海棠 CMS"), ("en-us", "Haitang CMS")],
        ),
        (
            "site_copyright",
            "版权信息",
            true,
            "页脚版权文案",
            20,
            None,
            &[
                ("zh-cn", "© 2026 海棠 CMS"),
                ("en-us", "© 2026 Haitang CMS"),
            ],
        ),
    ]
}

/// 写入或跳过默认字典种子
pub async fn seed_default_dicts(db: &mut toasty::Db) {
    let existing = match DictMeta::all().exec(db).await {
        Ok(m) => m,
        Err(_) => return,
    };
    let existing_codes: std::collections::HashSet<_> =
        existing.iter().map(|m| m.code.as_str()).collect();

    let mut created = 0;
    for (code, label, translatable, description, sort, global_value, i18n_pairs) in
        default_dict_seed()
    {
        if existing_codes.contains(code) {
            continue;
        }

        if DictMeta::create()
            .code(*code)
            .label(*label)
            .description(*description)
            .translatable(*translatable)
            .sort(*sort)
            .exec(db)
            .await
            .is_err()
        {
            eprintln!("[种子] 创建字典 meta {code} 失败");
            continue;
        }

        if let Some(val) = global_value
            && DictValue::create()
                .code(*code)
                .lang(LANG_GLOBAL)
                .value(*val)
                .exec(db)
                .await
                .is_err()
        {
            eprintln!("[种子] 创建字典 value {code} 失败");
            continue;
        }

        for (lang, val) in *i18n_pairs {
            if DictValue::create()
                .code(*code)
                .lang(*lang)
                .value(*val)
                .exec(db)
                .await
                .is_err()
            {
                eprintln!("[种子] 创建字典 value {code}/{lang} 失败");
            }
        }

        created += 1;
    }

    if created > 0 {
        println!("[种子] 已创建 {created} 条默认字典项");
    }
}

/// 删除字典 meta 及其全部 value
pub async fn delete_dict_by_code(db: &mut toasty::Db, code: &str) -> Result<(), String> {
    let meta = find_dict_meta_by_code(db, code).await?;
    let values = get_dict_values_for_code(db, code).await?;
    for v in values {
        v.delete()
            .exec(db)
            .await
            .map_err(|e| format!("删除字典值失败: {e}"))?;
    }
    meta.delete()
        .exec(db)
        .await
        .map_err(|e| format!("删除字典 meta 失败: {e}"))?;
    Ok(())
}

/// upsert 某 code 的多语言 values
pub async fn upsert_dict_values(
    db: &mut toasty::Db,
    code: &str,
    values: HashMap<String, String>,
) -> Result<(), String> {
    let meta = find_dict_meta_by_code(db, code).await?;
    let existing = get_dict_values_for_code(db, code).await?;

    for (lang, value) in values {
        let lang = if meta.translatable {
            super::locale::normalize_lang(&lang)
        } else {
            LANG_GLOBAL.to_string()
        };

        if let Some(row) = existing.iter().find(|v| v.lang == lang) {
            row.clone()
                .update()
                .value(&value)
                .exec(db)
                .await
                .map_err(|e| format!("更新字典值失败: {e}"))?;
        } else {
            DictValue::create()
                .code(code)
                .lang(&lang)
                .value(&value)
                .exec(db)
                .await
                .map_err(|e| format!("创建字典值失败: {e}"))?;
        }
    }
    Ok(())
}
