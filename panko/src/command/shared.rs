use std::io;
use std::io::BufReader;
use clap::Parser;
use serde_json::{Deserializer, Value};

#[derive(Parser, Debug)]
pub struct Shared {
}

impl Shared {
    pub fn run(&self) {
        let stdio = io::stdin().lock();
        let stdio = BufReader::new(stdio);
        let mut stream = Deserializer::from_reader(stdio).into_iter::<Value>();
        let mut first = stream.next().unwrap().unwrap();
        let second = stream.next().unwrap().unwrap();
        let mut first = first.as_object_mut().unwrap();
        let second = second.as_object().unwrap();
        first.retain(|k, _| second.contains_key(k));
        println!("{}", serde_json::to_string_pretty(&first).unwrap());
    }
}
