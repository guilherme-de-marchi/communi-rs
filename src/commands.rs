use std::io;
use std::collections::HashMap;
use std::sync::{mpsc, Mutex, Arc};

pub struct CommandHandler {
    pub commands: HashMap<String, fn(&CommandHandler, Vec<&str>) -> Result<String, io::Error>>,
    pub hosts: Arc<Mutex<Vec<String>>>
}

pub fn new() -> CommandHandler {
    let mut ch = CommandHandler {
        commands: HashMap::new(),
        hosts: Arc::new(Mutex::new(Vec::new()))
    };
    ch.commands.insert("register".to_string(), register);
    ch.commands.insert("list".to_string(), list);
    ch
}

impl CommandHandler {
    pub fn handle_input(&self, input: &String) -> Result<String, io::Error> {
        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.len() == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                "input with length zero"
            ))
        }
    
        match self.commands.get(tokens[0]) {
            Some(f) => return f(self, tokens),
            None => return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                "command not found"
            ))
        }
    }    
}

pub fn register(ch: &CommandHandler, tokens: Vec<&str>) -> Result<String, io::Error> {
    if tokens.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            "no enought arguments"
        ))
    }

    ch.hosts.lock().unwrap().insert(0, tokens[1].to_string());
    Ok("registered successfully".to_string())
}

pub fn list(ch: &CommandHandler, _: Vec<&str>) -> Result<String, io::Error> {
    Ok(ch.hosts.lock().unwrap().join("\n"))
}