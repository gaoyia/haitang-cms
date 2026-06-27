use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{EncodingKey, Header, encode};
use rocket::State;
use rocket::serde::json::Json;

use crate::config::AdminWebConfig;
use crate::guards::auth::JwtConfig;
use crate::models::{
    ApiResponse, Banner, BannerGroup, Claims, LoginRequest, LoginResponse, LoginUserInfo,
    MenuGroup, Role, User, UserRole, all_permission_codes, ensure_banner_seed_asset_link,
    find_banner_group_by_code, seed_default_banner_asset, seed_menu_with_i18n,
};
use crate::storage::StorageService;

/// bcrypt 密码哈希
pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("密码哈希失败")
}

/// 校验明文密码与 bcrypt 哈希是否匹配
pub fn verify_password(password: &str, password_hash: &str) -> bool {
    verify(password, password_hash).unwrap_or(false)
}

/// 简易 Unix 时间戳（避免额外依赖 chrono）
pub fn epoch_secs() -> usize {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

/// 种子数据：确保默认管理员和角色存在，并同步 admin 角色权限
pub async fn seed_admin(db: &mut toasty::Db) {
    sync_admin_role_permissions(db).await;
    seed_default_site_menus(db).await;
    crate::models::seed_default_dicts(db).await;
    crate::models::seed_default_categories(db).await;

    // 检查是否已有用户
    let user_count = match User::all().exec(db).await {
        Ok(users) => users.len(),
        Err(_) => 0,
    };
    if user_count > 0 {
        return; // 已有用户，跳过种子
    }

    println!("[种子] 创建默认管理员账户...");

    // 创建超级管理员角色
    let all_perms = all_permission_codes().join(",");
    let admin_role = Role::create()
        .name("admin")
        .description("超级管理员")
        .permissions(&all_perms)
        .exec(db)
        .await
        .expect("创建管理员角色失败");

    // 创建默认管理员
    let admin_user = User::create()
        .username("admin")
        .password_hash(hash_password("admin123"))
        .nickname("管理员")
        .email("admin@localhost")
        .status(1i64)
        .exec(db)
        .await
        .expect("创建管理员用户失败");

    // 关联角色
    UserRole::create()
        .user_id(admin_user.id)
        .role_id(admin_role.id)
        .exec(db)
        .await
        .expect("创建用户角色关联失败");

    println!("[种子] 管理员账户已创建: admin / admin123");
}

/// 将 admin 角色权限同步为系统最新权限列表（新增权限后自动补全）
async fn sync_admin_role_permissions(db: &mut toasty::Db) {
    let all_perms = all_permission_codes();
    let all_perms_csv = all_perms.join(",");

    let roles = match Role::all().exec(db).await {
        Ok(r) => r,
        Err(_) => return,
    };

    for mut role in roles {
        if role.name != "admin" {
            continue;
        }

        let current: std::collections::HashSet<_> = role.permissions_vec().into_iter().collect();
        let latest: std::collections::HashSet<_> = all_perms.iter().cloned().collect();
        if current == latest {
            continue;
        }

        if role
            .update()
            .permissions(&all_perms_csv)
            .exec(db)
            .await
            .is_ok()
        {
            println!("[种子] 已同步 admin 角色权限");
        }
    }
}

/// 种子数据：初始化公开页默认菜单组与菜单项
async fn seed_default_site_menus(db: &mut toasty::Db) {
    let admin_path = AdminWebConfig::from_env().mount_path;
    let groups = match MenuGroup::all().exec(db).await {
        Ok(g) => g,
        Err(_) => return,
    };

    if groups.iter().any(|g| g.code == "site_header") {
        return;
    }

    println!("[种子] 创建默认公开页菜单...");

    let header = MenuGroup::create()
        .name("页头菜单")
        .code("site_header")
        .description("公开网站顶栏导航")
        .sort(10)
        .status(1i64)
        .exec(db)
        .await
        .expect("创建页头菜单组失败");

    let footer = MenuGroup::create()
        .name("页脚菜单")
        .code("site_footer")
        .description("公开网站页脚链接")
        .sort(20)
        .status(1i64)
        .exec(db)
        .await
        .expect("创建页脚菜单组失败");

    let header_items: &[(&str, &str, &str, &str, i64)] = &[
        ("首页", "/zh-cn/", "Home", "/en-us/", 0),
        ("最新文章", "/zh-cn/posts", "Posts", "/en-us/posts", 10),
        (
            "管理后台",
            admin_path.as_str(),
            "Admin",
            admin_path.as_str(),
            20,
        ),
    ];
    for (zh_title, zh_path, en_title, en_path, sort) in header_items {
        seed_menu_with_i18n(
            db,
            header.id,
            0,
            *sort,
            "",
            "",
            &[
                ("zh-cn", *zh_title, *zh_path),
                ("en-us", *en_title, *en_path),
            ],
        )
        .await
        .expect("创建页头菜单失败");
    }

    let footer_items: &[(&str, &str, &str, &str, i64)] = &[
        ("首页", "/zh-cn/", "Home", "/en-us/", 0),
        ("最新文章", "/zh-cn/posts", "Posts", "/en-us/posts", 5),
        (
            "管理后台",
            admin_path.as_str(),
            "Admin",
            admin_path.as_str(),
            10,
        ),
        ("关于我们", "/zh-cn/about", "About", "/en-us/about", 20),
    ];
    for (zh_title, zh_path, en_title, en_path, sort) in footer_items {
        seed_menu_with_i18n(
            db,
            footer.id,
            0,
            *sort,
            "",
            "",
            &[
                ("zh-cn", *zh_title, *zh_path),
                ("en-us", *en_title, *en_path),
            ],
        )
        .await
        .expect("创建页脚菜单失败");
    }

    println!("[种子] 默认公开页菜单已创建");
}

/// 种子数据：初始化默认首页轮播图组、示例条目及关联资源
pub async fn seed_default_banner_data(db: &mut toasty::Db, storage: &StorageService) {
    let group = match find_banner_group_by_code(db, "home_banner").await {
        Ok(g) => g,
        Err(_) => {
            println!("[种子] 创建默认轮播图组...");

            match BannerGroup::create()
                .name("首页轮播")
                .code("home_banner")
                .description("网站首页顶部轮播图")
                .sort(0)
                .status(1i64)
                .exec(db)
                .await
            {
                Ok(g) => {
                    println!("[种子] 默认轮播图组 home_banner 已创建");
                    g
                }
                Err(e) => {
                    eprintln!("[种子] 创建默认轮播图组失败: {e}");
                    return;
                }
            }
        }
    };

    let asset = match seed_default_banner_asset(db, storage).await {
        Ok(a) => a,
        Err(e) => {
            eprintln!("[种子] 默认轮播图资源: {e}");
            return;
        }
    };

    let banners = match Banner::all().exec(db).await {
        Ok(list) => list,
        Err(_) => return,
    };

    let banner = if let Some(existing) = banners.iter().find(|b| b.group_id == group.id) {
        existing.clone()
    } else {
        println!("[种子] 创建默认轮播图...");

        match Banner::create()
            .group_id(group.id)
            .title("简约工作台")
            .image_url("")
            .link_url("")
            .description("明亮极简的工作空间，银色笔记本电脑与红色花卉。")
            .sort(0)
            .status(1)
            .exec(db)
            .await
        {
            Ok(b) => {
                println!("[种子] 默认轮播图 banner-1 已创建");
                b
            }
            Err(e) => {
                eprintln!("[种子] 创建默认轮播图失败: {e}");
                return;
            }
        }
    };

    if let Err(e) = ensure_banner_seed_asset_link(db, storage, banner.id, asset.id).await {
        eprintln!("[种子] 关联默认轮播图资源失败: {e}");
    }
}

/// 根据用户 ID 获取角色 ID 列表
pub async fn get_user_role_ids(db: &mut toasty::Db, user_id: i64) -> Vec<i64> {
    match UserRole::all().exec(db).await {
        Ok(links) => links
            .into_iter()
            .filter(|ur| ur.user_id == user_id)
            .map(|ur| ur.role_id)
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// 根据角色 ID 列表获取所有角色名称和合并权限
pub async fn get_roles_info(db: &mut toasty::Db, role_ids: &[i64]) -> (Vec<String>, Vec<String>) {
    let all_roles = match Role::all().exec(db).await {
        Ok(roles) => roles,
        Err(_) => return (Vec::new(), Vec::new()),
    };

    let mut names = Vec::new();
    let mut perms_set = std::collections::HashSet::new();

    for role in all_roles {
        if role_ids.contains(&role.id) {
            for p in role.permissions_vec() {
                perms_set.insert(p);
            }
            names.push(role.name);
        }
    }

    let perms: Vec<String> = perms_set.into_iter().collect();
    (names, perms)
}

/// 管理员登录 — 获取 JWT token
#[post("/api/admin/login", data = "<input>")]
pub async fn login(
    jwt_config: &State<JwtConfig>,
    db: &State<toasty::Db>,
    input: Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    let mut db = db.inner().clone();

    // 查找用户
    let users = match User::all().exec(&mut db).await {
        Ok(u) => u,
        Err(e) => return Json(ApiResponse::error(500, format!("数据库查询失败: {e}"))),
    };

    let user = match users.into_iter().find(|u| u.username == input.username) {
        Some(u) => u,
        None => return Json(ApiResponse::error(401, "用户名或密码错误")),
    };

    // 验证密码
    if !verify_password(&input.password, &user.password_hash) {
        return Json(ApiResponse::error(401, "用户名或密码错误"));
    }

    // 检查状态
    if user.status != 1 {
        return Json(ApiResponse::error(403, "账户已被禁用"));
    }

    // 获取角色和权限
    let role_ids = get_user_role_ids(&mut db, user.id).await;
    let (roles, permissions) = get_roles_info(&mut db, &role_ids).await;

    // 签发 JWT
    let expiration = epoch_secs() + 86400; // 24 小时
    let claims = Claims {
        sub: user.username.clone(),
        user_id: user.id,
        roles: roles.clone(),
        permissions: permissions.clone(),
        exp: expiration,
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_config.secret.as_bytes()),
    ) {
        Ok(token) => Json(ApiResponse::success(LoginResponse {
            token,
            user: LoginUserInfo {
                id: user.id,
                username: user.username,
                nickname: user.nickname,
                roles,
                permissions,
            },
        })),
        Err(e) => Json(ApiResponse::error(500, format!("生成令牌失败: {e}"))),
    }
}

/// 获取当前用户信息（通过 JWT 解析，权限从数据库实时读取）
#[get("/api/admin/me")]
pub async fn me(
    auth: crate::guards::AdminAuth,
    db: &State<toasty::Db>,
) -> Json<ApiResponse<LoginUserInfo>> {
    let mut db = db.inner().clone();
    let role_ids = get_user_role_ids(&mut db, auth.claims.user_id).await;
    let (roles, permissions) = get_roles_info(&mut db, &role_ids).await;

    let nickname = match User::get_by_id(&mut db, &auth.claims.user_id).await {
        Ok(user) => user.nickname,
        Err(_) => String::new(),
    };

    Json(ApiResponse::success(LoginUserInfo {
        id: auth.claims.user_id,
        username: auth.claims.sub,
        nickname,
        roles,
        permissions,
    }))
}
