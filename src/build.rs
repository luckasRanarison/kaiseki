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

pub fn read_mecab_file(filename: &str) -> Result<String, Error> {
    let path = Path::new("dict").join(filename);
    let bytes = fs::read(path)?;
    let (buffer, _, _) = EUC_JP.decode(&bytes);

    Ok(buffer.to_string())
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
    let mut lines = buffer.lines();
    let header = lines.next().unwrap();
    let header: Vec<_> = header.split_whitespace().collect();
    let row: usize = header[0].parse()?;
    let col: usize = header[1].parse()?;
    let mut cost_matrix = vec![vec![0; row]; col];

    for line in lines {
        let values: Vec<_> = line.split_whitespace().collect();
        let right_id: usize = values[0].parse()?;
        let left_id: usize = values[1].parse()?;
        let cost: i16 = values[2].parse()?;

        cost_matrix[right_id][left_id] = cost;
    }

    let dict_path = Path::new("dict");
    let config = config::standard();
    let mut handle = File::create(dict_path.join("matrix.bin"))?;

    encode_into_std_write(cost_matrix, &mut handle, config)?;

    print!("matrix.bin has been created");

    Ok(())
}

pub fn build_char_def() -> Result<(), Error> {
    let buffer = read_mecab_file("char.def")?;
    let mut char_map = Vec::new();

    for line in buffer.lines() {
        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        let fields: Vec<_> = line.split_whitespace().collect();

        if line.starts_with("0x") {
            let bounds: Vec<_> = fields[0].split("..").collect();

            let (lower, upper) = match bounds.len() {
                1 => {
                    let bound = parse_hex(bounds[0])?;
                    (bound, bound)
                }
                _ => {
                    let lower = parse_hex(bounds[0])?;
                    let upper = parse_hex(bounds[1])?;
                    (lower, upper)
                }
            };

            let mut categories = Vec::new();

            for &category in &fields[1..] {
                if category == "#" {
                    break;
                }

                categories.push(category);
            }

            char_map.push((lower, upper, categories));
        } else {
            //
        }
    }

    panic!("{:?}", char_map);

    Ok(())
}

fn parse_hex(hex: &str) -> Result<u32, Error> {
    let radix = hex.trim_start_matches("0x");
    let parsed = u32::from_str_radix(radix, 16)?;

    Ok(parsed)
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
