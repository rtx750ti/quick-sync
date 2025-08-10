use std::path::PathBuf;

use sql_manager::manager::SqlManager;

use crate::error::ClientError;

pub mod error;

pub struct QuickSyncClient {
    pub db_manager: SqlManager,
}

impl QuickSyncClient {
    pub async fn new(db_path: &PathBuf) -> Result<Self, ClientError> {
        let db = SqlManager::new(db_path).await?;
        Ok(QuickSyncClient { db_manager: db })
    }
}
