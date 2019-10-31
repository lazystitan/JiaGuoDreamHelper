use std::collections::HashMap;
use std::rc::Rc;

use super::GlobalBuff;
use crate::buildings::{Building, BuildingType};
use crate::buff::{Buff, BuffType};
use crate::global_buff::GlobalBuffType;

type Rbd = Rc<Building>;

pub struct Global {
    global_buff_map : HashMap<String, GlobalBuff>,
    buildings_map : HashMap<String, Rbd>,
    industrial_map: HashMap<String, Rbd>,
    commercial_map : HashMap<String, Rbd>,
    housing_map : HashMap<String, Rbd>
}

impl Global {

    pub fn new() -> Global {
        Global {
            global_buff_map: HashMap::new(),
            buildings_map: HashMap::new(),
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

    pub fn add_building(&mut self, building : Building) -> Result<(), &'static str> {
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
            Ok(())
        }
    }

    pub fn add_global_buff(&mut self, buff : GlobalBuff) -> Result<(), &'static str> {
        let name = buff.get_name().to_string();
        match self.global_buff_map.insert(name, buff) {
            Some(_) => Err("Already exist a global buff has same name"),
            None => Ok(())
        }
    }

    pub fn get_building_names(&self) -> Vec<String>{
        self.buildings_map.keys().map(|x| x.clone() ).collect()
    }

    fn get_class_revenue(map : &HashMap<String, Rbd>, effect : f64) -> f64 {
        let mut sum = 0.0;
        for (_ , b_) in map {
            sum = sum + b_.get_revenue();
        }
        sum * effect
    }

    pub fn get_online_revenue(&self) -> f64 {
        let mut result = 0.0;

        for (_ , b) in &self.buildings_map {
            result = result + b.get_revenue();
            for Buff(buff_type, effect) in b.get_buff() {
                result +=  match buff_type {
                    BuffType::Industrial => Self::get_class_revenue(&self.industrial_map, *effect),
                    BuffType::Commercial => Self::get_class_revenue(&self.commercial_map, *effect),
                    BuffType::Housing => Self::get_class_revenue(&self.housing_map, *effect),
                    BuffType::All => Self::get_class_revenue(&self.buildings_map, *effect),
                    BuffType::Online => Self::get_class_revenue(&self.buildings_map, *effect),
                    BuffType::Offline => 0.0,
                    BuffType::Normal(name) => {
                        match self.buildings_map.get(name) {
                            Some(b_) => {
                                b_.get_revenue() * *effect
                            },
                            None => 0.0
                        }
                    }
                };
            }
        }

        for (_, gb) in &self.global_buff_map {
            result += match gb.get_type() {
                GlobalBuffType::Industrial => Self::get_class_revenue(&self.industrial_map, gb.get_effect()),
                GlobalBuffType::Commercial => Self::get_class_revenue(&self.commercial_map, gb.get_effect()),
                GlobalBuffType::Housing => Self::get_class_revenue(&self.housing_map, gb.get_effect()),
                GlobalBuffType::All => Self::get_class_revenue(&self.buildings_map, gb.get_effect()),
                GlobalBuffType::Online => Self::get_class_revenue(&self.buildings_map, gb.get_effect()),
                GlobalBuffType::Offline => 0.0,
            };
        }

        result
    }


}