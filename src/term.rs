use bincode::{Decode, Encode};

pub type TermId = usize;

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Term {
    pub context_id: u16,
    pub cost: i16,
}

impl Term {
    pub fn new(context_id: u16, cost: i16) -> Self {
        Self { context_id, cost }
    }
}
