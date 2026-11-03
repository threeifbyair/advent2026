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
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.values[y + 1][x + 1]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.values[y + 1][x + 1]
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.x_size, self.y_size)
    }

    /// Returns the 8 cells surrounding `(x, y)`, ordered column-major
    /// (left column top-to-bottom, then straight up/down, then right column top-to-bottom).
    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<&T> {
        let mut neighbors = Vec::with_capacity(8);
        for dx in 0..3 {
            for dy in 0..3 {
                if dx == 1 && dy == 1 {
                    continue;
                }
                neighbors.push(&self.values[y + dy][x + dx]);
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
            writeln!(f, "{row:?}")?;
        }
        Ok(())
    }
}
