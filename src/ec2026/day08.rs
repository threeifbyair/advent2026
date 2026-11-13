use crate::util::grid::Grid;
use crate::util::grid::Coord;
use crate::Args;
use std::io;

fn grow_ents(grid: &mut Grid<char>, verbose: bool) -> u32 {
    let (x_size, y_size) = grid.get_size();
    let mut total_warriors: u32 = 0;
    for y in 0..y_size {
        for x in 0..x_size {
            let coord = Coord::from((x, y));
            let cell = grid.get_mut(coord);
            match cell {
                'o'..='v' => {
                    let height = (*cell as u32) - ('n' as u32);
                    *cell = std::char::from_digit(height + 1, 10).unwrap();
                }
                'w' => {
                    *cell = 'W';
                    total_warriors += 1;
                    if verbose {
                        println!("Ent at ({x},{y}) has grown to warrior W");
                    }
                }
                'W' => {
                    *cell = 'A';
                }
                'A' => {
                    *cell = 'B';
                }
                'B' => {
                    *cell = '1';
                }
                _ => {}
            }
        }
    }
    total_warriors
}

fn check_for_ents(grid: &mut Grid<char>, orig_x: usize, orig_y: usize, x_inc: i32, y_inc: i32, verbose: bool, part_three: bool) -> u32 {
    let mut shadow_size: u32 = 0;
    let mut num_ents: u32 = 0;
    let mut x = orig_x;
    let mut y = orig_y;
    let (x_size, y_size) = grid.get_size();
    if verbose {
        println!("Checking for ents starting at ({orig_x},{orig_y}) with increments ({x_inc},{y_inc})");
    }
    while x < x_size && y < y_size  {
        let cell = grid.get_mut((x, y));
        if verbose {
            println!("Checking cell ({x},{y}) with value {cell} and shadow size {shadow_size}");
        }
        match cell {
            '0'..='9' => {
                let height = cell.to_digit(10).unwrap();
                if height > shadow_size {
                    num_ents += 1;
                    shadow_size = height;
                    if part_three {
                        // Mark the ent to indicate it received energy and should grow.
                        *cell = (height as u8 + b'n') as char;
                    }
                    if verbose {
                        println!("Ent height {height} at ({x},{y})");
                    }
                }
                else if verbose {
                    println!("Ent height {height} at ({x},{y}) shadowed by {shadow_size}");
                }
            }
            'W' | 'A' | 'B' if verbose => {
                println!("Found seed {cell} at ({x},{y})");
            }
            _ => {}
        }
        if *cell != '.' && y_inc == 0 {
            // Hexagonal grid, so we need to skip every other row when moving horizontally.
            if x == 0 && x_inc < 0 {
                break;
            }
            x = (x as i32 + x_inc) as usize;
        }
        if (x == 0 && x_inc < 0) || (y == 0 && y_inc < 0) {
            break;
        }
        x = (x as i32 + x_inc) as usize;
        y = (y as i32 + y_inc) as usize;
        shadow_size = shadow_size.saturating_sub(1);
    }
    if verbose {
        println!("Found {num_ents} ents starting at ({orig_x},{orig_y}) with increments ({x_inc},{y_inc})");
    }
    num_ents
}

fn find_ents(grid: &mut Grid<char>, x_inc: i32, y_inc: i32, verbose: bool, part_three: bool) -> u32 {
    if verbose {
        println!("Finding ents with increments ({x_inc},{y_inc})");
    }
    let (x_size, y_size) = grid.get_size();
    let mut total_ents: u32 = 0;
    if y_inc != 0 {
        for start_x in 0..x_size {
            total_ents += check_for_ents(grid, start_x, if y_inc < 0 { y_size - 1 } else { 0 }, x_inc, y_inc, verbose, part_three);
        }
    }
    for start_y in 0..y_size {
        if (y_inc > 0 && start_y == 0) || (y_inc < 0 && start_y == y_size - 1) {
            // Already checked this one.
            continue;
        }
        total_ents += check_for_ents(grid, if x_inc < 0 { x_size - 1 } else { 0 }, start_y, x_inc, y_inc, verbose, part_three);
    }
    if verbose {
        println!("Found {total_ents} ents with increments ({x_inc},{y_inc})\n");
    }
    total_ents
}

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut grid: Grid<char> = Grid::new(data, '.');
    let increment_round: [(i32, i32); 6] = [(1, 1), (-1, 1), (-1, 0), (-1, -1), (1, -1), (1, 0)];

    if args.verbose {
        println!("Initial grid:\n{grid:?}");
    }

    // For part 1, we look along the top and along the left side.
    let total_ents = if args.part_three {
        let mut total_warriors: u32 = 0;
        let mut total_energy: u32 = 0;
        let mut round_offset: usize = 0;
        let mut round : u32 = 1;
        let target_warriors: u32 = if args.argint > 0 { args.argint as u32 } else { 100000 };
        loop {
            let (x_inc, y_inc) = increment_round[round_offset];
            total_energy += find_ents(&mut grid, x_inc, y_inc, args.verbose, args.part_three);
            total_warriors += grow_ents(&mut grid, args.verbose);
            if args.verbose {
                println!("After round {round} with increments ({x_inc},{y_inc}):\n{grid:?}");
                println!("Total energy: {total_energy}, Total warriors: {total_warriors}\n");
            }
            if total_warriors >= target_warriors {
                break;
            }
            round_offset = (round_offset + 1) % increment_round.len();
            round += 1;
        }
        total_energy
    }
    else if args.part_two {
        let mut total_ents: u32 = 0;
        for (x_inc, y_inc) in increment_round {
            total_ents += find_ents(&mut grid, x_inc, y_inc, args.verbose, args.part_three);
        }
        total_ents
    }
    else {
        find_ents(&mut grid, 1, 1, args.verbose, args.part_three)
    };

    if args.verbose {
        println!("Final grid:\n{grid:?}");
    }

    Ok(format!("Total ents: {}", total_ents))
}
