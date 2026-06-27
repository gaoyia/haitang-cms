use std::path::PathBuf;

use super::{AssetStorage, StorageError};

/// 对象存储占位实现（阿里云 OSS / 腾讯云 COS 待接入）
#[derive(Clone, Debug)]
pub struct OssStorage {
    _marker: PathBuf,
}

impl OssStorage {
    pub fn new() -> Self {
        Self {
            _marker: PathBuf::from("oss-stub"),
        }
    }
}

impl AssetStorage for OssStorage {
    fn put(&self, _storage_key: &str, _data: &[u8]) -> Result<(), StorageError> {
        Err(StorageError::Unsupported(
            "对象存储尚未配置，请设置 STORAGE_BACKEND=local 或实现 OSS 适配".to_string(),
        ))
    }

    fn delete(&self, _storage_key: &str) -> Result<(), StorageError> {
        Err(StorageError::Unsupported("对象存储尚未配置".to_string()))
    }

    fn public_url(&self, _storage_key: &str) -> String {
        String::new()
    }

    fn new_object_key(&self, user_id: i64, extension: Option<&str>) -> String {
        super::new_object_key(user_id, extension)
    }
}
