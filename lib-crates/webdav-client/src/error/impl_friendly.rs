use crate::error::WebDavClientError;
use crate::language::{LANG, Lang};
use crate::traits::friendly_trait::Friendly;

const MAX_LEN: usize = 15;

#[inline]
fn truncate_msg(msg: &str) -> String {
    if msg.chars().count() > MAX_LEN {
        let mut short = msg.chars().take(MAX_LEN).collect::<String>();
        short.push('…');
        short
    } else {
        msg.to_string()
    }
}

#[cfg(feature = "friendly-error")]
impl Friendly for WebDavClientError {
    fn to_friendly_string(&self) -> String {
        match self {
            WebDavClientError::RequestErr(_) => match LANG {
                Lang::Zh => truncate_msg("网络请求失败"),
                Lang::En => truncate_msg("Network request failed"),
            },
            WebDavClientError::StdIoErr(_) => match LANG {
                Lang::Zh => truncate_msg("文件或 I/O 操作失败"),
                Lang::En => truncate_msg("File or I/O operation failed"),
            },
            WebDavClientError::String(err_msg) => match LANG {
                Lang::Zh => format!("[WebDav错误信息]{}", truncate_msg(err_msg)),
                Lang::En => format!("[WebDavErrInfo]{}", truncate_msg(err_msg)),
            },

            WebDavClientError::InvalidHeaderValue(err_msg) => {
                format!("[WebDavHeaderErr]{}", truncate_msg(err_msg))
            }
        }
    }
}
