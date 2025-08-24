use crate::client::error::WebDavClientError;
use crate::client::structs::webdav_child_client::WebDavChildClientKey;
use async_trait::async_trait;

pub enum ThreadMode {
    Auto,
    SingleThread,
    MultipleThread,
}

pub struct DownloadConfig {
    /// 线程模式
    pub thread_mode: ThreadMode,
    /// 自动分片
    pub auto_segment_file: bool,
}

impl DownloadConfig {
    pub fn new(thread_mode: ThreadMode, auto_segment_file: bool) -> Self {
        Self { thread_mode, auto_segment_file }
    }

    pub fn new_default_config() -> Self {
        Self { thread_mode: ThreadMode::Auto, auto_segment_file: true }
    }
}

#[async_trait]
pub trait Download {
    async fn download_files(
        &self,
        web_dav_child_client_key: &WebDavChildClientKey,
        files_path: Vec<String>,
        output_path: &str,
        download_config: Option<DownloadConfig>,
    ) -> Result<String, WebDavClientError>;
}
