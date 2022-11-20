use std::net;

pub struct Peer {
    pub stream: net::TcpStream,
}