use crate::util::parse::letter_and_number;
use crate::Args;
use std::io;

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut names: Vec<&str> = data[0].split(',').collect();
    let commands: Vec<&str> = data[2].split(',').collect();
    let name_len = names.len();
    let mut pos: usize = 0;
    for command in commands {
        let (direction, num): (char, usize) = letter_and_number(command);
        let old_pos = pos;

        if args.verbose {
            println!("Command: {}, Pos: {}", command, pos);
        }
        match direction {
            'L' => {
                if args.part_three {
                    names.swap(0, name_len - (num % name_len));
                } else if args.part_two {
                    pos += num * name_len;
                    pos -= num;
                    pos %= name_len;
                } else if num > pos {
                    pos = 0;
                } else {
                    pos -= num;
                }
            }
            'R' => {
                if args.part_three {
                    names.swap(0, num % name_len);
                } else if args.part_two {
                    pos += num;
                    pos %= name_len;
                } else {
                    pos += num;
                    if pos >= name_len {
                        pos = name_len - 1;
                    }
                }
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid character")),
        }
        if args.verbose {
            println!("OldPos: {}, NewPos: {}, Name: {}", old_pos, pos, names[pos]);
        }
    }
    Ok(if args.part_three {
        format!("Name: {}", names[0])
    } else {
        format!("Position: {}  Name: {}", pos, names[pos])
    })
}
