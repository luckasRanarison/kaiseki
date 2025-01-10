use crate::{
    features::Feature,
    term::{Term, TermId},
};

use std::collections::HashMap;

use bincode::{Decode, Encode};

type TermMap = HashMap<String, Vec<(TermId, Term)>>;

#[derive(Encode, Decode)]
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
    pub fn get_terms(&self, key: &str) -> Option<&Vec<(TermId, Term)>> {
        self.terms.get(key)
    }

    pub fn get_feat(&self, id: TermId) -> Option<&Feature> {
        self.feature.get(id)
    }
}
