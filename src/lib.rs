mod commands;

use std::{process, thread, io::{Read, Write}};
use std::net;
use std::sync::{mpsc, Mutex};

pub fn run(addr: &String) {
    let mut input: String;
    loop {
        input = text_io::read!("{}\n");
        match input.as_str() {
            "listen" => listen(addr),
            "connect" => connect(addr),
            _ => continue
        }
    }    
}

fn listen(addr: &String) {
    println!("started listening to connections");

    let command_handler = commands::new();
    let listener = net::TcpListener::bind(addr).unwrap_or_else(|e| {
        eprintln!("could not bind: {e}");
        process::exit(1);
    });

    thread::scope(|scope| {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let thread_command_handler = &command_handler;
                    scope.spawn(move || loop {
                        handle_connection(&mut stream, thread_command_handler);
                    });
                },
                Err(e) => {
                    eprintln!("error handling connection: {e}");
                    continue
                }
            }
        }
    });
}

fn handle_connection(stream: &mut net::TcpStream, command_handler: &commands::CommandHandler) {
    let mut buf = [0; 1024];
    let buf_read = match stream.read(&mut buf) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error handling connection: {e}");
            return
        }
    };

    let mut input = String::from_utf8_lossy(&buf[..buf_read]).to_string();
    input = input[..input.len()-2].to_string();
    println!("input received: '{input}'");

    match command_handler.handle_input(&input) {
        Ok(v) => {
            if let Err(e) = stream.write_all((v + "\n").as_bytes()) {
                eprintln!("error writting: {e}")
            }
        },
        Err(e) => {
            if let Err(e) = stream.write_all((e.to_string() + "\n").as_bytes()) {
                eprintln!("error writting: {e}")
            }
        }
    }
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