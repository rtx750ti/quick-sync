use crate::client::WebDavClient;
use crate::client::error::WebDavClientError;
use crate::client::structs::webdav_child_client::{
    WebDavChildClientKey, WebDavChildClientValue,
};
use crate::client::traits::safe_atomic_ops::SafeAtomicOps;
use base64::Engine;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use std::sync::Arc;
use tokio::sync::RwLock;

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

impl SafeAtomicOps for WebDavClient {
    fn add_account(
        &mut self,
        base_url: &str,
        username: &str,
        password: &str,
    ) -> Result<(), WebDavClientError> {
        let mut base_url = Url::parse(base_url).map_err(|_| {
            WebDavClientError::String("WebDav地址出错".to_string())
        })?;

        if !base_url.path().ends_with('/') {
            let new_path = format!("{}/", base_url.path());
            base_url.set_path(&new_path);
        }

        let client = build_client_with_auth(username, password)?;

        self.clients.insert(
            WebDavChildClientKey::new(
                &base_url.to_string(),
                &username.to_string(),
            )?,
            Arc::new(RwLock::new(WebDavChildClientValue {
                base_url,
                client,
            })),
        );

        Ok(())
    }

    fn remove_account(&mut self, base_url: &str, username: &str) -> bool {
        let key = WebDavChildClientKey {
            base_url: base_url.to_string(),
            username: username.to_string(),
        };

        if let Some(client_arc) = self.clients.get(&key) {
            if !Self::can_modify_value(client_arc) {
                return false;
            }
            self.clients.remove(&key);
            return true;
        }
        false
    }

    fn replace_account(
        &mut self,
        base_url: &str,
        username: &str,
        password: &str,
    ) -> Result<bool, WebDavClientError> {
        let key = WebDavChildClientKey {
            base_url: base_url.to_string(),
            username: username.to_string(),
        };

        if let Some(old_arc) = self.clients.get(&key) {
            // 检查是否可以安全替换
            if !Self::can_modify_value(old_arc) {
                return Ok(false);
            }
        }

        // 构建新客户端
        let base_url_parsed = Url::parse(base_url).map_err(|_| {
            WebDavClientError::String("WebDav地址出错".to_string())
        })?;
        let client = build_client_with_auth(username, password)?;

        // 原子替换
        self.clients.insert(
            key,
            Arc::new(RwLock::new(WebDavChildClientValue {
                base_url: base_url_parsed,
                client,
            })),
        );

        Ok(true)
    }
}
