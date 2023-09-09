use crate::{conjugation::ConjugationForm, row::Row, PartOfSpeech};
use bincode::{Decode, Encode};

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
pub struct Feature {
    pub part_of_speech: Vec<PartOfSpeech>,
    pub conjugation_form: Option<ConjugationForm>,
    pub base_form: Option<String>,
    pub reading: Option<String>,
}

impl From<&Row<'_>> for Feature {
    fn from(value: &Row) -> Self {
        let mut part_of_speech = vec![PartOfSpeech::from(value.pos)];
        let pos_sub = &[value.pos_sub1, value.pos_sub2, value.pos_sub3];

        for pos in pos_sub.iter().flatten() {
            part_of_speech.push(PartOfSpeech::from(*pos));
        }

        Self {
            part_of_speech,
            conjugation_form: value.conjugation_form.map(ConjugationForm::from),
            base_form: value.base_form.map(str::to_string),
            reading: value.reading.map(str::to_string),
        }
    }
}
