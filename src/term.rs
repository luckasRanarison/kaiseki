use bincode::{Decode, Encode};

use crate::row::Row;

pub type TermId = usize;

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Term {
    pub context_id: u16,
    pub cost: i16,
}

impl From<&Row<'_>> for Term {
    fn from(value: &Row) -> Self {
        Self {
            context_id: value.left_id,
            cost: value.cost,
        }
    }
}

impl Term {
    pub fn new(context_id: u16, cost: i16) -> Self {
        Self { context_id, cost }
    }
}
