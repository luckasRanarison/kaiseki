use crate::row::Row;
use bincode::{Decode, Encode};

pub type TermId = usize;

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Term {
    pub length: usize,
    pub context_id: u16,
    pub cost: i16,
}

impl From<&Row<'_>> for Term {
    fn from(value: &Row) -> Self {
        Self {
            length: value.surface_form.len(),
            context_id: value.left_id,
            cost: value.cost,
        }
    }
}
