use crate::client::webdav_struct::MultiStatus;
use crate::error::WebDavClientError;

pub trait WebDavClientTrait {
    async fn get_folders(&self) -> Result<MultiStatus, WebDavClientError>;
}
