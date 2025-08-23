#[cfg(test)]
pub mod traits_impl_test;
#[cfg(test)]
use dotenvy::from_filename_override;
#[cfg(test)]
use std::env;
#[cfg(test)]
use std::path::Path;

#[cfg(test)]
pub const WEBDAV_ENV_PATH_1: &str =
    "C:\\project\\rust\\quick-sync\\.env.jianguoyun";
#[cfg(test)]
pub const WEBDAV_ENV_PATH_2: &str =
    "C:\\project\\rust\\quick-sync\\.env.teracloud";

#[cfg(test)]
#[derive(Debug)]
pub struct WebDavAccount {
    url: String,
    username: String,
    password: String,
}

#[cfg(test)]
pub fn load_account(path: &str) -> WebDavAccount {
    from_filename_override(Path::new(path)).expect("无法加载 env 文件");
    WebDavAccount {
        url: env::var("WEBDAV_URL").expect("缺少 WEBDAV_URL"),
        username: env::var("WEBDAV_USERNAME")
            .expect("缺少 WEBDAV_USERNAME"),
        password: env::var("WEBDAV_PASSWORD")
            .expect("缺少 WEBDAV_PASSWORD"),
    }
}
