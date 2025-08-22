use crate::client::error::WebDavClientError;
use percent_encoding::percent_decode_str;
use reqwest::Url;
use std::path::{Path, PathBuf};

pub trait UrlParse {
    /// 检查路径开头是否合法
    fn check_start(&self, path: &str)
    -> Result<String, WebDavClientError>;

    /// 基于 base_url 拼接 URL
    fn try_parse_url(&self, path: &str) -> Result<Url, WebDavClientError>;

    /// 检查路径结尾是否合法
    fn check_end(&self, path: &str) -> Result<String, WebDavClientError>;

    /// 规范化路径（去掉 `.`、处理 `..` 等）
    fn normalize_path(&self, path: &Path) -> PathBuf;

    /// 判断 target 是否是 base 的子路径
    fn is_subpath(&self, base: &Path, target: &Path) -> bool;

    /// URL 解码
    fn decode_url_path(&self, p: &str) -> String;

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
    fn format_url_path(
        &self,
        path: &str,
    ) -> Result<String, WebDavClientError>;
}
