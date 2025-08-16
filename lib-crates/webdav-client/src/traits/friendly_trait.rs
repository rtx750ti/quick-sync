#[cfg(feature = "friendly-error")]
pub(crate) trait Friendly {
    fn to_friendly_string(&self) -> String;
}
