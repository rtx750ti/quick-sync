use crate::client::enums::client_enum::Depth;
use crate::client::error::WebDavClientError;
use crate::client::structs::raw_xml::MultiStatus;

pub trait Folder {
    /// 通过 WebDAV `PROPFIND` 请求获取指定路径下的资源列表或属性信息。
    ///
    /// 此方法执行以下步骤：
    /// 1. 将传入的 `path` 与 `base_url` 拼接并格式化为完整 URL（内部调用 [`format_url_path`]）。
    /// 2. 根据 `depth` 设置 `Depth` 请求头，决定返回结果的层级范围。
    /// 3. 发送带有 `<allprop/>` 的 `PROPFIND` 请求，并解析返回的 XML 为 [`MultiStatus`]。
    ///
    /// # 参数
    /// * `path` - 相对于 `base_url` 的路径，可以是目录或文件路径。
    /// * `depth` - [`Depth`] 枚举，表示 WebDAV `Depth` 请求头的值：
    ///   - [`Depth::Zero`]：只获取该路径本身的属性（不包含子项）。
    ///   - [`Depth::One`]：获取该路径及直接子项的属性（常用于列目录）。
    ///   - [`Depth::Infinity`]：递归获取所有子项属性（谨慎使用）。
    ///
    /// # 返回
    /// 成功时返回解析后的 [`MultiStatus`] 对象，包含该路径及子资源的元数据；
    /// — 失败时返回 [`WebDavClientError`]。
    ///
    /// # 示例
    /// ```
    /// use webdav_client::client::enums::client_enum::Depth;
    /// use webdav_client::client::WebDavClient;
    /// use webdav_client::client::error::WebDavClientError;
    /// use webdav_client::client::traits::folder::Folder;
    ///
    /// # async fn run() -> Result<(), WebDavClientError> {
    /// let client = WebDavClient::new(
    ///     "https://dav.example.com/dav/我的坚果云/",
    ///     "user",
    ///     "password"
    /// )?;
    ///
    /// // 获取指定目录下的直接子资源（Depth: 1）
    /// let multi_status = client.get_folders("文档/", Depth::One).await?;
    ///
    /// assert!(!multi_status.responses.is_empty());
    /// # Ok(())
    /// # }
    /// ```
    fn get_folders(
        &self,
        path: &str,
        depth: Depth,
    ) -> impl Future<Output = Result<MultiStatus, WebDavClientError>> + Send;

    /// 获取单个文件或目录的元数据（Depth 固定为 0）。
    ///
    /// 本方法是 [`get_folders`] 的语义化封装，等价于：
    /// ```ignore
    /// self.get_folders(file_path, Depth::Zero)
    /// ```
    ///
    /// 适合在需要精确获取单个资源属性（例如大小、修改时间、ETag 等）时使用。
    ///
    /// # 参数
    /// * `file_path` - 相对于 `base_url` 的文件或目录路径。
    ///
    /// # 返回
    /// 成功时返回解析后的 [`MultiStatus`] 对象，仅包含该资源本身的元数据；
    /// 失败时返回 [`WebDavClientError`]。
    ///
    /// # 示例
    /// ```
    /// use webdav_client::client::WebDavClient;
    /// use webdav_client::client::error::WebDavClientError;
    /// use webdav_client::client::traits::folder::Folder;
    ///
    /// async fn run() -> Result<(), WebDavClientError> {
    /// let client = WebDavClient::new(
    ///     "https://dav.example.com/dav/我的坚果云/",
    ///     "user",
    ///     "password"
    /// )?;
    ///
    /// // 获取单个文件的元数据
    /// let meta = client.get_file_meta("文档/报告.docx").await?;
    ///
    /// assert_eq!(meta.responses.len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn get_file_meta(
        &self,
        file_path: &str,
    ) -> impl Future<Output = Result<MultiStatus, WebDavClientError>> + Send;

    fn exists(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<bool, WebDavClientError>> + Send;
}
