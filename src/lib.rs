use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::str::FromStr;

pub struct Json<V> {
    data: HashMap<&'static str, V>,
}

impl<V: FromStr + Debug + Display> Json<V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn encode(&self) -> String {
        let mut result = String::new();
        result.add("{\n");
        for key in self.data.keys() {
            let value = self.data.get(key).unwrap();
            result.add(format!("\t\"{}\":\t{}\n", key, value).as_str());
        }
        result.add("\n}");
        result
    }

    pub fn decode(&mut self, src: &File) {
        let mut contents = String::new();
        src.read_to_string(&mut contents)
            .expect("Error reading source file");

        match contents.starts_with("{") && contents.ends_with("}") {
            true => (),
            false => return,
        }

        let mut lines: Vec<&str> = contents
            .replace("{", "")
            .replace("}", "")
            .replace("\n", "")
            .split(",")
            .collect();

        for line in lines.into_iter() {
            let line: Vec<&str> = line.trim().split(":").collect();
            let key: &str = line.get(0).unwrap().replace("\"", "").as_str();
            let value: &str = line.get(1).unwrap().replace("\"", "").as_str();
            let parsed_value = match value.parse::<V>() {
                Ok(v) => v,
                Err(e) => eprintln!("Error occurred while parsing value"),
            };

            self.data.insert(key, parsed_value);
        }
    }
}
