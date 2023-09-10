use crate::{verb::Verb, Morpheme, PartOfSpeech};

#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub part_of_speech: PartOfSpeechWord,
    pub reading: String,
}

impl From<&[&Morpheme]> for Word {
    fn from(value: &[&Morpheme]) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PartOfSpeechWord {
    Noun,
    Adnominal,
    Pronoun,
    Verb(Verb),
    Adverb,
    Adjective,
    Particle,
    Copula,
    Prefix,
    Suffix,
    Conjuntion,
    Interjection,
    Filler,
    Counter,
    Symbol,
    Other,
}
