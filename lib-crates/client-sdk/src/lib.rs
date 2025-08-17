use crate::error::ClientError;
use sql_manager::manager::SqlManager;
use std::path::PathBuf;
use webdav_client::traits::client::WebDavClientTrait;
use webdav_client::traits::friendly_trait::{
    FriendlyErrorTrait, FriendlyXmlTrait,
};

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
        // let (ws_stream, _) = connect_async(WEBSOCKET_URL).await?;
        //
        // let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        //
        // tokio::spawn(async move {
        //     while let Some(message) = ws_receiver.next().await {
        //         match message {
        //             Ok(msg) => println!("Received: {}", msg),
        //             Err(e) => eprintln!("Error receiving message: {}", e),
        //         }
        //     }
        // });
        //
        // let msg = "hello world".to_owned();
        //
        // ws_sender.send(Message::Text(msg.into())).await?;

        match webdav_client::client::WebDavClient::new(
            "https://aki.teracloud.jp/dav/",
            "",
            "",
        ) {
            Ok(client) => match client.get_folders().await {
                Ok(data) => {
                    let a = data.to_friendly()?;
                    println!("{:?}", a);
                    let a = data.to_friendly_json()?;
                    println!("{}", a);
                }
                Err(e) => {
                    eprintln!("{}", e.to_friendly_string())
                }
            },
            Err(e) => {
                eprintln!("{}", e.to_friendly_string())
            }
        }

        Ok(())
    }
}
