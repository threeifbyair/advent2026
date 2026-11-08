use crate::Args;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct PlantVariety {
    id: u32,
    growth: u64,
    limit: u64,
}

impl PlantVariety {
    pub fn new(record: &str) -> PlantVariety {
        let mut id = 0;
        let mut growth = 0;
        let mut limit = 0;
        for word in record.split(", ") {
            let (key, value) = word.split_once('=').unwrap();
            match key {
                "id" => id = value.parse().unwrap(),
                "growth" => growth = value.parse().unwrap(),
                "limit" => limit = value.parse().unwrap(),
                _ => {}
            }
        }
        PlantVariety { id, growth, limit }
    }
}

#[derive(Debug, Clone)]
struct GrowingPlant {
    id: u32,
    growth: u64,
    limit: u64,
    branches: HashMap<u64, u64>,
    branching_points: u64,
}

impl GrowingPlant {
    pub fn new(variety: &PlantVariety) -> GrowingPlant {
        let id = variety.id;
        let growth = variety.growth;
        let limit = variety.limit;
        let branches: HashMap<u64,u64> = HashMap::new();
        let branching_points: u64 = 0;
        GrowingPlant { id, growth, limit, branches, branching_points }
    }

    pub fn grow(self: &mut Self, fast_forward: bool) -> u64 {
        if self.branches.len() == 0 {
            self.branches = HashMap::new();
            self.branches.insert(1,1);
            1
        }
        else {
            let roll_time: u64 = if fast_forward {
                let biggest_branch = self.branches.keys().max().unwrap();
                self.growth - biggest_branch
            }
            else {
                1
            };
            let mut new_branches: HashMap<u64, u64> = HashMap::new();
            for (branch_length, repetition) in self.branches.clone() {
                if branch_length == self.growth - roll_time {
                    let repeat = new_branches.entry(self.growth / 2).or_insert(0);
                    *repeat += 2 * repetition;
                    let tiny_branch = new_branches.entry(1).or_insert(0);
                    *tiny_branch += repetition;
                    self.branching_points += repetition;
                }
                else {
                    let repeat = new_branches.entry(branch_length + roll_time).or_insert(0);
                    *repeat += repetition;
                }
            }
            self.branches = new_branches;
            roll_time
        }
    }

    pub fn size(self: &Self) -> u64 {
        if self.branches.len() > 0 {
            self.branches.iter().map(|(k,v)| k*v).sum::<u64>() + (self.branching_points as u64)
        }
        else {
            0
        }
    }
}

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    
    let mut plant_data: HashMap<u32, PlantVariety> = HashMap::new();

    // First build the list of clouds.
    for line in data {
        let plant_variety = PlantVariety::new(line);
        plant_data.insert(plant_variety.id, plant_variety);
    }

    let mut sum_of_times: u64 = 0;

    for plant_variety in plant_data.values() {
        let mut growing_plant = GrowingPlant::new(plant_variety);
        let mut time: u64 = 0;
        loop {
            time += growing_plant.grow(args.part_three);
            if args.verbose {
                println!("After {time} seconds, plant {} has size {}, branches {:?}, leaves {}", growing_plant.id, growing_plant.size(), growing_plant.branches, growing_plant.branching_points + 1);
            }
            let decision: u64 = if args.part_two || args.part_three { growing_plant.branching_points as u64 + 1 } else { growing_plant.size() };
            if decision >= growing_plant.limit {
                break;
            }
        }
        if args.verbose {
            println!("Plant {} reached its target size in {time} seconds", growing_plant.id);
        }
        sum_of_times += time as u64;
    }

    Ok(format!("Sum of times: {sum_of_times}"))
}
