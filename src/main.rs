extern crate toml;
extern crate serde_json;
use std::io;
use std::io::prelude::*;
use toml::Value;

fn main() {
    let stdin = io::stdin();
    let mut string = String::new();
    stdin.lock().read_to_string(&mut string).expect("error reading stdin");
    let value = string.parse::<Value>().expect("Error parsing TOML");
    let encoded = serde_json::to_string_pretty(&value).expect("Error encoding JSON");
    println!("{}", encoded);
}
