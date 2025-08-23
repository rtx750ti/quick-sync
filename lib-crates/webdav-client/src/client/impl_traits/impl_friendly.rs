use crate::client::structs::friendly_xml::FriendlyResource;
use crate::client::structs::raw_xml::MultiStatus;
use crate::client::error::WebDavClientError;
#[cfg(feature = "friendly-xml")]
use crate::public_traits::friendly::FriendlyXml;

#[cfg(feature = "friendly-xml")]
type TargetType = Vec<FriendlyResource>;

#[cfg(feature = "friendly-xml")]
impl FriendlyXml<FriendlyResource, TargetType> for MultiStatus {
    fn to_friendly(&self) -> Result<TargetType, WebDavClientError> {
        FriendlyResource::new(self.to_owned())
    }
}
