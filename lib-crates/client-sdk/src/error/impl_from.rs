use super::ClientError;
use env_config::error::EnvConfigError;
use sql_manager::error::SqlManagerError;
use tokio_tungstenite::tungstenite;
use webdav_client::error::WebDavClientError;

impl From<SqlManagerError> for ClientError {
    fn from(value: SqlManagerError) -> Self {
        ClientError::SqlError(value)
    }
}

impl From<std::io::Error> for ClientError {
    fn from(value: std::io::Error) -> Self {
        ClientError::StdError(value)
    }
}

impl From<EnvConfigError> for ClientError {
    fn from(value: EnvConfigError) -> Self {
        ClientError::EnvError(value)
    }
}

impl From<tungstenite::Error> for ClientError {
    fn from(value: tungstenite::Error) -> Self {
        ClientError::WebSocketError(value)
    }
}

impl From<webdav_client::error::WebDavClientError> for ClientError {
    fn from(value: WebDavClientError) -> Self {
        ClientError::WebDavClientError(value)
    }
}
