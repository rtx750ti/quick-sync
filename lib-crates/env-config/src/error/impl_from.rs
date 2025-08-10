use super::EnvConfigError;

impl From<std::io::Error> for EnvConfigError {
    fn from(value: std::io::Error) -> Self {
        EnvConfigError::StdError(value)
    }
}

impl From<std::env::VarError> for EnvConfigError {
    fn from(value: std::env::VarError) -> Self {
        EnvConfigError::EnvError(value)
    }
}
