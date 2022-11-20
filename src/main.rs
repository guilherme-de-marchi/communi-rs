use std::env;
use communi_rs;

fn main() {
    let addr = env::var("CLIENT_ADDR")
        .unwrap_or(String::from("0.0.0.0:8080"));
    
    communi_rs::run(&addr);
}
