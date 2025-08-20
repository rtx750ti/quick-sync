use crate::client::error::WebDavClientError;

#[cfg(feature = "friendly-error")]
pub trait FriendlyErrorTrait {
    fn to_friendly_string(&self) -> String;
}

pub trait FriendlyXmlTrait<T, U>
where
    T: serde::Serialize,
    U: serde::Serialize,
{
    fn to_friendly(&self) -> Result<U, WebDavClientError>;

    fn to_friendly_json(&self) -> Result<String, WebDavClientError> {
        let friendly = self.to_friendly()?;

        let friendly_json = serde_json::to_string_pretty(&friendly)
            .unwrap_or_else(|_| "{}".into());

        Ok(friendly_json)
    }
}
