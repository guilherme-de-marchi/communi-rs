use std::io;
use std::collections::HashMap;
use std::sync::mpsc;

pub struct CommandHandler {
    pub commands: HashMap<String, fn(&CommandHandler, &mpsc::Sender<String>, Vec<&str>) -> Result<String, io::Error>>,
    pub hosts: Vec<String>
}

pub fn new() -> CommandHandler {
    let mut ch = CommandHandler {
        commands: (HashMap::new()),
        hosts: Vec::new()
    };
    ch.commands.insert("register".to_string(), register);
    ch.commands.insert("list".to_string(), list);
    ch
}

impl CommandHandler {
    pub fn handle_input(&self, tx: &mpsc::Sender<String>, input: &String) -> Result<String, io::Error> {
        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.len() == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                "input with length zero"
            ))
        }
    
        match self.commands.get(tokens[0]) {
            Some(f) => return f(self, tx, tokens),
            None => return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                "command not found"
            ))
        }
    }    
}

pub fn register(ch: &CommandHandler, tx: &mpsc::Sender<String>, tokens: Vec<&str>) -> Result<String, io::Error> {
    if tokens.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            "no enought arguments"
        ))
    }

    match tx.send(tokens[1].to_string()) {
        Ok(_) => Ok("registered successfully".to_string()),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            "could not send to channel"
        ))
    }
}

pub fn list(ch: &CommandHandler, tx: &mpsc::Sender<String>, tokens: Vec<&str>) -> Result<String, io::Error> {
    Ok(ch.hosts.join("\n"))
}