use crate::error::Error;

#[derive(Debug, Default, PartialEq)]
pub struct Row<'a> {
    pub surface_form: &'a str,
    pub left_id: u16,
    pub right_id: u16,
    pub cost: i16,
    pub main_pos: &'a str,
    pub sub_pos1: Option<&'a str>,
    pub sub_pos2: Option<&'a str>,
    pub sub_pos3: Option<&'a str>,
    pub conjugation_type: Option<&'a str>,
    pub conjugation_form: Option<&'a str>,
    pub base_form: Option<&'a str>,
    pub reading: Option<&'a str>,
    pub pronounciation: Option<&'a str>,
}

impl<'a> TryFrom<&'a str> for Row<'a> {
    type Error = Error;

    fn try_from(line: &'a str) -> Result<Self, Error> {
        let values: Vec<_> = line.split(',').collect();
        let map = |idx: usize| values.get(idx).filter(|val| *val != &"*").cloned();

        Ok(Row {
            surface_form: values[0],
            left_id: values[1].parse()?,
            right_id: values[2].parse()?,
            cost: values[3].parse()?,
            main_pos: values[4],
            sub_pos1: map(5),
            sub_pos2: map(6),
            sub_pos3: map(7),
            conjugation_type: map(8),
            conjugation_form: map(9),
            base_form: map(10),
            reading: map(11),
            pronounciation: map(12),
        })
    }
}
