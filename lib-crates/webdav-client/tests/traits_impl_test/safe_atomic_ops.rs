use crate::{
    WEBDAV_ENV_PATH_1, WEBDAV_ENV_PATH_2, assert_test_result, load_account,
};
use webdav_client::client::WebDavClient;
use webdav_client::client::error::WebDavClientError;
use webdav_client::client::traits::safe_atomic_ops::SafeAtomicOps;

#[tokio::test]
async fn test_add_account() -> Result<(), WebDavClientError> {
    println!("======新增账号测试开始======");

    let mut ok_count = 0;
    let mut err_count = 0;

    let test_data =
        vec![(WEBDAV_ENV_PATH_1, true), (WEBDAV_ENV_PATH_2, true)];

    let mut client = WebDavClient::new();

    for (env_path, expected_ok) in &test_data {
        let acc = load_account(env_path);

        let result =
            client.add_account(&acc.url, &acc.username, &acc.password);

        let is_ok = result.is_ok();

        if is_ok {
            #[cfg(feature = "show-test-detail")]
            {
                println!("✅ 新增账号地址: {}", &acc.url);
                println!("✅ 成功新增账号: {}", &acc.username);
            }
            ok_count += 1;
        } else {
            #[cfg(feature = "show-test-detail")]
            {
                println!("✅ 新增账号地址: {}", &acc.url);
                println!(
                    "❌ 新增账号失败: {},错误: {}",
                    &acc.username,
                    result.unwrap_err()
                );
            }
            err_count += 1;
        }

        assert_eq!(
            is_ok, *expected_ok,
            "新增webdav账号测试失败: {:?}",
            acc
        );

        #[cfg(feature = "show-test-detail")]
        {
            println!();
        }
    }

    let expected_ok_count = test_data.iter().filter(|(_, ok)| *ok).count();
    let expected_err_count = test_data.len() - expected_ok_count;

    assert_test_result(
        ok_count,
        err_count,
        expected_ok_count,
        expected_err_count,
        "新增账号",
    );

    println!("======新增账号测试结束======");
    Ok(())
}

#[tokio::test]
async fn test_remove_account() -> Result<(), WebDavClientError> {
    println!("======删除账号测试开始======");

    let mut ok_count = 0;
    let mut err_count = 0;

    let test_data = vec![WEBDAV_ENV_PATH_1, WEBDAV_ENV_PATH_2];

    let mut client = WebDavClient::new();

    for env_path in &test_data {
        let acc = load_account(env_path);

        // 先确保账号存在
        let _ =
            client.add_account(&acc.url, &acc.username, &acc.password)?;

        // 测试删除账号
        let remove_result = client.remove_account(&acc.url, &acc.username);
        let is_ok = remove_result.is_ok();

        if is_ok {
            #[cfg(feature = "show-test-detail")]
            {
                println!("✅ 成功删除账号: {}", &acc.username);
            }
            ok_count += 1;
        } else {
            #[cfg(feature = "show-test-detail")]
            {
                println!(
                    "❌ 删除账号失败: {}, 错误: {}",
                    &acc.username,
                    remove_result.unwrap_err()
                );
            }
            err_count += 1;
        }

        assert!(is_ok, "删除 WebDav 账号测试失败: {:?}", acc);

        #[cfg(feature = "show-test-detail")]
        {
            println!();
        }
    }

    let expected_ok_count = test_data.len();
    let expected_err_count = 0;

    assert_test_result(
        ok_count,
        err_count,
        expected_ok_count,
        expected_err_count,
        "删除账号",
    );

    println!("======删除账号测试结束======");
    Ok(())
}
