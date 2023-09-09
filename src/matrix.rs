use crate::{bincode::decode_slice, error::Error};
use bincode::{Decode, Encode};

const COST_MATRIX: &[u8] = include_bytes!("../bin/matrix.bin");

#[derive(Encode, Decode)]
pub struct CostMatrix {
    matrix: Vec<Vec<i16>>,
}

impl CostMatrix {
    pub fn new(matrix: Vec<Vec<i16>>) -> Self {
        Self { matrix }
    }

    pub fn load() -> Result<Self, Error> {
        decode_slice(COST_MATRIX)
    }

    pub fn get(&self, right_id: u16, left_id: u16) -> i16 {
        self.matrix[right_id as usize][left_id as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::CostMatrix;

    #[test]
    fn test_cost_matrix() {
        let cost_matrix = CostMatrix::load().unwrap();

        assert_eq!(cost_matrix.get(0, 0), -434);
    }
}
