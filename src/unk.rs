use crate::term::{Term, TermId};
use anyhow::Error;
use bincode::{config, decode_from_slice, Decode, Encode};
use std::collections::HashMap;

const UNK: &'static [u8] = include_bytes!("../dict/unk.bin");

type TermMap = HashMap<String, Vec<(TermId, Term)>>;

#[derive(Debug, Encode, Decode)]
pub struct UnknownDictionary {
    terms: TermMap,
}

impl UnknownDictionary {
    pub fn new(terms: TermMap) -> Self {
        Self { terms }
    }
}

impl UnknownDictionary {
    pub fn load() -> Result<Self, Error> {
        let config = config::standard();
        let (unk_dict, _) = decode_from_slice(UNK, config)?;

        Ok(unk_dict)
    }

    pub fn get_terms(&self, key: &str) -> Option<&Vec<(TermId, Term)>> {
        self.terms.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_term() {
        let unk_dict = UnknownDictionary::load().unwrap();
        let terms = unk_dict.get_terms("DEFAULT").unwrap();
        let (id, term) = &terms[0];

        assert_eq!(0, *id);
        assert_eq!(5, term.context_id);
        assert_eq!(4769, term.cost);
    }
}
