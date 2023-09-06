use anyhow::Error;
use bincode::{config, encode_into_std_write};
use encoding_rs::EUC_JP;
use fst::MapBuilder;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    path::Path,
};

use crate::term::Term;

pub fn build_fst() -> Result<(), Error> {
    println!("Buiding FST...");

    let mut decoded_files = Vec::new();

    for file in get_csv_files()? {
        let decoded = read_mecab_file(&file)?;
        decoded_files.push(decoded);
    }

    let mut rows = Vec::new();

    for file in &decoded_files {
        for line in file.lines() {
            rows.push(Row::from(line));
        }
    }

    let mut term_map: BTreeMap<String, Vec<Term>> = BTreeMap::new();

    for row in rows {
        let term = Term {
            context_id: row.left_id, // left_id == right_id
            cost: row.cost,
        };

        term_map
            .entry(row.surface_form.to_owned())
            .or_insert_with(Vec::new)
            .push(term)
    }

    let dict_path = Path::new("dict");
    let handle = File::create(dict_path.join("dict.fst"))?;
    let mut map_builder = MapBuilder::new(handle)?;
    let mut id = 0u64;

    for (key, terms) in &term_map {
        let len = terms.len() as u64;
        let value = id << 5 | len; // encode the offset, max len == 20
        map_builder.insert(key, value)?;
        id += len;
    }

    map_builder.finish()?;
    println!("dict.fst has been written to the disk");

    let mut term_values = Vec::new();

    for value in term_map.values() {
        term_values.extend(value.clone());
    }

    let config = config::standard();
    let mut handle = File::create(dict_path.join("dict.bin"))?;

    encode_into_std_write(term_values, &mut handle, config)?;
    println!("dict.bin has been written to the disk");

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Row<'a> {
    surface_form: &'a str,
    left_id: u16,
    right_id: u16,
    cost: i16,
    pos: &'a str,
    pos_sub1: &'a str,
    pos_sub2: &'a str,
    pos_sub3: &'a str,
    conjugation_type: &'a str,
    conjugation_form: &'a str,
    base_form: &'a str,
    reading: &'a str,
    pronounciation: &'a str,
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
    fn from(line: &'a str) -> Row {
        let values: Vec<_> = line.split(",").collect();

        Row {
            surface_form: values[0],
            left_id: values[1].parse::<u16>().expect("failed to parse left_id"),
            right_id: values[2].parse::<u16>().expect("failed to parse right_id"),
            cost: values[3].parse::<i16>().expect("failed to parse cost"),
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

fn read_mecab_file(filename: &str) -> Result<String, Error> {
    let path = Path::new("dict").join(filename);
    let bytes = fs::read(path)?;
    let (buffer, _, _) = EUC_JP.decode(&bytes);

    Ok(buffer.to_string())
}

fn get_csv_files() -> Result<Vec<String>, Error> {
    let path = Path::new("dict");
    let entries = path.read_dir()?;
    let mut files = Vec::new();

    for entry in entries {
        let file_name = entry?.file_name();

        if let Some(file_name) = file_name.to_str() {
            if file_name.ends_with(".csv") {
                files.push(file_name.to_owned());
            }
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::{read_mecab_file, Row};

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
        let entries: Vec<Row> = buffer.lines().map(Row::from).collect();

        assert_eq!(expected, entries);
    }
}
