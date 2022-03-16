mod server;
mod http;
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:7877".to_string());
    server.run();
}
