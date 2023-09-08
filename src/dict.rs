use crate::term::{Term, TermId};
use anyhow::Error;
use bincode::{config, decode_from_slice};

const TERMS: &'static [u8] = include_bytes!("../dict/term.bin");

pub struct EntryDictionary {
    terms: Vec<Term>,
}

impl EntryDictionary {
    pub fn load() -> Result<Self, Error> {
        let config = config::standard();
        let (terms, _) = decode_from_slice(TERMS, config)?;

        Ok(Self { terms })
    }

    pub fn get_term(&self, id: TermId) -> Option<&Term> {
        self.terms.get(id)
    }
}
