//! Errors for nanoserve.

use super::ParseRequestError;
use std::{fmt, io::Error as IoError};

/// Possible errors in nanoserve.
#[derive(Debug)]
pub enum NanoserveError {
    /// IO error.
    Io(IoError),
    /// Error parsing request.
    ParseRequest(ParseRequestError),
}

impl From<IoError> for NanoserveError {
    fn from(error: IoError) -> Self {
        Self::Io(error)
    }
}

impl From<ParseRequestError> for NanoserveError {
    fn from(error: ParseRequestError) -> Self {
        Self::ParseRequest(error)
    }
}

impl fmt::Display for NanoserveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::ParseRequest(e) => write!(f, "Parse request error: {e}"),
        }
    }
}
