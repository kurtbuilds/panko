use std::io;
use std::io::BufReader;
use clap::Parser;
use serde_json::{Deserializer, Value};

#[derive(Parser, Debug)]
pub struct Struct {
    // a: Option<String>,
    // b: Option<String>,
}

impl Struct {
    pub fn run(&self) {
        let stdio = io::stdin().lock();
        let stdio = BufReader::new(stdio);
        let stream: Value = serde_json::from_reader(stdio).unwrap();

    }
}
