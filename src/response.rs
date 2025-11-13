//! Response module for Nanoserve HTTP server.

use super::Request;
use compio::io::AsyncWriteExt;
use std::io::Result as IoResult;

/// An HTTP response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    /// The response code.
    pub code: ResponseCode,
}

/// Response codes used by Nanoserve.
#[repr(u16)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseCode {
    /// 200 OK
    #[default]
    Ok = 200,
    /// 206 Partial Content
    PartialContent = 206,
    /// 404 Not Found
    NotFound = 404,
    /// 500 Internal Server Error
    InternalServerError = 500,
}

impl Response {
    /// Handles a [`Request`].
    #[must_use]
    pub const fn handle(_request: &Request<'_>) -> Self {
        Self {
            code: ResponseCode::Ok,
        }
    }

    /// Write this [`Response`] to the given destination.
    ///
    /// # Errors
    ///
    /// Returns an [`IoError`](std::io::Error) if writing fails.
    pub async fn write_to<D: AsyncWriteExt>(&self, dest: &mut D) -> IoResult<()> {
        // Start line and headers (empty for now)
        dest.write_all("HTTP/1.1 ").await.0?;
        dest.write_all(self.code.description()).await.0?;
        dest.write_all("\r\n\r\n").await.0?;

        // Dummy body
        dest.write_all("Hello from Nanoserve!").await.0?;

        Ok(())
    }
}

impl ResponseCode {
    /// Get description of the response code.
    pub const fn description(self) -> &'static str {
        match self {
            Self::Ok => "200 OK",
            Self::PartialContent => "206 Partial Content",
            Self::NotFound => "404 Not Found",
            Self::InternalServerError => "500 Internal Server Error",
        }
    }
}
