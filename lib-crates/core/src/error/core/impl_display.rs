use std::fmt::Display;

use crate::error::core::CoreError;

impl Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::SqlError(sql_manager_error) => {
                write!(f, "{}", sql_manager_error.to_string())
            }
            CoreError::WebSocketError(ws_error) => {
                write!(f, "{}", ws_error.to_string())
            }
        }
    }
}
