use crate::util::parse::letter_and_number;
use crate::Args;
use std::io;

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut num_zeros = 0;
    let mut num_part_two_zeros = 0;
    let mut pos: i32 = 50;
    for line in data {
        let (direction, num): (char, i32) = letter_and_number(line);
        let old_pos = pos;

        if args.verbose {
            println!("Line: {line}, Pos: {pos}, Turn: {num}");
            if num >= 100 {
                println!("Warning: Turn {direction} value {num} is >= 100, which is not shown in the example.");
            }
        }
        let rotations = num / 100;
        let partial_rotation = num % 100;
        match direction {
            'L' => {
                num_part_two_zeros += rotations.abs();
                pos = (pos - partial_rotation).rem_euclid(100);
                if old_pos != 0 && old_pos - partial_rotation < 0 {
                    num_part_two_zeros += 1;
                }
            }
            'R' => {
                num_part_two_zeros += rotations.abs();
                pos = (pos + partial_rotation).rem_euclid(100);
                if old_pos + partial_rotation > 100 {
                    num_part_two_zeros += 1;
                }
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid character")),
        }
        if args.verbose {
            println!(
                "Rotations: {rotations}, Partial: {partial_rotation}, OldPos: {old_pos}, NewPos: {pos}, Zeros: {num_part_two_zeros}"
            );
        }
        if pos == 0 {
            num_zeros += 1;
            num_part_two_zeros += 1;
            if args.verbose {
                println!("Found zero: pos {pos} zeros {num_part_two_zeros}");
            }
        }
    }
    Ok(if args.part_two {
        format!("Part Two Zeros: {num_part_two_zeros}")
    } else {
        format!("Zeros: {num_zeros}")
    })
}
