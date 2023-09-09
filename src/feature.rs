use crate::{conjugation::ConjugationForm, row::Row, ConjugationType, PosMain, PosSub};
use bincode::{Decode, Encode};

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
pub struct Feature {
    pub main_pos: PosMain,
    pub sub_pos: Vec<PosSub>,
    pub conjugation_type: Option<ConjugationType>,
    pub conjugation_form: Option<ConjugationForm>,
    pub base_form: Option<String>,
    pub reading: Option<String>,
}

impl From<&Row<'_>> for Feature {
    fn from(value: &Row) -> Self {
        let mut sub_pos = Vec::new();
        let sub_pos_row = &[value.sub_pos1, value.sub_pos2, value.sub_pos3];

        for pos in sub_pos_row.iter().flatten() {
            sub_pos.push(PosSub::from(*pos));
        }

        Self {
            sub_pos,
            main_pos: PosMain::from(value.main_pos),
            conjugation_type: value.conjugation_type.map(ConjugationType::from),
            conjugation_form: value.conjugation_form.map(ConjugationForm::from),
            base_form: value.base_form.map(str::to_string),
            reading: value.reading.map(str::to_string),
        }
    }
}
