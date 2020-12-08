use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseChannelConfigError {
    #[error("`{0}` is not a valid duration suffix, valid suffixes are: d, w")]
    InvalidDurationSuffix(char),
    #[error("duration cannot be empty")]
    NoDuration,
    #[error("invalid format")]
    InvalidFormat,
}
