use crate::{conjugation::*, pos::*, Feature};

#[derive(Debug, Clone, PartialEq)]
pub struct Morpheme {
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub part_of_speech: PartOfSpeech,
    pub sub_part_of_speech: Vec<SubPartOfSpeech>,
    pub conjugation_type: Option<ConjugationType>,
    pub conjugation_form: Option<ConjugationForm>,
    pub reading: Option<String>,
}

impl Morpheme {
    pub fn new(text: String, start: usize, end: usize, feature: Feature) -> Self {
        Self {
            text,
            start,
            end,
            part_of_speech: feature.part_of_speech,
            sub_part_of_speech: feature.sub_part_of_speech,
            conjugation_type: feature.conjugation_type,
            conjugation_form: feature.conjugation_form,
            reading: feature.reading,
        }
    }

    pub fn is_verb(&self) -> bool {
        matches!(self.part_of_speech, PartOfSpeech::Verb)
    }

    pub fn is_ni_verb(&self) -> bool {
        self.is_verb()
            && self
                .sub_part_of_speech
                .contains(&SubPartOfSpeech::NonIndependent)
    }

    pub fn is_conjunctive_particle(&self) -> bool {
        self.sub_part_of_speech
            .contains(&SubPartOfSpeech::ConjunctiveParticle)
    }

    pub fn is_auxiliary_verb(&self) -> bool {
        matches!(self.part_of_speech, PartOfSpeech::AuxiliaryVerb)
    }

    pub fn is_inflection(&self) -> bool {
        self.is_ni_verb() || self.is_auxiliary_verb() || self.is_conjunctive_particle()
    }
}
