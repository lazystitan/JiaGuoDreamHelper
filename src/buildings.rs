use serde_json::Value;
use std::fmt;
use std::fmt::Formatter;
use std::convert::{TryFrom, TryInto};
use crate::buff::BuildingBuff;
use crate::convert::Convert;

pub enum BuildingType {
    Industrial,
    Commercial,
    Housing
}

impl TryFrom<String> for BuildingType {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
            if value == "industrial" {
                Ok(BuildingType::Industrial)
            } else if value == "commercial" {
                Ok(BuildingType::Commercial)
            } else if value == "housing" {
                Ok(BuildingType::Housing)
            } else {
                return Err("Cannot convert to building types");
            }
    }
}

impl fmt::Display for BuildingType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BuildingType::Industrial => write!(f, "Industrial"),
            BuildingType::Commercial => write!(f, "Commercial"),
            BuildingType::Housing => write!(f, "Housing")
        }
    }
}



pub struct Building {
    name : String,
    bd_type : BuildingType,
    income: f64,
    buff : Vec<BuildingBuff>
}

impl Building {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &BuildingType {
        &self.bd_type
    }

    pub fn get_income(&self) -> f64 {
        self.income
    }

    pub fn get_buff(&self) -> &Vec<BuildingBuff> {
        &self.buff
    }
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

impl Convert<Value> for Building {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(mut map) => {
                let result = Building {
                    name: String::convert(map["name"].take())?,
                    bd_type: String::convert(map["type"].take())?.try_into()?,
                    income: f64::convert(map["income"].take())?,
                    buff: Vec::convert(map["effect"].take())?
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
        write!(f, "Building name : {}, type : {}, revenue : {}, ", self.name, self.bd_type, self.income)?;
        write!(f, "[ ")?;
        for i in 0..(&self.buff.len()-1) {
            write!(f, "{}, ", &self.buff[i])?;
        }
        write!(f, "{} ]", self.buff.last().unwrap())
    }
}

impl Convert<Value> for Vec<Building> {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        let mut buildings = Vec::new();
        if let Value::Array(v) = value {
            for item in v {
                buildings.push(Building::convert(item)?);
            }
        }
        Ok(buildings)
    }
}