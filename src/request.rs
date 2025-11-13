//! Request parsing module.

use std::{fmt, str::{Utf8Error, from_utf8}};

/// An HTTP request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request<'a> {
    /// The request method.
    pub method: &'a str,
    /// The request path.
    pub path: &'a str,
    /// The HTTP version.
    pub version: &'a str,
    /// The headers.
    pub headers: Vec<(&'a str, &'a str)>,
    /// The body.
    pub body: &'a [u8],
}

/// Possible errors when parsing an HTTP packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseRequestError {
    /// The packet does not contain a valid HTTP request line.
    InvalidRequestLine,
    /// The packet header is not properly encoded in UTF-8.
    InvalidUtf8,
    /// IO error while reading lines.
    IoError,
}

impl<'a> Request<'a> {
    /// Parses a raw HTTP request.
    ///
    /// # Errors
    ///
    /// See [`ParseRequestError`].
    pub fn parse(request: &'a [u8]) -> Result<Self, ParseRequestError> {
        // Find the header/body separator in raw bytes (double CRLF or double LF)
        let separator = request
            .windows(4)
            .position(|w| w == b"\r\n\r\n")
            .map(|pos| pos + 4)
            .or_else(|| request.windows(2).position(|w| w == b"\n\n").map(|pos| pos + 2))
            .unwrap_or(request.len());

        // Split header and data at byte level
        let header_bytes = &request[..separator.min(request.len())];
        let body = &request[separator.min(request.len())..];

        // Now parse only the header section as UTF-8
        let header_text = from_utf8(header_bytes)?;
        let mut lines = header_text.lines();

        // Parse the first line (status line or request line)
        let first_line = lines
            .next()
            .ok_or(ParseRequestError::InvalidRequestLine)?
            .trim();

        let mut parts = first_line.split_whitespace();
        let method = parts.next().ok_or(ParseRequestError::InvalidRequestLine)?;
        let path = parts.next().ok_or(ParseRequestError::InvalidRequestLine)?;
        let version_part = parts.next().ok_or(ParseRequestError::InvalidRequestLine)?;
        let version = version_part
            .strip_prefix("HTTP/")
            .ok_or(ParseRequestError::InvalidRequestLine)?;

        // Parse headers
        let headers = Self::parse_headers(&mut lines);

        Ok(Self {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    /// Parse HTTP headers from lines.
    fn parse_headers<'b>(lines: &mut impl Iterator<Item = &'b str>) -> Vec<(&'b str, &'b str)> {
        let mut headers = Vec::new();
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                break; // End of headers
            }
            if let Some((key, value)) = line.split_once(':') {
                headers.push((key.trim(), value.trim()));
            }
        }
        headers
    }
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {} HTTP/{}", self.method, self.path, self.version)?;
        for (key, value) in &self.headers {
            writeln!(f, "{key}: {value}")?;
        }
        let body_length = self.body.len();
        writeln!(f, "\n[Body: {body_length} bytes]")?;
        Ok(())
    }
}

impl fmt::Display for ParseRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseRequestError::InvalidRequestLine => write!(f, "Invalid request line"),
            ParseRequestError::InvalidUtf8 => write!(f, "Invalid UTF-8 in request"),
            ParseRequestError::IoError => write!(f, "IO error while reading request"),
        }
    }
}

impl From<Utf8Error> for ParseRequestError {
    fn from(_: Utf8Error) -> Self {
        ParseRequestError::InvalidUtf8
    }
}
