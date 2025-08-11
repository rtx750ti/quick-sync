use std::path::PathBuf;

use env_config::static_env::WEBSOCKET_URL;
use futures_util::{SinkExt, StreamExt as _};
use sql_manager::manager::SqlManager;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::error::ClientError;

pub mod error;

pub struct QuickSyncClient {
    pub db_manager: SqlManager,
}

impl QuickSyncClient {
    pub async fn new(db_path: &PathBuf) -> Result<Self, ClientError> {
        let db = SqlManager::new(db_path).await?;
        Ok(QuickSyncClient { db_manager: db })
    }

    pub async fn run(&self) -> Result<(), ClientError> {
        let (ws_stream, _) = connect_async(WEBSOCKET_URL).await?;

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        tokio::spawn(async move {
            while let Some(message) = ws_receiver.next().await {
                match message {
                    Ok(msg) => println!("Received: {}", msg),
                    Err(e) => eprintln!("Error receiving message: {}", e),
                }
            }
        });

        let msg = "hello world".to_owned();

        ws_sender.send(Message::Text(msg.into())).await?;

        Ok(())
    }
}
