use std::collections::HashMap;
use crate::Args;
use std::io;


#[derive(Debug)]
struct WarriorResult {
    id: u32,
    quest: u32,
    part: u32,
    time: u32
}

impl WarriorResult {
    pub fn new(record: &str) -> WarriorResult {
        let words: Vec<&str> = record.split(' ').collect();
        let mut id = 0;
        let mut quest = 0;
        let mut part = 0;
        let mut time = 0;
        for word in words {
            let split: Vec<&str> = word.split('=').collect();
            let value: u32 = split[1].parse().unwrap();
            match split[0] {
                "id" => {id = value}
                "quest" => {quest = value}
                "part" => {part = value}
                "time" => {time = value}
                _ => {}
            }
        }
        WarriorResult { id, quest, part, time }
    }
}


pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut warrior_data: HashMap<u32, (u32, u32)> = HashMap::new();
    
    // First build the list of times.
    for line in data {
        let warrior_result = WarriorResult::new(line);

        let data = warrior_data.entry(warrior_result.id).or_insert((0, 0));

        data.0 += 1;
        data.1 += warrior_result.time;

        if args.verbose {
            println!(
                "Data: {:?}   Total: {:?}", warrior_result, warrior_data[&warrior_result.id]
            );
        }
    }

    let mut warrior_entries: Vec<(u32, (u32, u32))> = warrior_data.into_iter().collect();

    warrior_entries.sort_unstable_by(|a, b| if a.1.0 == b.1.0 {a.1.1.cmp(&b.1.1)} else {b.1.0.cmp(&a.1.0)});

    if args.verbose {
        println!(
            "Sorted times:\n{:#?}", warrior_entries
        )
    }

    let checksum: usize = warrior_entries.iter().enumerate().map(|(i, e)| (i+1)*(e.1.1 as usize)).sum();

    Ok(
        format!("Checksum: {checksum}")
    )
}
