use std::fmt;
use std::fmt::Formatter;

use serde_json::Value;

use crate::convert::Convert;

pub enum GlobalBuffType {
    Industrial,
    Commercial,
    Housing,
    All,
    Online,
    Offline
}

impl fmt::Display for GlobalBuffType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GlobalBuffType::Offline =>  write!(f, "offline"),
            GlobalBuffType::Online => write!(f, "online"),
            GlobalBuffType::All => write!(f, "all"),
            GlobalBuffType::Industrial => write!(f, "industrial"),
            GlobalBuffType::Commercial => write!(f, "commercial"),
            GlobalBuffType::Housing => write!(f, "housing"),
        }
    }
}

impl Convert<Value> for GlobalBuffType {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => {
                if s == "industrial" {
                    Ok(GlobalBuffType::Industrial)
                } else if s == "commercial" {
                    Ok(GlobalBuffType::Commercial)
                } else if s == "housing" {
                    Ok(GlobalBuffType::Housing)
                } else if s == "all" {
                    Ok(GlobalBuffType::All)
                } else if s == "online" {
                    Ok(GlobalBuffType::Online)
                } else if s == "offline" {
                    Ok(GlobalBuffType::Offline)
                } else {
                    Err("Cannot convert to GlobalBuff")
                }
            },

            _ => Err("Cannot convert to GlobalBuff")
        }
    }
}

pub struct GlobalBuff(String, GlobalBuffType, f64);

impl GlobalBuff {
    pub fn get_name(&self) -> &str {
        &self.0
    }

    pub fn get_type(&self) -> &GlobalBuffType {
        &self.1
    }

    pub fn get_effect(&self) -> f64 {
        self.2
    }
}

impl fmt::Display for GlobalBuff {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GlobalBuff name : {}, type : {}, effect : {}", self.0, self.1, self.2)
    }
}

impl Convert<Value> for Vec<GlobalBuff> {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        let err = Err("Cannot convert to Vec<GlobalBuff>");
        let mut result = Vec::new();
        match value {
            Value::Array(arr) => {
                for item in arr {
                    match item {
                        Value::Object(mut map) => {
                            let name = String::convert(map["name"].take())?;
                            let gb_type = GlobalBuffType::convert(map["type"].take())?;
                            let number = f64::convert(map["buff"].take())?;
                            result.push(GlobalBuff(name, gb_type, number))
                        },
                        _ => return err
                    }
                }
                Ok(result)
            },
            _ => err
        }
    }
}
