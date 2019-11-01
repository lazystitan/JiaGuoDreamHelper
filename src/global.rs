use std::collections::HashMap;
use std::rc::Rc;

use crate::buildings::{Building, BuildingType};
use crate::buff::{BuildingBuffType, PolicyBuffType, PolicyBuff, BuildingBuff};

type Rb = Rc<Building>;
type Vrb = Vec<Rc<BuildingBuff>>;
type ThisError = &'static str;

struct BuildingBuffMap (HashMap<BuildingBuffType, Vrb>);

impl BuildingBuffMap {
    fn new() -> BuildingBuffMap {
        let mut map = HashMap::new();

        map.insert(BuildingBuffType::Offline,Vec::new());
        map.insert(BuildingBuffType::All,Vec::new());
        map.insert(BuildingBuffType::Online,Vec::new());
        map.insert(BuildingBuffType::Housing,Vec::new());
        map.insert(BuildingBuffType::Industrial,Vec::new());
        map.insert(BuildingBuffType::Commercial,Vec::new());

        BuildingBuffMap(map)
    }

    fn add_normal_buff(&mut self, buff : &Rc<BuildingBuff>) -> Result<(), ThisError> {
        match self.0.get_mut(&buff.0) {
            Some(v) => v.push(buff.clone()),
            None => {
                if let BuildingBuffType::Normal(s) = &buff.0 {
                    self.0.insert(BuildingBuffType::Normal(s.clone()), vec![buff.clone()]);
                } else {
                    return Err("buff type is not satisfied");
                }
            }
        }

        Ok(())
    }

    fn add_buff(&mut self, buff : &Rc<BuildingBuff>) -> Result<(), ThisError> {
        match &buff.0 {
            BuildingBuffType::Normal(_) => self.add_normal_buff(buff)?,
            _ => self.0.get_mut(&buff.0).unwrap().push(buff.clone()),
        }
        Ok(())
    }

    fn get_normal_effect(&self, name : &str) -> f64 {
        let mut result = 0.0;
        match self.0.get(&BuildingBuffType::Normal(name.to_string())) {
            Some(v) => {
                for b in v {
                    result += b.1
                }
                result
            }

            None => result
        }
    }

//    pub struct BuildingBuff(pub BuildingBuffType, pub f64);

    fn get_wide_effect(&self, building_buff_type : &BuildingBuffType) -> f64 {
        let mut result = 0.0;
        for b in self.0.get(building_buff_type).unwrap() {
            result += b.1;
        }
        result
    }

    fn get_industrial_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::Industrial)
    }

    fn get_commercial_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::Commercial)
    }

    fn get_housing_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::Housing)
    }

    fn get_all_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::All)
    }

    fn get_online_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::Online)
    }

    fn get_offline_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::Offline)
    }
}

pub struct Global {
    policy_buff_map : HashMap<String, PolicyBuff>,
    building_buff_map : BuildingBuffMap,
    buildings_map : HashMap<String, Rb>,
    industrial_map: HashMap<String, Rb>,
    commercial_map : HashMap<String, Rb>,
    housing_map : HashMap<String, Rb>
}

impl Global {

    pub fn new() -> Global {
        Global {
            policy_buff_map : HashMap::new(),
            building_buff_map : BuildingBuffMap::new(),
            buildings_map : HashMap::new(),
            industrial_map: HashMap::new(),
            commercial_map: HashMap::new(),
            housing_map: HashMap::new()
        }
    }

    pub fn is_full(&self) -> bool {
        if self.buildings_map.len() == 9 {
            true
        } else {
            false
        }
    }

    fn add_building_buff(building : Rc<Building>, map : &mut BuildingBuffMap ) -> Result<(), ThisError> {
        for r in building.get_buff() {
            map.add_buff(r)?;
        }
        Ok(())
    }

//    pub struct Building {
//      name : String,
//      bd_type : BuildingType,
//      level : u32,
//      star : u32,
//      income: f64,
//      buff : Vec<Rc<BuildingBuff>> *
//    }

    pub fn add_building(&mut self, building : Building) -> Result<(), ThisError> {
        let name = building.get_name().to_string();
        let r = Rc::new(building);

        if self.buildings_map.len() == 9 {
            return Err("Not enough space");
        }

        match self.buildings_map.insert(name.clone(), r.clone()) {
            Some(_) => return Err("Already exist a building has same name"),
            None => ()
        }
        let temp;
        match r.get_type() {
            BuildingType::Industrial => temp = &mut self.industrial_map,
            BuildingType::Commercial => temp = &mut self.commercial_map,
            BuildingType::Housing => temp = &mut self.housing_map,
        }

        if temp.len() >= 3 {
            Err("Not enough space")
        } else {
            temp.insert(name.clone(), r.clone());
            Self::add_building_buff(r.clone(), &mut self.building_buff_map)?;
            Ok(())
        }
    }

    pub fn add_global_buff(&mut self, buff : PolicyBuff) -> Result<(), ThisError> {
        let name = buff.get_name().to_string();
        match self.policy_buff_map.insert(name, buff) {
            Some(_) => Err("Already exist a global buff has same name"),
            None => Ok(())
        }
    }

    pub fn get_building_names(&self) -> Vec<String>{
        self.buildings_map.keys().map(|x| x.clone() ).collect()
    }

    fn get_class_income(map : &HashMap<String, Rb>, effect : f64) -> f64 {
        let mut sum = 0.0;
        for (_ , b_) in map {
            sum = sum + b_.get_income();
        }
        sum * effect
    }

    fn cal_building_effect(building : &Building, map : &BuildingBuffMap, online : bool) -> f64 {
        let mut result = 1.0;
        let name = building.get_name();

        match building.get_type() {
            BuildingType::Industrial => result += map.get_industrial_effect(),
            BuildingType::Housing => result += map.get_housing_effect(),
            BuildingType::Commercial => result += map.get_commercial_effect()
        }

        result += map.get_all_effect();
        result += map.get_normal_effect(name);
        if online {
            result += map.get_online_effect();
        } else {
            result += map.get_offline_effect();
        }

        result
    }



    pub fn get_online_income(&self) -> f64 {
        let mut result = 0.0;

        for (_ , b) in &self.buildings_map {
            let effect = 1.0 + Self::cal_building_effect(b.as_ref(), &self.building_buff_map, true);
            result = result + b.get_income() * effect;

        }

        for (_, gb) in &self.policy_buff_map {
            result += match gb.get_type() {
                PolicyBuffType::Industrial => Self::get_class_income(&self.industrial_map, gb.get_effect()),
                PolicyBuffType::Commercial => Self::get_class_income(&self.commercial_map, gb.get_effect()),
                PolicyBuffType::Housing => Self::get_class_income(&self.housing_map, gb.get_effect()),
                PolicyBuffType::All => Self::get_class_income(&self.buildings_map, gb.get_effect()),
                PolicyBuffType::Online => Self::get_class_income(&self.buildings_map, gb.get_effect()),
                PolicyBuffType::Offline => 0.0,
            };
        }

        result
    }


}