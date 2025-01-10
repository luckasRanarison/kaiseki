use crate::utils::FlatMatrix;

use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub struct CostMatrix {
    matrix: FlatMatrix<i16>,
}

impl CostMatrix {
    pub fn new(matrix: FlatMatrix<i16>) -> Self {
        Self { matrix }
    }

    pub fn get(&self, right_id: u16, left_id: u16) -> i16 {
        self.matrix.get_cloned(right_id as usize, left_id as usize)
    }
}
