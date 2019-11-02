mod buildings;
mod convert;
mod global;
mod buff;

use std::io;
use std::io::Read;
use std::fs::File;

use serde_json::Value;

use convert::Convert;
use buildings::Building;
use global::Global;
use buff::PolicyBuff;

fn read_file(filename : &str) -> Result<String, io::Error> {
    let mut result = String::new();
    let mut file = File::open(&filename)?;
    file.read_to_string(&mut result)?;
    Ok(result)
}

fn process() -> Result<(), &'static str> {
    let content = read_file("content.json").unwrap();
    let v : Value = serde_json::from_str(&content).unwrap();
    let mut global_buff : Vec<PolicyBuff> = Vec::new();
    let mut buildings : Vec<Building> = Vec::new();
    if let  Value::Object(mut map) = v {
        buildings = Vec::convert(map["buildings"].take())?;
        global_buff = Vec::convert(map["policy"].take())?;
    }

    let mut g = Global::new();

    for b in buildings {
        g.add_building(b)?;
    }

    for b in global_buff {
        g.add_policy_buff(b)?;
    }

    let names = g.get_building_names();
    for n in &names {
        println!("{}", n);
    }

    println!("{}", g.get_online_income());

    println!("{}", g.is_full());

    Ok(())
}

fn main() {
    process().unwrap();
//    let num : u64 = 30_096_900_000_000;
//    let num : u64 = 50_898_100_000_000;
//    let num : u64 = 1_787_285_321_811_827_000_000_000;
//    let num : u64 = 135_169_420_000_000;
//    test();
}
