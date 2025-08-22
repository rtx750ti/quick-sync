use dotenvy::from_filename_override;
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
        username: env::var("WEBDAV_USERNAME").expect("缺少 WEBDAV_USERNAME"),
        password: env::var("WEBDAV_PASSWORD").expect("缺少 WEBDAV_PASSWORD"),
    }
}

#[tokio::test]
async fn path_test_cases() -> Result<(), WebDavClientError> {
    let test_path = vec![
        "./",
        "../",
        "/dav/我的坚果云/Java.emmx",
        "./dav/我的坚果云/Java.emmx",
        "/dav/我的坚果云/Java.emmx/",
        "/dav/我的坚果云/Java.emmx/&@%>?=.,",
        "/dav/Java.emmx",
        "./dav/我的坚果云",
    ];

    let account = load_account(WEBDAV_ENV_PATH_1);
    let client = WebDavClient::new(&account.url, &account.username, &account.password)?;

    println!("=== 📂 Path Test Cases ===");
    for path in test_path {
        match client.get_folders(path, Depth::Zero).await {
            Ok(_) => println!("✅ {}", path),
            Err(e) => println!("❌ {} -> {}", path, e.to_friendly_string()),
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_get_file_meta() -> Result<(), WebDavClientError> {
    let file_path1 = "./算法与分析.nol";
    let file_path2 = "./test.txt";

    println!("\n=== 📄 File Meta Test ===");
    for (env_path, file_path) in [
        (WEBDAV_ENV_PATH_1, file_path1),
        (WEBDAV_ENV_PATH_2, file_path2),
    ] {
        let acc = load_account(env_path);
        let client = WebDavClient::new(&acc.url, &acc.username, &acc.password)?;
        match client.get_file_meta(file_path).await {
            Ok(meta) => {
                println!("✅ 账号: {env_path} -> {}", file_path);
                println!("   meta: {:?}", meta.to_friendly());
                println!("   meta JSON: {}", meta.to_friendly_json()?);
            }
            Err(e) => {
                println!("❌ 账号: {env_path} -> {} 错误: {}", file_path, e.to_friendly_string());
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_get_folders() -> Result<(), WebDavClientError> {
    let folder_path1 = "./";
    let folder_path2 = "./";

    println!("\n=== 📂 Folder List Test ===");
    for (env_path, folder_path) in [
        (WEBDAV_ENV_PATH_1, folder_path1),
        (WEBDAV_ENV_PATH_2, folder_path2),
    ] {
        let acc = load_account(env_path);
        let client = WebDavClient::new(&acc.url, &acc.username, &acc.password)?;
        match client.get_folders(folder_path, Depth::One).await {
            Ok(data) => {
                println!("✅ 账号: {env_path} -> {}", folder_path);
                println!("{}", data.to_friendly_json()?);
            }
            Err(e) => {
                println!("❌ 账号: {env_path} -> {} 错误: {}", folder_path, e.to_friendly_string());
            }
        }
    }
    Ok(())
}
