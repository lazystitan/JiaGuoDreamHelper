use std::collections::HashMap;
use crate::global_buff::GlobalBuff;
use crate::buildings::{Building, BuildingTypes};
use std::rc::Rc;

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
        match self.buildings_map.insert(name.clone(), r.clone()) {
            Some(_) => return Err("Already exist a building has same name"),
            None => ()
        }
        let temp;
        match r.get_type() {
            BuildingTypes::Industrial => temp = &mut self.industrial_map,
            BuildingTypes::Commercial => temp = &mut self.commercial_map,
            BuildingTypes::Housing => temp = &mut self.housing_map,
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


}