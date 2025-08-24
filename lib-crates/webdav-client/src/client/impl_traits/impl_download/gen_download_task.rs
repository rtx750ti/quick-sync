use crate::client::error::WebDavClientError;
use crate::client::impl_traits::impl_download::TSuccessMetas;
use crate::client::impl_traits::impl_download::download_file::download_file;
use crate::public_traits::friendly::FriendlyXml;
use futures_util::future::BoxFuture;
use futures_util::stream::FuturesUnordered;
use reqwest::Client;

type TDownloadTask = BoxFuture<'static, Result<(), WebDavClientError>>;

/// 生成下载任务，但不执行
pub fn gen_download_tasks(
    http_client: &Client,
    file_metas: &TSuccessMetas,
    output_path: &str,
    auto_segment_file: bool,
) -> FuturesUnordered<TDownloadTask> {
    let download_tasks: FuturesUnordered<TDownloadTask> =
        FuturesUnordered::new();

    // 这里就把 output_path 转成 String，move 进去
    let output_path = output_path.to_string();

    for file_meta in file_metas {
        if let Ok(friendly_webdav_files_xml) = file_meta.to_friendly() {
            if let Some(friendly_resource) =
                friendly_webdav_files_xml.first()
            {
                let resource = friendly_resource.clone();
                let client = http_client.clone();
                let output_path = output_path.clone();

                let fut: TDownloadTask = Box::pin(async move {
                    download_file(
                        &client,
                        &resource,
                        &output_path,
                        auto_segment_file,
                    )
                    .await
                });

                download_tasks.push(fut);
            }
        }
    }

    download_tasks
}
