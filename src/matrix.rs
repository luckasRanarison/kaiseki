use anyhow::Error;
use bincode::{config, decode_from_slice};

const COST_MATRIX: &'static [u8] = include_bytes!("../dict/matrix.bin");

pub struct CostMatrix {
    matrix: Vec<Vec<i16>>,
}

impl CostMatrix {
    pub fn load() -> Result<Self, Error> {
        let config = config::standard();
        let (matrix, _) = decode_from_slice(&COST_MATRIX, config)?;

        Ok(Self { matrix })
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
