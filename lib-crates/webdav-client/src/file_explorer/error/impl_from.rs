
use std::io::Error;
use crate::file_explorer::error::FileExplorerError;

impl From<std::io::Error> for FileExplorerError {
    fn from(value: Error) -> Self {
        Self::StdIoErr(value)
    }
}

impl From<reqwest::Error> for FileExplorerError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestErr(value)
    }
}

impl From<String> for FileExplorerError {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
