use core::{
    error::core::CoreError,
    socket::{ServerConfig, WebSocketServer},
};

#[tokio::main]
async fn main() -> Result<(), CoreError> {

    let config = ServerConfig::new("127.0.0.1:13985")?;
    let server = WebSocketServer::new(config)?;

    server.run().await?;

    Ok(())
}
