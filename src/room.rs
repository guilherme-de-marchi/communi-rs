use std::io;
use std::io::Write;

use crate::peer;

pub struct Room {
    name: String,
    consumers: Vec<peer::Peer>,
}

impl Room {
    pub fn new(name: String) -> Room {
        Room {
            name: name,
            consumers: Vec::new(), 
        }
    }

    pub fn subscribe(mut self, peer: peer::Peer) {
        self.consumers.push(peer);
    }

    pub fn broadcast(&self, data: Vec<u8>) -> Result<(), io::Error> {
        for c in &self.consumers {
            let mut stream = &c.stream;
            stream.write_all(data.as_slice())?
        }
        Ok(())
    }
}