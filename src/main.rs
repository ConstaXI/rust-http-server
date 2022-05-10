mod server;
mod http;

use server::Server;
use http::Request;
use http::Method;

fn main() {
    let server = Server::new("127.0.0.1:3333".to_string());
    server.run();
}
