pub mod enums;
pub mod error;
pub mod impl_traits;
pub mod structs;
pub mod traits;

use crate::file_explorer::FileExplorer;
use base64::Engine;
use error::WebDavClientError;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, Url};

pub struct WebDavClient {
    pub(crate) base_url: Url,
    pub(crate) file_explorer: FileExplorer,
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

        let file_explorer = FileExplorer::new(client, base_url.to_owned());

        Ok(Self { base_url, file_explorer })
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

    fn check_start<'a>(
        &self,
        path: &'a str,
    ) -> Result<&'a str, WebDavClientError> {
        let path = path.trim();

        if path.is_empty() {
            return Err(WebDavClientError::ParseUrlErr(
                "路径为空".to_string(),
            ));
        }

        if path.starts_with("../") {
            return Err(WebDavClientError::ParseUrlErr(
                "禁止返回上一级".to_string(),
            ));
        }

        if path.contains("..") {
            return Err(WebDavClientError::ParseUrlErr(
                "路径不能出现'..'".to_string(),
            ));
        }

        Ok(path)
    }

    fn check_parse_url(
        &self,
        path: &str,
    ) -> Result<Url, WebDavClientError> {
        let url = self.base_url.to_owned();
        let url = url.join(path).map_err(|err| {
            WebDavClientError::ParseUrlErr(err.to_string())
        })?;
        Ok(url)
    }

    fn check_end<'a>(
        &self,
        path: &'a str,
    ) -> Result<&'a str, WebDavClientError> {
        let path = path.trim();

        // 去掉末尾 / 再判断文件类型
        let trimmed_path = path.trim_end_matches('/');
        let last_segment =
            trimmed_path.rsplit('/').next().ok_or_else(|| {
                WebDavClientError::ParseUrlErr("路径格式错误".to_string())
            })?;

        let is_file = last_segment.contains('.');

        // 如果是文件但原路径以 / 结尾，报错
        if is_file && path.ends_with('/') {
            return Err(WebDavClientError::ParseUrlErr(format!(
                "'{}'不能以 '/' 结尾",
                path
            )));
        }

        // 跨平台最大兼容非法字符（文件夹和文件都检查）
        let invalid_chars =
            ['\\', '/', ':', '*', '?', '"', '<', '>', '|', '\0'];
        if last_segment.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(WebDavClientError::ParseUrlErr(format!(
                "'{}'包含非法字符",
                path
            )));
        }

        Ok(path)
    }

    /// 将输入 path 解析为 Url，做规则校验并格式化为相对于 base_url 的相对路径。
    pub fn format_url_path(
        &self,
        path: &str,
    ) -> Result<String, WebDavClientError> {
        self.check_start(path)?;
        let path_url_entity = self.check_parse_url(path)?;
        self.check_end(path)?;

        Ok(path_url_entity.to_string())
    }
}
