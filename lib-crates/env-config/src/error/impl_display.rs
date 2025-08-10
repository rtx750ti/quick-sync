use std::fmt::Display;

use crate::error::EnvConfigError;

impl Display for EnvConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvConfigError::StdError(error) => {
                write!(f, "{}", error.to_string())
            }
            EnvConfigError::EnvError(var_error) => {
                write!(f, "{}", var_error.to_string())
            }
            EnvConfigError::String(msg) => write!(f, "{}", msg),
        }
    }
}
