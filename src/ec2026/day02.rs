use crate::Args;
use std::io;
use std::collections::{HashMap, HashSet};

fn recursive_find_stream(curstream : &mut Vec<u32>, bigger: &HashMap<u32, HashSet<u32>>, len_needed: usize) -> bool {
    if len_needed == 0 {
        true
    }
    else {
        let last = curstream.last().unwrap();
        let options = bigger.get(last);
        if options.is_some() {
            for num in options.unwrap() {
                curstream.push(*num);
                if recursive_find_stream(curstream, bigger, len_needed - 1) {
                    return true;
                }
                curstream.pop();
            }
    }
        false
    }
}


pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut total: u64 = 0;
    for line in data {
        let groups: Vec<(u32,u32)> = line.split(' ').map(
            |g| if g.contains('<') {
                let pair: Vec<u32> = g.split('<').filter_map(|n| n.parse().ok()).collect();
                (pair[1], pair[0])
            }
            else {
                let pair: Vec<u32> = g.split('>').filter_map(|n| n.parse().ok()).collect();
                (pair[0], pair[1])
            }
        ).collect();

        let mut seen_bigger: HashSet<u32> = HashSet::new();
        let mut bigger: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut seen: HashSet<u32> = HashSet::new();

        if args.verbose {
            println!("Groups: {:?}", &groups);
        }

        for g in groups {
            seen.insert(g.0);
            seen.insert(g.1);
            seen_bigger.insert(g.1);
            bigger.entry(g.0).or_insert_with(HashSet::new).insert(g.1);
        }

        if args.verbose {
            println!("Seen: {:?}", &seen);
            println!("Seen bigger: {:?}", &seen_bigger);
            println!("Relations: {:#?}", &bigger);
        }

        let diff: HashSet<u32> = seen.difference(&seen_bigger).copied().collect();
        // hopefully diff contains exactly one entry -- the highest. Do a depth-first search to find a stream of length seen.len().
        assert_eq!(diff.len(), 1);
        let highest = *diff.iter().next().unwrap();        
        let mut ordered_answer : Vec<u32> = Vec::new();
        ordered_answer.push(highest);
        let stream_ok: bool = recursive_find_stream(&mut ordered_answer, &bigger, seen.len() - 1);
        assert_eq!(stream_ok, true);

        let mut checksum: u64 = 0;
        for v in ordered_answer {
            checksum = checksum * 10 + v as u64;
        }
        if args.verbose {
            println!("Checksum is {checksum}");
        }

        total += checksum;
    }

    Ok(format!("Total: {}", total))
}
