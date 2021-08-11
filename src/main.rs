use std::collections::HashMap;
use std::env;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::str::FromStr;

pub struct Json<V: FromStr + Debug + Display> {
    data: HashMap<String, V>,
}

impl<V: FromStr + Debug + Display> Json<V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn encode(&self, indent: usize) -> String {
        let mut result = String::new();
        result.push_str("{\n");
        for key in self.data.keys() {
            let value = self.data.get(key).unwrap();
            let indent_space = " ".repeat(indent);
            result.push_str(
                format!("{}\"{}\":{}{},\n", indent_space, key, indent_space, value).as_str(),
            );
        }
        result.push_str("}");
        result
    }

    pub fn decode(&mut self, src: &mut File)
    where
        <V as FromStr>::Err: Debug + Display,
    {
        let mut contents = String::new();
        src.read_to_string(&mut contents)
            .expect("Error reading source file");

        if !contents.starts_with("{") && !contents.ends_with("}") {
            process::exit(1);
        }

        let parsed_contents = contents.replace("{", "").replace("}", "").replace("\n", "");

        let lines: Vec<&str> = parsed_contents.split(",").collect();

        for line in lines.into_iter() {
            let line: Vec<&str> = line.trim().split(":").collect();
            let key = line.get(0).unwrap().replace("\"", "");
            let value = line
                .get(1)
                .expect("Value might be empty")
                .trim()
                .replace("\"", "");

            let parsed_value: V = match value.parse() {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            };

            self.data.insert(key, parsed_value);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut src = File::open(&args[1]).expect("Error opening file");
    let mut json_data: Json<i32> = Json::new();
    json_data.decode(&mut src);
    let encoded_data = json_data.encode(2);
    println!("{}", encoded_data);
}
