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
    pub base_form: Option<String>,
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
            base_form: feature.base_form,
            reading: feature.reading,
        }
    }

    pub fn has_sub_pos(&self, pos: &SubPartOfSpeech) -> bool {
        self.sub_part_of_speech.contains(pos)
    }

    pub fn is_verb(&self) -> bool {
        self.part_of_speech == PartOfSpeech::Verb
    }

    pub fn is_adjective(&self) -> bool {
        self.part_of_speech == PartOfSpeech::Adjective
    }

    pub fn is_symbol(&self) -> bool {
        self.part_of_speech == PartOfSpeech::Symbol
    }

    pub fn is_inflection(&self) -> bool {
        self.is_ni_verb()
            || self.is_aux_verb()
            || self.is_sfx_verb()
            || self.is_te()
            || self.is_ba()
    }

    fn is_ni_verb(&self) -> bool {
        self.is_verb() && self.has_sub_pos(&SubPartOfSpeech::NonIndependent)
    }

    fn is_sfx_verb(&self) -> bool {
        self.is_verb() && self.has_sub_pos(&SubPartOfSpeech::Suffix)
    }

    fn is_te(&self) -> bool {
        matches!(self.text.as_str(), "て" | "で")
            && self.has_sub_pos(&SubPartOfSpeech::ConjunctiveParticle)
    }

    fn is_ba(&self) -> bool {
        self.text == "ば" && self.has_sub_pos(&SubPartOfSpeech::ConjunctiveParticle)
    }

    fn is_aux_verb(&self) -> bool {
        self.part_of_speech == PartOfSpeech::AuxiliaryVerb
    }
}
