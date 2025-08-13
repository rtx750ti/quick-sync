use core::{
    error::core::CoreError,
    socket::{ServerConfig, WebSocketServer},
};

use env_config::static_env::WEBSOCKET_HOST;

use libloading::{Library, Symbol};

#[tokio::main]
async fn main() -> Result<(), CoreError> {
    let is_release =
        std::env::var("PROFILE").unwrap_or_default() == "release";

    let mut path = std::env::current_exe().unwrap();
    path.pop();

    path.push(if is_release {
        "diff.dll"
    } else {
        "diff.dll"
    });

    // 加载动态库（路径根据平台调整）
    unsafe {
        // 加载动态库（路径根据平台调整）
        let lib = Library::new(path) // Windows
            // .new("target/release/libdiff.so") // Linux/macOS
            .expect("Failed to load library");

        // 加载符号（函数）
        let add: Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
            lib.get(b"diff_add").expect("Failed to load symbol");

        let result = add(5, 7);
        println!("5 + 7 = {}", result);
    }

    let config = ServerConfig::new(WEBSOCKET_HOST)?;

    let server = WebSocketServer::new(config)?;

    server.run().await?;

    Ok(())
}
