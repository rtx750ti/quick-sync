use std::net::AddrParseError;

use crate::error::websocket::WebSocketError;

impl From<AddrParseError> for WebSocketError {
    fn from(value: AddrParseError) -> Self {
        WebSocketError::AddrParseError(value)
    }
}

impl From<std::io::Error> for WebSocketError {
    fn from(value: std::io::Error) -> Self {
        WebSocketError::StdIoError(value)
    }
}
