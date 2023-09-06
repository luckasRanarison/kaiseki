use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Term {
    pub context_id: u16,
    pub cost: i16,
}
