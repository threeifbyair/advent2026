use crate::Args;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct CloudPower {
    id: u32,
    energy: u32,
    connections: Vec<u32>,
}

type PendingEnergies = HashMap<(u32, u32), u32>; // (cloud_id, energy) -> repeat_count

impl CloudPower {
    pub fn new(record: &str) -> CloudPower {
        let mut id = 0;
        let mut energy = 0;
        let mut connections: Vec<u32> = Vec::new();
        for word in record.split(", ") {
            let (key, value) = word.split_once('=').unwrap();
            match key {
                "id" => id = value.parse().unwrap(),
                "energy" => energy = value.parse().unwrap(),
                "connections" => connections = value.split(',').filter_map(|s| s.parse().ok()).collect(),
                _ => {}
            }
        }
        CloudPower { id, energy, connections }
    }
}

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    fn radiate_pulse_breadthfirst(cloud_data: &mut HashMap<u32, CloudPower>, cloud: u32, energy: u32, limit: u32, verbose: bool) {
        let mut this_round: PendingEnergies = HashMap::new();
        this_round.insert((cloud, energy), 1);
        while !this_round.is_empty() {
            let mut next_round: PendingEnergies = HashMap::new();
            for ((cloud_id, this_energy), cloud_repeat) in this_round {
                let Some(connections) = cloud_data.get_mut(&cloud_id).map(|this_cloud| {
                    this_cloud.energy += this_energy * cloud_repeat;
                    if verbose {
                        println!("Cloud {} received {} energy, now has {}, forwarding to neighbors {:#?}", cloud_id, this_energy, this_cloud.energy, this_cloud.connections);
                    }
                    this_cloud.connections.clone()
                }) else {
                    if verbose {
                        println!("Cloud {} not found, skipping", cloud_id);
                    }
                    continue;
                };
                if this_energy > 0 {
                    for connection in connections {
                        *next_round.entry((connection, this_energy - 1)).or_insert(0) += cloud_repeat;
                    }
                }
            }
            if limit > 0 {
                let exploding: Vec<u32> = cloud_data
                    .iter()
                    .filter(|(_, cloud)| cloud.energy >= limit)
                    .map(|(&id, _)| id)
                    .collect();
                for id in exploding {
                    // This cloud just exploded.
                    let exploded_cloud = cloud_data.remove(&id).unwrap();
                    if verbose {
                        println!("Cloud {} exploded with {} energy, forwarding to neighbors {:#?}", exploded_cloud.id, exploded_cloud.energy, exploded_cloud.connections);
                    }
                    for connection in exploded_cloud.connections {
                        *next_round.entry((connection, 10)).or_insert(0) += 1;
                    }
                }
            }
            this_round = next_round;
        }
    }
    
    let mut cloud_data: HashMap<u32, CloudPower> = HashMap::new();

    // First build the list of clouds.
    for line in data {
        let cloud_power = CloudPower::new(line);
        cloud_data.insert(cloud_power.id, cloud_power);
    }

    if args.part_three {
        // For part three, we need to try each cloud.
        let mut max_energy = 0;
        for cloud_id in cloud_data.keys().copied() {
            let mut cloud_data = cloud_data.clone();
            let initial_energy = 10;
            if args.verbose {
                println!("Radiating pulse from cloud {} with initial energy {}", cloud_id, initial_energy);
            }
            radiate_pulse_breadthfirst(&mut cloud_data, cloud_id, initial_energy, if args.argint > 0 { args.argint as u32 } else { 1000000 }, args.verbose);
            let total_energy: u32 = cloud_data.values().map(|x| x.energy).sum();
            if args.verbose {
                println!("Total energy after radiating from cloud {}: {}\n\n", cloud_id, total_energy);
            }
            max_energy = max_energy.max(total_energy);
        }
        Ok(format!("Highest total energy in all clouds: {}", max_energy))

    }
    else {
        // Now find the most-connected cloud.
        let most_connected = cloud_data.values().max_by_key(|x| x.connections.len()).unwrap();

        // And radiate a pulse from it.
        let id = most_connected.id;
        let initial_energy = if args.part_two { 10 } else { 5 };
        radiate_pulse_breadthfirst(&mut cloud_data, id, initial_energy, 0, args.verbose);

        // And finally, total the energy in all clouds.
        let total_energy: u32 = cloud_data.values().map(|x| x.energy).sum();
        if args.verbose {
            println!("All cloud data:\n{:#?}", cloud_data);
        }

        Ok(format!("Total energy in all clouds: {}", total_energy))
    }
}
