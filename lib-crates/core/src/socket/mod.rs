use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    routing::get,
    Router,
};
use tokio::net::TcpListener;

use crate::error::websocket::WebSocketError;

pub struct ServerConfig {
    pub addr: SocketAddr,
}

impl ServerConfig {
    pub fn new(addr: &str) -> Result<Self, WebSocketError> {
        let addr = addr.parse()?;
        Ok(ServerConfig { addr })
    }
}

pub struct WebSocketServer {
    pub config: ServerConfig,
}

impl WebSocketServer {
    pub fn new(config: ServerConfig) -> Result<Self, WebSocketError> {
        Ok(WebSocketServer { config })
    }

    pub async fn run(&self) -> Result<(), WebSocketError> {
        let app = Router::new().route(
            "/ws",
            get(|ws: WebSocketUpgrade| async move {
                ws.on_upgrade(Self::handle_socket)
            }),
        );

        let listener = TcpListener::bind(&self.config.addr).await?;

        axum::serve(listener, app).await?;

        Ok(())
    }

    async fn handle_socket(mut socket: WebSocket) {
        while let Some(Ok(msg)) = socket.recv().await {
            if let Message::Text(text) = msg {
                let _ = socket
                    .send(Message::Text(format!("Echo: {}", text).into()))
                    .await;
            }
        }
    }
}
