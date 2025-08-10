mod impl_display;
mod impl_from;

#[derive(Debug)]
pub enum EnvConfigError {
    StdError(std::io::Error),
    EnvError(std::env::VarError),
    String(String),
}
