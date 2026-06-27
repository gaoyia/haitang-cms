use std::path::PathBuf;

/// 运行环境（控制启动时是否写入种子数据，不影响 schema patch）
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppEnvironment {
    Development,
    Production,
}

/// 应用运行配置
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub environment: AppEnvironment,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let raw = std::env::var("HAITANG_ENV").unwrap_or_else(|_| "development".to_string());
        let environment = match raw.trim().to_lowercase().as_str() {
            "production" | "prod" => AppEnvironment::Production,
            "development" | "dev" => AppEnvironment::Development,
            other => {
                eprintln!("[警告] 未知 HAITANG_ENV={other}，按 development 处理");
                AppEnvironment::Development
            }
        };
        Self { environment }
    }

    pub fn is_development(&self) -> bool {
        matches!(self.environment, AppEnvironment::Development)
    }
}

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
    /// 开发环境 Vite dev server 根地址（无尾斜杠），如 `http://127.0.0.1:5174`
    pub dev_server_url: String,
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

        let dev_server_url = std::env::var("ADMIN_WEB_DEV_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5174".to_string())
            .trim()
            .trim_end_matches('/')
            .to_string();

        Self {
            mount_path: format!("/{segment}"),
            path_segment: segment,
            static_dir,
            dev_server_url,
        }
    }

    /// 开发环境：将挂载路径下的子路径重定向到 Vite（dev base 为 `/`，故去掉后台前缀段）
    pub fn dev_redirect_url(&self, subpath: &std::path::Path, query: Option<&str>) -> String {
        let mut url = format!("{}/", self.dev_server_url);
        let sub = subpath.to_string_lossy();
        if !sub.is_empty() {
            url = format!(
                "{}/{}",
                self.dev_server_url,
                sub.trim_start_matches('/').replace('\\', "/")
            );
        }
        if let Some(q) = query {
            let q = q.trim_start_matches('?');
            if !q.is_empty() {
                url.push('?');
                url.push_str(q);
            }
        }
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cfg() -> AdminWebConfig {
        AdminWebConfig {
            path_segment: "haitang-cms-admin".to_string(),
            mount_path: "/haitang-cms-admin".to_string(),
            static_dir: PathBuf::from("static/haitang-cms-admin"),
            dev_server_url: "http://127.0.0.1:5174".to_string(),
        }
    }

    #[test]
    fn dev_redirect_strips_mount_segment() {
        let cfg = test_cfg();
        assert_eq!(cfg.dev_redirect_url(std::path::Path::new(""), None), "http://127.0.0.1:5174/");
        assert_eq!(
            cfg.dev_redirect_url(std::path::Path::new("dicts"), None),
            "http://127.0.0.1:5174/dicts"
        );
        assert_eq!(
            cfg.dev_redirect_url(std::path::Path::new("dicts"), Some("tab=1")),
            "http://127.0.0.1:5174/dicts?tab=1"
        );
    }
}
