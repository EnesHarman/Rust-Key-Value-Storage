use std::io;
use std::path::Path;
use std::sync::{Arc};
use tokio::sync::Mutex;
use crate::storage::storage::ActionKV;

pub struct AppState {
    pub kv_storage: Arc<Mutex<ActionKV>>,
}

impl AppState {
    pub async fn new(path: &Path) -> Result<Self, io::Error> {
        let mut storage = ActionKV::open(path).expect("Error while creating storage!");
        storage.load().expect("Failed to load file!");
        Ok(Self {
            kv_storage: Arc::new(Mutex::new(storage)),
        })
    }
}