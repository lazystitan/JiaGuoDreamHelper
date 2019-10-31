use std::collections::HashMap;
use std::rc::Rc;

use crate::buildings::{Building, BuildingType};
use crate::buff::{BuildingBuffType, PolicyBuffType, PolicyBuff, BuildingBuff};

type Rbd = Rc<Building>;

pub struct Global {
    global_buff_map : HashMap<String, PolicyBuff>,
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

    pub fn add_global_buff(&mut self, buff : PolicyBuff) -> Result<(), &'static str> {
        let name = buff.get_name().to_string();
        match self.global_buff_map.insert(name, buff) {
            Some(_) => Err("Already exist a global buff has same name"),
            None => Ok(())
        }
    }

    pub fn get_building_names(&self) -> Vec<String>{
        self.buildings_map.keys().map(|x| x.clone() ).collect()
    }

    fn get_class_income(map : &HashMap<String, Rbd>, effect : f64) -> f64 {
        let mut sum = 0.0;
        for (_ , b_) in map {
            sum = sum + b_.get_income();
        }
        sum * effect
    }

    pub fn get_online_income(&self) -> f64 {
        let mut result = 0.0;

        for (_ , b) in &self.buildings_map {
            result = result + b.get_income();
            for BuildingBuff(buff_type, effect) in b.get_buff() {
                result +=  match buff_type {
                    BuildingBuffType::Industrial => Self::get_class_income(&self.industrial_map, *effect),
                    BuildingBuffType::Commercial => Self::get_class_income(&self.commercial_map, *effect),
                    BuildingBuffType::Housing => Self::get_class_income(&self.housing_map, *effect),
                    BuildingBuffType::All => Self::get_class_income(&self.buildings_map, *effect),
                    BuildingBuffType::Online => Self::get_class_income(&self.buildings_map, *effect),
                    BuildingBuffType::Offline => 0.0,
                    BuildingBuffType::Normal(name) => {
                        match self.buildings_map.get(name) {
                            Some(b_) => {
                                b_.get_income() * *effect
                            },
                            None => 0.0
                        }
                    }
                };
            }
        }

        for (_, gb) in &self.global_buff_map {
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