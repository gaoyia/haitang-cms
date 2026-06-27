use std::path::PathBuf;

/// 上传存储配置
#[derive(Clone, Debug)]
pub struct StorageConfig {
    /// local | aliyun | tencent
    pub backend: String,
    pub local_dir: PathBuf,
    /// 对外 URL 前缀，如 /static/uploads
    pub public_url_prefix: String,
    pub max_bytes: u64,
}

impl StorageConfig {
    pub fn from_env() -> Self {
        let backend = std::env::var("STORAGE_BACKEND").unwrap_or_else(|_| "local".to_string());
        let local_dir = std::env::var("STORAGE_LOCAL_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("static/uploads"));
        let public_url_prefix = std::env::var("STORAGE_PUBLIC_PREFIX")
            .unwrap_or_else(|_| "/static/uploads".to_string());
        let max_bytes = std::env::var("STORAGE_MAX_BYTES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10 * 1024 * 1024);

        Self {
            backend: backend.trim().to_lowercase(),
            local_dir,
            public_url_prefix: public_url_prefix.trim_end_matches('/').to_string(),
            max_bytes,
        }
    }
}

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
        let segment =
            std::env::var("ADMIN_WEB_PATH").unwrap_or_else(|_| "haitang-cms-admin".to_string());
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
