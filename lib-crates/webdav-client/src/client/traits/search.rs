pub trait SearchFile {
    fn search(
        &self,
        keyword: &str,
    ) -> impl Future<Output = Vec<String>> + Send;
}
