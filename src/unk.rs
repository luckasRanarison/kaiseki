use crate::term::Term;
use anyhow::Error;
use bincode::{config, decode_from_slice, Decode, Encode};
use std::collections::HashMap;

const UNK: &'static [u8] = include_bytes!("../dict/unk.bin");

type TermMap = HashMap<String, Vec<Term>>;

#[derive(Debug, Encode, Decode)]
pub struct UnkDictionary {
    terms: TermMap,
}

impl UnkDictionary {
    pub fn new(terms: TermMap) -> Self {
        Self { terms }
    }
}

impl UnkDictionary {
    pub fn load() -> Result<Self, Error> {
        let config = config::standard();
        let (unk_dict, _) = decode_from_slice(UNK, config)?;

        Ok(unk_dict)
    }

    pub fn get_terms(&self, key: &str) -> Option<&Vec<Term>> {
        self.terms.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_term() {
        let unk_dict = UnkDictionary::load().unwrap();
        let terms = unk_dict.get_terms("DEFAULT").unwrap();
        let term = &terms[0];

        assert_eq!(5, term.context_id);
        assert_eq!(4769, term.cost);
    }
}
