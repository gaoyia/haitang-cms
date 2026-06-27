use serde::Serialize;

/// 系统所有可用权限定义
#[derive(Debug, Serialize)]
pub struct PermissionDef {
    pub code: &'static str,
    pub label: &'static str,
    pub group: &'static str,
}

pub const ALL_PERMISSIONS: &[PermissionDef] = &[
    PermissionDef { code: "post:read",    label: "查看文章", group: "文章管理" },
    PermissionDef { code: "post:create",  label: "创建文章", group: "文章管理" },
    PermissionDef { code: "post:update",  label: "编辑文章", group: "文章管理" },
    PermissionDef { code: "post:delete",  label: "删除文章", group: "文章管理" },

    PermissionDef { code: "category:read",    label: "查看分类", group: "分类管理" },
    PermissionDef { code: "category:create",  label: "创建分类", group: "分类管理" },
    PermissionDef { code: "category:update",  label: "编辑分类", group: "分类管理" },
    PermissionDef { code: "category:delete",  label: "删除分类", group: "分类管理" },

    PermissionDef { code: "user:read",    label: "查看用户", group: "用户管理" },
    PermissionDef { code: "user:create",  label: "创建用户", group: "用户管理" },
    PermissionDef { code: "user:update",  label: "编辑用户", group: "用户管理" },
    PermissionDef { code: "user:delete",  label: "删除用户", group: "用户管理" },

    PermissionDef { code: "role:read",    label: "查看角色", group: "角色管理" },
    PermissionDef { code: "role:create",  label: "创建角色", group: "角色管理" },
    PermissionDef { code: "role:update",  label: "编辑角色", group: "角色管理" },
    PermissionDef { code: "role:delete",  label: "删除角色", group: "角色管理" },

    PermissionDef { code: "menu_group:read",   label: "查看菜单组", group: "菜单组管理" },
    PermissionDef { code: "menu_group:create", label: "创建菜单组", group: "菜单组管理" },
    PermissionDef { code: "menu_group:update", label: "编辑菜单组", group: "菜单组管理" },
    PermissionDef { code: "menu_group:delete", label: "删除菜单组", group: "菜单组管理" },

    PermissionDef { code: "menu:read",    label: "查看菜单", group: "菜单管理" },
    PermissionDef { code: "menu:create",  label: "创建菜单", group: "菜单管理" },
    PermissionDef { code: "menu:update",  label: "编辑菜单", group: "菜单管理" },
    PermissionDef { code: "menu:delete",  label: "删除菜单", group: "菜单管理" },

    PermissionDef { code: "banner_group:read",   label: "查看轮播图组", group: "轮播图组管理" },
    PermissionDef { code: "banner_group:create", label: "创建轮播图组", group: "轮播图组管理" },
    PermissionDef { code: "banner_group:update", label: "编辑轮播图组", group: "轮播图组管理" },
    PermissionDef { code: "banner_group:delete", label: "删除轮播图组", group: "轮播图组管理" },

    PermissionDef { code: "banner:read",    label: "查看轮播图", group: "轮播图管理" },
    PermissionDef { code: "banner:create",  label: "创建轮播图", group: "轮播图管理" },
    PermissionDef { code: "banner:update",  label: "编辑轮播图", group: "轮播图管理" },
    PermissionDef { code: "banner:delete",  label: "删除轮播图", group: "轮播图管理" },

    PermissionDef { code: "dict:read",    label: "查看字典", group: "字典管理" },
    PermissionDef { code: "dict:create",  label: "创建字典", group: "字典管理" },
    PermissionDef { code: "dict:update",  label: "编辑字典", group: "字典管理" },
    PermissionDef { code: "dict:delete",  label: "删除字典", group: "字典管理" },
];

/// 所有权限代码列表（用于超级管理员）
pub fn all_permission_codes() -> Vec<String> {
    ALL_PERMISSIONS.iter().map(|p| p.code.to_string()).collect()
}
