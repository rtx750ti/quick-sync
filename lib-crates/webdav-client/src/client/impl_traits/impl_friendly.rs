use crate::client::structs::friendly_xml::FriendlyResource;
use crate::client::structs::raw_xml::MultiStatus;
use crate::client::error::WebDavClientError;
use crate::public_traits::friendly_trait::FriendlyXmlTrait;

type TargetType = Vec<FriendlyResource>;

impl FriendlyXmlTrait<FriendlyResource, TargetType> for MultiStatus {
    fn to_friendly(&self) -> Result<TargetType, WebDavClientError> {
        // 调用你刚才写的 FriendlyResource::new
        FriendlyResource::new(self.to_owned())
    }
}
