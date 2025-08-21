use crate::error::ClientError;
use sql_manager::manager::SqlManager;
use std::path::PathBuf;
use webdav_client::client::enums::client_enum::Depth;
use webdav_client::client::traits::folder::Folder;
use webdav_client::public_traits::friendly_trait::{
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
            "https://dav.jianguoyun.com/dav/我的坚果云",
            "",
            "",
        ) {
            Ok(client) => {
                match client.get_folders("./客户.xlsx", Depth::One).await
                {
                    Ok(data) => {
                        // let a = data.to_friendly()?;
                        // println!("{:?}", a);
                        let a = data.to_friendly_json()?;
                        println!("{}", a);
                    }
                    Err(e) => {
                        eprintln!("{}", e.to_friendly_string())
                    }
                }

                let file_meta =  client.get_file_meta("/dav/%e6%88%91%e7%9a%84%e5%9d%9a%e6%9e%9c%e4%ba%91/%e6%96%b0%e5%bb%ba%e6%96%87%e6%9c%ac%e6%96%87%e6%a1%a3.txt").await?;

                println!("文件meta1  {:?}", file_meta.to_friendly());
                println!("文件meta2  {}", file_meta.to_friendly_json()?);
            }
            Err(e) => {
                eprintln!("{}", e.to_friendly_string())
            }
        }

        Ok(())
    }
}
