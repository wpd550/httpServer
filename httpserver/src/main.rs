mod server;
mod router;
mod handler;

use server::Server;

fn main() {

    let server = Server::new("127.0.0.1:9527");
    server.run();
}
