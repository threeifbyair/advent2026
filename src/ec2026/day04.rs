use crate::Args;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug)]
struct WarriorResult {
    id: u32,
    quest: u32,
    part: u32,
    time: u32,
}

impl WarriorResult {
    pub fn new(record: &str) -> WarriorResult {
        let mut id = 0;
        let mut quest = 0;
        let mut part = 0;
        let mut time = 0;
        for word in record.split(' ') {
            let (key, value) = word.split_once('=').unwrap();
            let value: u32 = value.parse().unwrap();
            match key {
                "id" => id = value,
                "quest" => quest = value,
                "part" => part = value,
                "time" => time = value,
                _ => {}
            }
        }
        WarriorResult { id, quest, part, time }
    }
}

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut warrior_data: HashMap<u32, (u32, u32)> = HashMap::new();
    let mut per_quest_data: HashMap<(u32, u32), HashMap<u32, u32>> = HashMap::new();

    // First build the list of times.
    for line in data {
        let warrior_result = WarriorResult::new(line);

        let totals = warrior_data.entry(warrior_result.id).or_default();
        totals.0 += 1;
        totals.1 += warrior_result.time;

        if args.verbose {
            println!("Data: {:?}   Total: {:?}", warrior_result, warrior_data[&warrior_result.id]);
        }

        per_quest_data
            .entry((warrior_result.quest, warrior_result.part))
            .or_default()
            .insert(warrior_result.id, warrior_result.time);
    }

    if args.part_three {
        let mut points: HashMap<u32, u32> = HashMap::new();
        for quest in 1..=20 {
            let mut times: HashMap<u32, u32> = HashMap::new();
            for part in 1..=3 {
                let Some(quest_part_times) = per_quest_data.get(&(quest, part)) else {
                    continue;
                };

                // OK, we have the results. Add them to the participants' times.
                let mut participation: HashSet<u32> = HashSet::new();
                for (id, time) in quest_part_times {
                    *times.entry(*id).or_insert(0) += time;
                    participation.insert(*id);
                }

                // Now group participants by time, and sort those groups from fastest to slowest.
                let mut ranking: HashMap<u32, HashSet<u32>> = HashMap::new();
                for (id, time) in &times {
                    ranking.entry(*time).or_default().insert(*id);
                }
                let mut ranking_vec: Vec<(u32, HashSet<u32>)> = ranking.into_iter().collect();
                ranking_vec.sort_unstable_by_key(|(time, _)| *time);

                // And award points!
                let mut points_to_award: i32 = (50 * part) as i32;
                if args.verbose {
                    println!("After quest {quest} part {part}:");
                }
                for (_time, ids) in ranking_vec {
                    let participating_ids: HashSet<u32> = ids.intersection(&participation).copied().collect();
                    let num_ids = participating_ids.len();
                    for id in participating_ids {
                        *points.entry(id).or_insert(0) += points_to_award as u32;
                        if args.verbose {
                            println!("ID: {id}  Time: {_time}  Points: {points_to_award}");
                        }
                    }
                    points_to_award -= num_ids as i32;
                    if points_to_award <= 0 {
                        break;
                    }
                }
            }
        }

        let mut scores_with_frequency: HashMap<u32, u32> = HashMap::new();
        for score in points.values() {
            *scores_with_frequency.entry(*score).or_insert(0) += 1;
        }
        let mut sortable_scores: Vec<(u32, u32)> = scores_with_frequency.into_iter().collect();
        sortable_scores.sort_unstable_by_key(|(score, _)| Reverse(*score));
        if args.verbose {
            println!("Final ranking:\n {sortable_scores:#?}");
        }

        let mut checksum = 0;
        let mut position = 1;
        for (points, frequency) in sortable_scores {
            checksum += points * position * frequency;
            position += frequency;
        }

        Ok(format!("Checksum: {checksum}"))
    } else {
        let mut warrior_entries: Vec<(u32, (u32, u32))> = warrior_data.into_iter().collect();
        warrior_entries.sort_unstable_by_key(|(_, (count, time))| (Reverse(*count), *time));

        if args.verbose {
            println!("Sorted times:\n{warrior_entries:#?}");
        }

        let checksum: usize = warrior_entries
            .iter()
            .enumerate()
            .map(|(i, (_, (_, time)))| (i + 1) * (*time as usize))
            .sum();

        Ok(format!("Checksum: {checksum}"))
    }
}
