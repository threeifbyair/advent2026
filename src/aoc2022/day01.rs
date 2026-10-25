use crate::Args;
use std::io;

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut v: Vec<i32> = vec![];
    let mut cur: i32 = 0;
    for line in data {
        if line.is_empty() {
            v.push(cur);
            cur = 0;
        } else {
            let upd: i32 = line.parse().unwrap();
            cur += upd;
        }
    }
    if cur > 0 {
        v.push(cur);
    }
    v.sort();
    v.reverse();
    Ok(if args.part_two {
        format!("Top 3: {}", v[0] + v[1] + v[2])
    } else {
        format!("Max: {}", v[0])
    })
}
