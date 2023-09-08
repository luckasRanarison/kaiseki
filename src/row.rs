use anyhow::Error;

#[derive(Debug, Default, PartialEq)]
pub struct Row<'a> {
    pub surface_form: &'a str,
    pub left_id: u16,
    pub right_id: u16,
    pub cost: i16,
    pub pos: &'a str,
    pub pos_sub1: Option<&'a str>,
    pub pos_sub2: Option<&'a str>,
    pub pos_sub3: Option<&'a str>,
    pub conjugation_type: Option<&'a str>,
    pub conjugation_form: Option<&'a str>,
    pub base_form: Option<&'a str>,
    pub reading: Option<&'a str>,
    pub pronounciation: Option<&'a str>,
}

impl<'a> TryFrom<&'a str> for Row<'a> {
    type Error = Error;

    fn try_from(line: &'a str) -> Result<Self, Error> {
        let values: Vec<_> = line.split(",").collect();
        let map = |idx: usize| values.get(idx).filter(|val| *val != &"*").cloned();

        Ok(Row {
            surface_form: values[0],
            left_id: values[1].parse()?,
            right_id: values[2].parse()?,
            cost: values[3].parse()?,
            pos: values[4],
            pos_sub1: map(5),
            pos_sub2: map(6),
            pos_sub3: map(7),
            conjugation_type: map(8),
            conjugation_form: map(9),
            base_form: map(10),
            reading: map(11),
            pronounciation: map(12),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::read_mecab_file;

    #[test]
    fn test_csv_row() {
        let buffer = read_mecab_file("Others.csv").unwrap();
        let expected = vec![
            Row {
                surface_form: "よ",
                left_id: 1,
                right_id: 1,
                cost: 6514,
                base_form: Some("よ"),
                reading: Some("ヨ"),
                pos: "その他",
                pos_sub1: Some("間投"),
                pronounciation: Some("ヨ"),
                ..Default::default()
            },
            Row {
                surface_form: "ァ",
                left_id: 1,
                right_id: 1,
                cost: 2356,
                pos: "その他",
                pos_sub1: Some("間投"),
                base_form: Some("ァ"),
                reading: Some("ァ"),
                pronounciation: Some("ア"),
                ..Default::default()
            },
        ];
        let entries: Vec<Row> = buffer
            .lines()
            .map(|line| Row::try_from(line).unwrap())
            .collect();

        assert_eq!(expected, entries);
    }
}
