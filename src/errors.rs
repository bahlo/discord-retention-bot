use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseChannelConfigError {
    #[error("`{0}` is not a valid duration suffix, valid suffixes are: d, w")]
    InvalidDurationSuffix(char),
    #[error("a duration needs a suffix, valid suffixes are: d, w")]
    NoDurationSuffix,
    #[error("invalid format, should be $CHANNEL_NAME:$DURATION,...")]
    InvalidFormat,
}
