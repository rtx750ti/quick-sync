use crate::client::WebDavClient;
use crate::client::enums::client_enum::Depth;
use crate::client::error::WebDavClientError;
use crate::client::structs::raw_xml::MultiStatus;
use crate::client::structs::webdav_child_client::WebDavChildClientKey;
use crate::client::traits::folder::Folder;
use crate::client::traits::url_trait::UrlParse;
use crate::public_enums::WebDavMethod;
use async_trait::async_trait;
use quick_xml::de::from_str;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use std::sync::Arc;

#[async_trait]
impl Folder for WebDavClient {
    async fn get_folders(
        &self,
        web_dav_child_client_key: &WebDavChildClientKey,
        path: &str,
        depth: Depth,
    ) -> Result<MultiStatus, WebDavClientError> {
        let client = self.try_get_client(web_dav_child_client_key)?;

        let client = Arc::clone(&client);
        // 上读锁
        let guard = client.read().await;
        
        let url = self.format_url_path(web_dav_child_client_key, path).await?;

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
        headers.insert("Depth", HeaderValue::from_static(depth.as_str()));
        headers
            .insert("Accept", HeaderValue::from_static("application/xml"));

        let method = WebDavMethod::PROPFIND.try_into()?;

        // 发送 PROPFIND 到基准目录（已保证有尾部斜杠）
        let res = guard
            .client
            .request(method, url)
            .headers(headers)
            .body(propfind_body)
            .send()
            .await?;

        let status = res.status();

        let xml_text = res.text().await?;

        if !status.is_success() && status.as_u16() != 207 {
            return Err(WebDavClientError::String(format!(
                "状态解析异常 {status}: {xml}",
                status = status,
                xml = xml_text
            )));
        }

        let multi_status: MultiStatus = from_str(&xml_text)
            .map_err(|e| WebDavClientError::SerdeErr(e.to_string()))?;

        Ok(multi_status)
    }

    async fn get_file_meta(
        &self,
        web_dav_child_client_key: &WebDavChildClientKey,
        file_path: &str,
    ) -> Result<MultiStatus, WebDavClientError> {
        self.get_folders(web_dav_child_client_key, file_path, Depth::Zero)
            .await
    }
}
