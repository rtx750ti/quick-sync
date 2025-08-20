pub mod enums;
pub mod error;
pub mod impl_traits;
pub mod structs;
pub mod traits;

use crate::file_explorer::enums::DownloadMode;
use crate::file_explorer::error::FileExplorerError;
use reqwest::{Client, Url};
use std::future;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task;

pub struct FileExplorer {
    pub(crate) client: Client,
    pub(crate) base_url: Url, // 这个是webdav地址
}

impl FileExplorer {
    pub fn new(client: Client, base_url: Url) -> Self {
        Self { client, base_url }
    }

    pub async fn download_file(
        &self,
        path: &str,
        target_path: &str,
        download_mode: DownloadMode,
    ) -> Result<String, FileExplorerError> {
        let url = self
            .base_url
            .join(path)
            .map_err(|e| FileExplorerError::ParseUrlErr(e.to_string()))?;

        let mode = match download_mode {
            DownloadMode::Auto => {
                if path.contains(',') {
                    DownloadMode::MultiThread
                } else {
                    DownloadMode::SingleThread
                }
            }
            m => m,
        };

        match mode {
            DownloadMode::SingleThread => {
                self.download_one(&url, target_path).await?;
            }
            DownloadMode::MultiThread => {}
            DownloadMode::Auto => unreachable!(),
        }

        Ok(format!("下载完成: {}", target_path))
    }

    async fn download_one(
        &self,
        url: &Url,
        target_path: &str,
    ) -> Result<(), FileExplorerError> {
        Ok(())
    }
}
