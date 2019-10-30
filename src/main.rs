mod buildings;
mod convert;
mod global_buff;

use serde_json::Value;
use buildings::Building;
use std::io;
use std::fs::File;
use std::io::Read;
use convert::Convert;
use crate::global_buff::GlobalBuff;

fn read_file(filename : &str) -> Result<String, io::Error> {
    let mut result = String::new();
    let mut file = File::open(&filename)?;
    file.read_to_string(&mut result)?;
    Ok(result)
}

fn process() -> Result<(), &'static str> {
    let content = read_file("content.json").unwrap();
    let v : Value = serde_json::from_str(&content).unwrap();
    let mut global_buff : Vec<GlobalBuff> = Vec::new();
    let mut buildings : Vec<Building> = Vec::new();
    if let  Value::Object(mut map) = v {
        buildings = Vec::convert(map["buildings"].take())?;
        global_buff = Vec::convert(map["global"].take())?;
    }
    for b in &buildings {
        println!("{}", b);
    }

    for g in &global_buff {
        println!("{}", g);
    }
    Ok(())
}

fn main() {
    process().unwrap();
}
