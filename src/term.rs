use crate::row::Row;
use bincode::{Decode, Encode};

pub type TermId = usize;

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Term {
    pub context_id: u16,
    pub cost: i16,
}

impl From<&Row<'_>> for Term {
    fn from(value: &Row) -> Self {
        Self {
            context_id: value.left_id, // assert_eq!(value.left_id, value.right_id);
            cost: value.cost,
        }
    }
}

impl Term {
    pub fn new(context_id: u16, cost: i16) -> Self {
        Self { context_id, cost }
    }
}

#[derive(Debug)]
pub struct ExtratedTerm {
    pub id: TermId,
    pub unknown: bool,
    pub length: usize,
    pub value: Term,
}

impl ExtratedTerm {
    pub fn new(id: TermId, unknown: bool, length: usize, value: Term) -> Self {
        Self {
            id,
            unknown,
            length,
            value,
        }
    }
}
