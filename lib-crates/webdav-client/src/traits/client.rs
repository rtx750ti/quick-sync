use crate::client::structs::raw_xml::MultiStatus;
use crate::error::WebDavClientError;

pub trait WebDavClientTrait {
    fn get_folders(
        &self,
    ) -> impl std::future::Future<
        Output = Result<MultiStatus, WebDavClientError>,
    > + Send; // 实现异步
}
