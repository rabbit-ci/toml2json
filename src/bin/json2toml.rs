extern crate toml;
extern crate serde_json;
use std::io;
use std::io::prelude::*;
// use serde_json::Value;

fn main() {
    let stdin = io::stdin();
    let mut string = String::new();
    stdin.lock().read_to_string(&mut string).expect("Error reading stdin");
    let value: serde_json::Value = serde_json::from_str(&string).expect("Error parsing JSON");
    let encoded = toml::to_string_pretty(&value).expect("Error encoding TOML");
    println!("{}", encoded);
}
