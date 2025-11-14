//! Request parsing module.

use std::{
    fmt,
    num::ParseIntError,
    str::{Utf8Error, from_utf8},
};

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

/// Range header representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeHeader {
    /// A valid range with start and end.
    Bytes(Option<u64>, Option<u64>),
    /// Invalid or unsupported range format.
    Invalid,
    /// No range specified.
    None,
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
            .or_else(|| {
                request
                    .windows(2)
                    .position(|w| w == b"\n\n")
                    .map(|pos| pos + 2)
            })
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

    /// Parse the `Range` header, if present.
    #[must_use]
    pub fn parse_range_header(&self) -> RangeHeader {
        for (key, value) in &self.headers {
            if key.eq_ignore_ascii_case("Range") {
                // Expect format: bytes=start-end
                // start or end can be omitted
                let Some(range_part) = value.strip_prefix("bytes=") else {
                    return RangeHeader::Invalid;
                };
                let mut parts = range_part.split('-');
                let (Some(start_str), Some(end_str)) = (parts.next(), parts.next()) else {
                    return RangeHeader::Invalid;
                };

                match (
                    Self::parse_optional(start_str),
                    Self::parse_optional(end_str),
                ) {
                    (Ok(start), Ok(end)) => return RangeHeader::Bytes(start, end),
                    _ => return RangeHeader::Invalid,
                }
            }
        }
        RangeHeader::None
    }

    /// Helper to parse an optional u64 from a &str.
    fn parse_optional(s: &str) -> Result<Option<u64>, ParseIntError> {
        if s.is_empty() {
            Ok(None)
        } else {
            s.parse().map(Some)
        }
    }
}

impl fmt::Display for Request<'_> {
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

impl ParseRequestError {
    /// Get a description of the error.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::InvalidRequestLine => "Invalid request line",
            Self::InvalidUtf8 => "Invalid UTF-8 in request",
            Self::IoError => "IO error while reading request",
        }
    }
}

impl fmt::Display for ParseRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<Utf8Error> for ParseRequestError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidUtf8
    }
}
