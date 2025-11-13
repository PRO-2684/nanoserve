#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use compio::{signal::ctrl_c, runtime::spawn};
use nanoserve::HTTPServer;

#[compio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let server = HTTPServer::new(addr).await.expect("Failed to create server");
    println!("Server listening on http://{}", server.local_addr().unwrap());

    // Spawn the server in a separate task (moves ownership)
    let server_task = spawn(async move {
        server.run().await
    });

    // Wait for Ctrl+C
    ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("\nReceived Ctrl+C, shutting down server...");

    // Cancel the server task
    drop(server_task);

    println!("Server closed successfully");
}
