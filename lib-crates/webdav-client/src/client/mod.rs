pub mod webdav_struct;

use crate::client::webdav_struct::MultiStatus;
use crate::error::WebDavClientError;
use crate::traits::client::WebDavClientTrait;
use base64::Engine;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::{Client, Url};
use quick_xml::de::from_str;

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
        let mut base_url = Url::parse(base_url).map_err(|e| {
            WebDavClientError::String(format!("Invalid base_url: {e}"))
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

impl WebDavClientTrait for WebDavClient {
    async fn get_folders(&self) -> Result<MultiStatus, WebDavClientError> {
        // WebDAV PROPFIND 请求体
        let propfind_body = r#"<?xml version="1.0" encoding="utf-8" ?>
<D:propfind xmlns:D="DAV:">
  <D:allprop/>
</D:propfind>"#;

        // 组装请求头
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/xml"),
        );
        headers.insert("Depth", HeaderValue::from_static("1"));
        headers
            .insert("Accept", HeaderValue::from_static("application/xml"));

        // 发送 PROPFIND 到基准目录（已保证有尾部斜杠）
        let res = self
            .client
            .request(
                reqwest::Method::from_bytes(b"PROPFIND").unwrap(),
                self.base_url.clone(),
            )
            .headers(headers)
            .body(propfind_body)
            .send()
            .await
            .map_err(|e| WebDavClientError::String(e.to_string()))?;

        let status = res.status();
        let xml_text = res
            .text()
            .await
            .map_err(|e| WebDavClientError::String(e.to_string()))?;

        if !status.is_success() && status.as_u16() != 207 {
            // WebDAV 成功常见为 207 Multi-Status
            return Err(WebDavClientError::String(format!(
                "Unexpected status {status}: {xml}",
                status = status,
                xml = xml_text
            )));
        }

        let multi_status: MultiStatus = from_str(&xml_text)
            .map_err(|e| WebDavClientError::String(e.to_string()))?;

        Ok(multi_status)
    }
}
