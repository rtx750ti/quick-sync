use crate::client::enums::client_enum::Depth;
use crate::client::error::WebDavClientError;
use crate::client::impl_traits::impl_folder::get_folders_with_client;
use crate::client::structs::friendly_xml::FriendlyResource;
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use reqwest::Client;
use reqwest::header::RANGE;
use std::cmp::min;
use std::path::Path;
use tokio::fs::{self, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};

const CHUNK_SIZE: u64 = 4 * 1024 * 1024;

/// === 工具函数：列出目录下的子资源 ===
async fn list_directory(
    http_client: &Client,
    dir_url: &str,
) -> Result<Vec<FriendlyResource>, WebDavClientError> {
    // 发 PROPFIND 获取子文件列表
    let resp =
        get_folders_with_client(http_client, dir_url, &Depth::One).await?;

    // 转换成 FriendlyResource 列表
    let resources = FriendlyResource::new(resp)?;
    Ok(resources)
}

pub fn download_file<'a>(
    http_client: &'a Client,
    resource: &'a FriendlyResource,
    output_path: &'a str,
    auto_segment_file: bool,
) -> BoxFuture<'a, Result<(), WebDavClientError>> {
    async move {
        if resource.is_dir {
            let dir_path = format!("{}/{}", output_path, resource.name);
            fs::create_dir_all(&dir_path).await?;

            let children =
                list_directory(http_client, &resource.full_path).await?;
            for child in children {
                if child.full_path == resource.full_path {
                    continue;
                }
                download_file(
                    http_client,
                    &child,
                    &dir_path,
                    auto_segment_file,
                )
                .await?;
            }
            return Ok(());
        }

        let file_url = &resource.full_path;
        let file_name = &resource.name;
        let output_file_path = format!("{}/{}", output_path, file_name);
        let total_size = resource.size.unwrap_or(0);

        if let Some(parent) = Path::new(&output_file_path).parent() {
            fs::create_dir_all(parent).await?;
        }

        if total_size == 0 || !auto_segment_file {
            let resp = http_client.get(file_url).send().await?;
            let bytes = resp.bytes().await?;
            tokio::fs::write(&output_file_path, &bytes).await?;
            return Ok(());
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&output_file_path)
            .await?;

        let mut start: u64 = 0;
        while start < total_size {
            let end = min(start + CHUNK_SIZE - 1, total_size - 1);
            let range_header = format!("bytes={}-{}", start, end);

            let resp = http_client
                .get(file_url)
                .header(RANGE, range_header)
                .send()
                .await?;

            let chunk = resp.bytes().await?;
            file.seek(std::io::SeekFrom::Start(start)).await?;
            file.write_all(&chunk).await?;
            start += CHUNK_SIZE;
        }

        file.flush().await?;
        Ok(())
    }
    .boxed()
}
