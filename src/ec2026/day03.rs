use crate::Args;
use std::collections::HashMap;
use std::io;

fn calc_vote(bases: &[u32], value: &[u32]) -> u64 {
    value
        .iter()
        .enumerate()
        .map(|(i, &digit)| {
            let base = bases[i % bases.len()] as u64;
            digit as u64 * base.pow(i as u32)
        })
        .sum()
}

fn find_lowest(bases: &[u32], value: u64) -> Vec<u32> {
    // Brute-force every 6-digit combination and keep the first (lowest) one mapping to each vote.
    let mut lowest_map: HashMap<u64, Vec<u32>> = HashMap::new();
    for num in 0..1_000_000u32 {
        let mut n = num;
        let mut val = [0u32; 6];
        for digit in &mut val {
            *digit = n % 10;
            n /= 10;
        }
        lowest_map.entry(calc_vote(bases, &val)).or_insert_with(|| val.to_vec());
    }

    let mut result = lowest_map.remove(&value).unwrap();
    while result.last() == Some(&0) {
        result.pop();
    }
    result
}

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut total: u64 = 0;
    for line in data {
        let iso2 = line.chars().nth(3) == Some('2');
        let numbers: Vec<Vec<u32>> = line[(if iso2 { 5 } else { 4 })..]
            .split(' ')
            .map(|s| s.chars().rev().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        if args.verbose {
            println!("Numbers: {numbers:?}");
        }

        let vote = calc_vote(&numbers[0], &numbers[1]);

        if args.verbose {
            println!("Vote is {vote}");
        }

        if iso2 {
            let lowest = find_lowest(&numbers[0], vote);
            if lowest == numbers[1] {
                total += vote;
                if args.verbose {
                    println!("Matches lowest and counts");
                }
            } else if args.verbose {
                println!("Value {:?}, lowest {lowest:?}, does not match", numbers[1]);
            }
        } else {
            total += vote;
        }
    }

    Ok(format!("Total: {total}"))
}
