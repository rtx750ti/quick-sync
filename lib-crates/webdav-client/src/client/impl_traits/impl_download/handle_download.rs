use futures_util::StreamExt;
use reqwest::Client;
use crate::client::error::WebDavClientError;
use crate::client::impl_traits::impl_download::download_file::download_file;
use crate::client::impl_traits::impl_download::gen_download_task::gen_download_tasks;
use crate::client::impl_traits::impl_download::TSuccessMetas;
use crate::client::impl_traits::impl_download::chunked_download_blacklist::is_chunked_download_blacklisted;
use crate::client::structs::webdav_child_client::WebDavChildClientKey;
use crate::client::traits::download::{DownloadConfig, ThreadMode};
use crate::public_traits::friendly::FriendlyXml;

/// 串行下载
async fn download_single_thread(
    http_client: &Client,
    file_metas: &TSuccessMetas,
    output_path: &str,
    auto_segment_file: bool,
) -> Result<(), WebDavClientError> {
    for file_meta in file_metas {
        if let Ok(friendly_webdav_files_xml) = file_meta.to_friendly() {
            if let Some(resource) = friendly_webdav_files_xml.first() {
                download_file(
                    http_client,
                    resource,
                    output_path,
                    auto_segment_file,
                )
                .await?;
            }
        }
    }
    Ok(())
}

/// 并行下载
async fn download_multi_thread(
    http_client: &Client,
    file_metas: &TSuccessMetas,
    output_path: &str,
    auto_segment_file: bool,
) -> Result<(), WebDavClientError> {
    let mut tasks = gen_download_tasks(
        http_client,
        file_metas,
        output_path,
        auto_segment_file,
    );

    let mut errors = Vec::new();

    while let Some(result) = tasks.next().await {
        if let Err(e) = result {
            errors.push(e);
        }
    }

    if !errors.is_empty() {
        return Err(errors.into_iter().next().unwrap());
    }

    Ok(())
}

/// 自动选择模式
async fn download_auto(
    http_client: &Client,
    file_metas: &TSuccessMetas,
    output_path: &str,
    auto_segment_file: bool,
) -> Result<(), WebDavClientError> {
    if file_metas.len() > 1 {
        download_multi_thread(
            http_client,
            file_metas,
            output_path,
            auto_segment_file,
        )
        .await
    } else {
        download_single_thread(
            http_client,
            file_metas,
            output_path,
            auto_segment_file,
        )
        .await
    }
}

pub async fn handle_download(
    http_client: &Client,
    file_metas: &TSuccessMetas,
    output_path: &str,
    thread_mode: &ThreadMode,
    auto_segment_file: bool,
) -> Result<(), WebDavClientError> {
    match thread_mode {
        ThreadMode::SingleThread => {
            download_single_thread(
                http_client,
                file_metas,
                output_path,
                auto_segment_file,
            )
            .await
        }
        ThreadMode::MultipleThread => {
            download_multi_thread(
                http_client,
                file_metas,
                output_path,
                auto_segment_file,
            )
            .await
        }
        ThreadMode::Auto => {
            download_auto(
                http_client,
                file_metas,
                output_path,
                auto_segment_file,
            )
            .await
        }
    }
}

/// 预处理下载
pub async fn preprocessing_download(
    web_dav_child_client_key: &WebDavChildClientKey,
    download_config: &DownloadConfig,
    http_client: &Client,
    file_metas: &TSuccessMetas,
    output_path: &str,
) -> Result<(), WebDavClientError> {
    let DownloadConfig { thread_mode, auto_segment_file } =
        download_config;

    let base_url = web_dav_child_client_key.get_base_url();

    let auto_segment_file = if is_chunked_download_blacklisted(&base_url) {
        false
    } else {
        *auto_segment_file
    };

    handle_download(
        http_client,
        file_metas,
        output_path,
        thread_mode,
        auto_segment_file,
    )
    .await?;

    Ok(())
}
