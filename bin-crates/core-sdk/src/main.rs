use core::{
    error::core::CoreError,
    socket::{ServerConfig, WebSocketServer},
};

use env_config::static_env::WEBSOCKET_HOST;

#[tokio::main]
async fn main() -> Result<(), CoreError> {
    let config = ServerConfig::new(WEBSOCKET_HOST)?;

    let server = WebSocketServer::new(config)?;

    server.run().await?;

    Ok(())
}
