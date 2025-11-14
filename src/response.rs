//! Response module for Nanoserve HTTP server.

use super::Request;
use compio::{
    fs::File,
    io::{AsyncReadAt, AsyncWriteExt},
};
use std::{io::Result as IoResult, path::Path};

/// An HTTP response.
#[derive(Debug, Clone)]
pub struct Response {
    /// The response code.
    pub code: ResponseCode,
    /// The response body.
    pub body: ResponseBody,
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
    /// 400 Bad Request
    BadRequest = 400,
    /// 404 Not Found
    NotFound = 404,
    /// 405 Method Not Allowed
    MethodNotAllowed = 405,
    /// 416 Range Not Satisfiable
    RangeNotSatisfiable = 416,
    // /// 500 Internal Server Error
    // InternalServerError = 500,
}

/// Response body.
#[derive(Debug, Clone)]
pub enum ResponseBody {
    /// Static body.
    Static(&'static str),
    /// From file.
    File { file: File, start: u64, end: u64 },
}

impl Response {
    /// Create a new response with the given response code and static message.
    #[must_use]
    pub const fn new(code: ResponseCode, body: &'static str) -> Self {
        let body = ResponseBody::Static(body);
        Self { code, body }
    }

    /// Construct a new [`BadRequest`](ResponseCode::BadRequest) response with the given body.
    #[must_use]
    pub const fn bad_request(body: &'static str) -> Self {
        Self::new(ResponseCode::BadRequest, body)
    }

    /// Construct a new [`NotFound`](ResponseCode::NotFound) response.
    #[must_use]
    pub const fn not_found() -> Self {
        Self::new(ResponseCode::NotFound, "404 Not Found")
    }

    /// Handles a well-formed [`Request`].
    #[must_use]
    pub async fn handle(request: &Request<'_>) -> Self {
        // Version & Method check
        if request.version != "1.1" {
            return Self::new(ResponseCode::BadRequest, "Unsupported HTTP Version");
        };
        if request.method != "GET" {
            return Self::new(ResponseCode::MethodNotAllowed, "405 Method Not Allowed");
        };
        // Resolve path relative to current directory
        let trimmed = request.path.trim_start_matches('/');
        let path = Path::new(".").join(trimmed);
        if !path.exists() || !path.is_file() {
            return Self::not_found();
        }
        // Open file and read metadata
        let file = match File::open(&path).await {
            Ok(f) => f,
            Err(_) => return Self::not_found(),
        };
        let metadata = match file.metadata().await {
            Ok(m) => m,
            Err(_) => return Self::not_found(),
        };
        if !metadata.is_file() {
            return Self::not_found();
        }
        // Create response
        let body = ResponseBody::File {
            file,
            start: 0,
            end: metadata.len(),
        };
        Self { code: ResponseCode::Ok, body }
    }

    /// Write this [`Response`] to the given destination.
    ///
    /// # Errors
    ///
    /// Returns an [`IoError`](std::io::Error) if writing fails.
    pub async fn write_to<D: AsyncWriteExt>(self, dest: &mut D) -> IoResult<()> {
        // Start line and headers (empty for now)
        dest.write_all("HTTP/1.1 ").await.0?;
        dest.write_all(self.code.description()).await.0?;
        dest.write_all("\r\n\r\n").await.0?;

        // // Dummy body
        match self.body {
            ResponseBody::Static(body) => {
                dest.write_all(body).await.0?;
            }
            ResponseBody::File { file, start, end } => {
                const BUF_LEN: usize = 8192;
                let mut buffer = Vec::with_capacity(BUF_LEN);
                let mut position = start;
                while position < end {
                    let result = file.read_at(buffer, position).await;
                    let (read_bytes, mut buf) = (result.0?, result.1);
                    if read_bytes == 0 {
                        break;
                    }
                    buf.truncate(read_bytes);
                    let result = dest.write_all(buf).await;
                    result.0?;
                    buffer = result.1;
                    buffer.resize(BUF_LEN, 0);
                    position += read_bytes as u64;
                }
            }
        }

        Ok(())
    }
}

impl ResponseCode {
    /// Get description of the response code.
    pub const fn description(self) -> &'static str {
        match self {
            Self::Ok => "200 OK",
            Self::PartialContent => "206 Partial Content",
            Self::BadRequest => "400 Bad Request",
            Self::NotFound => "404 Not Found",
            Self::MethodNotAllowed => "405 Method Not Allowed",
            Self::RangeNotSatisfiable => "416 Range Not Satisfiable",
            // Self::InternalServerError => "500 Internal Server Error",
        }
    }
}
