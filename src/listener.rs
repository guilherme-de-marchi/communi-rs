use std::net;

pub struct Listener {
    pub listener: net::TcpListener,
    pub read_buffer_size: u16,
    pub host: String,
    pub port: String
}