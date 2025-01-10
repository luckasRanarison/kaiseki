use std::num::ParseIntError;

#[derive(Debug, Default, PartialEq)]
pub struct Row<'a> {
    pub surface_form: &'a str,
    pub left_id: u16,
    pub right_id: u16,
    pub cost: i16,
    pub part_of_speech: &'a str,
    pub sub_part_of_speech1: Option<&'a str>,
    pub sub_part_of_speech2: Option<&'a str>,
    pub sub_part_of_speech3: Option<&'a str>,
    pub conjugation_type: Option<&'a str>,
    pub conjugation_form: Option<&'a str>,
    pub base_form: Option<&'a str>,
    pub reading: Option<&'a str>,
    pub pronounciation: Option<&'a str>,
}

impl<'a> TryFrom<&'a str> for Row<'a> {
    type Error = ParseIntError;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let values = line.split(',').collect::<Vec<_>>();

        let get_optional = |idx: usize| {
            values
                .get(idx)
                .cloned()
                .filter(|&val| val != "*" && !val.is_empty())
        };

        Ok(Row {
            surface_form: values[0],
            left_id: values[1].parse()?,
            right_id: values[2].parse()?,
            cost: values[3].parse()?,
            part_of_speech: values[4],
            sub_part_of_speech1: get_optional(5),
            sub_part_of_speech2: get_optional(6),
            sub_part_of_speech3: get_optional(7),
            conjugation_type: get_optional(8),
            conjugation_form: get_optional(9),
            base_form: get_optional(10),
            reading: get_optional(11),
            pronounciation: get_optional(12),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Row;

    #[test]
    fn test_row_parsing() {
        let line = "真,560,560,7716,接頭詞,名詞接続,*,*,*,*,真,マ,";
        let row = Row::try_from(line).unwrap();

        let expected = Row {
            surface_form: "真",
            left_id: 560,
            right_id: 560,
            cost: 7716,
            part_of_speech: "接頭詞",
            sub_part_of_speech1: Some("名詞接続"),
            sub_part_of_speech2: None,
            sub_part_of_speech3: None,
            conjugation_type: None,
            conjugation_form: None,
            base_form: Some("真"),
            reading: Some("マ"),
            pronounciation: None,
        };

        assert_eq!(row, expected);
    }
}
