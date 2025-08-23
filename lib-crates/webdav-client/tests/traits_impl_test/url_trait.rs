use crate::{WEBDAV_ENV_PATH_1, load_account};
use webdav_client::client::WebDavClient;
use webdav_client::client::error::WebDavClientError;
use webdav_client::client::structs::webdav_child_client::WebDavChildClientKey;
use webdav_client::client::traits::safe_atomic_ops::SafeAtomicOps;
use webdav_client::client::traits::url_trait::UrlParse;

#[tokio::test]
async fn url_parse_test() -> Result<(), WebDavClientError> {
    println!("======URL解析测试开始======");

    // (路径, 预期是否成功)
    let test_data = vec![
        ("", true),
        ("/", false),
        ("./", true),
        ("../", false),
        (
            "https://dav.jianguoyun.com/dav/%E7%AE%97%E6%B3%95%E4%B8%8E%E5%88%86%E6%9E%90/",
            true,
        ),
        ("/算法与分析", false),
        ("算法与分析/算法与分析.nol", true),
        ("算法与分析/", true),
        ("算法与分析", true),
        (
            "https://dav.jianguoyun.com/dav/%E7%AE%97%E6%B3%95%E4%B8%8E%E5%88%86%E6%9E%90/%E7%AE%97%E6%B3%95%E4%B8%8E%E5%88%86%E6%9E%90.nol",
            true,
        ),
        ("/dav/算法与分析/算法与分析.nol", true),
        ("/dav/算法与分析/算法与分析/.nol", true),
        ("./dav/算法与分析/算法与分析.nol", true),
        ("/dav/算法与分析/算法与分析.nol/", true),
        ("/dav/算法与分析/算法与分析.nol/&@%>?=.,", true),
        ("/dav/算法与分析.nol", false),
        ("./dav/算法与分析", true),
        ("/dav2/算法与分析", false),
        ("/davxxx/算法与分析", false),
        ("/dav%32/算法与分析", false),
        ("/dav%2F算法与分析", false),
        ("/dav/../dav2/算法与分析", false),
        ("https://dav.jianguoyun.com/dav2/算法与分析", false),
        ("https://dav.jianguoyun.com/davxxx/算法与分析", false),
    ];

    let account = load_account(WEBDAV_ENV_PATH_1);

    let mut client = WebDavClient::new();

    let webdav_child_client_key =
        WebDavChildClientKey::new(&account.url, &account.username)?;

    client.add_account(
        &account.url,
        &account.username,
        &account.password,
    )?;

    let mut ok_count = 0;
    let mut err_count = 0;

    for (path, expected_ok) in &test_data {
        #[cfg(feature = "show-test-detail")]
        {
            println!("请求路径：{}", path);
            println!("基础路径：{}", &account.url);
        }

        let result =
            client.format_url_path(&webdav_child_client_key, path).await;

        let is_ok = result.is_ok();

        if is_ok {
            #[cfg(feature = "show-test-detail")]
            {
                println!("✅ {}", path);
            }

            ok_count += 1;
        } else {
            #[cfg(feature = "show-test-detail")]
            {
                println!("❌ {} -> {}", path, result.unwrap_err());
            }
            err_count += 1;
        }

        #[cfg(feature = "show-test-detail")]
        {
            println!(" ");
        }

        // 单个用例断言
        assert_eq!(is_ok, *expected_ok, "路径 {} 结果不符合预期", path);
    }

    println!("统计结果：正确 {} 个，错误 {} 个", ok_count, err_count);

    let expected_ok_count = test_data.iter().filter(|(_, ok)| *ok).count();
    let expected_err_count = test_data.len() - expected_ok_count;

    if ok_count == expected_ok_count && err_count == expected_err_count {
        println!("测试结果: OK ✅");
    } else {
        panic!("测试异常 ❌：统计数量不匹配");
    }
    println!("======URL解析测试结束======");

    Ok(())
}
