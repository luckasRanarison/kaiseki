use crate::{inflection::Inflection, morpheme::Morpheme};

use std::fmt;

use mecab_types::pos::PartOfSpeech;

#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub base_form: String,
    pub class: WordClass,
    pub morphemes: Vec<Morpheme>,
    pub inflections: Vec<Inflection>,
}

impl Word {
    pub fn from_morphemes(morphemes: &[Morpheme]) -> Option<Self> {
        morphemes.first().map(|main| {
            let start = main.start;
            let end = morphemes.last().map_or(main.end, |m| m.end);
            let text = morphemes.iter().map(|m| m.text.to_owned()).collect();
            let base_form = main.base_form.as_ref().unwrap_or(&main.text).to_owned();
            let class = WordClass::from(main);
            let inflections = Inflection::from_morphemes(morphemes);
            let morphemes = morphemes.to_vec();

            Self {
                text,
                start,
                end,
                base_form,
                class,
                morphemes,
                inflections,
            }
        })
    }

    pub fn is_noun(&self) -> bool {
        self.class == WordClass::Noun
    }

    pub fn is_pronoun(&self) -> bool {
        self.class == WordClass::Pronoun
    }

    pub fn is_adnominal(&self) -> bool {
        self.class == WordClass::PreNoun
    }

    pub fn is_particle(&self) -> bool {
        self.class == WordClass::Particle
    }

    pub fn is_verb(&self) -> bool {
        self.class == WordClass::Verb
    }

    pub fn is_adverb(&self) -> bool {
        self.class == WordClass::Adverb
    }

    pub fn is_auxiliary_verb(&self) -> bool {
        self.class == WordClass::AuxiliaryVerb
    }

    pub fn is_adjective(&self) -> bool {
        self.class == WordClass::Adjective
    }

    pub fn is_prefix(&self) -> bool {
        self.class == WordClass::Prefix
    }

    pub fn is_suffix(&self) -> bool {
        self.class == WordClass::Suffix
    }

    pub fn is_counter(&self) -> bool {
        self.class == WordClass::Expression
    }

    pub fn is_filler(&self) -> bool {
        self.class == WordClass::Filler
    }

    pub fn is_interjection(&self) -> bool {
        self.class == WordClass::Interjection
    }

    pub fn is_conjunction(&self) -> bool {
        self.class == WordClass::Conjunction
    }

    pub fn is_expression(&self) -> bool {
        self.class == WordClass::Expression
    }

    pub fn has_inflections(&self) -> bool {
        !self.inflections.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WordClass {
    Noun,
    PreNoun,
    Pronoun,
    Particle,
    Verb,
    AuxiliaryVerb,
    Adverb,
    Adjective,
    Prefix,
    Suffix,
    Counter,
    Conjunction,
    Filler,
    Interjection,
    Expression,
    Unclassified,
}

impl WordClass {
    fn from_pos(value: PartOfSpeech) -> Self {
        match value {
            PartOfSpeech::Noun => Self::Noun,
            PartOfSpeech::Verb => Self::Verb,
            PartOfSpeech::AuxiliaryVerb => Self::AuxiliaryVerb,
            PartOfSpeech::Adverb => Self::Adverb,
            PartOfSpeech::Adjective => Self::Adjective,
            PartOfSpeech::Adnominal => Self::PreNoun,
            PartOfSpeech::Particle => Self::Particle,
            PartOfSpeech::Conjunction => Self::Conjunction,
            PartOfSpeech::Prefix => Self::Prefix,
            PartOfSpeech::Filler => Self::Filler,
            PartOfSpeech::Interjection => Self::Interjection,
            _ => Self::Unclassified,
        }
    }
}

impl From<&Morpheme> for WordClass {
    fn from(value: &Morpheme) -> Self {
        match value {
            value if value.is_pronoun() => Self::Pronoun,
            value if value.is_adjectivial_noun() => Self::Adjective,
            value if value.is_counter() => Self::Counter,
            value if value.is_suffix() => Self::Suffix,
            value if value.is_expression() => Self::Expression,
            _ => WordClass::from_pos(value.part_of_speech),
        }
    }
}

impl fmt::Display for WordClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WordClass::PreNoun => write!(f, "Pre-noun"),
            WordClass::AuxiliaryVerb => write!(f, "Auxiliary verb"),
            _ => write!(f, "{:?}", self),
        }
    }
}
