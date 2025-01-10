mod utils;

use crate::utils::{parse_category, parse_char_map, FeatMap, Result, TermMap};

use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File},
    path::Path,
};

use bincode::Encode;
use clap::Parser;
use encoding_rs::EUC_JP;
use fst::MapBuilder;
use mecab_types::{
    bincode::encode, char::CharTable, cost::CostMatrix, dict::EntryDictionary, features::Feature,
    row::Row, term::Term, unk::UnknownDictionary, utils::FlatMatrix,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Command {
    /// Directory containing extracted mecab-ipadic files
    #[arg(short, long)]
    input_dir: String,

    /// Transformed binaries destination directory
    #[arg(short, long)]
    out_dir: String,
}

impl Command {
    fn get_entry_map(&self) -> Result<(TermMap, FeatMap)> {
        let mut decoded_files = Vec::new();
        let mut rows = Vec::new();
        let mut term_map = BTreeMap::new();
        let mut feat_map = BTreeMap::new();

        for file in self.read_csv_files()? {
            decoded_files.push(self.read_mecab_file(&file)?);
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

            let value = Feature::try_from(&row)?;
            feat_map
                .entry(row.surface_form.to_owned())
                .or_insert_with(Vec::new)
                .push(value);
        }

        Ok((term_map, feat_map))
    }

    fn build_fst(&self, term_map: &TermMap) -> Result<()> {
        let handle = self.create_output_file("term.fst")?;
        let mut map_builder = MapBuilder::new(handle)?;
        let mut id = 0u64;

        for (key, terms) in term_map {
            let len = terms.len() as u64;
            let value = id << 5 | len; // encode the offset, assert!(len < 32)
            map_builder.insert(key, value)?;
            id += len;
        }

        map_builder.finish()?;

        Ok(())
    }

    fn build_dict(&self, term_map: TermMap, feat_map: FeatMap) -> Result<()> {
        let mut terms = Vec::new();
        let mut features = Vec::new();

        for value in term_map.values() {
            terms.extend(value.clone());
        }

        for value in feat_map.values() {
            features.extend(value.clone());
        }

        let dict = EntryDictionary::new(terms, features);

        self.write_output_file("dict.bin", dict)?;

        Ok(())
    }

    fn build_matrix(&self) -> Result<()> {
        let buffer = self.read_mecab_file("matrix.def")?;
        let mut lines = buffer.lines();
        let header = lines.next().unwrap();
        let header: Vec<_> = header.split_whitespace().collect();
        let row: usize = header[0].parse()?;
        let col: usize = header[1].parse()?;
        let mut cost_matrix = FlatMatrix::new(row, col);

        for line in lines {
            let values: Vec<_> = line.split_whitespace().collect();
            let right_id: usize = values[0].parse()?;
            let left_id: usize = values[1].parse()?;
            let cost: i16 = values[2].parse()?;

            cost_matrix.set(right_id, left_id, cost);
        }

        let cost_matrix = CostMatrix::new(cost_matrix);

        self.write_output_file("matrix.bin", cost_matrix)?;

        Ok(())
    }

    fn build_char_def(&self) -> Result<()> {
        let buffer = self.read_mecab_file("char.def")?;
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

        let mut char_category_map = HashMap::new();

        for (lower, upper, keys) in boundaries {
            let mut categories = Vec::new();

            for key in keys {
                if let Some(category) = category_def_map.get(key) {
                    categories.push(category.clone());
                }
            }

            for index in lower..=upper {
                char_category_map.insert(index, categories.clone());
            }
        }

        let char_table = CharTable::new(char_category_map);

        self.write_output_file("char.bin", char_table)?;

        Ok(())
    }

    fn build_unk(&self) -> Result<()> {
        let buffer = self.read_mecab_file("unk.def")?;
        let mut unk_term_map = HashMap::new();
        let mut feature = Vec::new();

        for (id, line) in buffer.lines().enumerate() {
            let row = Row::try_from(line)?;
            let term = Term::from(&row);

            unk_term_map
                .entry(row.surface_form.to_string())
                .or_insert_with(Vec::new)
                .push((id, term));

            feature.push(Feature::try_from(&row)?);
        }

        let unk_dict = UnknownDictionary::new(unk_term_map, feature);

        self.write_output_file("unk.bin", unk_dict)?;

        Ok(())
    }

    fn read_csv_files(&self) -> Result<Vec<String>> {
        let entries = fs::read_dir(&self.input_dir)?;
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

    fn read_mecab_file(&self, filename: &str) -> Result<String> {
        let path = Path::new(&self.input_dir).join(filename);
        let bytes = fs::read(path)?;
        let (buffer, _, _) = EUC_JP.decode(&bytes);

        Ok(buffer.to_string())
    }

    fn create_output_file(&self, filename: &str) -> Result<File> {
        let path = Path::new(&self.out_dir).join(filename);
        let handle = File::create(path)?;

        Ok(handle)
    }

    fn write_output_file<E: Encode>(&self, filename: &str, data: E) -> Result<()> {
        let mut handle = self.create_output_file(filename)?;

        encode(data, &mut handle)?;

        Ok(())
    }
}

fn main() -> Result<()> {
    let cmd = Command::parse();

    println!("----- Building kaiseki ressources -----\n");

    if !fs::exists(&cmd.out_dir)? {
        fs::create_dir(&cmd.out_dir)?;
    }

    println!("> Decoding mecab IPA dictionary files...");

    let (term_map, feat_map) = cmd.get_entry_map()?;

    println!("> Buiding char definition..");

    cmd.build_char_def()?;

    println!("> Building unknown dictionary...");

    cmd.build_unk()?;

    println!("> Building cost matrix...");

    cmd.build_matrix()?;

    println!("> Building FST...");

    cmd.build_fst(&term_map)?;

    println!("> Building entry dictionary...");

    cmd.build_dict(term_map, feat_map)?;

    println!("\nBuild complete!");

    Ok(())
}
