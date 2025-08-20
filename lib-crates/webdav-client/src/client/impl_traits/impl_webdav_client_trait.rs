use crate::client::WebDavClient;
use crate::client::structs::raw_xml::MultiStatus;
use crate::client::error::WebDavClientError;
use quick_xml::de::from_str;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use crate::client::traits::webdav_client_trait::WebDavClientTrait;
use crate::file_explorer::enums::DownloadMode;

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
            .file_explorer
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

    async fn get_file_meta(
        &self,
        file_path: &str,
    ) -> Result<String, WebDavClientError> {
        let url = self.format_url_path(file_path)?;
        println!("路径{}", url);
        // 用 PROPFIND 获取文件属性
        // let url = self.base_url.join(file_path).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/xml"),
        );
        headers.insert("Depth", HeaderValue::from_static("0"));

        let propfind_body = r#"<?xml version="1.0" encoding="utf-8" ?>
<D:propfind xmlns:D="DAV:">
  <D:allprop/>
</D:propfind>"#;

        let res = self
            .file_explorer
            .client
            .request(
                reqwest::Method::from_bytes(b"PROPFIND").unwrap(),
                url,
            )
            .headers(headers)
            .body(propfind_body)
            .send()
            .await
            .unwrap();

        // let a  =  res.text().await.unwrap_or_default();

        println!("{:?}", res);
        // TODO 获取文件meta信息

        Ok("".to_string())
    }

    async fn download_file<'a>(
        &self,
        files_path: &'a Vec<&'a str>,
        output_path: &'a str,
        download_mode: Option<DownloadMode>,
    ) -> String {
        // 判断下载模式如果不存在，则默认自动模式
        let download_mode = download_mode.unwrap_or(DownloadMode::Auto);

        "".to_string()
    }

    async fn put_file(
        &self,
        _file_path: &str,
        _config: Option<String>,
    ) -> String {
        todo!()
    }

    async fn mkdir(&self, _file_path: &str, _dir_name: &str) -> String {
        todo!()
    }

    async fn rm_file(&self, _file_path: &str, _force: bool) -> String {
        todo!()
    }

    async fn rmdir(
        &self,
        _file_path: &str,
        _dir_name: &str,
        _force: bool,
    ) -> String {
        todo!()
    }

    async fn rename(&self, _file_path: &str, _new_name: &str) -> String {
        todo!()
    }

    async fn move_item(&self, _from_path: &str, _to_path: &str) -> String {
        todo!()
    }

    async fn exists(&self, _path: &str) -> bool {
        todo!()
    }

    async fn search(&self, _keyword: &str) -> Vec<String> {
        todo!()
    }
}
