mod handler;
mod server;
mod router;

use server::Server;

fn main() {
    let server = Server::new(String::from("localhost:3000"));
    server.run();
}
