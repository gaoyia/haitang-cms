/// 用户-角色关联表 — Toasty ORM
#[derive(Debug, toasty::Model)]
pub struct UserRole {
    #[key]
    #[auto]
    pub id: i64,

    pub user_id: i64,

    pub role_id: i64,
}
