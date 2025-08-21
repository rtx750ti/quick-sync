pub trait Upload {
    fn put_file(
        &self,
        file_path: &str,
        config: Option<String>,
    ) -> impl Future<Output = String> + Send;
}
