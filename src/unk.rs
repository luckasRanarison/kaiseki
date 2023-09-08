use crate::{
    error::Error,
    feature::Feature,
    term::{Term, TermId},
    utils::BINCODE_CONFIG,
};
use bincode::{decode_from_slice, Decode, Encode};
use std::collections::HashMap;

const UNK: &'static [u8] = include_bytes!("../mecab/unk.bin");

type TermMap = HashMap<String, Vec<(TermId, Term)>>;

#[derive(Debug, Encode, Decode)]
pub struct UnknownDictionary {
    terms: TermMap,
    feature: Vec<Feature>,
}

impl UnknownDictionary {
    pub fn new(terms: TermMap, feature: Vec<Feature>) -> Self {
        Self { terms, feature }
    }
}

impl UnknownDictionary {
    pub fn load() -> Result<Self, Error> {
        let (unk_dict, _) = decode_from_slice(UNK, *BINCODE_CONFIG)?;

        Ok(unk_dict)
    }

    pub fn get_terms(&self, key: &str) -> Option<&Vec<(TermId, Term)>> {
        self.terms.get(key)
    }

    pub fn get_feat(&self, id: TermId) -> Option<&Feature> {
        self.feature.get(id)
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
