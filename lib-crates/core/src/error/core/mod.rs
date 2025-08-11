pub mod impl_display;
pub mod impl_from;

use sql_manager::error::SqlManagerError;

use crate::error::websocket::WebSocketError;

#[derive(Debug)]
pub enum CoreError {
    SqlError(SqlManagerError),
    WebSocketError(WebSocketError),
}
