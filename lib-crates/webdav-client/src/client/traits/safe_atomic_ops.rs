use std::sync::Arc;
use tokio::sync::RwLock;
use crate::client::error::WebDavClientError;

pub trait SafeAtomicOps {
    fn add_account(
        &mut self,
        base_url: &str,
        username: &str,
        password: &str,
    ) -> Result<(), WebDavClientError>;

    fn can_modify_value<T>(arc_val: &Arc<RwLock<T>>) -> bool {
        if Arc::strong_count(arc_val) > 1 {
            return false;
        }
        arc_val.try_write().is_ok()
    }

    fn remove_account(&mut self, base_url: &str, username: &str) -> bool;

    fn replace_account(
        &mut self,
        base_url: &str,
        username: &str,
        password: &str,
    ) -> Result<bool, WebDavClientError>;
}
