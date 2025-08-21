use crate::client::WebDavClient;
use crate::client::enums::client_enum::Depth;
use crate::client::error::WebDavClientError;
use crate::client::structs::raw_xml::MultiStatus;
use crate::client::traits::folder::Folder;
use crate::public_enums::WebDavMethod;
use quick_xml::de::from_str;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

impl Folder for WebDavClient {
    async fn get_folders(
        &self,
        path: &str,
        depth: Depth,
    ) -> Result<MultiStatus, WebDavClientError> {
        let url = self.format_url_path(path)?;

        println!("最后请求的地址 {}", url);

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
        let res = self
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
        file_path: &str,
    ) -> Result<MultiStatus, WebDavClientError> {
        self.get_folders(file_path, Depth::Zero).await
    }

    async fn exists(&self, _path: &str) -> Result<bool, WebDavClientError> {
        Err(WebDavClientError::String("todo".to_string()))
    }
}
