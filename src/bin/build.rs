use bincode::encode_into_std_write;
use encoding_rs::EUC_JP;
use fst::MapBuilder;
use kaiseki::{build::*, error::Error};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File},
    path::Path,
};

type TermMap = BTreeMap<String, Vec<Term>>;
type FeatMap = HashMap<String, Vec<Feature>>;

pub fn get_entry_map() -> Result<(TermMap, FeatMap), Error> {
    println!("Decoding mecab IPA dictionary files...");

    let mut decoded_files = Vec::new();
    let mut rows = Vec::new();
    let mut term_map = BTreeMap::new();
    let mut feat_map = HashMap::new();

    for file in get_csv_files()? {
        decoded_files.push(read_mecab_file(&file)?);
    }

    for file in &decoded_files {
        for line in file.lines() {
            rows.push(Row::try_from(line)?);
        }
    }

    for row in rows {
        term_map
            .entry(row.surface_form.to_owned())
            .or_insert_with(Vec::new)
            .push(Term::from(&row));

        feat_map
            .entry(row.surface_form.to_owned())
            .or_insert_with(Vec::new)
            .push(Feature::from(&row));
    }

    Ok((term_map, feat_map))
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
        let value = id << 5 | len; // encode the offset, assert!(len < 32)
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

    let path = Path::new("dict").join("term.bin");
    let mut handle = File::create(path)?;

    encode_into_std_write(term_values, &mut handle, *BINCODE_CONFIG)?;

    println!("term.bin has been created");

    Ok(())
}

pub fn build_feature(feat_map: FeatMap) -> Result<(), Error> {
    println!("Building feature...");

    let mut feat_values = Vec::new();

    for value in feat_map.values() {
        feat_values.extend(value.clone());
    }

    let path = Path::new("dict").join("feature.bin");
    let mut handle = File::create(path)?;

    encode_into_std_write(feat_values, &mut handle, *BINCODE_CONFIG)?;

    println!("feature.bin has been created");

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

    let path = Path::new("dict").join("matrix.bin");
    let mut handle = File::create(path)?;

    encode_into_std_write(cost_matrix, &mut handle, *BINCODE_CONFIG)?;

    println!("matrix.bin has been created");

    Ok(())
}

pub fn build_char_def() -> Result<(), Error> {
    println!("Buiding char definition..");

    let buffer = read_mecab_file("char.def")?;
    let mut boundaries = Vec::new();
    let mut category_def_map = HashMap::new();

    for line in buffer.lines() {
        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        if line.starts_with("0x") {
            let values = parse_char_map(line)?;
            boundaries.push(values);
        } else {
            let (name, value) = parse_category(line)?;
            category_def_map.insert(name, value);
        }
    }

    let mut char_category_map = vec![vec![]; 0xFFFF + 1];

    for (lower, upper, keys) in boundaries {
        let mut categories = Vec::new();

        for key in keys {
            if let Some(category) = category_def_map.get(key) {
                categories.push(category.clone());
            }
        }

        for index in lower..=upper {
            char_category_map[index as usize] = categories.clone();
        }
    }

    let char_table = CharTable::new(char_category_map);
    let path = Path::new("dict").join("char.bin");
    let mut handle = File::create(path)?;

    encode_into_std_write(char_table, &mut handle, *BINCODE_CONFIG)?;

    println!("char.bin has been created");

    Ok(())
}

pub fn build_unk() -> Result<(), Error> {
    println!("Building unknown dictionary...");

    let buffer = read_mecab_file("unk.def")?;
    let mut unk_term_map = HashMap::new();

    for (id, line) in buffer.lines().enumerate() {
        let row = Row::try_from(line)?;
        let term = Term::from(&row);

        unk_term_map
            .entry(row.surface_form.to_string())
            .or_insert_with(Vec::new)
            .push((id, term));
    }

    let unk_dict = UnknownDictionary::new(unk_term_map);
    let path = Path::new("dict").join("unk.bin");
    let mut handle = File::create(path)?;

    encode_into_std_write(unk_dict, &mut handle, *BINCODE_CONFIG)?;

    println!("unk.bin has been created");

    Ok(())
}

fn parse_char_map(line: &str) -> Result<(u32, u32, Vec<&str>), Error> {
    let fields: Vec<_> = line.split_whitespace().collect();
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

    Ok((lower, upper, categories))
}

fn parse_hex(hex: &str) -> Result<u32, Error> {
    let radix = hex.trim_start_matches("0x");
    let parsed = u32::from_str_radix(radix, 16)?;

    Ok(parsed)
}

fn parse_category(line: &str) -> Result<(String, CharCategory), Error> {
    let fields: Vec<_> = line.split_whitespace().collect();
    let name = fields[0].to_owned();
    let invoke: u8 = fields[1].parse()?;
    let invoke = match invoke {
        1 => true,
        _ => false,
    };
    let group: u8 = fields[2].parse()?;
    let group = match group {
        1 => true,
        _ => false,
    };
    let length = fields[3].parse()?;
    let category = CharCategory::new(name.clone(), invoke, group, length);

    Ok((name, category))
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

fn main() -> Result<(), Error> {
    let (term_map, feat_map) = get_entry_map()?;

    build_char_def()?;
    build_unk()?;
    build_matrix()?;
    build_fst(&term_map)?;
    build_term(&term_map)?;
    build_feature(feat_map)?;

    println!("Build complete!");

    Ok(())
}
