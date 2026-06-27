use std::path::PathBuf;

use super::{AssetStorage, StorageError};

/// 本地磁盘存储（写入 static/uploads，经 /static 挂载访问）
#[derive(Clone, Debug)]
pub struct LocalStorage {
    root_dir: PathBuf,
    public_url_prefix: String,
}

impl LocalStorage {
    pub fn new(root_dir: PathBuf, public_url_prefix: String) -> Self {
        Self {
            root_dir,
            public_url_prefix: public_url_prefix.trim_end_matches('/').to_string(),
        }
    }

    fn abs_path(&self, storage_key: &str) -> PathBuf {
        self.root_dir.join(storage_key)
    }
}

impl AssetStorage for LocalStorage {
    fn put(&self, storage_key: &str, data: &[u8]) -> Result<(), StorageError> {
        let path = self.abs_path(storage_key);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| StorageError::Io(e.to_string()))?;
        }
        std::fs::write(&path, data).map_err(|e| StorageError::Io(e.to_string()))?;
        Ok(())
    }

    fn delete(&self, storage_key: &str) -> Result<(), StorageError> {
        let path = self.abs_path(storage_key);
        if path.exists() {
            std::fs::remove_file(&path).map_err(|e| StorageError::Io(e.to_string()))?;
        }
        Ok(())
    }

    fn public_url(&self, storage_key: &str) -> String {
        let key = storage_key.trim_start_matches('/');
        format!("{}/{key}", self.public_url_prefix)
    }

    fn new_object_key(&self, user_id: i64, extension: Option<&str>) -> String {
        super::new_object_key(user_id, extension)
    }
}
