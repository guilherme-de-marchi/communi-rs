mod user;
mod peer;
mod room;

use std::{process, thread, io::{Read, Write}};
use std::net;
use std::io;
use text_io::read;


pub fn run(addr: &String) {
    let u = user::User::new();
    u.add_room(room::Room::new(String::from("room_1")));
    
    let listener = net::TcpListener::bind(addr).unwrap_or_else(|e| {
        eprintln!("could not bind: {e}");
        process::exit(1);
    });

    for stream in listener.incoming() {
        handle_stream(stream);
    }
}

fn handle_stream(stream: Result<net::TcpStream, io::Error>) -> Result<(), io::Error> {
    match stream {
        Err(e) => { return Err(e) },
        Ok(mut stream) => {
            let mut buf: Vec<u8> = Vec::new(); 
            if let Err(e) = stream.read_to_end(&mut buf) {
                return Err(e);
            }

            
        }
    }
    Ok(())
}