mod commands;

use std::{process, thread, io::{Read, Write}};
use std::net;
use std::sync::mpsc;

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
    let mut command_handler = commands::new();
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    thread::spawn(move || loop {
        let received = match rx.recv() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error receiving from channel: {e}");
                return
            }
        };

        command_handler.hosts.insert(0, received);
    });

    let listener = net::TcpListener::bind(addr).unwrap_or_else(|e| {
        eprintln!("could not bind: {e}");
        process::exit(1);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // let thread_ch = &command_handler;
                let thread_tx = tx.clone();
                thread::spawn(move || loop {
                    handle_connection(stream, &command_handler, &thread_tx)
                });
            },
            Err(e) => {
                eprintln!("error handling connection: {e}");
                continue
            }
        }
    }
}

fn handle_connection(mut stream: net::TcpStream, command_handler: &commands::CommandHandler, tx: &mpsc::Sender<String>) {
    let mut buf = [0; 1024];
    if let Err(e) = stream.read(&mut buf) {
        eprintln!("error handling connection: {e}");
        return
    }

    let input = String::from_utf8_lossy(&buf).to_string();
    println!("{input}");

    if let Err(e) = command_handler.handle_input(tx, &input) {
        eprintln!("error handling input '{input}': {e}");
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