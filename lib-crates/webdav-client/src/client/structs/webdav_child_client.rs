use crate::client::error::WebDavClientError;
use crate::client::{BaseUrl, UserName};
use reqwest::{Client, Url};
use std::fmt::{Display, Formatter};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct WebDavChildClientKey {
    pub(crate) base_url: String,
    pub(crate) username: String,
}

impl WebDavChildClientKey {
    pub fn new(
        base_url: &BaseUrl,
        username: &UserName,
    ) -> Result<Self, WebDavClientError> {
        let mut base_url = Url::parse(base_url).map_err(|_| {
            WebDavClientError::String("WebDav地址出错".to_string())
        })?;

        if !base_url.path().ends_with('/') {
            let new_path = format!("{}/", base_url.path());
            base_url.set_path(&new_path);
        }
        
        Ok(Self {
            base_url: base_url.to_string(),
            username: username.to_string(),
        })
    }
}

impl Display for WebDavChildClientKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}&{}", self.base_url, self.username)
    }
}

#[derive(Clone, Debug)]
pub struct WebDavChildClientValue {
    pub(crate) base_url: Url,
    pub(crate) client: Client,
}
