use serde_json::Value;
use std::fmt;
use std::fmt::Formatter;
use std::convert::{TryFrom, TryInto};

pub enum BuildingTypes {
    Industrial,
    Commercial,
    Housing
}

impl TryFrom<String> for BuildingTypes {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
            if value == "industrial" {
                Ok(BuildingTypes::Industrial)
            } else if value == "commercial" {
                Ok(BuildingTypes::Commercial)
            } else if value == "housing" {
                Ok(BuildingTypes::Housing)
            } else {
                return Err("Cannot convert to building types");
            }
    }
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

pub struct Buff(String, f64);

impl fmt::Display for Buff {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

pub struct Building {
    name : String,
    bd_type : BuildingTypes,
    revenue : f64,
    buff : Vec<Buff>
}

pub trait Convert<T> : Sized {
    type Error;
    fn convert(value : T) -> Result<Self, Self::Error>;
}

impl Convert<Value> for String {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err("Cannot cast to String")
        }
    }
}

impl Convert<Value> for f64 {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        let err = Err("Cannot cast to f64");
        match value {
            Value::Number(n) => {
                match n.as_f64() {
                    Some(n) => Ok(n),
                    None => err
                }
            }
            _ => err
        }
    }
}

impl Convert<Value> for Vec<Buff> {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(map) => {
                let mut result = Vec::with_capacity(map.len());
                for (key , item) in map {
                    result.push(Buff(key, f64::convert(item)?));
                }
                Ok(result)
            },
            _ => Err("Cannot cast to Vec<Buff>")
        }
    }
}

impl Convert<Value> for Building {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(mut map) => {
                let result = Building {
                    name: String::convert(map["name"].take())?,
                    bd_type: String::convert(map["type"].take())?.try_into()?,
                    revenue: f64::convert(map["revenue"].take())?,
                    buff: Vec::convert(map["buff"].take())?
                };
                Ok(result)
            },
            _ => Err("Cannot convert to Building")
        }
    }
}

impl fmt::Display for Building {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.buff.len() == 0 {
            write!(f, "[]")?;
            return Ok(());
        }
        write!(f, "name : {}, type : {}, revenue : {}, ", self.name, self.bd_type, self.revenue)?;
        write!(f, "[ ")?;
        for i in 0..(&self.buff.len()-1) {
            write!(f, "{}, ", &self.buff[i])?;
        }
        write!(f, "{} ]", self.buff.last().unwrap())
    }
}