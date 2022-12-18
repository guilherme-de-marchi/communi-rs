use std::{process, thread, io::{Read, Write}};
use std::net;
use std::io;
use text_io::read;


struct Peer {
    listener: net::TcpListener,
    clients: Vec<net::TcpStream>
}

pub fn run(addr: &String) {
    let input: String = text_io::read!("{}\n");
    match input.as_str() {
        "listen" => listen(addr),
        "connect" => connect(addr),
        _ => {
            eprintln!("invalid input");
            process::exit(1);
        }
    }

    
}

fn listen(addr: &String) {
    let listener = net::TcpListener::bind(addr).unwrap_or_else(|e| {
        eprintln!("could not bind: {e}");
        process::exit(1);
    });

    for stream in listener.incoming() {
        if let Err(e) = handle_stream(stream) {
            eprintln!("error handling connection: {e}");
            process::exit(1);
        }
    }
}

fn handle_stream(r: Result<net::TcpStream, io::Error>) -> Result<(), io::Error> {
    match r {
        Err(e) => { return Err(e) },
        Ok(mut stream) => {
            // let mut buf: Vec<u8> = Vec::new(); 
            let mut buf = [0; 10];
            if let Err(e) = stream.read(&mut buf) {
                return Err(e);
            }
            println!("{:?}", buf)
        }
    }
    Ok(())
}

fn connect(addr: &String) {
   let stream = net::TcpStream::connect(addr);
   match stream {
        Err(e) => {
            eprintln!("error connecting: {e}");
            process::exit(1);
        },
        Ok(mut stream) => {
            loop {
                let data: String = text_io::read!("{}\n");
                if let Err(e) = stream.write_all(data.as_bytes()) {
                    eprintln!("error writting: {e}");
                    process::exit(1);
                }
            }
        }
   }
}