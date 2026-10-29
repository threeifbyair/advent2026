use crate::Args;
use std::io;
use std::collections::HashMap;

fn calc_vote(bases:&Vec<u32>, value: &Vec<u32>) -> u64 {
    
    let mut vote: u64 = 0;
    let numlen = bases.len();
    for (i, e) in value.iter().enumerate() {
        let mut subvote:u64 = *e as u64;
        for _ in 0..i {
            subvote *= bases[i % numlen] as u64;
        }
        vote += subvote;
    }
    vote
}


fn find_lowest(bases: &Vec<u32>, value: u64) -> Vec<u32> {
    // This is kinda horrible but we can get away with it
    let mut lowest_map: HashMap<u64, Vec<u32>> = HashMap::new();
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    for e in 0..10 {
                        for f in 0..10 {
                            let val:Vec<u32> = vec![f,e,d,c,b,a];
                            let vote = calc_vote(bases, &val);
                            if !lowest_map.contains_key(&vote) {
                                lowest_map.insert(vote, val);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut result = <Vec<u32> as Clone>::clone(&lowest_map[&value]);
    while result.last() == Some(&0) {
        result.pop();
    }
    result
}



pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut total: u64 = 0;
    for line in data {
        let iso2: bool = line.chars().nth(3) == Some('2');
        let numbers: Vec<Vec<u32>> = line[(if iso2 {5} else {4})..].split(' ').map(
            |s| s.chars().rev().map(|u| u.to_digit(10).unwrap()).collect()
        ).collect();

        if args.verbose {
            println!("Numbers: {:?}", &numbers);
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
            }
            else {
                if args.verbose {
                    println!("Value {:?}, lowest {:?}, does not match", numbers[1], lowest);
                }
            }
        }
        else {
            total += vote;
        }
    }

    Ok(format!("Total: {}", total))
}
