pub mod impl_display;
pub mod impl_from;

use std::net::AddrParseError;

#[derive(Debug)]
pub enum WebSocketError {
    AddrParseError(AddrParseError),
    StdIoError(std::io::Error),
}
