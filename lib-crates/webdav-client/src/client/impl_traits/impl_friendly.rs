use crate::client::error::WebDavClientError;
use crate::client::structs::friendly_xml::FriendlyResource;
use crate::client::structs::raw_xml::MultiStatus;
#[cfg(feature = "friendly-xml")]
use crate::public_traits::friendly::FriendlyXml;

#[cfg(feature = "friendly-xml")]
pub type TFriendlyWebDavFilesXml = Vec<FriendlyResource>;

#[cfg(feature = "friendly-xml")]
impl FriendlyXml<FriendlyResource, TFriendlyWebDavFilesXml>
    for MultiStatus
{
    fn to_friendly(
        &self,
    ) -> Result<TFriendlyWebDavFilesXml, WebDavClientError> {
        FriendlyResource::new(self.to_owned())
    }
}
