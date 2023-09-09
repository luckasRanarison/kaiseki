use crate::{
    bincode::decode_slice,
    error::Error,
    feature::Feature,
    term::{Term, TermId},
};
use bincode::{Decode, Encode};

const DICT: &[u8] = include_bytes!("../bin/dict.bin");

#[derive(Encode, Decode)]
pub struct EntryDictionary {
    terms: Vec<Term>,
    features: Vec<Feature>,
}

impl EntryDictionary {
    pub fn new(terms: Vec<Term>, features: Vec<Feature>) -> Self {
        Self { terms, features }
    }

    pub fn load() -> Result<Self, Error> {
        decode_slice(DICT)
    }

    pub fn get_term(&self, id: TermId) -> Option<&Term> {
        self.terms.get(id)
    }

    pub fn get_feat(&self, id: TermId) -> Option<&Feature> {
        self.features.get(id)
    }
}
