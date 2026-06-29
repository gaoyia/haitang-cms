use serde::Serialize;

use super::admin_sidebar::get_admin_sidebar_tree;
use super::dict::{get_site_default_locale, get_site_locales, load_dict_map};
use super::locale::{html_lang_attr, locale_path, locale_switch_suffix, public_page_title};
use super::menu_group::{MenuGroup, MenuGroupView, admin_sidebar_group_view};
use super::menu_item::{MenuView, build_menu_tree, load_merged_menu_items};

/// 公开页菜单链接（供 Tera 模板渲染）
#[derive(Debug, Clone, Serialize)]
pub struct PublicMenuLink {
    pub title: String,
    pub path: String,
}

/// 菜单管理页：菜单组及其完整菜单树
#[derive(Debug, Serialize)]
pub struct MenuGroupTreeView {
    pub group: MenuGroupView,
    pub menus: Vec<MenuView>,
}

/// 从数据库获取公开页菜单链接（顶级、启用、有路径）
pub async fn get_public_menu_links(
    db: &mut toasty::Db,
    code: &str,
    lang: Option<&str>,
) -> Vec<PublicMenuLink> {
    let group = match super::menu_group::find_menu_group_by_code(db, code).await {
        Ok(g) => g,
        Err(_) => return Vec::new(),
    };

    if group.status != 1 {
        return Vec::new();
    }

    let merged = match load_merged_menu_items(db, group.id, lang).await {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };

    let mut items: Vec<_> = merged
        .iter()
        .filter(|m| m.parent_id == 0 && m.status == 1 && !m.route_path.is_empty())
        .collect();
    items.sort_by_key(|m| m.sort);

    items
        .into_iter()
        .map(|m| PublicMenuLink {
            title: m.title.clone(),
            path: m.route_path.clone(),
        })
        .collect()
}

/// 公开页模板上下文
pub async fn site_page_context(
    db: &mut toasty::Db,
    page_slug: &str,
    current_path: &str,
    lang: Option<&str>,
) -> serde_json::Value {
    let default_lang = get_site_default_locale(db).await;
    let resolved_lang = lang
        .map(super::locale::normalize_lang)
        .unwrap_or(default_lang.clone());
    let supported = get_site_locales(db).await;
    let dict_map = load_dict_map(db, Some(&resolved_lang)).await;
    let site_icp = dict_map.get("site_icp").cloned().unwrap_or_default();
    let site_name = dict_map
        .get("site_name")
        .cloned()
        .unwrap_or_else(|| "海棠 CMS".to_string());
    let site_copyright = dict_map
        .get("site_copyright")
        .cloned()
        .unwrap_or_else(|| "© 2026 海棠 CMS".to_string());
    let title = public_page_title(&resolved_lang, page_slug);
    let path_suffix = locale_switch_suffix(current_path);
    let locale_links: Vec<_> = supported
        .iter()
        .map(|loc| {
            serde_json::json!({
                "lang": loc,
                "label": if loc == "en-us" { "English" } else { "中文" },
                "url": locale_path(loc, &path_suffix),
                "active": loc == &resolved_lang,
            })
        })
        .collect();

    serde_json::json!({
        "title": title,
        "page_slug": page_slug,
        "lang": resolved_lang,
        "html_lang": html_lang_attr(&resolved_lang),
        "locale_prefix": format!("/{}", resolved_lang),
        "locale_links": locale_links,
        "current_path": current_path,
        "header_menus": get_public_menu_links(db, "site_header", Some(&resolved_lang)).await,
        "footer_menus": get_public_menu_links(db, "site_footer", Some(&resolved_lang)).await,
        "dict": dict_map,
        "site_icp": site_icp,
        "site_name": site_name,
        "site_copyright": site_copyright,
    })
}

/// 获取数据库菜单组的菜单树
pub async fn get_db_menu_tree(
    db: &mut toasty::Db,
    group_id: i64,
    lang: Option<&str>,
) -> Result<Vec<MenuView>, String> {
    let merged = load_merged_menu_items(db, group_id, lang).await?;
    Ok(build_menu_tree(&merged, 0))
}

/// 获取所有菜单组及菜单树（含只读后台侧边栏）
pub async fn all_menu_group_trees(
    db: &mut toasty::Db,
    lang: Option<&str>,
) -> Result<Vec<MenuGroupTreeView>, String> {
    let mut result = vec![MenuGroupTreeView {
        group: admin_sidebar_group_view(),
        menus: get_admin_sidebar_tree(),
    }];

    let mut groups = MenuGroup::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询菜单组失败: {e}"))?;
    groups.sort_by_key(|g| g.sort);

    for group in groups {
        let menus = get_db_menu_tree(db, group.id, lang).await?;
        result.push(MenuGroupTreeView {
            group: group.to_view(),
            menus,
        });
    }

    Ok(result)
}
