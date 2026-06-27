pub mod filename;
pub mod local;
pub mod oss;
pub mod policy;

use std::sync::Arc;

use crate::config::StorageConfig;

pub use filename::{extension_for, resolve_upload_file_meta};

pub use local::LocalStorage;
pub use oss::OssStorage;
pub use policy::{AssetPurpose, BannerAssetRole, PostAssetRole};

/// 生成上传对象 ID（UUID v7，按时间有序，紧凑无连字符）
pub fn new_upload_uuid() -> String {
    uuid::Uuid::now_v7().simple().to_string()
}

/// 生成相对存储根的对象键：`{YYYY-MM-DD}/{user_id}/{uuid_v7}.{ext}`
pub fn new_object_key(user_id: i64, extension: Option<&str>) -> String {
    let date_dir = upload_date_dir();
    let id = new_upload_uuid();
    match extension.filter(|e| !e.is_empty()) {
        Some(ext) => format!("{date_dir}/{user_id}/{id}.{ext}"),
        None => format!("{date_dir}/{user_id}/{id}"),
    }
}

/// 上传文件 UTC 日期目录，格式 `YYYY-MM-DD`
pub fn upload_date_dir() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format_utc_date(secs)
}

/// Unix 秒 → UTC `YYYY-MM-DD`（不引入 chrono）
fn format_utc_date(secs: u64) -> String {
    let days = secs / 86_400;
    let (y, m, d) = civil_from_days(days as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

/// 算法来源：https://howardhinnant.github.io/date_algorithms.html
fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i32 + (era * 400) as i32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod date_tests {
    use super::*;

    #[test]
    fn epoch_start_is_1970_01_01() {
        assert_eq!(format_utc_date(0), "1970-01-01");
    }

    #[test]
    fn date_dir_matches_pattern() {
        let dir = upload_date_dir();
        assert!(dir.len() == 10);
        assert_eq!(dir.as_bytes()[4], b'-');
        assert_eq!(dir.as_bytes()[7], b'-');
    }
}

/// 存储层错误
#[derive(Debug)]
pub enum StorageError {
    Io(String),
    Unsupported(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "{msg}"),
            Self::Unsupported(msg) => write!(f, "{msg}"),
        }
    }
}

/// 可插拔存储后端
pub trait AssetStorage: Send + Sync {
    fn put(&self, storage_key: &str, data: &[u8]) -> Result<(), StorageError>;
    fn delete(&self, storage_key: &str) -> Result<(), StorageError>;
    fn public_url(&self, storage_key: &str) -> String;
    fn new_object_key(&self, user_id: i64, extension: Option<&str>) -> String;
}

/// 运行时存储服务（Rocket State）
#[derive(Clone)]
pub struct StorageService {
    inner: Arc<dyn AssetStorage>,
    pub config: StorageConfig,
}

impl StorageService {
    pub fn from_config(config: StorageConfig) -> Result<Self, StorageError> {
        let inner: Arc<dyn AssetStorage> = match config.backend.as_str() {
            "local" => Arc::new(LocalStorage::new(
                config.local_dir.clone(),
                config.public_url_prefix.clone(),
            )),
            "aliyun" | "tencent" | "oss" => Arc::new(OssStorage::new()),
            other => {
                return Err(StorageError::Unsupported(format!(
                    "未知 STORAGE_BACKEND: {other}"
                )));
            }
        };
        Ok(Self { inner, config })
    }

    pub fn put(&self, storage_key: &str, data: &[u8]) -> Result<(), StorageError> {
        self.inner.put(storage_key, data)
    }

    pub fn delete(&self, storage_key: &str) -> Result<(), StorageError> {
        self.inner.delete(storage_key)
    }

    pub fn public_url(&self, storage_key: &str) -> String {
        self.inner.public_url(storage_key)
    }

    pub fn new_object_key(&self, user_id: i64, extension: Option<&str>) -> String {
        self.inner.new_object_key(user_id, extension)
    }

    pub fn ensure_local_dir(&self) -> Result<(), StorageError> {
        if self.config.backend == "local" {
            std::fs::create_dir_all(&self.config.local_dir)
                .map_err(|e| StorageError::Io(e.to_string()))?;
        }
        Ok(())
    }
}
