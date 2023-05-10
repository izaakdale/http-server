use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("localhost:9080".to_string());
    server.run();
}
