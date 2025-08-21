pub mod enums;
pub mod error;
pub mod impl_traits;
pub mod structs;
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

        if path.eq("/") || path.eq("./") {
            return Ok(path);
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

        if path.eq("/") || path.eq("./") {
            return Ok(path);
        }

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

    /// 将用户输入的路径进行基础校验和 URL 拼接，返回完整的访问路径字符串。
    ///
    /// 该方法执行三个步骤：
    /// 1. **check_start**：初步校验路径开头是否合法（空路径、回溯 `..` 等）。
    /// 2. **check_parse_url**：基于 `self.base_url` 调用 [`Url::join`] 生成完整 URL，自动归一化路径（去掉 `./`、多余斜杠等）。
    /// 3. **check_end**：校验路径结尾是否合法（文件不能以 `/` 结尾、末段不能含非法字符）。
    ///
    /// 此方法只做轻量校验，避免明显错误，其余规则交由服务器处理。
    ///
    /// # 参数
    /// * `path` - 相对于 `base_url` 的路径，可以包含相对路径符号（如 `./foo`）
    ///
    /// # 返回
    /// 成功时返回完整 URL 字符串；失败时返回 [`WebDavClientError`]
    ///
    /// # 示例
    /// ```
    /// use reqwest::Url;
    /// use webdav_client::client::error::WebDavClientError;
    /// use webdav_client::client::WebDavClient;
    ///
    /// fn example() -> Result<(), WebDavClientError> {
    /// let client = WebDavClient::new(
    ///     "https://dav.example.com/dav/我的坚果云/",
    ///     "user",
    ///     "password"
    /// )?;
    ///
    /// // 传入相对路径，自动规范化为绝对 URL
    /// let url_str = client.format_url_path("./书签")?;
    /// assert_eq!(
    ///     url_str,
    ///     "https://dav.example.com/dav/%E6%88%91%E7%9A%84%E5%9D%9A%E6%9E%9C%E4%BA%91/%E4%B9%A6%E7%AD%BE"
    /// );
    /// assert_eq!(
    ///     url_str,
    ///     "https://dav.example.com/dav/我的坚果云/书签"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn format_url_path(
        &self,
        path: &str,
    ) -> Result<String, WebDavClientError> {
        self.check_start(path)?; // 先检查地址开头有没有问题
        let path_url_entity = self.check_parse_url(path)?; // 地址开头没问题就检查解析有没有问题
        self.check_end(path)?; // 最后检查解析完的地址有没有问题

        Ok(path_url_entity.to_string())
    }
}
