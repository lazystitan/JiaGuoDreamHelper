use std::fmt;
use serde_json::Value;
use crate::convert::Convert;
use std::rc::Rc;

#[derive(Eq, PartialEq, Hash)]
pub enum BuildingBuffType {
    Normal(String),
    Industrial,
    Commercial,
    Housing,
    All,
    Online,
    Offline
}

impl fmt::Display for BuildingBuffType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildingBuffType::Normal(s) => write!(f, "Normal ( {} )", s),
            BuildingBuffType::Offline =>  write!(f, "offline"),
            BuildingBuffType::Online => write!(f, "online"),
            BuildingBuffType::All => write!(f, "all"),
            BuildingBuffType::Industrial => write!(f, "industrial"),
            BuildingBuffType::Commercial => write!(f, "commercial"),
            BuildingBuffType::Housing => write!(f, "housing"),
        }
    }
}

impl Convert<String> for BuildingBuffType {
    type Error = &'static str;

    fn convert(value: String) -> Result<Self, Self::Error> {
        if value == "industrial" {
            Ok(BuildingBuffType::Industrial)
        } else if value == "commercial" {
            Ok(BuildingBuffType::Commercial)
        } else if value == "housing" {
            Ok(BuildingBuffType::Housing)
        } else if value == "all" {
            Ok(BuildingBuffType::All)
        } else if value == "online" {
            Ok(BuildingBuffType::Online)
        } else if value == "offline" {
            Ok(BuildingBuffType::Offline)
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
                return Ok(BuildingBuffType::Normal(part2[0 .. part2.len() - 1].to_string()))

            } else {
                return Err("Cannot convert to BuffType");
            }

        }
    }
}

pub struct BuildingBuff(pub BuildingBuffType, pub f64);



impl Convert<Value> for Vec<Rc<BuildingBuff>> {
    type Error = &'static str;

    fn convert(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(map) => {
                let mut result = Vec::with_capacity(map.len());
                for (key , item) in map {
                    result.push(Rc::new(BuildingBuff::new(BuildingBuffType::convert(key)?, f64::convert(item)?)));
                }
                Ok(result)
            },
            _ => Err("Cannot cast to Vec<Buff>")
        }
    }
}

impl BuildingBuff {
    pub fn new(name : BuildingBuffType, effect : f64) -> Self {
        BuildingBuff(name, effect)
    }
}

impl fmt::Display for BuildingBuff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test1() {
        let a = BuildingBuffType::Commercial;
        let b = BuildingBuffType::Commercial;
        assert!(a == b);
    }

    #[test]
    fn test2() {
        let a = BuildingBuffType::Commercial;
        let b = BuildingBuffType::Industrial;
        assert!(!(a == b))
    }

    #[test]
    fn test3() {
        let a = BuildingBuffType::Normal("123".to_string());
        let b = BuildingBuffType::Industrial;
        assert!(!(a == b))
    }

    #[test]
    fn test4() {
        let a = BuildingBuffType::Normal("123".to_string());
        let b = BuildingBuffType::Normal("123".to_string());
        assert!(a == b)
    }

    #[test]
    fn test5() {
        let a = BuildingBuffType::Normal("321".to_string());
        let b = BuildingBuffType::Normal("123".to_string());
        assert!(a != b)
    }

    #[test]
    fn test6() {
        let mut m = HashMap::new();
        let a = BuildingBuffType::Industrial;
        m.insert(a, vec![12,3,4]);
        let b = BuildingBuffType::Normal("Last".to_string());
        m.insert(b, vec![2,34]);
        match m.get_mut(&BuildingBuffType::Normal("Last".to_string())) {
            Some(v) => v.push(4),
            None => (),
        }
    }
}