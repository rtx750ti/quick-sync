use crate::error::WebDavClientError;
use std::fmt::{Display, Formatter};

impl Display for WebDavClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WebDavClientError::RequestErr(e) => {
                write!(f, "{}", e.to_string())
            }
            WebDavClientError::StdIoErr(e) => {
                write!(f, "{}", e.to_string())
            }
            WebDavClientError::String(e) => {
                write!(f, "{}", e)
            }
            WebDavClientError::InvalidHeaderValue(e) => {
                write!(f, "{}", e)
            }
        }
    }
}
