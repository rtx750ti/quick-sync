pub mod enums;
pub mod error;
pub mod impl_traits;
pub mod structs;
pub mod traits;

use crate::client::structs::webdav_child_client::{
    WebDavChildClientKey, WebDavChildClientValue,
};
use error::WebDavClientError;
use std::collections::HashMap;
use std::sync::Arc;
use reqwest::Client;
use tokio::sync::RwLock;

pub type TWebDavChildClientValue = Arc<RwLock<WebDavChildClientValue>>;

pub struct WebDavClient {
    pub(crate) clients:
        HashMap<WebDavChildClientKey, TWebDavChildClientValue>,
}

impl WebDavClient {
    pub fn new() -> Self {
        Self { clients: HashMap::new() }
    }

    /// 获取http客户端实体
    /// - 但是这个实体是基于Arc智能指针构建的，它本身的Clone行为会变成Arc::clone
    async fn try_get_client_entity(
        &self,
        web_dav_child_client_key: &WebDavChildClientKey,
    ) -> Result<Client, WebDavClientError> {
        let client = match self.clients.get(web_dav_child_client_key) {
            Some(c) => Ok(c),
            None => Err(WebDavClientError::NotFindClient(
                web_dav_child_client_key.to_string(),
            )),
        }?;

        let client = Arc::clone(client);
        let guard = client.read().await;
        let cloned_guard = guard.client.clone();
        drop(guard);
        drop(client);

        Ok(cloned_guard)
    }

    fn try_get_client_arc(
        &self,
        web_dav_child_client_key: &WebDavChildClientKey,
    ) -> Result<&TWebDavChildClientValue, WebDavClientError> {
        return match self.clients.get(web_dav_child_client_key) {
            Some(c) => Ok(c),
            None => Err(WebDavClientError::NotFindClient(
                web_dav_child_client_key.to_string(),
            )),
        };
    }
}
