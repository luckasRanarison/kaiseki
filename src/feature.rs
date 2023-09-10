use crate::{conjugation::*, pos::*, row::Row};
use bincode::{Decode, Encode};

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
pub struct Feature {
    pub part_of_speech: PartOfSpeech,
    pub sub_part_of_speech: Vec<SubPartOfSpeech>,
    pub conjugation_type: Option<ConjugationType>,
    pub conjugation_form: Option<ConjugationForm>,
    pub base_form: Option<String>,
    pub reading: Option<String>,
}

impl From<&Row<'_>> for Feature {
    fn from(value: &Row) -> Self {
        let sub_pos_row = &[
            value.sub_part_of_speech1,
            value.sub_part_of_speech2,
            value.sub_part_of_speech3,
        ];
        let sub_part_of_speech = sub_pos_row
            .iter()
            .flatten()
            .map(|pos| SubPartOfSpeech::from(*pos))
            .collect();

        Self {
            sub_part_of_speech,
            part_of_speech: PartOfSpeech::from(value.part_of_speech),
            conjugation_type: value.conjugation_type.map(ConjugationType::from),
            conjugation_form: value.conjugation_form.map(ConjugationForm::from),
            base_form: value.base_form.map(str::to_string),
            reading: value.reading.map(str::to_string),
        }
    }
}
