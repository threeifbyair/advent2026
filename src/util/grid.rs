use std::ops::Add;

// Grid

#[derive(Clone)]
pub struct Grid<T> {
    values: Vec<Vec<T>>,
    x_size: usize,
    y_size: usize,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize
}

impl Coord {
    pub fn new(x: isize, y:isize) -> Self {
        Coord { x, y }
    }
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Coord { x, y }
    }
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord { x: x as isize, y: y as isize }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub const ALL: [Direction; 8] = [
        Direction::North, Direction::NorthEast, Direction::East, Direction::SouthEast,
        Direction::South, Direction::SouthWest, Direction::West, Direction::NorthWest,
    ];

    pub fn delta(self) -> Coord {
        match self {
            Direction::North     => Coord::from((0isize, -1isize)),
            Direction::NorthEast => Coord::from((1isize, -1isize)),
            Direction::East      => Coord::from((1isize, 0isize)),
            Direction::SouthEast => Coord::from((1isize, 1isize)),
            Direction::South     => Coord::from((0isize, 1isize)),
            Direction::SouthWest => Coord::from((-1isize, 1isize)),
            Direction::West      => Coord::from((-1isize, 0isize)),
            Direction::NorthWest => Coord::from((-1isize, -1isize)),
        }
    }

    pub fn turn(self, steps: i32) -> Direction {
        let next = (self as i32 + steps).rem_euclid(8);
        Self::ALL[next as usize]
    }


    pub fn opposite(self) -> Direction {
        self.turn(4)
    }

    pub fn clockwise(self) -> Direction {
        self.turn(2)
    }

    pub fn counterclockwise(self) -> Direction {
        self.turn(-2) 
    }

}

impl<T> Grid<T>
where
    T: From<char> + Clone,
{
    pub fn new(data: &[&str], surround: T) -> Self {
        let x_size = data[0].chars().count();
        let y_size = data.len();
        let linelen = x_size + 2;

        let mut values = Vec::with_capacity(y_size + 2);
        values.push(vec![surround.clone(); linelen]);
        for line in data {
            let mut row = Vec::with_capacity(linelen);
            row.push(surround.clone());
            row.extend(line.chars().map(T::from));
            row.push(surround.clone());
            values.push(row);
        }
        values.push(vec![surround.clone(); linelen]);

        Self { values, x_size, y_size }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, index: impl Into<Coord>) -> &T {
        let coord: Coord = index.into();
        &self.values[(coord.y + 1) as usize][(coord.x + 1) as usize]
    }

    pub fn get_mut(&mut self, index: impl Into<Coord>) -> &mut T {
        let coord: Coord = index.into();
        &mut self.values[(coord.y + 1) as usize][(coord.x + 1) as usize]
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.x_size, self.y_size)
    }

    /// Returns the 8 cells surrounding `(x, y)`, ordered column-major
    /// (left column top-to-bottom, then straight up/down, then right column top-to-bottom).
    pub fn get_neighbors(&self, index: impl Into<Coord>) -> Vec<&T> {
        let coord: Coord = index.into();
        let mut neighbors = Vec::with_capacity(8);
        for dx in 0..3 {
            for dy in 0..3 {
                if dx == 1 && dy == 1 {
                    continue;
                }
                neighbors.push(&self.values[(coord.y + dy) as usize][(coord.x + dx) as usize]);
            }
        }
        neighbors
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.values {
            let row: String = row.iter().map(|t| format!("{t:?}")).collect::<Vec<String>>().join("");
            let _ = writeln!(f, "{row}");
        }
        Ok(())
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.values {
            let row: String = row.iter().map(|t| format!("{t}")).collect::<Vec<String>>().join("");
            let _ = writeln!(f, "{row}");
        }
        Ok(())
    }
}
