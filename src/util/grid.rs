// Grid

pub struct Grid<T> {
    values: Vec<Vec<T>>,
    x_size: usize,
    y_size: usize,
}

impl<T> Grid<T> 
where
    T: From<char> + Clone,
{
    pub fn new(data: &[&str], surround: T) -> Self {
        let mut grid = Self {
            values: Vec::new(),
            x_size: 0,
            y_size: 0,
        };
        grid.values = Vec::new();
        let linelen = data[0].chars().count() + 2;
        grid.x_size = linelen - 2;
        grid.y_size = data.len();
        grid.values.push(vec![surround.clone(); linelen]);
        for line in data {
            let mut val = Vec::with_capacity(linelen);
            val.push(surround.clone());
            val.extend(line.chars().map(|s| T::from(s))); 
            val.push(surround.clone());
            grid.values.push(val);
        }
        grid.values.push(vec![surround.clone(); linelen]);
        grid
    }

    pub fn get(self: &Grid<T>, x: usize, y: usize) -> &T {
        &self.values[y+1][x+1]
    }

    pub fn get_mut(self: &mut Grid<T>, x: usize, y: usize) -> &mut T {
        &mut self.values[y+1][x+1]
    }

    pub fn get_size(self: &Grid<T>) -> (usize, usize) {
        (self.x_size, self.y_size)
    }

    pub fn get_neighbors(self: &Grid<T>, x: usize, y: usize) -> Vec<&T> {
        let mut neighbors = Vec::new();
        neighbors.push(&self.values[y][x]);
        neighbors.push(&self.values[y+1][x]);
        neighbors.push(&self.values[y+2][x]);
        neighbors.push(&self.values[y][x+1]);
        neighbors.push(&self.values[y+2][x+1]);
        neighbors.push(&self.values[y][x+2]);
        neighbors.push(&self.values[y+1][x+2]);
        neighbors.push(&self.values[y+2][x+2]);
        neighbors
    }
}

impl<T> std::fmt::Debug for Grid<T> 
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.values {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}
