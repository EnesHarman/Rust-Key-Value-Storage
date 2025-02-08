use std::sync::{Arc};
use tokio::sync::Mutex;
use crate::StorageError;
use crate::storage::storage::ActionKV;

pub struct StorageService {
    storage: Arc<Mutex<ActionKV>>
}

impl StorageService {
    pub fn new(storage: Arc<Mutex<ActionKV>>) -> Self {
        Self {
            storage
        }
    }

    pub async fn insert(&self, key: String, value: String) -> Result<String, StorageError> {
        let mut storage = self.storage.lock().await;
        match storage.insert(&key.into_bytes(), &value.into_bytes()) {
            Ok(_) => Ok("The item has been insterted.".to_string()),
            Err(e) => Err(StorageError::StorageFailure(e))
        }
    }

    pub async fn get(&self, key: String) -> Result<String, StorageError> {
        let mut storage = self.storage.lock().await;
        match storage.get(&key.into_bytes()) {
            Ok(Some(data)) => Ok(String::from_utf8(data)?),
            Ok(None) => Err(StorageError::NotFound),
            Err(e) => Err(StorageError::StorageFailure(e)),
        }
    }

    pub async fn delete(&self, key: String) -> Result<String, StorageError> {
        let mut storage = self.storage.lock().await;
        match storage.delete(&key.into_bytes()) {
            Ok(()) => Ok("The data is deleted.".to_string()),
            Err(e) => Err(StorageError::StorageFailure(e))
        }
    }

    pub async fn update(&self, key: String, value: String) -> Result<String, StorageError> {
        let mut storage = self.storage.lock().await;
        match storage.update(&key.into_bytes(), &value.into_bytes()) {
            Err(e) => Err(StorageError::StorageFailure(e)),
            Ok(_) => Ok("The entry has been updated.".to_string())
        }
    }
}