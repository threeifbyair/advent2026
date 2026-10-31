use crate::Args;
use std::io;

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let sizes: Vec<u32> = data[0].split(',').filter_map(|s| s.parse().ok()).collect();

    // Monotonically increasing, so the first entry >= a given size is also the smallest such entry.
    let squares: Vec<usize> = if args.part_three {
        (1..10000).map(|s| ((s * 2 + 1) * (s * 2 + 1)) - 1).collect()
    } else if args.part_two {
        (3..5000).map(|s| (s * s) - (s - 2) * (s - 2)).collect()
    } else {
        (1..33).map(|s| s * s).collect()
    };

    let mut needed: usize = 0;
    for size in sizes {
        if !(1..=10000000).contains(&size) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid size: {size}"),
            ));
        }
        let size = size as usize;
        let highest = squares.iter().find(|&&s| s >= size).copied().unwrap_or(0);
        if args.verbose {
            println!("Size: {size}, Highest: {highest}, Needed: {}", highest - size);
        }
        needed += highest - size;
    }

    Ok(format!("Needed: {needed}"))
}
