use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidDurationError {}

impl fmt::Display for InvalidDurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid duration, possible suffixes: d, w")
    }
}

impl Error for InvalidDurationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct InvalidChannelConfigError {}

impl fmt::Display for InvalidChannelConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid channel configuration, format: $name:$duration,..."
        )
    }
}

impl Error for InvalidChannelConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
