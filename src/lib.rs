//! # `nanoserve` library crate
//!
//! If you are reading this, you are reading the documentation for the `nanoserve` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::multiple_crate_versions, // dependency issues
    clippy::future_not_send, // compio is single-threaded by design
)]

use compio::{
    io::{AsyncRead, AsyncWriteExt},
    net::TcpListener,
    runtime::spawn,
};
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
            let (mut stream, addr) = self.listener.accept().await?;
            let task = spawn(async move {
                let result = stream.read([0; 1024]).await;
                let (result, _buffer) = (result.0, result.1);
                match result {
                    Ok(size) => {
                        println!("Received {size} bytes from {addr}");
                        let response =
                            b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
                        let _ = stream.write_all(response).await;
                    }
                    Err(e) => eprintln!("Failed to read from socket: {e}"),
                }
            });
            task.detach();
        }
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
