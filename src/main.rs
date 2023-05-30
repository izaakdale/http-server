#![allow(dead_code)]

use server::Server;
use handler::WebHandler;
use std::env;

mod http;
mod server;
mod handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("localhost:9080".to_string());
    server.run(WebHandler::new(public_path));
}