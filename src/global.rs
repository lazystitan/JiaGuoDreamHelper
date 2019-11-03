use std::collections::HashMap;
use std::rc::Rc;

use crate::buildings::{Building, BuildingType};
use crate::buff::{BuildingBuffType, PolicyBuffType, PolicyBuff, BuildingBuff};
use std::hash::Hash;
use crate::buff_map::{PolicyBuffMap, BuildingBuffMap};

type Rb = Rc<Building>;
type ThisError = &'static str;

pub struct Global {
    policy_buff_map : PolicyBuffMap,
    building_buff_map : BuildingBuffMap,
    buildings_map : HashMap<String, Rb>,
    industrial_map: HashMap<String, Rb>,
    commercial_map : HashMap<String, Rb>,
    housing_map : HashMap<String, Rb>
}

impl Global {

    pub fn new() -> Global {
        Global {
            policy_buff_map : PolicyBuffMap::new(),
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

    pub fn add_building(&mut self, building : Rb) -> Result<(), ThisError> {
        let name = building.get_name().to_string();

        if self.buildings_map.len() == 9 {
            return Err("Not enough space");
        }

        match self.buildings_map.insert(name.clone(), building.clone()) {
            Some(_) => return Err("Already exist a building has same name"),
            None => ()
        }
        let temp;
        match building.get_type() {
            BuildingType::Industrial => temp = &mut self.industrial_map,
            BuildingType::Commercial => temp = &mut self.commercial_map,
            BuildingType::Housing => temp = &mut self.housing_map,
        }

        if temp.len() >= 3 {
            Err("Not enough space")
        } else {
            temp.insert(name.clone(), building.clone());
            Self::add_building_buff(building.clone(), &mut self.building_buff_map)?;
            Ok(())
        }
    }

    pub fn add_policy_buff(&mut self, buff : PolicyBuff) {
        self.policy_buff_map.add_policy(buff);
    }

    pub fn add_multiple_policy_buff(&mut self, map : PolicyBuffMap) {
        self.policy_buff_map = map;
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

    fn cal_policy_effect(builidng : &Building, map : &PolicyBuffMap, online : bool) -> f64 {
        map.get_effect(builidng.get_type(), online)
    }

    pub fn get_online_income(&self) -> f64 {
        let mut result = 0.0;
        let mut effect = 1.0;

        //单个建筑收到的建筑buff后的收入
        for (_ , b) in &self.buildings_map {
            effect = 1.0;
            effect *= 1.0 + Self::cal_building_effect(b.as_ref(), &self.building_buff_map, true);
            effect *= 1.0 + Self::cal_policy_effect(b.as_ref(), &self.policy_buff_map, true);
//            effect *= 1.0 + Self::cal_picture_effect(b.as_ref(), &self.picture_buff_map, true);
            result += b.get_income() * effect;
        }

        result
    }

    pub fn get_offline_income(&self) -> f64 {
        let mut result = 0.0;
        let mut effect = 1.0;
        let offline = false;

        //单个建筑收到的建筑buff后的收入
        for (_ , b) in &self.buildings_map {
            effect = 1.0;
            effect *= 1.0 + Self::cal_building_effect(b.as_ref(), &self.building_buff_map,  offline) * 0.5;
            effect *= 1.0 + Self::cal_policy_effect(b.as_ref(), &self.policy_buff_map,  offline) * 0.5;
//            effect *= 1.0 + Self::cal_picture_effect(b.as_ref(), &self.picture_buff_map, offline) * 0.5;
            result += b.get_income() * effect;
        }

        result
    }

}