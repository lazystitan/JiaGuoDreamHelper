use crate::buff_map::{PolicyBuffMap, PictureBuffMap};
use crate::buildings::{Building, BuildingType};
use crate::global::Global;
use std::rc::Rc;
use crate::buff::PolicyBuff;

type Rb = Rc<Building>;

pub struct Manager {
    policy_buff_map : PolicyBuffMap,
    picture_buff_map : PictureBuffMap,
    industrial_buildings : Vec<Rb>,
    commercial_buildings : Vec<Rb>,
    housing_buildings : Vec<Rb>,
    globals : Vec<Global>
}

impl Manager {
    pub fn new() -> Manager{
        Manager {
            policy_buff_map : PolicyBuffMap::new(),
            picture_buff_map : PictureBuffMap::new(),
            industrial_buildings : Vec::new(),
            commercial_buildings : Vec::new(),
            housing_buildings : Vec::new(),
            globals : Vec::new()
        }
    }

    pub fn add_building(&mut self, building : Building) {
        match building.get_type() {
            BuildingType::Industrial => self.industrial_buildings.push(Rc::new(building)),
            BuildingType::Commercial => self.commercial_buildings.push(Rc::new(building)),
            BuildingType::Housing => self.housing_buildings.push(Rc::new(building)),
        }
    }

    pub fn add_policy_buff(&mut self, buff : PolicyBuff) {
        self.policy_buff_map.add_policy(buff);
    }

    pub fn generate_globals(&mut self) {
        self.globals.clear();
        let all = self.generate_vector();
        for v in all {
            let mut g = Global::new();
            g.add_multiple_policy_buff(self.policy_buff_map.clone());
//            g.add_multiple_picture_buff(self.picture_buff_map.clone());
            for b in v {
                g.add_building(b);
            }
            self.globals.push(g);
        }
    }

    fn select_three(vec : &Vec<Rb>) -> Vec<Vec<Rb>> {
        let mut result = Vec::new();
        for i in 0..(vec.len()-2) {
            for j in (i+1)..(vec.len()-1) {
                for k in (j+1)..vec.len() {
                    result.push(Self::collect_three(vec, i, j, k));
                }
            }
        }
        result
    }

    fn collect_three(vec : &Vec<Rb>, i : usize, j : usize, k : usize) -> Vec<Rb> {
        let mut result = Vec::with_capacity(3);
        result.push(vec[i].clone());
        result.push(vec[j].clone());
        result.push(vec[k].clone());
        result
    }

    fn generate_vector(&self) -> Vec<Vec<Rb>> {
        let mut result = Vec::new();
        let mut industrial = Self::select_three(&self.industrial_buildings);
        let mut commercial = Self::select_three(&self.commercial_buildings);
        let mut housing = Self::select_three(&self.housing_buildings);
        for i in 0..industrial.len() {
            for j in 0..commercial.len() {
                for k in 0..housing.len() {
                    let mut temp = Vec::with_capacity(9);
                    temp.append(&mut industrial[i]);
                    temp.append(&mut commercial[j]);
                    temp.append(&mut housing[k]);
                    result.push(temp)
                }
            }
        }

        result
    }

    pub fn get_max_online_income_global(&mut self) -> &Global {
        let mut max = 0.0;
        let mut result = 0;
        for (index, g) in self.globals.iter_mut().enumerate() {
            let temp = g.get_online_income();
            if temp > max {
                result = index;
            }
        }

        &self.globals[result]
    }

    pub fn get_globals(&self) -> &Vec<Global> {
        &self.globals
    }
}