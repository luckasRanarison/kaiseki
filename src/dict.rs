use crate::{
    error::Error,
    feature::Feature,
    term::{Term, TermId},
    utils::BINCODE_CONFIG,
};
use bincode::{decode_from_slice, Decode, Encode};

const DICT: &'static [u8] = include_bytes!("../bin/dict.bin");

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
        let (dict, _) = decode_from_slice(DICT, *BINCODE_CONFIG)?;

        Ok(dict)
    }

    pub fn get_term(&self, id: TermId) -> Option<&Term> {
        self.terms.get(id)
    }

    pub fn get_feat(&self, id: TermId) -> Option<&Feature> {
        self.features.get(id)
    }
}
