use std::path::PathBuf;

/// 管理后台 SPA 静态资源部署配置（运行时，与前端 `VITE_BASE` 路径段一致）
#[derive(Clone, Debug)]
pub struct AdminWebConfig {
    /// URL 路径段，如 `haitang-cms-admin`
    pub path_segment: String,
    /// 挂载路径，如 `/haitang-cms-admin`
    pub mount_path: String,
    /// 磁盘目录，如 `static/haitang-cms-admin`
    pub static_dir: PathBuf,
}

impl AdminWebConfig {
    pub fn from_env() -> Self {
        let segment = std::env::var("ADMIN_WEB_PATH")
            .unwrap_or_else(|_| "haitang-cms-admin".to_string());
        let segment = segment.trim().trim_matches('/').to_string();
        assert!(!segment.is_empty(), "ADMIN_WEB_PATH 不能为空");

        let static_dir = std::env::var("ADMIN_WEB_STATIC_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("static").join(&segment));

        Self {
            mount_path: format!("/{segment}"),
            path_segment: segment,
            static_dir,
        }
    }
}
