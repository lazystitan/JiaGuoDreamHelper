use std::fmt;
use crate::convert::Convert;
use serde_json::Value;
use std::fmt::{Formatter, Error};

pub enum BuffType {
    Normal(String),
    Industrial,
    Commercial,
    Housing,
    All,
    Online,
    Offline
}

impl fmt::Display for BuffType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BuffType::Normal(s) => write!(f, "Normal ( {} )", s),
            BuffType::Offline =>  write!(f, "offline"),
            BuffType::Online => write!(f, "online"),
            BuffType::All => write!(f, "all"),
            BuffType::Industrial => write!(f, "industrial"),
            BuffType::Commercial => write!(f, "commercial"),
            BuffType::Housing => write!(f, "housing"),
        }
    }
}

impl Convert<String> for BuffType {
    type Error = &'static str;

    fn convert(value: String) -> Result<Self, Self::Error> {
        if value == "industrial" {
            Ok(BuffType::Industrial)
        } else if value == "commercial" {
            Ok(BuffType::Commercial)
        } else if value == "housing" {
            Ok(BuffType::Housing)
        } else if value == "all" {
            Ok(BuffType::All)
        } else if value == "online" {
            Ok(BuffType::Online)
        } else if value == "offline" {
            Ok(BuffType::Offline)
        } else {
            let mut s = value.split('(');
            if let Some(part1) = s.next() {
                if part1 != "normal" {
                    return Err("Cannot convert to BuffType");
                }
            } else {
                return Err("Cannot convert to BuffType");
            }
            if let Some(part2) = s.next() {
                if s.next() != None {
                    return Err("Cannot convert to BuffType");
                }

                if &part2[part2.len() - 1..] != ")" {
                    return Err("Cannot convert to BuffType");
                }
                return Ok(BuffType::Normal(part2[0 .. part2.len() - 1].to_string()))

            } else {
                return Err("Cannot convert to BuffType");
            }

        }
    }
}

pub struct Buff(BuffType, f64);


impl Convert<Value> for Vec<Buff> {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(map) => {
                let mut result = Vec::with_capacity(map.len());
                for (key , item) in map {
                    result.push(Buff::new(BuffType::convert(key)?, f64::convert(item)?));
                }
                Ok(result)
            },
            _ => Err("Cannot cast to Vec<Buff>")
        }
    }
}

impl Buff {
    pub fn new(name : BuffType, effect : f64) -> Self {
        Buff(name, effect)
    }
}

impl fmt::Display for Buff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let a = "normal(BuffName)".to_string();
        let b = BuffType::convert(a).unwrap();
        if let BuffType::Normal(s) = b {
            assert_eq!(s, "BuffName".to_string());
        }
    }

    #[test]
    fn test2() {
        let a = "normal(BuffName))".to_string();
        let b = BuffType::convert(a).unwrap();
        if let BuffType::Normal(s) = b {
            assert_eq!(s, "BuffName)".to_string());
        }
    }
}