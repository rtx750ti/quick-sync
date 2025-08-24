use crate::client::TWebDavChildClientValue;
use crate::client::error::WebDavClientError;
use base64::Engine;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use sha2::{Digest, Sha256};
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct WebDavChildClientKey {
    base_url: String, // 私有化是为了强制使用new来构建结构体
    username: String,
}

fn format_url(url: &str) -> Result<Url, WebDavClientError> {
    let mut base_url = Url::parse(url).map_err(|_| {
        WebDavClientError::String("WebDav地址出错".to_string())
    })?;

    if !base_url.path().ends_with('/') {
        let new_path = format!("{}/", base_url.path());
        base_url.set_path(&new_path);
    }

    Ok(base_url)
}

pub(crate) fn build_client_with_auth(
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

    let client =
        Client::builder().http1_only().default_headers(headers).build()?;

    Ok(client)
}

impl WebDavChildClientKey {
    pub fn new(
        base_url: &str,
        username: &str,
    ) -> Result<Self, WebDavClientError> {
        let url = format_url(base_url)?;
        Ok(Self {
            base_url: url.to_string(),
            username: username.to_string(),
        })
    }

    pub fn get_base_url(&self) -> String {
        self.base_url.to_owned()
    }

    pub fn get_username(&self) -> String {
        self.base_url.to_owned()
    }
}

impl Display for WebDavChildClientKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}&{}", self.base_url, self.username)
    }
}

pub type EncryptedPassword = String;
pub type EncryptedUsername = String;

#[derive(Clone, Debug)]

pub struct WebDavChildClientValue {
    base_url: Url,
    pub(crate) client: Client,
    encrypted_username: EncryptedUsername,
    encrypted_password: EncryptedPassword,
}

fn encrypted_account(
    username: &str,
    password: &str,
) -> (EncryptedUsername, EncryptedPassword) {
    let encrypted_username = {
        let mut hasher = Sha256::new();
        hasher.update(username.as_bytes());
        format!("{:x}", hasher.finalize())
    };

    let encrypted_password = {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        format!("{:x}", hasher.finalize())
    };

    (encrypted_username, encrypted_password)
}

impl WebDavChildClientValue {
    pub fn new(
        base_url: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, WebDavClientError> {
        let base_url = format_url(base_url)?;

        let client = build_client_with_auth(username, password)?;

        let (encrypted_username, encrypted_password) =
            encrypted_account(username, password);

        Ok(Self {
            base_url,
            client,
            encrypted_username, // sha-256加密
            encrypted_password,
        })
    }

    pub fn get_base_url(&self) -> Url {
        self.base_url.to_owned()
    }

    #[cfg(feature = "show-test-detail")]
    pub(crate) fn get_encrypted_username(&self) -> String {
        self.encrypted_username.to_owned()
    }

    pub fn into(self) -> TWebDavChildClientValue {
        Arc::new(RwLock::new(self))
    }
}

impl PartialEq for WebDavChildClientValue {
    fn eq(&self, other: &Self) -> bool {
        self.base_url == other.base_url
            && self.encrypted_username.eq(&other.encrypted_username)
            && self.encrypted_password.eq(&other.encrypted_password)
    }
}
