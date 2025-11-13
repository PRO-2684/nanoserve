//! # `nanoserve` library crate
//!
//! If you are reading this, you are reading the documentation for the `nanoserve` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::multiple_crate_versions, // dependency issues
    clippy::future_not_send, // compio is single-threaded by design
)]

mod error;
mod request;

use compio::{
    io::{AsyncRead, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    runtime::spawn,
};
pub use error::NanoserveError;
pub use request::{Request, ParseRequestError};
use std::{io::Error as IoError, net::SocketAddr};

/// A HTTP/1.1 server.
///
/// # Usage
///
/// - [`new`](Self::new): Creates a new HTTP server that listens on the given address.
/// - [`run`](Self::run): Runs the server, accepting and handling connections.
/// - [`local_addr`](Self::local_addr): Gets the local address of the server.
#[derive(Debug, Clone)]
pub struct HTTPServer {
    /// The TCP listener.
    listener: TcpListener,
}

impl HTTPServer {
    /// Creates a new HTTP server that listens on the given address.
    ///
    /// # Errors
    ///
    /// Returns an [`IoError`] if the server fails to bind to the address.
    pub async fn new(addr: &str) -> Result<Self, IoError> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }

    /// Runs the server.
    ///
    /// # Errors
    ///
    /// Returns an [`IoError`] if the server fails to start.
    pub async fn run(&self) -> Result<(), IoError> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            println!("Accepted connection from {addr}");
            let task = spawn(async move {
                Self::handle_connection(stream).await.unwrap_or_else(|e| {
                    eprintln!("Error while handling connection from {addr}: {e}");
                });
            });
            task.detach();
        }
    }

    /// Handles a single connection.
    async fn handle_connection(mut stream: TcpStream) -> Result<(), NanoserveError> {
        let result = stream.read([0; 1024]).await;
        let (size, buffer) = (result.0?, result.1);
        println!("Received {size} bytes");
        let request = Request::parse(&buffer[..size])?;
        println!("{request}");
        // TODO: Actually handle the request and generate a response
        let response = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
        stream.write_all(response).await.0?;
        stream.close().await?;

        Ok(())
    }

    /// Get the local address of the server.
    ///
    /// # Errors
    ///
    /// Returns an [`IoError`] if unable to retrieve the local address.
    pub fn local_addr(&self) -> Result<SocketAddr, IoError> {
        self.listener.local_addr()
    }
}
