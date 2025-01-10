use crate::{
    features::Feature,
    term::{Term, TermId},
};

use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub struct EntryDictionary {
    terms: Vec<Term>,
    features: Vec<Feature>,
}

impl EntryDictionary {
    pub fn new(terms: Vec<Term>, features: Vec<Feature>) -> Self {
        Self { terms, features }
    }

    pub fn get_term(&self, id: TermId) -> Option<&Term> {
        self.terms.get(id)
    }

    pub fn get_feature(&self, id: TermId) -> Option<&Feature> {
        self.features.get(id)
    }
}
