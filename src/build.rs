use crate::{row::Row, term::Term};
use anyhow::{Error, Ok};
use bincode::{config, encode_into_std_write};
use encoding_rs::EUC_JP;
use fst::MapBuilder;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    path::Path,
};

pub type TermMap = BTreeMap<String, Vec<Term>>;

pub fn get_term_map() -> Result<TermMap, Error> {
    let mut decoded_files = Vec::new();
    let mut rows = Vec::new();
    let mut term_map = BTreeMap::new();

    for file in get_csv_files()? {
        let decoded = read_mecab_file(&file)?;
        decoded_files.push(decoded);
    }

    for file in &decoded_files {
        for line in file.lines() {
            rows.push(Row::from_line(line));
        }
    }

    for row in rows {
        let term = Term::new(row.left_id, row.cost); // left_id == right_id

        term_map
            .entry(row.surface_form.to_owned())
            .or_insert_with(Vec::new)
            .push(term)
    }

    Ok(term_map)
}

pub fn build_fst(term_map: &TermMap) -> Result<(), Error> {
    println!("Building FST...");

    let path = Path::new("dict").join("term.fst");
    let handle = File::create(path)?;
    let mut map_builder = MapBuilder::new(handle)?;
    let mut id = 0u64;

    for (key, terms) in term_map {
        let len = terms.len() as u64;
        let value = id << 5 | len; // encode the offset, max len == 20
        map_builder.insert(key, value)?;
        id += len;
    }

    map_builder.finish()?;

    println!("term.fst has been created");

    Ok(())
}

pub fn build_term(term_map: &TermMap) -> Result<(), Error> {
    println!("Building term values...");

    let mut term_values = Vec::new();

    for value in term_map.values() {
        term_values.extend(value.clone());
    }

    let config = config::standard();
    let path = Path::new("dict").join("term.bin");
    let mut handle = File::create(path)?;

    encode_into_std_write(term_values, &mut handle, config)?;

    println!("term.bin has been created");

    Ok(())
}

pub fn build_matrix() -> Result<(), Error> {
    println!("Building cost matrix...");

    let buffer = read_mecab_file("matrix.def")?;
    let mut cost_matrix = vec![vec![0; 1316]; 1316];

    for line in buffer.lines().skip(1) {
        let values: Vec<_> = line.split(" ").collect();
        let left_id: usize = values[0].parse().expect("failed to parse left_id");
        let right_id: usize = values[1].parse().expect("failed to parse right_id");
        let cost: i32 = values[2].parse().expect("failed to parse cost");

        cost_matrix[left_id][right_id] = cost;
    }

    let dict_path = Path::new("dict");
    let config = config::standard();
    let mut handle = File::create(dict_path.join("matrix.bin"))?;

    encode_into_std_write(cost_matrix, &mut handle, config)?;

    print!("matrix.bin has been created");

    Ok(())
}

pub fn read_mecab_file(filename: &str) -> Result<String, Error> {
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
