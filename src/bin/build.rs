use encoding_rs::EUC_JP;
use fst::MapBuilder;
use kaiseki::{bincode::encode, error::Error, mecab::*, Feature};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File},
    path::Path,
};

type TermMap = BTreeMap<String, Vec<Term>>;
type FeatMap = BTreeMap<String, Vec<Feature>>;

fn get_entry_map() -> Result<(TermMap, FeatMap), Error> {
    println!("> Decoding mecab IPA dictionary files...");

    let mut decoded_files = Vec::new();
    let mut rows = Vec::new();
    let mut term_map = BTreeMap::new();
    let mut feat_map = BTreeMap::new();

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

    println!("File decoding complete ✓");

    Ok((term_map, feat_map))
}

fn read_mecab_file(filename: &str) -> Result<String, Error> {
    let path = Path::new("mecab").join(filename);
    let bytes = fs::read(path)?;
    let (buffer, _, _) = EUC_JP.decode(&bytes);

    Ok(buffer.to_string())
}

fn build_fst(term_map: &TermMap) -> Result<(), Error> {
    println!("> Building FST...");

    let handle = File::create("bin/term.fst")?;
    let mut map_builder = MapBuilder::new(handle)?;
    let mut id = 0u64;

    for (key, terms) in term_map {
        let len = terms.len() as u64;
        let value = id << 5 | len; // encode the offset, assert!(len < 32)
        map_builder.insert(key, value)?;
        id += len;
    }

    map_builder.finish()?;

    println!("term.fst has been created ✓");

    Ok(())
}

fn build_dict(term_map: TermMap, feat_map: FeatMap) -> Result<(), Error> {
    println!("> Building entry dictionary...");

    let mut terms = Vec::new();
    let mut features = Vec::new();

    for value in term_map.values() {
        terms.extend(value.clone());
    }

    for value in feat_map.values() {
        features.extend(value.clone());
    }

    let dict = EntryDictionary::new(terms, features);
    let mut handle = File::create("bin/dict.bin")?;

    encode(dict, &mut handle)?;

    println!("dict.bin has been created ✓");

    Ok(())
}

fn build_matrix() -> Result<(), Error> {
    println!("> Building cost matrix...");

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

    let cost_matrix = CostMatrix::new(cost_matrix);
    let mut handle = File::create("bin/matrix.bin")?;

    encode(cost_matrix, &mut handle)?;

    println!("matrix.bin has been created ✓");

    Ok(())
}

fn build_char_def() -> Result<(), Error> {
    println!("> Buiding char definition..");

    let buffer = read_mecab_file("char.def")?;
    let mut boundaries = Vec::new();
    let mut category_def_map = HashMap::new();

    for line in buffer.lines() {
        if line.starts_with('#') || line.is_empty() {
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
    let mut handle = File::create("bin/char.bin")?;

    encode(char_table, &mut handle)?;

    println!("char.bin has been created ✓");

    Ok(())
}

fn build_unk() -> Result<(), Error> {
    println!("> Building unknown dictionary...");

    let buffer = read_mecab_file("unk.def")?;
    let mut unk_term_map = HashMap::new();
    let mut feature = Vec::new();

    for (id, line) in buffer.lines().enumerate() {
        let row = Row::try_from(line)?;
        let term = Term::from(&row);

        unk_term_map
            .entry(row.surface_form.to_string())
            .or_insert_with(Vec::new)
            .push((id, term));

        feature.push(Feature::from(&row));
    }

    let unk_dict = UnknownDictionary::new(unk_term_map, feature);
    let mut handle = File::create("bin/unk.bin")?;

    encode(unk_dict, &mut handle)?;

    println!("unk.bin has been created ✓");

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
    let invoke = matches!(invoke, 1);
    let group: u8 = fields[2].parse()?;
    let group = matches!(group, 1);
    let length = fields[3].parse()?;
    let category = CharCategory::new(name.clone(), invoke, group, length);

    Ok((name, category))
}

fn get_csv_files() -> Result<Vec<String>, Error> {
    let entries = fs::read_dir("mecab")?;
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
    println!("----- Building kaiseki ressources -----\n");

    let (term_map, feat_map) = get_entry_map()?;

    build_char_def()?;
    build_unk()?;
    build_matrix()?;
    build_fst(&term_map)?;
    build_dict(term_map, feat_map)?;

    println!("\nBuild complete!");

    Ok(())
}
