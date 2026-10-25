use crate::Args;
use std::io;

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut totals: Vec<i32> = Vec::new();
    let mut current = 0;
    for line in data {
        if line.is_empty() {
            totals.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    if current > 0 {
        totals.push(current);
    }

    totals.sort_unstable_by(|a, b| b.cmp(a));

    Ok(if args.part_two {
        format!("Top 3: {}", totals[0] + totals[1] + totals[2])
    } else {
        format!("Max: {}", totals[0])
    })
}
