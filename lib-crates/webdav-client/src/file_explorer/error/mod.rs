mod impl_display;
mod impl_friendly;
mod impl_from;

pub enum FileExplorerError {
    RequestErr(reqwest::Error),
    StdIoErr(std::io::Error),
    String(String),
    InvalidHeaderValue(String),
    SerdeJsonErr(serde_json::Error),
    SerdeErr(String),
    ParseUrlErr(String),
}
