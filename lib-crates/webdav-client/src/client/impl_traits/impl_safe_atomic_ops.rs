use crate::client::WebDavClient;
use crate::client::error::WebDavClientError;
use crate::client::structs::webdav_child_client::{
    WebDavChildClientKey, WebDavChildClientValue,
};
use crate::client::traits::safe_atomic_ops::SafeAtomicOps;
use async_trait::async_trait;

#[async_trait]
impl SafeAtomicOps for WebDavClient {
    fn add_account(
        &mut self,
        base_url: &str,
        username: &str,
        password: &str,
    ) -> Result<WebDavChildClientKey, WebDavClientError> {
        let webdav_child_client_key = WebDavChildClientKey::new(
            &base_url.to_string(),
            &username.to_string(),
        )?;

        let webdav_child_client_value = WebDavChildClientValue::new(
            &webdav_child_client_key.get_base_url(),
            &username,
            &password,
        )?;

        let _ = &webdav_child_client_value.get_base_url(); // 读取一次避免Strut那里报未使用警告

        #[cfg(feature = "show-test-detail")]
        {
            println!("新增的账号地址：{}", &base_url.to_string());
            println!("新增的key：{:?}", webdav_child_client_key);
            println!(
                "新增的value: {:?}",
                webdav_child_client_value.get_encrypted_username()
            )
        }

        self.clients.insert(
            webdav_child_client_key.to_owned(),
            webdav_child_client_value.into(),
        );

        Ok(webdav_child_client_key)
    }

    async fn remove_account(
        &mut self,
        base_url: &str,
        username: &str,
    ) -> Result<(), WebDavClientError> {
        let key = WebDavChildClientKey::new(
            &base_url.to_string(),
            &username.to_string(),
        )?;

        #[cfg(feature = "show-test-detail")]
        {
            println!("当前的Map: {:?}", self.clients.keys());
            println!("构建的key: {:?}", key);
        }

        let client = self.try_get_client_arc(&key)?;

        if !Self::can_modify_value(client) {
            return Err(WebDavClientError::String(
                "该账号未释放".to_string(),
            ));
        }

        match self.clients.remove(&key) {
            Some(_) => {
                #[cfg(feature = "show-test-detail")]
                {
                    println!("删除后的Map: {:?}", self.clients.keys());
                }
                Ok(())
            }
            None => Err(WebDavClientError::String(
                "[remove_account] 删除失败".to_string(),
            )),
        }
    }
}
