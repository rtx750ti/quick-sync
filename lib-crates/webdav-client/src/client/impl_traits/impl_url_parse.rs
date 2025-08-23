use crate::client::WebDavClient;
use crate::client::error::WebDavClientError;
use crate::client::structs::webdav_child_client::WebDavChildClientKey;
use crate::client::traits::url_trait::UrlParse;
use async_trait::async_trait;
use reqwest::Url;
use std::str::FromStr;
use std::sync::Arc;

#[async_trait]
impl UrlParse for WebDavClient {
    async fn format_url_path(
        &self,
        web_dav_child_client_key: &WebDavChildClientKey,
        path: &str,
    ) -> Result<String, WebDavClientError> {
        #[cfg(feature = "show-test-detail")]
        {
            println!("--- format_url_path 调试信息 ---");
            println!("输入 path: {}", path);
            println!(
                "WebDavChildClientKey.base_url: {}",
                web_dav_child_client_key.base_url
            );
            println!(
                "WebDavChildClientKey.username: {}",
                web_dav_child_client_key.username
            );
        }

        let base_url = Url::from_str(&web_dav_child_client_key.base_url)
            .expect("base_url 在 new 时已验证为合法 URL"); // 这个panic永远不会触发，因为在web_dav_child_client_key构建时就已经避免了这个问题

        let joined_url = base_url
            .join(path)
            .map_err(|e| WebDavClientError::ParseUrlErr(e.to_string()))?;

        #[cfg(feature = "show-test-detail")]
        {
            println!("拼接后的 joined_url: {}", joined_url);
        }

        let err = Err(WebDavClientError::ParseUrlErr(
            "路径越界，禁止访问上级目录".to_string(),
        ));

        if !joined_url.as_str().starts_with(base_url.as_str()) {
            #[cfg(feature = "show-test-detail")]
            {
                println!("❌ 检查失败：joined_url 不以 base_url 开头");
            }
            return err;
        }

        if joined_url.scheme() != base_url.scheme()
            || joined_url.host_str() != base_url.host_str()
            || !joined_url.path().starts_with(base_url.path())
        {
            #[cfg(feature = "show-test-detail")]
            {
                println!("❌ 检查失败：scheme/host/path 不匹配");
                println!("base_url.scheme(): {}", base_url.scheme());
                println!("joined_url.scheme(): {}", joined_url.scheme());
                println!("base_url.host_str(): {:?}", base_url.host_str());
                println!(
                    "joined_url.host_str(): {:?}",
                    joined_url.host_str()
                );
                println!("base_url.path(): {}", base_url.path());
                println!("joined_url.path(): {}", joined_url.path());
            }
            return err;
        }

        #[cfg(feature = "show-test-detail")]
        {
            println!("✅ 检查通过，返回 URL: {}", joined_url);
            println!("------------------------------");
        }

        Ok(joined_url.to_string())
    }
}
