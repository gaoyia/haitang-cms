use super::menu_item::MenuView;

/// 后台侧边栏菜单组标识（内置只读，不入库）
pub const ADMIN_SIDEBAR_CODE: &str = "admin_sidebar";

/// 是否为内置只读的后台侧边栏菜单组
pub fn is_admin_sidebar_code(code: &str) -> bool {
    code == ADMIN_SIDEBAR_CODE
}

/// 内置侧边栏菜单项
#[derive(Debug, Clone)]
struct SidebarDef {
    id: i64,
    parent_id: i64,
    title: &'static str,
    path: &'static str,
    icon: &'static str,
    permission: &'static str,
    sort: i64,
}

fn sidebar_defs() -> &'static [SidebarDef] {
    &[
        SidebarDef {
            id: 101,
            parent_id: 0,
            title: "仪表盘",
            path: "/",
            icon: "📊",
            permission: "",
            sort: 0,
        },
        SidebarDef {
            id: 102,
            parent_id: 0,
            title: "文章管理",
            path: "",
            icon: "📝",
            permission: "",
            sort: 10,
        },
        SidebarDef {
            id: 108,
            parent_id: 102,
            title: "文章列表",
            path: "/posts",
            icon: "",
            permission: "post:read",
            sort: 0,
        },
        SidebarDef {
            id: 107,
            parent_id: 102,
            title: "分类管理",
            path: "/categories",
            icon: "",
            permission: "",
            sort: 10,
        },
        SidebarDef {
            id: 109,
            parent_id: 0,
            title: "轮播图管理",
            path: "",
            icon: "🖼️",
            permission: "",
            sort: 15,
        },
        SidebarDef {
            id: 110,
            parent_id: 109,
            title: "轮播图组",
            path: "/banner-groups",
            icon: "",
            permission: "",
            sort: 0,
        },
        SidebarDef {
            id: 111,
            parent_id: 109,
            title: "轮播图列表",
            path: "/banners",
            icon: "",
            permission: "",
            sort: 10,
        },
        SidebarDef {
            id: 106,
            parent_id: 0,
            title: "菜单管理",
            path: "",
            icon: "📋",
            permission: "",
            sort: 17,
        },
        SidebarDef {
            id: 113,
            parent_id: 106,
            title: "菜单树",
            path: "/menus",
            icon: "",
            permission: "",
            sort: 0,
        },
        SidebarDef {
            id: 114,
            parent_id: 106,
            title: "菜单组",
            path: "/menu-groups",
            icon: "",
            permission: "",
            sort: 10,
        },
        SidebarDef {
            id: 103,
            parent_id: 0,
            title: "用户管理",
            path: "",
            icon: "👥",
            permission: "",
            sort: 20,
        },
        SidebarDef {
            id: 104,
            parent_id: 103,
            title: "用户列表",
            path: "/users",
            icon: "",
            permission: "user:read",
            sort: 0,
        },
        SidebarDef {
            id: 105,
            parent_id: 103,
            title: "角色管理",
            path: "/roles",
            icon: "",
            permission: "role:read",
            sort: 10,
        },
        SidebarDef {
            id: 112,
            parent_id: 0,
            title: "字典管理",
            path: "/dicts",
            icon: "📖",
            permission: "",
            sort: 25,
        },
    ]
}

fn def_to_view(def: &SidebarDef, children: Vec<MenuView>) -> MenuView {
    MenuView {
        id: def.id,
        group_id: 0,
        parent_id: def.parent_id,
        title: def.title.to_string(),
        path: def.path.to_string(),
        icon: def.icon.to_string(),
        permission: def.permission.to_string(),
        sort: def.sort,
        status: 1,
        children,
    }
}

fn build_sidebar_tree(defs: &[&SidebarDef], parent_id: i64) -> Vec<MenuView> {
    let mut nodes: Vec<&&SidebarDef> = defs
        .iter()
        .filter(|m| m.parent_id == parent_id)
        .collect();
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

/// 获取后台侧边栏菜单树（按 JWT 权限过滤）
pub fn get_admin_sidebar_nav(permissions: &[String]) -> Vec<MenuView> {
    let visible: Vec<&SidebarDef> = sidebar_defs()
        .iter()
        .filter(|m| {
            m.permission.is_empty() || permissions.iter().any(|p| p == m.permission)
        })
        .collect();
    prune_empty_groups(build_sidebar_tree(&visible, 0))
}

/// 获取后台侧边栏完整菜单树（管理页只读展示）
pub fn get_admin_sidebar_tree() -> Vec<MenuView> {
    let defs: Vec<&SidebarDef> = sidebar_defs().iter().collect();
    build_sidebar_tree(&defs, 0)
}
