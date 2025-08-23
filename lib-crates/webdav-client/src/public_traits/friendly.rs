use crate::client::error::WebDavClientError;

#[cfg(feature = "friendly-xml")]
pub trait FriendlyXml<T, U>
where
    T: serde::Serialize,
    U: serde::Serialize,
{
    fn to_friendly(&self) -> Result<U, WebDavClientError>;

    fn to_friendly_json(&self) -> Result<String, WebDavClientError> {
        let friendly = self.to_friendly()?;

        let friendly_json = serde_json::to_string_pretty(&friendly)
            .unwrap_or_else(|_| "{}".into());

        Ok(friendly_json.to_string())// 这里再to_string是为了避免\n导致字符串出现问题
    }
}
