mod buildings;
mod convert;
mod global;
mod buff;
mod manager;
mod buff_map;

use std::io;
use std::io::Read;
use std::fs::File;

use serde_json::Value;

use convert::Convert;
use buildings::Building;
use global::Global;
use buff::PolicyBuff;
use crate::manager::Manager;

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

    let mut m = Manager::new();
    for b in buildings {
        m.add_building(b);
    }

    for p in global_buff {
        m.add_policy_buff(p);
    }



    Ok(())
}

fn main() {
    process().unwrap();
//    let num : u64 = 30_096_900_000_000;
//    let num : u64 = 50_898_100_000_000;
//    let num : u64 = 1_787_285_321_811_827_000_000_000;
//    let num : u64 = 135_169_420_000_000;
//    let num : u64 = 1_040_106_800_000_000; 1aa 1040t
//    let num : u64 = 360_291_025_000_000; 0.3aa 360t
//    test();
}
