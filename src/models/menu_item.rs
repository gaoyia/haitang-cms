use serde::{Deserialize, Serialize};

use super::entity_seq::next_entity_id;
use super::locale::{pick_i18n_row, resolve_locale};

/// 菜单结构（不分语言）
#[derive(Debug, Clone, toasty::Model)]
pub struct MenuItemMeta {
    #[key]
    #[auto]
    pub id: i64,

    pub group_id: i64,

    /// 父菜单逻辑 ID，0 表示顶级
    pub parent_id: i64,

    pub icon: String,

    pub permission: String,

    pub sort: i64,

    /// 0 = 禁用, 1 = 启用
    pub status: i64,
}

/// 菜单文案与路径（按语言）
#[derive(Debug, Clone, toasty::Model)]
#[key(menu_item_id, lang)]
pub struct MenuItemI18n {
    pub menu_item_id: i64,

    pub lang: String,

    pub title: String,

    pub route_path: String,
}

/// 创建菜单
#[derive(Debug, Deserialize)]
pub struct CreateMenuItem {
    pub group_id: i64,
    pub parent_id: Option<i64>,
    pub title: String,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub permission: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
    pub lang: Option<String>,
}

/// 更新菜单
#[derive(Debug, Deserialize)]
pub struct UpdateMenuItem {
    pub group_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub title: Option<String>,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub permission: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
    pub lang: Option<String>,
}

/// 菜单序列化视图（含子节点，已 merge 当前语言）
#[derive(Debug, Clone, Serialize)]
pub struct MenuView {
    pub id: i64,
    pub group_id: i64,
    pub parent_id: i64,
    pub title: String,
    pub path: String,
    pub icon: String,
    pub permission: String,
    pub sort: i64,
    pub status: i64,
    pub children: Vec<MenuView>,
}

/// 合并后的菜单节点（内部建树用）
#[derive(Debug, Clone)]
pub struct MergedMenuItem {
    pub id: i64,
    pub group_id: i64,
    pub parent_id: i64,
    pub title: String,
    pub route_path: String,
    pub icon: String,
    pub permission: String,
    pub sort: i64,
    pub status: i64,
}

impl MergedMenuItem {
    pub fn to_view(&self, children: Vec<MenuView>) -> MenuView {
        MenuView {
            id: self.id,
            group_id: self.group_id,
            parent_id: self.parent_id,
            title: self.title.clone(),
            path: self.route_path.clone(),
            icon: self.icon.clone(),
            permission: self.permission.clone(),
            sort: self.sort,
            status: self.status,
            children,
        }
    }

    pub fn to_flat_view(&self) -> MenuView {
        self.to_view(Vec::new())
    }
}

pub async fn menu_i18n_rows(
    db: &mut toasty::Db,
    menu_item_id: i64,
) -> Result<Vec<MenuItemI18n>, String> {
    let all = MenuItemI18n::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询菜单翻译失败: {e}"))?;
    Ok(all
        .into_iter()
        .filter(|r| r.menu_item_id == menu_item_id)
        .collect())
}

pub async fn load_merged_menu_items(
    db: &mut toasty::Db,
    group_id: i64,
    lang: Option<&str>,
) -> Result<Vec<MergedMenuItem>, String> {
    let default = super::dict::get_site_default_locale(db).await;
    let resolved = resolve_locale(lang, &default);

    let metas: Vec<MenuItemMeta> = MenuItemMeta::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询菜单失败: {e}"))?
        .into_iter()
        .filter(|m| m.group_id == group_id)
        .collect();

    let mut merged = Vec::new();
    for meta in metas {
        let rows = menu_i18n_rows(db, meta.id).await?;
        let i18n = pick_i18n_row(&rows, &resolved, &default, |r| r.lang.as_str())
            .ok_or_else(|| format!("菜单 {} 缺少语言 {}", meta.id, resolved))?;
        merged.push(MergedMenuItem {
            id: meta.id,
            group_id: meta.group_id,
            parent_id: meta.parent_id,
            title: i18n.title.clone(),
            route_path: i18n.route_path.clone(),
            icon: meta.icon.clone(),
            permission: meta.permission.clone(),
            sort: meta.sort,
            status: meta.status,
        });
    }
    Ok(merged)
}

pub fn build_menu_tree(items: &[MergedMenuItem], parent_id: i64) -> Vec<MenuView> {
    let mut nodes: Vec<&MergedMenuItem> =
        items.iter().filter(|m| m.parent_id == parent_id).collect();
    nodes.sort_by_key(|m| m.sort);

    nodes
        .iter()
        .map(|m| {
            let children = build_menu_tree(items, m.id);
            m.to_view(children)
        })
        .collect()
}

pub fn validate_parent_id(
    items: &[MenuItemMeta],
    item_id: Option<i64>,
    group_id: i64,
    parent_id: i64,
) -> Result<(), String> {
    if parent_id == 0 {
        return Ok(());
    }

    let parent = items
        .iter()
        .find(|m| m.id == parent_id)
        .ok_or_else(|| "父菜单不存在".to_string())?;

    if parent.group_id != group_id {
        return Err("父菜单必须属于同一菜单组".to_string());
    }

    if let Some(id) = item_id {
        if parent_id == id {
            return Err("不能将自身设为父菜单".to_string());
        }
        if is_descendant_meta(items, id, parent_id) {
            return Err("不能将子菜单设为父菜单".to_string());
        }
    }

    Ok(())
}

fn is_descendant_meta(items: &[MenuItemMeta], ancestor_id: i64, candidate_id: i64) -> bool {
    let mut current = candidate_id;
    while current != 0 {
        if current == ancestor_id {
            return true;
        }
        current = items
            .iter()
            .find(|m| m.id == current)
            .map(|m| m.parent_id)
            .unwrap_or(0);
    }
    false
}

pub fn group_has_menus(items: &[MenuItemMeta], group_id: i64) -> bool {
    items.iter().any(|m| m.group_id == group_id)
}

pub fn menu_has_children(items: &[MenuItemMeta], menu_id: i64) -> bool {
    items.iter().any(|m| m.parent_id == menu_id)
}

pub async fn create_menu_item(
    db: &mut toasty::Db,
    input: &CreateMenuItem,
    default_lang: &str,
) -> Result<MenuItemMeta, String> {
    let lang = resolve_locale(input.lang.as_deref(), default_lang);
    let parent_id = input.parent_id.unwrap_or(0);
    let path = input.path.as_deref().unwrap_or("");
    let icon = input.icon.as_deref().unwrap_or("");
    let permission = input.permission.as_deref().unwrap_or("");
    let sort = input.sort.unwrap_or(0);
    let status = input.status.unwrap_or(1);

    let all_menus = MenuItemMeta::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询菜单失败: {e}"))?;

    validate_parent_id(&all_menus, None, input.group_id, parent_id)?;

    let meta = MenuItemMeta::create()
        .group_id(input.group_id)
        .parent_id(parent_id)
        .icon(icon)
        .permission(permission)
        .sort(sort)
        .status(status)
        .exec(db)
        .await
        .map_err(|e| format!("创建菜单失败: {e}"))?;

    MenuItemI18n::create()
        .menu_item_id(meta.id)
        .lang(&lang)
        .title(&input.title)
        .route_path(path)
        .exec(db)
        .await
        .map_err(|e| format!("创建菜单翻译失败: {e}"))?;

    Ok(meta)
}

pub async fn merged_menu_item(
    db: &mut toasty::Db,
    meta: &MenuItemMeta,
    lang: Option<&str>,
) -> Result<MergedMenuItem, String> {
    let items = load_merged_menu_items(db, meta.group_id, lang).await?;
    items
        .into_iter()
        .find(|m| m.id == meta.id)
        .ok_or_else(|| "菜单不存在".to_string())
}

pub async fn upsert_menu_i18n(
    db: &mut toasty::Db,
    menu_item_id: i64,
    lang: &str,
    title: &str,
    route_path: &str,
) -> Result<(), String> {
    let lang = super::locale::normalize_lang(lang);
    match MenuItemI18n::get_by_menu_item_id_and_lang(db, &menu_item_id, &lang).await {
        Ok(mut row) => {
            row.update()
                .title(title)
                .route_path(route_path)
                .exec(db)
                .await
                .map_err(|e| format!("更新菜单翻译失败: {e}"))?;
        }
        Err(_) => {
            MenuItemI18n::create()
                .menu_item_id(menu_item_id)
                .lang(&lang)
                .title(title)
                .route_path(route_path)
                .exec(db)
                .await
                .map_err(|e| format!("创建菜单翻译失败: {e}"))?;
        }
    }
    Ok(())
}

pub async fn delete_menu_item(db: &mut toasty::Db, id: i64) -> Result<(), String> {
    let meta = MenuItemMeta::get_by_id(db, &id)
        .await
        .map_err(|_| "菜单不存在".to_string())?;
    let rows = menu_i18n_rows(db, id).await?;
    for row in rows {
        row.delete()
            .exec(db)
            .await
            .map_err(|e| format!("删除菜单翻译失败: {e}"))?;
    }
    meta.delete()
        .exec(db)
        .await
        .map_err(|e| format!("删除菜单失败: {e}"))?;
    Ok(())
}

/// 种子：创建菜单 meta + 多语言 i18n
pub async fn seed_menu_with_i18n(
    db: &mut toasty::Db,
    group_id: i64,
    parent_id: i64,
    sort: i64,
    icon: &str,
    permission: &str,
    translations: &[(&str, &str, &str)],
) -> Result<(), String> {
    let meta = MenuItemMeta::create()
        .group_id(group_id)
        .parent_id(parent_id)
        .icon(icon)
        .permission(permission)
        .sort(sort)
        .status(1)
        .exec(db)
        .await
        .map_err(|e| format!("创建菜单失败: {e}"))?;

    for (lang, title, path) in translations {
        MenuItemI18n::create()
            .menu_item_id(meta.id)
            .lang(*lang)
            .title(*title)
            .route_path(*path)
            .exec(db)
            .await
            .map_err(|e| format!("创建菜单翻译失败: {e}"))?;
    }
    Ok(())
}

/// 预分配菜单 ID（扩展用）
#[allow(dead_code)]
pub async fn next_menu_item_id(db: &mut toasty::Db) -> Result<i64, String> {
    next_entity_id(db, "menu_item").await
}
