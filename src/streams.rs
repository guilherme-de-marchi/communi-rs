use std::net;

pub struct Streams {
    pub streams: Vec<net::TcpStream>,
    pub max_streams: u8
}