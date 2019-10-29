use std::fs::File;
use std::io::{Read, Error};
use serde_json::Value;
use serde_json::ser::Compound::Map;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

enum BuildingTypes {
    Industrial,
    Commercial,
    Housing
}

impl fmt::Display for BuildingTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BuildingTypes::Industrial => write!(f, "Industrial"),
            BuildingTypes::Commercial => write!(f, "Commercial"),
            BuildingTypes::Housing => write!(f, "Housing")
        }
    }
}

struct Buff(String, f64);

impl fmt::Display for Buff {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

struct Building {
    name : String,
    bd_type : BuildingTypes,
    revenue : f64,
    buff : Vec<Buff>
}

fn read_file(filename : &str) -> Result<String, Error> {
    let mut result = String::new();
    let mut file = File::open(&filename)?;
    file.read_to_string(&mut result);
    Ok(result)
}

impl Building {
    fn new(json_data : Value) -> Result<Building, &'static str> {
        let name;
        let mut bd_type = BuildingTypes::Housing;
        let revenue;
        let mut buff= Vec::new();
        let err = Err("failed to convert");
        match json_data {
            Value::Object(mut map) => {
                if let Value::String(str) = map["name"].take() {
                    name = str;
                } else {
                    return err;
                }

                if let Value::String(t) = map["type"].take() {
                    bd_type = if t == "industrial" {
                        BuildingTypes::Industrial
                    } else if t == "commercial" {
                        BuildingTypes::Commercial
                    } else if t == "housing" {
                        BuildingTypes::Housing
                    } else {
                        return err;
                    }
                }

                if let Value::Number(t) = map["revenue"].take() {
                    revenue = t.as_f64().unwrap();
                } else {
                    return err;
                }

                if let Value::Object(v) = map["buff"].take() {
                    for i in v {
                        if let Value::Number(n) = i.1 {
                            buff.push(Buff(i.0, n.as_f64().unwrap()));
                        } else {
                            return err;
                        }
                    }
                } else {
                    return err;
                }

            }
            _ => return err
        }
        Ok(Building {
            name,
            bd_type,
            revenue,
            buff
        })
    }
}

impl fmt::Display for Building {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "name : {}, type : {}, revenue : {}, ", self.name, self.bd_type, self.revenue);
        write!(f, "[");
        for b in &self.buff {
            write!(f, "{}, ", b);
        }
        write!(f, "]")
    }
}

fn main() {
    let content = read_file("content.json").unwrap();
    let v : Value = serde_json::from_str(&content).unwrap();
    let mut buildings = Vec::new();
    if let Value::Array(v) = v {
        for item in v {
            buildings.push(Building::new(item).unwrap());
        }
    }

    for b in &buildings {
        println!("{}", b);
    }
}
