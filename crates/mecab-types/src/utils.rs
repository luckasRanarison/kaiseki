use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub struct FlatMatrix<T: Default + Clone + 'static> {
    rows: usize,
    cols: usize,
    values: Vec<T>,
}

impl<T: Default + Clone> FlatMatrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            values: vec![T::default(); rows * cols],
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.values[row * self.rows + col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.values[row * self.rows + col]
    }

    pub fn get_cloned(&self, row: usize, col: usize) -> T {
        self.values[row * self.rows + col].clone()
    }

    pub fn get_row(&self, row: usize) -> &[T] {
        &self.values[row..row + self.cols]
    }
}
