use client_sdk::{error::ClientError, QuickSyncClient};
use env_config::get_db_path;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let db_path = get_db_path().await?;

    print!("{:?}", db_path);

    match QuickSyncClient::new(&db_path).await {
        Ok(client_sdk) => {
            client_sdk.run().await?;
        }
        Err(error) => {
            println!("客户端出错了：{}", error.to_string())
        }
    }

    Ok(())
}

