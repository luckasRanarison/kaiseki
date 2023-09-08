use crate::{
    error::Error,
    feature::Feature,
    term::{Term, TermId},
    utils::BINCODE_CONFIG,
};
use bincode::decode_from_slice;

const TERMS: &'static [u8] = include_bytes!("../dict/term.bin");
const FEATURES: &'static [u8] = include_bytes!("../dict/feature.bin");

pub struct EntryDictionary {
    terms: Vec<Term>,
    features: Vec<Feature>,
}

impl EntryDictionary {
    pub fn load() -> Result<Self, Error> {
        let (terms, _) = decode_from_slice(TERMS, *BINCODE_CONFIG)?;
        let (features, _) = decode_from_slice(FEATURES, *BINCODE_CONFIG)?;

        Ok(Self { terms, features })
    }

    pub fn get_term(&self, id: TermId) -> Option<&Term> {
        self.terms.get(id)
    }

    pub fn get_feat(&self, id: TermId) -> Option<&Feature> {
        self.features.get(id)
    }
}
