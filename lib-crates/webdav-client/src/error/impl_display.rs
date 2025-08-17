use crate::error::WebDavClientError;
use std::fmt::{Display, Formatter};

impl Display for WebDavClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WebDavClientError::RequestErr(e) => write!(f, "{}", e),
            WebDavClientError::StdIoErr(e) => write!(f, "{}", e),
            WebDavClientError::String(e) => write!(f, "{}", e),
            WebDavClientError::InvalidHeaderValue(e) => write!(f, "{}", e),
            WebDavClientError::SerdeJsonErr(e) => write!(f, "{}", e),
            WebDavClientError::SerdeErr(e) => write!(f, "{}", e),
        }
    }
}
