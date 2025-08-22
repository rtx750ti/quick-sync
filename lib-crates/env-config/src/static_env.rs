#[cfg(feature = "runtime-mode")]
pub const ROOT_DIR: &str = "quick-sync";

#[cfg(feature = "runtime-mode")]
pub const APP_DIR: &str = "QuickSync";

#[cfg(feature = "runtime-mode")]
pub const DB_NAME: &str = "quick-sync.db";

#[cfg(feature = "runtime-mode")]
pub const WEBSOCKET_HOST: &str = "127.0.0.1:13985";

#[cfg(feature = "runtime-mode")]
pub const WEBSOCKET_URL: &str = "ws://127.0.0.1:13985/ws";

#[cfg(feature = "build-mode")]
pub const PROJECT_ROOT_PATH: &str = "C:\\project\\rust\\quick-sync";

#[cfg(debug_assertions)]
pub const RUNTIME_ENV: &str = "debug";

#[cfg(not(debug_assertions))]
pub const RUNTIME_ENV: &str = "release";

#[cfg(debug_assertions)]
pub const IS_DEBUG: bool = true;

#[cfg(not(debug_assertions))]
pub const IS_DEBUG: bool = false;
