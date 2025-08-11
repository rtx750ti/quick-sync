use sql_manager::error::SqlManagerError;

use crate::error::websocket::WebSocketError;

use super::CoreError;

impl From<SqlManagerError> for CoreError {
    fn from(value: SqlManagerError) -> Self {
        CoreError::SqlError(value)
    }
}

impl From<WebSocketError> for CoreError {
    fn from(value: WebSocketError) -> Self {
        CoreError::WebSocketError(value)
    }
}
