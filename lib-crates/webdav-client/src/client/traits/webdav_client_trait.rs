use crate::client::error::WebDavClientError;
use crate::client::structs::raw_xml::MultiStatus;
use crate::file_explorer::enums::DownloadMode;

pub trait WebDavClientTrait {
    /// 列出根目录或指定路径下的文件夹
    fn get_folders(
        &self,
    ) -> impl Future<Output = Result<MultiStatus, WebDavClientError>> + Send;

    /// 获取文件元数据
    fn get_file_meta(
        &self,
        file_path: &str,
    ) -> impl Future<Output = Result<String, WebDavClientError>> + Send;

    /// 下载文件
    fn download_file<'a>(
        &self,
        files_path: &'a Vec<&'a str>,
        output_path: &'a str,
        download_mode: Option<DownloadMode>,
    ) -> impl Future<Output = String> + Send;

    /// 上传文件
    fn put_file(
        &self,
        file_path: &str,
        config: Option<String>,
    ) -> impl Future<Output = String> + Send;

    /// 创建目录
    fn mkdir(
        &self,
        file_path: &str,
        dir_name: &str,
    ) -> impl Future<Output = String> + Send;

    /// 删除文件
    fn rm_file(
        &self,
        file_path: &str,
        force: bool,
    ) -> impl Future<Output = String> + Send;

    /// 删除目录
    fn rmdir(
        &self,
        file_path: &str,
        dir_name: &str,
        force: bool,
    ) -> impl Future<Output = String> + Send;

    /// 重命名（文件或目录）
    fn rename(
        &self,
        file_path: &str,
        new_name: &str,
    ) -> impl Future<Output = String> + Send;

    /// 移动（可跨目录）
    fn move_item(
        &self,
        from_path: &str,
        to_path: &str,
    ) -> impl Future<Output = String> + Send;

    /// 检查文件/目录是否存在
    fn exists(&self, path: &str) -> impl Future<Output = bool> + Send;

    /// 简单搜索（按关键字匹配文件名）
    fn search(
        &self,
        keyword: &str,
    ) -> impl Future<Output = Vec<String>> + Send;
}
