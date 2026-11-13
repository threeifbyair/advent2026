use crate::util::grid::Grid;
use crate::util::grid::Coord;
use crate::Args;
use std::collections::HashSet;
use std::io;

/// All in-bounds cells touching `(x, y)`, for queueing up cells that need re-checking.
fn neighbor_coords(grid: &Grid<char>, coord: Coord) -> Vec<Coord> {
    let (x_size, y_size) = grid.get_size();
    let (x, y) = (coord.x as usize, coord.y as usize);
    let x_range = x.saturating_sub(1)..=(x + 1).min(x_size - 1);
    let y_range = y.saturating_sub(1)..=(y + 1).min(y_size - 1);
    y_range
        .flat_map(|ny| x_range.clone().map(move |nx| Coord::from((nx, ny))))
        .filter(|&coord| coord != Coord::from((x, y)))
        .collect()
}

fn count_cells(grid: &Grid<char>, target: char) -> u64 {
    let (x_size, y_size) = grid.get_size();
    (0..y_size)
        .map(|y| (0..x_size).filter(|&x| *grid.get((x, y)) == target).count() as u64)
        .sum()
}

/// True if any of the 8 cells surrounding `(x, y)` is neither land nor a tree.
fn neighbors_blocked(grid: &Grid<char>, index: impl Into<Coord>) -> bool {
    let coord: Coord = index.into();
    grid.get_neighbors(coord).into_iter().any(|&c| c != '#' && c != '@')
}

/// Spreads `replacement` outward from `cells_to_check`: any queued cell that isn't
/// `uninteresting` and has 5+ `relevant` neighbors gets replaced, queueing its own
/// neighbors for the next round.
fn flood_fill(
    grid: &mut Grid<char>,
    cells_to_check: &mut HashSet<Coord>,
    verbose: bool,
    uninteresting: fn(&char) -> bool,
    relevant: fn(&char) -> bool,
    replacement: char,
) {
    while let Some(&coord) = cells_to_check.iter().next() {
        cells_to_check.remove(&coord);
        let cell = grid.get(coord);
        let (x, y) = (coord.x, coord.y);
        if verbose {
            println!("Checking cell ({x}, {y}) with value '{cell}'");
        }
        if uninteresting(cell) {
            continue;
        }

        let count = grid.get_neighbors(coord).into_iter().filter(|c| relevant(c)).count();
        if verbose {
            println!("Cell ({x}, {y}) has {count} relevant neighbors");
        }

        if count >= 5 {
            *grid.get_mut(coord) = replacement;
            cells_to_check.extend(neighbor_coords(grid, coord));
            if verbose {
                println!("Cell ({x}, {y}) changed to '{replacement}', adding neighbors to check: {cells_to_check:?}");
            }
        }
    }

    if verbose {
        println!("Final grid:");
        println!("{grid:?}");
    }
}

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut grid: Grid<char> = Grid::new(data, '0');
    let (x_size, y_size) = grid.get_size();
    assert_eq!(x_size % 2, 1);
    assert_eq!(y_size % 2, 1);
    let x_center = x_size / 2;
    let y_center = y_size / 2;
    let center = Coord::from((x_center, y_center));
    *grid.get_mut(center) = '#';

    let mut cells_to_check: HashSet<Coord> = HashSet::from([center]);
    cells_to_check.extend(neighbor_coords(&grid, center));

    if args.verbose {
        println!("Initial grid:\n{grid:?}");
        println!("Cells to check: {cells_to_check:?}");
    }

    flood_fill(
        &mut grid,
        &mut cells_to_check,
        args.verbose,
        |c| *c == '#',
        |c| *c == '#' || *c == '1',
        '#',
    );

    if !(args.part_two || args.part_three) {
        return Ok(format!("Total land: {}", count_cells(&grid, '#')));
    }

    // Now let's turn some land into sand: any land cell touching the unclaimed border gets
    // washed away, and that erosion then spreads inward.
    let mut cells_to_check_part_two: HashSet<Coord> = HashSet::new();
    for y in 0..y_size {
        for x in 0..x_size {
            let coord = Coord::from((x, y));
            if *grid.get(coord) != '#' {
                continue;
            }
            let touches_edge = grid.get_neighbors(coord).into_iter().any(|&c| c == '0' || c == '1');
            if touches_edge {
                *grid.get_mut(coord) = '=';
                cells_to_check_part_two.extend(neighbor_coords(&grid, coord));
            }
        }
    }

    flood_fill(
        &mut grid,
        &mut cells_to_check_part_two,
        args.verbose,
        |c| *c != '#',
        |c| *c == '=',
        '=',
    );

    if args.part_two {
        return Ok(format!("Total sand: {}", count_cells(&grid, '=')));
    }

    // Now we need to place trees: a cell can host one once it, and its neighbors, are
    // all land or trees -- no sand nearby at all.
    for y in 0..y_size {
        for x in 0..x_size {
            let coord = Coord::from((x, y));
            if *grid.get(coord) != '#' || neighbors_blocked(&grid, coord) {
                continue;
            }
            // Check the neighbors of the neighbors too.
            let surroundings_clear = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .all(|(nx, ny)| !neighbors_blocked(&grid, (nx, ny)));
            if !surroundings_clear {
                continue;
            }

            // After all that, we can place a tree here.
            *grid.get_mut(coord) = '@';
            if args.verbose {
                println!("Placing tree at ({x}, {y})");
            }
        }
    }

    if args.verbose {
        println!("Grid after initial tree placement:\n{grid:?}");
    }

    // Two more non-blocking rounds: a land cell with 4+ tree neighbors grows one too,
    // as long as no neighboring sand stops it, with all of a round's trees placed at once.
    for _ in 0..2 {
        let mut pending_trees: Vec<Coord> = Vec::new();
        for y in 0..y_size {
            for x in 0..x_size {
                let coord: Coord = Coord::from((x, y));
                if *grid.get(coord) != '#' {
                    continue;
                }
                let neighbors = grid.get_neighbors(coord);
                if neighbors.iter().any(|&&c| c == '=') {
                    continue;
                }
                let tree_count = neighbors.iter().filter(|&&&c| c == '@').count();
                if tree_count >= 4 {
                    pending_trees.push(coord);
                    if args.verbose {
                        println!("Pending tree placement at ({x}, {y})");
                    }
                }
            }
        }

        for coord in pending_trees {
            *grid.get_mut(coord) = '@';
        }
        if args.verbose {
            println!("Grid after pending tree placement:\n{grid:?}");
        }
    }

    if args.verbose {
        println!("Final grid with trees:\n{grid:?}");
    }

    Ok(format!("Total trees: {}", count_cells(&grid, '@')))
}
