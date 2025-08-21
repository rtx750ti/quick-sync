pub trait Download {
    fn download_file<'a>(
        &self,
        files_path: &'a Vec<&'a str>,
        output_path: &'a str,
        download_mode: Option<String>,
    ) -> impl Future<Output = String> + Send;
}
