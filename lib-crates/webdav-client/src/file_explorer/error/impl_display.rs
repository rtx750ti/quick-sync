use crate::file_explorer::error::FileExplorerError;
use std::fmt::{Display, Formatter};

impl Display for FileExplorerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileExplorerError::RequestErr(e) => write!(f, "{}", e),
            FileExplorerError::StdIoErr(e) => write!(f, "{}", e),
            FileExplorerError::String(e) => write!(f, "{}", e),
            FileExplorerError::InvalidHeaderValue(e) => write!(f, "{}", e),
            FileExplorerError::SerdeJsonErr(e) => write!(f, "{}", e),
            FileExplorerError::SerdeErr(e) => write!(f, "{}", e),
            FileExplorerError::ParseUrlErr(e) => write!(f, "{}", e),
        }
    }
}
