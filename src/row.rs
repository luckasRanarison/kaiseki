#[derive(Debug, PartialEq)]
pub struct Row<'a> {
    pub surface_form: &'a str,
    pub left_id: u16,
    pub right_id: u16,
    pub cost: i16,
    pub pos: &'a str,
    pub pos_sub1: &'a str,
    pub pos_sub2: &'a str,
    pub pos_sub3: &'a str,
    pub conjugation_type: &'a str,
    pub conjugation_form: &'a str,
    pub base_form: &'a str,
    pub reading: &'a str,
    pub pronounciation: &'a str,
}

impl Default for Row<'_> {
    fn default() -> Self {
        Self {
            surface_form: "*",
            left_id: 0,
            right_id: 0,
            cost: 0,
            pos: "*",
            pos_sub1: "*",
            pos_sub2: "*",
            pos_sub3: "*",
            conjugation_type: "*",
            conjugation_form: "*",
            base_form: "*",
            reading: "*",
            pronounciation: "*",
        }
    }
}

impl<'a> Row<'a> {
    pub fn from_line(line: &'a str) -> Row {
        let values: Vec<_> = line.split(",").collect();

        Row {
            surface_form: values[0],
            left_id: values[1].parse().expect("failed to parse left_id"),
            right_id: values[2].parse().expect("failed to parse right_id"),
            cost: values[3].parse().expect("failed to parse cost"),
            pos: values[4],
            pos_sub1: values[5],
            pos_sub2: values[6],
            pos_sub3: values[7],
            conjugation_type: values[8],
            conjugation_form: values[9],
            base_form: values[10],
            reading: values[11],
            pronounciation: values[12],
        }
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
                base_form: "よ",
                reading: "ヨ",
                pos: "その他",
                pos_sub1: "間投",
                pronounciation: "ヨ",
                ..Default::default()
            },
            Row {
                surface_form: "ァ",
                left_id: 1,
                right_id: 1,
                cost: 2356,
                pos: "その他",
                pos_sub1: "間投",
                base_form: "ァ",
                reading: "ァ",
                pronounciation: "ア",
                ..Default::default()
            },
        ];
        let entries: Vec<Row> = buffer.lines().map(Row::from_line).collect();

        assert_eq!(expected, entries);
    }
}
