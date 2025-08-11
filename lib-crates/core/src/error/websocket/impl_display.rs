use std::fmt::Display;

use crate::error::websocket::WebSocketError;

impl Display for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebSocketError::AddrParseError(addr_parse_error) => {
                write!(f, "{}", addr_parse_error)
            }
            WebSocketError::StdIoError(error) => write!(f, "{}", error),
        }
    }
}
