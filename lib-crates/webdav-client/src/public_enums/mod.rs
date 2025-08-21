use crate::client::error::WebDavClientError;
use reqwest::Method;

pub enum WebDavMethod {
    PROPFIND,
}

impl WebDavMethod {
    pub fn to_string(&self) -> String {
        match self {
            WebDavMethod::PROPFIND => "PROPFIND".to_string(),
        }
    }
}

impl TryInto<Method> for WebDavMethod {
    type Error = WebDavClientError;

    fn try_into(self) -> Result<Method, Self::Error> {
        let method =
            reqwest::Method::from_bytes(self.to_string().as_bytes())
                .map_err(|e| WebDavClientError::String(e.to_string()))?;

        match self {
            WebDavMethod::PROPFIND => Ok(method),
        }
    }
}
