mod buildings;

use serde_json::Value;
use buildings::{ Building, Convert };
use std::io;
use std::fs::File;
use std::io::Read;

fn read_file(filename : &str) -> Result<String, io::Error> {
    let mut result = String::new();
    let mut file = File::open(&filename)?;
    file.read_to_string(&mut result)?;
    Ok(result)
}

fn process() {
    let content = read_file("content.json").unwrap();
    let v : Value = serde_json::from_str(&content).unwrap();
    let mut buildings = Vec::new();
    if let Value::Array(v) = v {
        for item in v {
            buildings.push(Building::convert(item).unwrap());
        }
    }

    for b in &buildings {
        println!("{}", b);
    }
}

fn main() {
    process();
}
