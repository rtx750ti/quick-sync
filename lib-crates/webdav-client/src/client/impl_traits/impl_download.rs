use async_trait::async_trait;
use crate::client::WebDavClient;
use crate::client::traits::download::{Download, DownloadConfig};

#[async_trait]
impl Download for WebDavClient {
    async fn download_file(&self, _files_path: Vec<String>, _output_path: String, _download_config: Option<DownloadConfig>) -> String {
        todo!()
    }
}
