use std::collections::HashMap;

enum BuildingType {Commercial, Industrial, Living}

struct Building {
    pub name : String,
    pub b_type :  BuildingType,
    pub revenue : u64,
    pub buff : Vec<(String, u64)>,
}

type Zone = [Option<Building>; 3];


struct Global<'a> {
    industrial_buildings : Zone,
    commercial_buildings : Zone,
    living_buildings : Zone,
    buildings_map : HashMap<String, &'a Building>,
    total_revenue : u64
}

impl<'a> Global<'a> {
    fn new() -> Global<'a> {
        Global {
            industrial_buildings : [None ; 3],
            commercial_buildings : [None ; 3],
            living_buildings : [None ; 3],
            buildings_map : HashMap::new(),
            total_revenue : 0
        }
    }

    fn add_building(&mut self, building : Building) -> Result<(), ()> {
        let mut ready;
        match building.b_type {
            BuildingType::Industrial => ready = &mut self.industrial_buildings,
            BuildingType::Commercial => ready = &mut self.commercial_buildings,
            BuildingType::Living => ready = &mut self.living_buildings,
        }

        self.buildings_map.insert(building.name.clone(), &building);

        for b in ready.iter_mut() {
            if let None = Building {
                b.replace(building);
                break;
                return Ok(());
            }
        }

        Err(())
    }

    fn delete_building(&mut self, name : &String) -> Result<(), ()> {
        match self.buildings_map.get(name) {
            Some(b) => {
                let mut ready;
                match b.b_type {
                    BuildingType::Industrial => ready = &mut self.industrial_buildings,
                    BuildingType::Commercial => ready = &mut self.commercial_buildings,
                    BuildingType::Living => ready = &mut self.living_buildings,
                }

                self.buildings_map.remove(name);

                for b in ready.iter_mut() {
                    if let Some(temp) = b{
                        if temp.name == *name {
                            b.take();
                            break;
                        }
                    }
                }

                Ok(())
            },
            None => Err(())
        }
    }

    fn total_revenue(&self) -> u64 {
        let temp = 0;
        for (this_name, building) in self.buildings_map.iter() {
            temp += building.revenue;
            for (name, n) in &building.buff {
                temp += self.buildings_map.get(name).unwrap().revenue * (*n);
            }
        }
        temp
    }

    fn get_names(&self) -> [String ; 9]{
        let mut result = ["".to_string(); 9];
        let mut i = 0;
        for (name, building) in &self.buildings_map {
            result[i] = name.clone();
            i = i + 1;
        }
        result
    }
}


fn main() {
    let a : Option<i32> = None;
    if let None = a {
        println!("true");
    }
    let a = [1 ; 3];
    for i in a.iter() {
        println!("{}", i);
    }

}
