#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

mod cli;

use cli::Cli;
use compio::{runtime::spawn, signal::ctrl_c};
use nanoserve::HTTPServer;
use std::net::SocketAddr;

#[compio::main]
async fn main() {
    let cli: Cli = argh::from_env();
    let addr = SocketAddr::new(cli.address, cli.port);
    let server = HTTPServer::new(addr)
        .await
        .expect("Failed to create server");
    println!("Server listening on http://{addr}");

    // Spawn the server in a separate task
    let server_task = spawn(async move { server.run().await });

    // Wait for Ctrl+C
    ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("Received Ctrl+C, shutting down server...");

    // Cancel the server task
    drop(server_task);
    println!("Server stopped successfully");
}
