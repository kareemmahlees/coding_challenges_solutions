use std::net::{Ipv4Addr, SocketAddrV4};

use roar::Server;

fn main() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 6379);
    let server = Server::new(addr);
    server.listen_and_serve();
}
