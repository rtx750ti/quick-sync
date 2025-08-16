mod impl_display;
mod impl_from;

use env_config::error::EnvConfigError;
use sql_manager::error::SqlManagerError;
use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum ClientError {
    SqlError(SqlManagerError),
    StdError(std::io::Error),
    EnvError(EnvConfigError),
    String(String),
    WebSocketError(tungstenite::Error),
    WebDavClientError(webdav_client::error::WebDavClientError),
}
