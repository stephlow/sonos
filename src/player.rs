use std::net::SocketAddr;

pub struct Player {
    address: SocketAddr,
}

impl Player {
    pub fn new(addr: SocketAddr) -> Player {
        Player { address: addr }
    }
}
