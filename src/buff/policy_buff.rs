use std::fmt;
use serde_json::Value;
use crate::convert::Convert;

#[derive(Eq, PartialEq, Hash)]
pub enum PolicyBuffType {
//    Goods,
    Industrial,
    Commercial,
    Housing,
    All,
    Online,
    Offline
}

impl fmt::Display for PolicyBuffType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolicyBuffType::Offline =>  write!(f, "offline"),
            PolicyBuffType::Online => write!(f, "online"),
            PolicyBuffType::All => write!(f, "all"),
            PolicyBuffType::Industrial => write!(f, "industrial"),
            PolicyBuffType::Commercial => write!(f, "commercial"),
            PolicyBuffType::Housing => write!(f, "housing"),
        }
    }
}

impl Convert<Value> for PolicyBuffType {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => {
                if s == "industrial" {
                    Ok(PolicyBuffType::Industrial)
                } else if s == "commercial" {
                    Ok(PolicyBuffType::Commercial)
                } else if s == "housing" {
                    Ok(PolicyBuffType::Housing)
                } else if s == "all" {
                    Ok(PolicyBuffType::All)
                } else if s == "online" {
                    Ok(PolicyBuffType::Online)
                } else if s == "offline" {
                    Ok(PolicyBuffType::Offline)
                } else {
                    Err("Cannot convert to GlobalBuff")
                }
            },

            _ => Err("Cannot convert to GlobalBuff")
        }
    }
}

pub struct PolicyBuff(String, PolicyBuffType, f64);

impl PolicyBuff {
    pub fn get_name(&self) -> &str {
        &self.0
    }

    pub fn get_type(&self) -> &PolicyBuffType {
        &self.1
    }

    pub fn get_effect(&self) -> f64 {
        self.2
    }
}

impl fmt::Display for PolicyBuff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GlobalBuff name : {}, type : {}, effect : {}", self.0, self.1, self.2)
    }
}

impl Convert<Value> for Vec<PolicyBuff> {
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
                            let gb_type = PolicyBuffType::convert(map["type"].take())?;
                            let number = f64::convert(map["effect"].take())?;
                            result.push(PolicyBuff(name, gb_type, number))
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
