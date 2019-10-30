mod buildings;
mod convert;
mod global_buff;
mod global;

use serde_json::Value;
use buildings::Building;
use std::io;
use std::fs::File;
use std::io::Read;
use convert::Convert;
use crate::global_buff::GlobalBuff;
use crate::global::Global;

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

    let mut g = Global::new();

    for b in buildings {
        g.add_building(b)?;
    }

    for b in global_buff {
        g.add_global_buff(b)?;
    }

    let names = g.get_building_names();
    for n in &names {
        println!("{}", n);
    }

    println!("{}", g.is_full());

    Ok(())
}

fn main() {
    process().unwrap();
}
