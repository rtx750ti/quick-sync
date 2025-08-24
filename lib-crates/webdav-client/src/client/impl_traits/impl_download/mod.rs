mod chunked_download_blacklist;
mod download_file;
mod gen_download_task;
mod handle_download;

use crate::client::WebDavClient;
use crate::client::error::WebDavClientError;
use crate::client::impl_traits::impl_download::handle_download::preprocessing_download;
use crate::client::structs::raw_xml::MultiStatus;
use crate::client::structs::webdav_child_client::WebDavChildClientKey;
use crate::client::traits::download::{Download, DownloadConfig};
use crate::client::traits::folder::{Folder, TFileMetas};
use async_trait::async_trait;

type TFailedMetasError = Vec<WebDavClientError>;

type TSuccessMetas = Vec<MultiStatus>;
fn collect_file_metas_result(
    file_metas_result: TFileMetas,
) -> (TSuccessMetas, TFailedMetasError) {
    let mut failed_metas_error = Vec::new();
    let mut success_metas = Vec::new();

    for file_meta_result in file_metas_result {
        match file_meta_result {
            Ok(file_meta) => {
                success_metas.push(file_meta);
            }
            Err(err) => {
                failed_metas_error.push(err);
            }
        }
    }

    (success_metas, failed_metas_error)
}

#[async_trait]
impl Download for WebDavClient {
    async fn download_files(
        &self,
        web_dav_child_client_key: &WebDavChildClientKey,
        files_path: Vec<String>,
        output_path: &str,
        download_config: Option<DownloadConfig>,
    ) -> Result<String, WebDavClientError> {
        let file_metas_result = self
            .collect_file_metas(web_dav_child_client_key, &files_path)
            .await?;

        let (success_metas, _failed_metas_error) =
            collect_file_metas_result(file_metas_result);

        let download_config = download_config
            .unwrap_or(DownloadConfig::new_default_config());

        let http_client =
            self.try_get_client_entity(web_dav_child_client_key).await?;

        preprocessing_download(
            web_dav_child_client_key,
            &download_config,
            &http_client,
            &success_metas,
            &output_path,
        )
        .await?;

        Ok("".to_string())
    }
}
