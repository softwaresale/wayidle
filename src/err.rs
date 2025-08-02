use std::io;

use toml::de;

#[derive(thiserror::Error, Debug)]
pub enum WayIdleError {
    #[error("Bad duration: {0}")]
    BadDuration(#[from] parse_duration::parse::Error),
    #[error("Could not find configuration file")]
    ConfigFileMissing,
    #[error("Config file deserialization error: {0}")]
    ConfigFormat(#[from] de::Error),
    #[error("I/O Error: {0}")]
    IOError(io::ErrorKind),
}

impl From<io::Error> for WayIdleError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value.kind())
    }
}

pub type WayIdleResult<T> = Result<T, WayIdleError>;
