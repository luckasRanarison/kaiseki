use crate::{
    term::{Term, TermId},
    utils::BINCODE_CONFIG,
};
use anyhow::Error;
use bincode::decode_from_slice;

const TERMS: &'static [u8] = include_bytes!("../dict/term.bin");

pub struct EntryDictionary {
    terms: Vec<Term>,
}

impl EntryDictionary {
    pub fn load() -> Result<Self, Error> {
        let (terms, _) = decode_from_slice(TERMS, *BINCODE_CONFIG)?;

        Ok(Self { terms })
    }

    pub fn get_term(&self, id: TermId) -> Option<&Term> {
        self.terms.get(id)
    }
}
