use crate::buildings::{Building, BuildingType};
use std::rc::Rc;
use crate::buff::{BuildingBuff, BuildingBuffType, PolicyBuffType, PolicyBuff, PictureBuffType, PictureBuff, WideBuff};
use std::collections::HashMap;

type Rb = Rc<Building>;
type Vrb = Vec<Rc<BuildingBuff>>;
type ThisError = &'static str;

pub struct BuildingBuffMap (HashMap<BuildingBuffType, Vrb>);

impl BuildingBuffMap {
    pub fn new() -> BuildingBuffMap {
        let mut map = HashMap::new();

        map.insert(BuildingBuffType::Offline,Vec::new());
        map.insert(BuildingBuffType::All,Vec::new());
        map.insert(BuildingBuffType::Online,Vec::new());
        map.insert(BuildingBuffType::Housing,Vec::new());
        map.insert(BuildingBuffType::Industrial,Vec::new());
        map.insert(BuildingBuffType::Commercial,Vec::new());

        BuildingBuffMap(map)
    }

    pub fn add_normal_buff(&mut self, buff : &Rc<BuildingBuff>) -> Result<(), ThisError> {
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

    pub fn add_buff(&mut self, buff : &Rc<BuildingBuff>) -> Result<(), ThisError> {
        match &buff.0 {
            BuildingBuffType::Normal(_) => self.add_normal_buff(buff)?,
            _ => self.0.get_mut(&buff.0).unwrap().push(buff.clone()),
        }
        Ok(())
    }

    pub fn get_normal_effect(&self, name : &str) -> f64 {
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

    pub fn get_industrial_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::Industrial)
    }

    pub fn get_commercial_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::Commercial)
    }

    pub fn get_housing_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::Housing)
    }

    pub fn get_all_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::All)
    }

    pub fn get_online_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::Online)
    }

    pub fn get_offline_effect(&self) -> f64 {
        self.get_wide_effect(&BuildingBuffType::Offline)
    }
}

///```
///pub enum PolicyBuffType {
///    Industrial,
///    Commercial,
///    Housing,
///    All,
///    Online,
///    Offline
///}
///pub struct PolicyBuff(String, PolicyBuffType, f64);
/// ```

#[derive(Clone)]
pub struct PolicyBuffMap(HashMap<PolicyBuffType, Vec<PolicyBuff>>);

impl PolicyBuffMap {
    pub fn new() -> PolicyBuffMap {
        let mut map = HashMap::new();
        map.insert(PolicyBuffType::Industrial, Vec::new());
        map.insert(PolicyBuffType::Commercial, Vec::new());
        map.insert(PolicyBuffType::Housing, Vec::new());
        map.insert(PolicyBuffType::All, Vec::new());
        map.insert(PolicyBuffType::Online, Vec::new());
        map.insert(PolicyBuffType::Offline, Vec::new());

        PolicyBuffMap(map)
    }

    pub fn add_policy(&mut self, policy : PolicyBuff) {
        self.0.get_mut(policy.get_type()).unwrap().push(policy);
    }

    pub fn get_effect(&self, building_type : &BuildingType, online : bool) -> f64 {
        let mut effect = 0.0;

        for p in self.0.get(&PolicyBuffType::All).unwrap() {
            effect += p.get_effect();
        }

        let temp;
        match building_type {
            BuildingType::Industrial => temp = self.0.get(&PolicyBuffType::Industrial).unwrap(),
            BuildingType::Commercial => temp = self.0.get(&PolicyBuffType::Commercial).unwrap(),
            BuildingType::Housing => temp = self.0.get(&PolicyBuffType::Housing).unwrap()
        }

        for p in temp {
            effect += p.get_effect();
        }

        if online {
            for p in self.0.get(&PolicyBuffType::Online).unwrap() {
                effect += p.get_effect();
            }
        } else {
            for p in self.0.get(&PolicyBuffType::Offline).unwrap() {
                effect += p.get_effect();
            }
        }

        effect
    }
}

#[derive(Clone)]
pub struct PictureBuffMap(HashMap<PictureBuffType, Vec<PictureBuff>>);

impl PictureBuffMap {
    pub fn new() -> PictureBuffMap {
        PictureBuffMap(HashMap::new())
    }
}

