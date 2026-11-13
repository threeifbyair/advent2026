use crate::util::grid::Grid;
use crate::util::grid::Coord;
use crate::util::grid::Direction;
use crate::Args;
use std::io;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Ghost {
    coord: Coord,
    direction: Direction,
    collisions: usize,
}

impl Ghost {
    pub fn new(index: impl Into<Coord>, direction: Direction) -> Self {
        let coord = index.into();
        Ghost { coord, direction, collisions: 0 }
    }
}



fn extract_ghosts(grid: &mut Grid<char>) -> Vec<Ghost> {
    let (x_size, y_size) = grid.get_size();
    let mut ghosts: Vec<Ghost> = Vec::new();
    for x in 0..x_size {
        for y in 0..y_size {
            let spot = grid.get_mut((x,y));
            match spot {
                '<' | '>' | '^' | 'v' => {
                    let direction: Option<Direction> = match spot {
                        '<' => Some(Direction::West),
                        '>' => Some(Direction::East),
                        '^' => Some(Direction::North),
                        'v' => Some(Direction::South),
                        _ => None
                    };
                    let ghost: Ghost = Ghost::new((x, y), direction.unwrap());
                    ghosts.push(ghost);
                    *spot = '.';
                }
                _ => ()
            }
        }
    }
    ghosts
}

fn build_ghost_map(ghosts: &Vec<Ghost>) -> HashMap<Coord, u32>
{
    let mut ghost_map: HashMap<Coord, u32> = HashMap::new();
    for ghost in ghosts.iter() {
        let ghost_coords = ghost_map.entry(ghost.coord).or_insert(0);
        *ghost_coords += 1;
    }
    ghost_map
}

fn move_ghosts(ghosts: &mut Vec<Ghost>, grid: &Grid<char>, verbose: bool, max_collisions: usize) -> usize
{
    let mut collisions: usize = 0;
    let mut new_grid = grid.clone();

    // First move all the ghosts.
    for ghost in ghosts.iter_mut() {
        let delta: Coord = ghost.direction.delta();
        let new_coord: Coord = ghost.coord + delta;
        let gridspot = grid.get(new_coord);
        if *gridspot == '#' {
            // We can't move. Turn 90 degrees clockwise.
            ghost.direction = ghost.direction.clockwise();
            ghost.collisions += 1;
            collisions += 1;
        }
        else {
            // We can move.
            ghost.coord = new_coord;
        }
    }

    if max_collisions > 0 {
        ghosts.retain(|g| g.collisions < max_collisions);
    }

    let ghost_map = build_ghost_map(ghosts);

    // Now let's see which ghosts collide.
    for ghost in ghosts.iter_mut() {
        let num_ghosts = ghost_map.get(&ghost.coord).unwrap();
        if *num_ghosts > 1 {
            ghost.direction = ghost.direction.opposite();
            ghost.collisions += 1;
            collisions += 1;
        }
        let target_gridspot = new_grid.get_mut(ghost.coord);
        match *target_gridspot {
            '.' => {
                *target_gridspot = match ghost.direction {
                    Direction::West => '<',
                    Direction::North => '^',
                    Direction::East => '>',
                    Direction::South => 'v',
                    _ => '?'
                };
            },
            '<' | '>' | '^' | 'v' => {
                *target_gridspot = '2';
            }
            '2'..='8' => {
                *target_gridspot = std::char::from_u32(*target_gridspot as u32 + 1).unwrap();
            },
            '9' => {
                *target_gridspot = 'X';
            },
            'X' => {},
            _ => {}
        };
    }
    if verbose {
        println!("Grid after this move with {collisions} collisions:\n{new_grid}");
    }
    if max_collisions > 0 {
        ghosts.retain(|g| g.collisions < max_collisions);
   }
    collisions
}

fn get_ghost_hash(ghosts: &Vec<Ghost>) -> usize {
    let mut hasher = DefaultHasher::new();
    ghosts.len().hash(&mut hasher);
    for ghost in ghosts {
        ghost.coord.x.hash(&mut hasher);
        ghost.coord.y.hash(&mut hasher);
        ghost.direction.hash(&mut hasher);
    }
    hasher.finish() as usize
}


fn find_ghost_period(ghosts: &mut Vec<Ghost>, grid: &Grid<char>, verbose: bool, max_collisions: usize) -> (usize, (usize, Vec<usize>)) {
    let mut round: usize = 0;
    let period: (usize, Vec<usize>);
    let mut status: HashMap<usize, (usize, Vec<usize>)> = HashMap::new();
    loop {
        _ = move_ghosts(ghosts, grid, verbose, max_collisions);
        round += 1;
        let ghost_hash = get_ghost_hash(&ghosts);
        if ghosts.len() == 0 || status.contains_key(&ghost_hash) {
            period = if ghosts.len() == 0 { (0, Vec::new() ) } else { status.get(&ghost_hash).unwrap().clone() };
            break;
        }
        status.insert(ghost_hash, (round, ghosts.iter().map(|g| g.collisions).collect()));
    }
    (round, period)
}

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let mut grid: Grid<char> = Grid::new(data, '#');

    if args.verbose {
        println!("Initial grid:");
        println!("{grid}");
    }

    let mut ghosts: Vec<Ghost> = extract_ghosts(&mut grid);
    let mut collisions: usize = 0;

    if args.part_two || args.part_three {
        let max_collisions: usize = if args.argint == 0 { if args.part_three { 1000000000000usize } else { 10000usize} } else { args.argint as usize };
        let mut round: usize = 0;
        loop {
            let (total_rounds, repeated_round) = find_ghost_period(&mut ghosts, &mut grid, args.verbose, max_collisions);
            round += total_rounds;
            if ghosts.len() == 0 {
                break;
            }
            // Now let's see how many times more we can go around this.
            let period = total_rounds - repeated_round.0;
            let increment: Vec<usize> = ghosts.iter().zip(repeated_round.1.clone()).map(|(ghost, old_value)| ghost.collisions - old_value).collect();

            // asking: how many times can we add value to increment without reaching max_collisions?
            // that would be (max_collisions - 1 - value) / increment 
            let multiplier = ghosts.iter().zip(increment.clone()).map(|(g, v)| (max_collisions - 1 - g.collisions) / v).min().unwrap_or(0);
            for (ghost, amount) in ghosts.iter_mut().zip(increment) {
                ghost.collisions += amount * multiplier;
            }
            round += period * multiplier;
        }
        Ok(format!("Total rounds: {}", round))
    }
    else {
        for _ in 0..10 {
            collisions += move_ghosts(&mut ghosts, &mut grid, args.verbose, 0);
        }

        Ok(format!("Total collisions: {}", collisions))
    }
}
