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
use tokio::sync::RwLock;

pub type UserName = String;
pub type BaseUrl = String;

pub type TWebDavChildClientValue = Arc<RwLock<WebDavChildClientValue>>;

pub struct WebDavClient {
    pub(crate) clients:
        HashMap<WebDavChildClientKey, TWebDavChildClientValue>,
}

impl WebDavClient {
    pub fn new() -> Self {
        Self { clients: HashMap::new() }
    }

    fn try_get_client(
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
