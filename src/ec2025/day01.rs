use crate::Args;
use std::io;

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut names: Vec<&str> = data[0].split(',').collect();
    let commands: Vec<&str> = data[2].split(',').collect();
    let namelen = names.len();
    let mut pos: usize = 0;
    for command in commands {
        let mut chars = command.chars();
        let first = chars.next().unwrap();      // char
        let rest: &str = chars.as_str();        // remainder as &str
        let num: usize = rest.parse().unwrap();
        let oldpos = pos;

        if args.verbose {
            println!("Command: {}, Pos: {}", command, pos);
        }
        match first {
            'L' => {
                if args.part_three {
                    names.swap(0, namelen - (num % namelen));
                }
                else if args.part_two {
                    pos += num * namelen;
                    pos -= num;
                    pos %= namelen;
                }
                else {
                    if num > pos {
                        pos = 0;
                    } else {
                        pos -= num;
                    }
                }
            }
            'R' => {
                if args.part_three {
                    names.swap(0, num % namelen);
                }
                else if args.part_two {
                    pos += num;
                    pos %= namelen;
                }
                else {
                    pos += num;
                    if pos >= namelen {
                        pos = namelen - 1;
                    }
                }
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid character")),
        }
        if args.verbose {
            println!("OldPos: {}, NewPos: {}, Name: {}", oldpos, pos, names[pos]);
        }
    }
    Ok(if args.part_three {
        format!("Name: {}", names[0])
    } else {
        format!("Position: {}  Name: {}", pos, names[pos])
    })
}
