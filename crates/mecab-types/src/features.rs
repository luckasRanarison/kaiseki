use crate::{conjugation::*, pos::*, row::Row};

use std::str::FromStr;

use bincode::{Decode, Encode};
use strum::ParseError;

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
pub struct Feature {
    pub part_of_speech: PartOfSpeech,
    pub sub_part_of_speech: Vec<SubPartOfSpeech>,
    pub conjugation_type: Option<ConjugationType>,
    pub conjugation_form: Option<ConjugationForm>,
    pub base_form: Option<String>,
    pub reading: Option<String>,
}

impl TryFrom<&Row<'_>> for Feature {
    type Error = ParseError;

    fn try_from(value: &Row) -> Result<Self, Self::Error> {
        let sub_pos_row = &[
            value.sub_part_of_speech1,
            value.sub_part_of_speech2,
            value.sub_part_of_speech3,
        ];
        let sub_part_of_speech = sub_pos_row
            .iter()
            .flatten()
            .map(|&pos| SubPartOfSpeech::from_str(pos))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            sub_part_of_speech,
            part_of_speech: PartOfSpeech::from_str(value.part_of_speech)?,
            base_form: value.base_form.map(str::to_string),
            reading: value.reading.map(str::to_string),
            conjugation_type: match value.conjugation_type {
                Some(value) => Some(ConjugationType::try_from(value)?),
                None => None,
            },
            conjugation_form: match value.conjugation_form {
                Some(value) => Some(ConjugationForm::try_from(value)?),
                None => None,
            },
        })
    }
}
