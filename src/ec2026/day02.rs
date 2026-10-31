use crate::Args;
use std::io;
use std::collections::{HashMap, HashSet};

fn recursive_find_stream(curstream: &mut Vec<u32>, bigger: &HashMap<u32, HashSet<u32>>, len_needed: usize) -> bool {
    if len_needed == 0 {
        return true;
    }

    let last = *curstream.last().unwrap();
    let Some(options) = bigger.get(&last) else {
        return false;
    };

    for &num in options {
        curstream.push(num);
        if recursive_find_stream(curstream, bigger, len_needed - 1) {
            return true;
        }
        curstream.pop();
    }

    false
}


pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut total: u64 = 0;
    for line in data {
        let groups: Vec<(u32, u32)> = line
            .split(' ')
            .map(|g| {
                let (sep, swapped) = if g.contains('<') { ('<', true) } else { ('>', false) };
                let pair: Vec<u32> = g.split(sep).filter_map(|n| n.parse().ok()).collect();
                if swapped { (pair[1], pair[0]) } else { (pair[0], pair[1]) }
            })
            .collect();

        let mut seen_bigger: HashSet<u32> = HashSet::new();
        let mut bigger: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut seen: HashSet<u32> = HashSet::new();

        if args.verbose {
            println!("Groups: {groups:?}");
        }

        for (smaller, larger) in &groups {
            seen.insert(*smaller);
            seen.insert(*larger);
            seen_bigger.insert(*larger);
            bigger.entry(*smaller).or_default().insert(*larger);
        }

        if args.verbose {
            println!("Seen: {seen:?}");
            println!("Seen bigger: {seen_bigger:?}");
            println!("Relations: {bigger:#?}");
        }

        // The one value never seen as "bigger" is the highest in the chain; do a depth-first
        // search from there to find the full stream of length seen.len().
        let diff: HashSet<u32> = seen.difference(&seen_bigger).copied().collect();
        assert_eq!(diff.len(), 1);
        let highest = *diff.iter().next().unwrap();
        let mut ordered_answer: Vec<u32> = vec![highest];
        let stream_ok = recursive_find_stream(&mut ordered_answer, &bigger, seen.len() - 1);
        assert!(stream_ok);

        let checksum: u64 = ordered_answer.into_iter().fold(0u64, |acc, v| acc * 10 + v as u64);
        if args.verbose {
            println!("Checksum is {checksum}");
        }

        total += checksum;
    }

    Ok(format!("Total: {total}"))
}
