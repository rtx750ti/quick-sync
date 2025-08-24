use crate::client::error::WebDavClientError;
use crate::client::structs::webdav_child_client::WebDavChildClientKey;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait SafeAtomicOps {
    fn add_account(
        &mut self,
        base_url: &str,
        username: &str,
        password: &str,
    ) -> Result<WebDavChildClientKey, WebDavClientError>;

    fn can_modify_value<T>(arc_client: &Arc<RwLock<T>>) -> bool {
        let strong = Arc::strong_count(&arc_client);
        println!("Arc strong_count = {}", strong);
        if strong > 2 {
            return false;
        }
        arc_client.try_write().is_ok()
    }

   async fn remove_account(
        &mut self,
        base_url: &str,
        username: &str,
    ) -> Result<(), WebDavClientError>;
}
