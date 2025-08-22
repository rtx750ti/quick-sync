pub mod enums;
pub mod error;
pub mod impl_traits;
pub mod structs;
mod tests;
pub mod traits;

use base64::Engine;
use error::WebDavClientError;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, Url};

pub struct WebDavClient {
    pub(crate) base_url: Url,
    pub(crate) client: Client,
}

impl WebDavClient {
    pub fn new(
        base_url: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, WebDavClientError> {
        // 1) 解析并规范化 base_url，确保结尾有 `/`
        let mut base_url = Url::parse(base_url).map_err(|_| {
            #[cfg(feature = "lang-en")]
            {
                WebDavClientError::String("Invalid WebDav url".to_string())
            }

            #[cfg(feature = "lang-zh")]
            {
                WebDavClientError::String("WebDav地址出错".to_string())
            }
        })?;

        if !base_url.path().ends_with('/') {
            let new_path = format!("{}/", base_url.path());
            base_url.set_path(&new_path);
        }

        // 2) 构建带授权头的 reqwest Client
        let client = Self::build_client_with_auth(username, password)?;

        Ok(Self { base_url, client })
    }

    /// 私有：构建带 Basic Auth 的 reqwest Client
    fn build_client_with_auth(
        username: &str,
        password: &str,
    ) -> Result<Client, WebDavClientError> {
        let mut headers = HeaderMap::new();

        let token = base64::engine::general_purpose::STANDARD
            .encode(format!("{username}:{password}"));

        let auth_val = HeaderValue::from_str(&format!("Basic {token}"))
            .map_err(|e| {
                WebDavClientError::InvalidHeaderValue(e.to_string())
            })?;

        headers.insert(AUTHORIZATION, auth_val);

        let client = reqwest::Client::builder()
            .http1_only()
            .default_headers(headers)
            .build()?;

        Ok(client)
    }
}
