extern crate toml;
extern crate rustc_serialize;
use rustc_serialize::json;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut string = String::new();
    let _ = stdin.lock().read_to_string(&mut string);
    let value = toml::Parser::new(&string).parse().unwrap();
    let encoded = json::encode(&value).unwrap();
    println!("{}", encoded);
}
