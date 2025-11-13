//! Errors for nanoserve.

use std::{fmt, io::Error as IoError};
use super::ParseRequestError;

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
            NanoserveError::Io(e) => write!(f, "IO error: {e}"),
            NanoserveError::ParseRequest(e) => write!(f, "Parse request error: {e}"),
        }
    }
}
