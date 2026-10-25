use crate::Args;
use std::io;

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut num_zeros = 0;
    let mut num_parttwo_zeros = 0;
    let mut pos: i32 = 50;
    for line in data {
        let mut chars = line.chars();
        let first = chars.next().unwrap();      // char
        let rest: &str = chars.as_str();        // remainder as &str
        let num: i32 = rest.parse().unwrap();
        let oldpos = pos;

        if args.verbose {
            println!("Line: {}, Pos: {}, Turn: {}", line, pos, num);
            if num >= 100 {
                println!("Warning: Turn {} value {} is >= 100, which is not shown in the example.", first, num);
            }
        }
        let rotations = num / 100;
        let partial_rotation = num % 100;
        match first {
            'L' => {
                num_parttwo_zeros += rotations.abs();
                pos -= partial_rotation;
                pos = (pos + 100) % 100;
                if oldpos != 0 && oldpos - partial_rotation < 0 {
                    num_parttwo_zeros += 1;
                }
            }
            'R' => {
                num_parttwo_zeros += rotations.abs();
                pos += partial_rotation;
                pos = (pos + 100) % 100;
                if oldpos + partial_rotation > 100 {
                    num_parttwo_zeros += 1;
                }
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid character")),
        }
        if args.verbose {
            println!("Rotations: {}, Partial: {}, OldPos: {}, NewPos: {}, Zeros: {}", rotations, partial_rotation, oldpos, pos, num_parttwo_zeros);
        }
        if pos == 0 {
            num_zeros += 1;
            num_parttwo_zeros += 1;
            if args.verbose {
                println!("Found zero: pos {} zeros {}", pos, num_parttwo_zeros);
            }
        }

    }
    Ok(if args.part_two {
        format!("Part Two Zeros: {}", num_parttwo_zeros)
    } else {
        format!("Zeros: {}", num_zeros)
    })
}
