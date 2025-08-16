use std::fmt::Display;

use crate::error::ClientError;

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::SqlError(sql_manager_error) => {
                write!(f, "{}", sql_manager_error.to_string())
            }
            ClientError::StdError(error) => {
                write!(f, "{}", error.to_string())
            }
            ClientError::EnvError(var_error) => {
                write!(f, "{}", var_error.to_string())
            }
            ClientError::String(msg) => write!(f, "{}", msg),
            ClientError::WebSocketError(ws_error) => {
                write!(f, "{}", ws_error.to_string())
            }
            ClientError::WebDavClientError(webdav_error) => {
                write!(f, "{}", webdav_error.to_string())
            }
        }
    }
}
