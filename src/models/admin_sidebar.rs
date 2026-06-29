use std::collections::HashMap;

use serde::Serialize;

use super::menu_item::MenuView;

/// 后台侧边栏菜单组标识（内置只读，不入库）
pub const ADMIN_SIDEBAR_CODE: &str = "admin_sidebar";

/// 是否为内置只读的后台侧边栏菜单组
pub fn is_admin_sidebar_code(code: &str) -> bool {
    code == ADMIN_SIDEBAR_CODE
}

/// 与 admin-web 路由生成器（authMenu.json）对齐的导航项
#[derive(Debug, Clone, Serialize)]
pub struct AdminNavMenuJsonItem {
    #[serde(rename = "menuId")]
    pub menu_id: i64,
    #[serde(rename = "menuName")]
    pub menu_name: String,
    #[serde(rename = "parentId")]
    pub parent_id: i64,
    #[serde(rename = "menuType")]
    pub menu_type: String,
    pub path: String,
    pub name: String,
    pub component: String,
    pub icon: String,
    #[serde(rename = "isVisible")]
    pub is_visible: String,
    #[serde(rename = "linkUrl")]
    pub link_url: String,
    #[serde(rename = "isKeepAlive")]
    pub is_keep_alive: String,
    #[serde(rename = "isTag")]
    pub is_tag: String,
    #[serde(rename = "isAffix")]
    pub is_affix: String,
    pub redirect: String,
}

/// 内置侧边栏菜单项（与 admin-web 当前展示结构一致）
#[derive(Debug, Clone)]
struct SidebarDef {
    id: i64,
    parent_id: i64,
    menu_name: &'static str,
    path: &'static str,
    name: &'static str,
    component: &'static str,
    icon: &'static str,
    permission: &'static str,
    menu_type: &'static str,
    redirect: &'static str,
    sort: i64,
}

fn sidebar_defs() -> &'static [SidebarDef] {
    &[
        SidebarDef {
            id: 100,
            parent_id: 0,
            menu_name: "menu.content.auth",
            path: "/content",
            name: "contentPage",
            component: "",
            icon: "koi-document",
            permission: "",
            menu_type: "1",
            redirect: "/content/posts",
            sort: 10,
        },
        SidebarDef {
            id: 101,
            parent_id: 100,
            menu_name: "menu.content.posts",
            path: "/content/posts",
            name: "postListPage",
            component: "content/posts/index",
            icon: "koi-note-text",
            permission: "post:read",
            menu_type: "2",
            redirect: "",
            sort: 0,
        },
        SidebarDef {
            id: 102,
            parent_id: 100,
            menu_name: "menu.content.categories",
            path: "/content/categories",
            name: "categoryListPage",
            component: "content/categories/index",
            icon: "koi-copy",
            permission: "",
            menu_type: "2",
            redirect: "",
            sort: 10,
        },
        SidebarDef {
            id: 150,
            parent_id: 0,
            menu_name: "menu.assets.auth",
            path: "/assets",
            name: "assetLibraryPage",
            component: "assets/index",
            icon: "koi-picture",
            permission: "",
            menu_type: "2",
            redirect: "",
            sort: 15,
        },
        SidebarDef {
            id: 200,
            parent_id: 0,
            menu_name: "menu.banner.auth",
            path: "/banner",
            name: "bannerManagePage",
            component: "banner/index",
            icon: "koi-picture",
            permission: "",
            menu_type: "2",
            redirect: "",
            sort: 20,
        },
        SidebarDef {
            id: 300,
            parent_id: 0,
            menu_name: "menu.system.auth",
            path: "/system",
            name: "systemPage",
            component: "",
            icon: "koi-setting",
            permission: "",
            menu_type: "1",
            redirect: "/system/users",
            sort: 30,
        },
        SidebarDef {
            id: 301,
            parent_id: 300,
            menu_name: "menu.system.users",
            path: "/system/users",
            name: "userListPage",
            component: "system/users/index",
            icon: "koi-enhance-user",
            permission: "user:read",
            menu_type: "2",
            redirect: "",
            sort: 0,
        },
        SidebarDef {
            id: 302,
            parent_id: 300,
            menu_name: "menu.system.roles",
            path: "/system/roles",
            name: "roleListPage",
            component: "system/roles/index",
            icon: "koi-user-search",
            permission: "role:read",
            menu_type: "2",
            redirect: "",
            sort: 10,
        },
        SidebarDef {
            id: 400,
            parent_id: 0,
            menu_name: "menu.menu.auth",
            path: "/menus",
            name: "menuManagePage",
            component: "menus/index",
            icon: "koi-tree-right",
            permission: "",
            menu_type: "2",
            redirect: "",
            sort: 40,
        },
        SidebarDef {
            id: 500,
            parent_id: 0,
            menu_name: "menu.dict.auth",
            path: "/dicts",
            name: "dictListPage",
            component: "dicts/index",
            icon: "koi-book-square",
            permission: "",
            menu_type: "2",
            redirect: "",
            sort: 50,
        },
    ]
}

fn def_by_id() -> HashMap<i64, &'static SidebarDef> {
    sidebar_defs().iter().map(|d| (d.id, d)).collect()
}

impl SidebarDef {
    fn to_nav_json(&self) -> AdminNavMenuJsonItem {
        let is_tag = if self.menu_type == "1" { "0" } else { "1" };
        AdminNavMenuJsonItem {
            menu_id: self.id,
            menu_name: self.menu_name.to_string(),
            parent_id: self.parent_id,
            menu_type: self.menu_type.to_string(),
            path: self.path.to_string(),
            name: self.name.to_string(),
            component: self.component.to_string(),
            icon: self.icon.to_string(),
            is_visible: "1".to_string(),
            link_url: String::new(),
            is_keep_alive: "1".to_string(),
            is_tag: is_tag.to_string(),
            is_affix: "0".to_string(),
            redirect: self.redirect.to_string(),
        }
    }
}

fn def_to_view(def: &SidebarDef, children: Vec<MenuView>) -> MenuView {
    MenuView {
        id: def.id,
        group_id: 0,
        parent_id: def.parent_id,
        title: def.menu_name.to_string(),
        path: def.path.to_string(),
        icon: def.icon.to_string(),
        permission: def.permission.to_string(),
        sort: def.sort,
        status: 1,
        children,
    }
}

fn build_sidebar_tree(defs: &[&SidebarDef], parent_id: i64) -> Vec<MenuView> {
    let mut nodes: Vec<&&SidebarDef> = defs.iter().filter(|m| m.parent_id == parent_id).collect();
    nodes.sort_by_key(|m| m.sort);

    nodes
        .iter()
        .map(|m| {
            let children = build_sidebar_tree(defs, m.id);
            def_to_view(m, children)
        })
        .collect()
}

fn prune_empty_groups(nodes: Vec<MenuView>) -> Vec<MenuView> {
    nodes
        .into_iter()
        .filter_map(|mut node| {
            node.children = prune_empty_groups(node.children);
            if node.path.is_empty() && node.children.is_empty() {
                None
            } else {
                Some(node)
            }
        })
        .collect()
}

fn collect_tree_ids(nodes: &[MenuView], out: &mut Vec<i64>) {
    for node in nodes {
        out.push(node.id);
        collect_tree_ids(&node.children, out);
    }
}

fn tree_to_nav_items(tree: &[MenuView]) -> Vec<AdminNavMenuJsonItem> {
    let map = def_by_id();
    let mut ids = Vec::new();
    collect_tree_ids(tree, &mut ids);
    ids.iter()
        .filter_map(|id| map.get(id).map(|def| def.to_nav_json()))
        .collect()
}

/// 获取当前用户可见的后台侧栏导航（扁平 JSON，供 admin-web 动态路由）
pub fn get_admin_sidebar_nav_items(permissions: &[String]) -> Vec<AdminNavMenuJsonItem> {
    let visible: Vec<&SidebarDef> = sidebar_defs()
        .iter()
        .filter(|m| {
            m.permission.is_empty() || permissions.iter().any(|p| p == m.permission)
        })
        .collect();
    let tree = prune_empty_groups(build_sidebar_tree(&visible, 0));
    tree_to_nav_items(&tree)
}

/// 获取后台侧边栏菜单树（管理页只读预览）
pub fn get_admin_sidebar_tree() -> Vec<MenuView> {
    let defs: Vec<&SidebarDef> = sidebar_defs().iter().collect();
    build_sidebar_tree(&defs, 0)
}

/// 按 id 获取单个内置侧边栏菜单项（管理页只读详情）
pub fn get_admin_sidebar_item(id: i64) -> Option<MenuView> {
    sidebar_defs()
        .iter()
        .find(|m| m.id == id)
        .map(|def| def_to_view(def, vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_sidebar_nav_matches_vue_routes() {
        let items = get_admin_sidebar_nav_items(&[
            "post:read".to_string(),
            "user:read".to_string(),
            "role:read".to_string(),
        ]);
        let posts = items
            .iter()
            .find(|item| item.menu_id == 101)
            .expect("文章列表应存在");
        assert_eq!(posts.path, "/content/posts");
        assert_eq!(posts.icon, "koi-note-text");
        let menus = items
            .iter()
            .find(|item| item.menu_id == 400)
            .expect("菜单管理应存在");
        assert_eq!(menus.icon, "koi-tree-right");
    }

    #[test]
    fn admin_sidebar_item_lookup_returns_builtin_menu() {
        let item = get_admin_sidebar_item(101).expect("文章列表应存在");
        assert_eq!(item.title, "menu.content.posts");
        assert_eq!(item.path, "/content/posts");
    }

    #[test]
    fn admin_sidebar_item_lookup_unknown_id_returns_none() {
        assert!(get_admin_sidebar_item(99999).is_none());
    }
}
