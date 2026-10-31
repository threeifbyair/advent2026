use std::collections::HashSet;
use crate::Args;
use crate::util::grid::Grid;
use std::io;


pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut grid: Grid<char> = Grid::new(data, '0');
    let (x_size, y_size) = grid.get_size();
    assert_eq!(x_size % 2, 1);
    assert_eq!(y_size % 2, 1);
    let x_center = x_size / 2;
    let y_center = y_size / 2;
    let center = grid.get_mut(x_center, y_center);
    *center = '#';
    let mut cells_to_check: HashSet<(usize, usize)> = HashSet::new();
    let get_neighbors = |x: usize, y: usize| -> Vec<(usize, usize)> {
        let mut vec : Vec<(usize, usize)> = Vec::new();
        if x > 0 && y > 0 {
            vec.push((x - 1, y - 1));
        }
        if y > 0 {
            vec.push((x, y - 1));
        }
        if x < x_size - 1 && y > 0 {
            vec.push((x + 1, y - 1));
        }
        if x > 0 {
            vec.push((x - 1, y));
        }
        if x < x_size - 1 {
            vec.push((x + 1, y));
        }
        if x > 0 && y < y_size - 1 {
            vec.push((x - 1, y + 1));
        }
        if y < y_size - 1 {
            vec.push((x, y + 1));
        }
        if x < x_size - 1 && y < y_size - 1 {
            vec.push((x + 1, y + 1));
        }
        vec
    };
    cells_to_check.insert((x_center, y_center));
    for neighbor in get_neighbors(x_center, y_center) {
        cells_to_check.insert(neighbor);
    }
    if args.verbose {
        println!("Initial grid:");
        for y in 0..y_size {
            for x in 0..x_size {
                print!("{}", grid.get(x, y));
            }
            println!();
        }
        println!("Cells to check: {cells_to_check:?}");
    }
    let check_cells = |grid: &mut Grid<char>, cells_to_check: &mut HashSet<(usize, usize)>, x_size: usize, y_size: usize, verbose: bool, uninteresting: fn(&char) -> bool, relevant: fn(&char) -> bool, replacement: char| {
        while let Some(&(x, y)) = cells_to_check.iter().next() {
            cells_to_check.remove(&(x, y));
            let cell = grid.get(x, y);
            if verbose {
                println!("Checking cell ({x}, {y}) with value '{}'", *cell);
            }
            if uninteresting(cell) {
                continue;
            }
            let neighbors = grid.get_neighbors(x, y);
            let mut count = 0;
            for neighbor in &neighbors {
                if relevant(neighbor) {
                    count += 1;
                }
            }
            if verbose {
                println!("Cell ({x}, {y}) has {count} relevant neighbors");
            }
            if count >= 5 {
                let cell = grid.get_mut(x, y);
                *cell = replacement;
                for neighbor in get_neighbors(x, y) {
                    cells_to_check.insert(neighbor);
                }
                if verbose {
                    println!("Cell ({x}, {y}) changed to '{replacement}', adding neighbors to check: {cells_to_check:?}");
                }
            }
        }
        if verbose {
            println!("Final grid:");
            for y in 0..y_size {
                for x in 0..x_size {
                    print!("{}", grid.get(x, y));
                }
                println!();
            }
        }
    };

    check_cells(&mut grid, &mut cells_to_check, x_size, y_size, args.verbose, |cell| *cell == '#', |cell| *cell == '#' || *cell == '1', '#');

    if args.part_two || args.part_three {
        // Now let's turn some land into sand.
        let mut cells_to_check_part_two: HashSet<(usize, usize)> = HashSet::new();
        for y in 0..y_size {
            for x in 0..x_size {
                let cell = grid.get(x, y);
                if *cell == '#' {
                    let neighbors = grid.get_neighbors(x, y);
                    for neighbor in &neighbors {
                        if **neighbor == '0' || **neighbor == '1' {
                            let cell = grid.get_mut(x, y);
                            *cell = '=';
                            for neighbor in get_neighbors(x, y) {
                                cells_to_check_part_two.insert(neighbor);
                            }
                            break;
                        }
                    }
                }
            }
        }

        check_cells(&mut grid, &mut cells_to_check_part_two, x_size, y_size, args.verbose, |cell| *cell != '#', |cell| *cell == '=', '=');

        if args.part_two {
            let mut total: u64 = 0;
            for y in 0..y_size {
                for x in 0..x_size {
                    if *grid.get(x, y) == '=' {
                        total += 1;
                    }
                }
            }
            Ok(format!("Total sand: {total}"))
        }
        else {
            // Now we need to place trees. 
            for y in 0..y_size {
                'per_cell: for x in 0..x_size {
                    let cell = grid.get(x, y);
                    if *cell == '#' {
                        for neighbor in grid.get_neighbors(x, y) {
                            if *neighbor != '#' && *neighbor != '@' {
                                continue 'per_cell;
                            }
                        }
                        // So all immediate neighbors are OK. Let's check the neighbors of the neighbors.
                        for neighbor_set in vec![grid.get_neighbors(x-1, y), grid.get_neighbors(x+1, y), grid.get_neighbors(x, y-1), grid.get_neighbors(x, y+1)] {
                            for neighbor in neighbor_set {
                                if *neighbor != '#' && *neighbor != '@' {
                                    continue 'per_cell;
                                }
                            }
                        }
                        // After all that, we can place a tree here.
                        let cell = grid.get_mut(x, y);
                        *cell = '@';
                        if args.verbose {
                            println!("Placing tree at ({x}, {y})");
                        }
                    }
                }
            }
            if args.verbose {
                println!("Grid after initial tree placement:");
                for y in 0..y_size {
                    for x in 0..x_size {
                        print!("{}", grid.get(x, y));
                    }
                    println!();
                }
            }
            // OK, we do two non-blocking rounds of tree placement.
            for _ in 0..2 {
                let mut pending_trees: Vec<(usize, usize)> = Vec::new();
                for y in 0..y_size {
                    'per_cell: for x in 0..x_size {
                        let cell = grid.get(x, y);
                        if *cell == '#' {
                            let neighbors = grid.get_neighbors(x, y);
                            let mut count = 0;
                            for neighbor in &neighbors {
                                if **neighbor == '=' {
                                    continue 'per_cell;
                                }
                                if **neighbor == '@' {
                                    count += 1;
                                }
                            }
                            if count >= 4 {
                                pending_trees.push((x, y));
                                if args.verbose {
                                    println!("Pending tree placement at ({x}, {y})");
                                }
                            }
                        }
                    }
                }
                for (x, y) in pending_trees {
                    let cell = grid.get_mut(x, y);
                    *cell = '@';
                }
                if args.verbose {
                    println!("Grid after pending tree placement:");
                    for y in 0..y_size {
                        for x in 0..x_size {
                            print!("{}", grid.get(x, y));
                        }
                        println!();
                    }
                }
            }
            if args.verbose {
                println!("Final grid with trees:");
                for y in 0..y_size {
                    for x in 0..x_size {
                        print!("{}", grid.get(x, y));
                    }
                    println!();
                }
            }

            let mut total: u64 = 0;
            for y in 0..y_size {
                for x in 0..x_size {
                    if *grid.get(x, y) == '@' {
                        total += 1;
                    }
                }
            }
            Ok(format!("Total trees: {total}"))
        }
    }
    else {
        let mut total: u64 = 0;
        for y in 0..y_size {
            for x in 0..x_size {
                if *grid.get(x, y) == '#' {
                    total += 1;
                }
            }
        }
        Ok(format!("Total land: {total}"))
    }
}
