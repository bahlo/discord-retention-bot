use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidDurationError {}

impl fmt::Display for InvalidDurationError {
    pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid duration, possible suffixes: d, w")
    }
}

impl Error for InvalidDurationError {
    pub fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
