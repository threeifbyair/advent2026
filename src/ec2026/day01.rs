use crate::Args;
use std::io;

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let sizes: Vec<u32> = data[0].split(',').filter_map(|s| s.parse().ok()).collect();
    let mut needed: usize = 0;
    let squares: Vec<usize> = if args.part_three {
        (1..10000).map(|s| (((s*2+1) * (s*2+1)) - 1) as usize).collect()
    } else if args.part_two {
        (3..5000).map(|s| (s * s) - ((s-2) * (s-2)) as usize).collect()
    } else {
        (1..33).map(|s| (s * s) as usize).collect()
    };

    for size in sizes {
        if size < 1 || size > 10000000 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid size: {}", size),
            ));
        }
        let highest: usize = *squares.iter().filter(|&s| *s >= size as usize).min().unwrap_or(&0);
        if args.verbose {
            println!("Size: {}, Highest: {}, Needed: {}", size, highest, highest - size as usize);
        }
        needed += highest - size as usize;
    }   

    Ok(format!("Needed: {}", needed))
}
