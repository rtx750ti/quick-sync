use dotenvy::{from_filename, from_filename_override};
use std::{env, path::Path};
use webdav_client::client::{
    WebDavClient, enums::client_enum::Depth, error::WebDavClientError,
    traits::folder::Folder,
};
use webdav_client::public_traits::friendly::{FriendlyError, FriendlyXml};

#[cfg(test)]
pub const WEBDAV_ENV_PATH_1: &str =
    "C:\\project\\rust\\quick-sync\\.env.jianguoyun";
#[cfg(test)]
pub const WEBDAV_ENV_PATH_2: &str =
    "C:\\project\\rust\\quick-sync\\.env.teracloud";

#[cfg(test)]
#[derive(Debug)]
struct WebDavAccount {
    url: String,
    username: String,
    password: String,
}

#[cfg(test)]
fn load_account(path: &str) -> WebDavAccount {
    from_filename_override(Path::new(path)).expect("无法加载 env 文件");
    WebDavAccount {
        url: env::var("WEBDAV_URL").expect("缺少 WEBDAV_URL"),
        username: env::var("WEBDAV_USERNAME")
            .expect("缺少 WEBDAV_USERNAME"),
        password: env::var("WEBDAV_PASSWORD")
            .expect("缺少 WEBDAV_PASSWORD"),
    }
}

#[tokio::test]
async fn test_get_file_meta() -> Result<(), WebDavClientError> {
    let file_path = "./新建文本文档.txt";

    for env_path in [WEBDAV_ENV_PATH_1, WEBDAV_ENV_PATH_2] {
        println!("地址：{}", env_path);
        let acc = load_account(env_path);
        let client =
            WebDavClient::new(&acc.url, &acc.username, &acc.password)?;
        let meta = client.get_file_meta(file_path).await?;
        println!("账号: {env_path}");
        println!("meta: {:?}", meta.to_friendly());
        println!("meta JSON: {}", meta.to_friendly_json()?);
    }
    Ok(())
}

#[tokio::test]
async fn test_get_folders() -> Result<(), WebDavClientError> {
    let folder_path = "./客户.xlsx";

    for env_path in [WEBDAV_ENV_PATH_1, WEBDAV_ENV_PATH_2] {
        let acc = load_account(env_path);
        let client =
            WebDavClient::new(&acc.url, &acc.username, &acc.password)?;
        match client.get_folders(folder_path, Depth::One).await {
            Ok(data) => {
                println!("账号: {env_path}\n{}", data.to_friendly_json()?)
            }
            Err(e) => eprintln!(
                "账号: {env_path} 错误: {}",
                e.to_friendly_string()
            ),
        }
    }
    Ok(())
}
