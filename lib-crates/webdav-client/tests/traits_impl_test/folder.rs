use crate::{WEBDAV_ENV_PATH_1, WEBDAV_ENV_PATH_2, load_account};
use webdav_client::client::WebDavClient;
use webdav_client::client::enums::client_enum::Depth;
use webdav_client::client::error::WebDavClientError;
use webdav_client::client::structs::raw_xml::MultiStatus;
use webdav_client::client::structs::webdav_child_client::WebDavChildClientKey;
use webdav_client::client::traits::folder::Folder;
use webdav_client::client::traits::safe_atomic_ops::SafeAtomicOps;

#[tokio::test]
async fn test_get_file_meta() -> Result<(), WebDavClientError> {
    println!("======获取文件Meta测试开始======");

    let test_data = vec![
        (WEBDAV_ENV_PATH_1, "./算法与分析.nol", true),
        (WEBDAV_ENV_PATH_2, "./test.txt", true),
        (WEBDAV_ENV_PATH_1, "./不存在的文件.txt", false),
    ];

    let mut ok_count = 0;
    let mut err_count = 0;

    let mut client = WebDavClient::new();

    // println!("\n=== 📄 File Meta Test ===");
    for (env_path, file_path, expected_ok) in &test_data {
        let acc = load_account(env_path);

        let webdav_child_client_key =
            WebDavChildClientKey::new(&acc.url, &acc.username)?;

        client.add_account(&acc.url, &acc.password, &acc.password)?;

        let result = client
            .get_file_meta(&webdav_child_client_key, file_path)
            .await;
        let is_ok = result.is_ok();

        if is_ok {
            let _meta = result.unwrap();
            // println!("✅ 账号: {env_path} -> {}", file_path);
            // 只打印一条 meta 信息
            // println!("   meta: {:?}", meta.to_friendly());
            // println!("   meta JSON: {}", meta.to_friendly_json()?);
            ok_count += 1;
        } else {
            // println!(
            //     "❌ 账号: {env_path} -> {} 错误: {}",
            //     file_path,
            //     result.unwrap_err()
            // );
            err_count += 1;
        }

        assert_eq!(
            is_ok, *expected_ok,
            "文件Meta测试失败: {} -> {}",
            env_path, file_path
        );
    }

    let expected_ok_count =
        test_data.iter().filter(|(_, _, ok)| *ok).count();
    let expected_err_count = test_data.len() - expected_ok_count;

    println!("统计结果：正确 {} 个，错误 {} 个", ok_count, err_count);

    if ok_count == expected_ok_count && err_count == expected_err_count {
        println!("测试结果: OK ✅");
    } else {
        panic!("测试异常 ❌：统计数量不匹配");
    }

    println!("======获取文件Meta测试结束======");
    Ok(())
}

#[tokio::test]
async fn test_get_folders() -> Result<(), WebDavClientError> {
    println!("======读取文件夹测试开始======");

    let test_data = vec![
        (WEBDAV_ENV_PATH_1, "./", true),
        (WEBDAV_ENV_PATH_2, "./", true),
        (WEBDAV_ENV_PATH_1, "./不存在的目录", false),
    ];

    let mut ok_count = 0;
    let mut err_count = 0;

    let mut client = WebDavClient::new();

    // println!("\n=== 📂 Folder List Test ===");
    for (env_path, folder_path, expected_ok) in &test_data {
        let acc = load_account(env_path);

        let webdav_child_client_key =
            WebDavChildClientKey::new(&acc.url, &acc.username)?;

        client.add_account(&acc.url, &acc.password, &acc.password)?;

        let result = client
            .get_folders(&webdav_child_client_key, folder_path, Depth::One)
            .await;

        let is_ok = result.is_ok();

        if is_ok {
            let data = result.unwrap();
            // println!("✅ 账号: {env_path} -> {}", folder_path);
            // 只取第一条文件夹信息
            if let Some(first) = data.responses.into_iter().next() {
                let _single = MultiStatus { responses: vec![first] };
                // println!("{}", single.to_friendly_json()?);
            }

            ok_count += 1;
        } else {
            // println!(
            //     "❌ 账号: {env_path} -> {} 错误: {}",
            //     folder_path,
            //     result.unwrap_err()
            // );
            err_count += 1;
        }

        assert_eq!(
            is_ok, *expected_ok,
            "文件夹读取测试失败: {} -> {}",
            env_path, folder_path
        );
    }

    let expected_ok_count =
        test_data.iter().filter(|(_, _, ok)| *ok).count();
    let expected_err_count = test_data.len() - expected_ok_count;

    println!("统计结果：正确 {} 个，错误 {} 个", ok_count, err_count);

    if ok_count == expected_ok_count && err_count == expected_err_count {
        println!("测试结果: OK ✅");
    } else {
        panic!("测试异常 ❌：统计数量不匹配");
    }

    println!("======读取文件夹测试结束======");
    Ok(())
}
