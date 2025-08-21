pub trait FileControl {
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
}
