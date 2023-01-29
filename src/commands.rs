use std::io;
use std::collections::HashMap;
use std::fmt::format;
use std::sync::{mpsc, Mutex, Arc};

pub struct CommandHandler {
    pub commands: HashMap<String, fn(&CommandHandler, Vec<&str>) -> Result<String, io::Error>>,
    pub hosts: Arc<Mutex<HashMap<String, String>>>
}

pub fn new() -> CommandHandler {
    let mut ch = CommandHandler {
        commands: HashMap::new(),
        hosts: Arc::new(Mutex::new(HashMap::new()))
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
    if tokens.len() < 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            "no enought arguments"
        ))
    }

    ch.hosts.lock().unwrap().insert(tokens[1].to_string(), tokens[2].to_string());
    Ok("registered successfully".to_string())
}

pub fn list(ch: &CommandHandler, _: Vec<&str>) -> Result<String, io::Error> {
    let mut res = String::from("");
    for (k, v) in &*ch.hosts.lock().unwrap() {
        res += [k, ":", v, "\n"].join("").as_str();
    }
    Ok(res)
}