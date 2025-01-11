use crate::{
    profile::{current_time, AsFileSize, SizeReport},
    utils::{parse_category, parse_char_map, Result},
};

use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File},
    path::Path,
};

use bincode::Encode;
use clap::{error::ErrorKind, CommandFactory, Parser};
use encoding_rs::EUC_JP;
use fst::MapBuilder;
use mecab_types::{
    bincode::encode, char::CharTable, cost::CostMatrix, dict::EntryDictionary, features::Feature,
    row::Row, term::Term, unk::UnknownDictionary, utils::FlatMatrix,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    /// Directory containing extracted mecab-ipadic files
    #[arg(short, long)]
    input_dir: String,

    /// Transformed binaries destination directory
    #[arg(short, long)]
    out_dir: String,

    #[arg(skip)]
    term_map: BTreeMap<String, Vec<Term>>,

    #[arg(skip)]
    feature_map: BTreeMap<String, Vec<Feature>>,

    #[arg(skip)]
    start_time: u128,
}

impl Cli {
    pub fn execute(mut self) -> Result<()> {
        if !fs::exists(&self.out_dir)? {
            fs::create_dir(&self.out_dir)?;
        }

        println!("Starting build...\n");
        self.start_time = current_time();

        println!("Decoding mecab IPA dictionary files...");
        self.fill_entry_maps()?;

        println!("Buiding char definition..");
        let char_def = self.build_char_def()?;

        println!("Building unknown dictionary...");
        let unk_dict = self.build_unk_dict()?;

        println!("Building cost matrix...");
        let cost_matrix = self.build_cost_matrix()?;

        println!("Building term FST...");
        let term_fst = self.build_term_fst()?;

        println!("Building entry dictionary...");
        let entry_dict = self.build_entry_dict()?;

        let report = SizeReport {
            char_def,
            unk_dict,
            cost_matrix,
            entry_dict,
            term_fst,
        };

        self.profile(report);

        Ok(())
    }

    pub fn validate_args(self) -> Result<Self> {
        if !fs::exists(&self.input_dir)? {
            Cli::command()
                .error(
                    ErrorKind::Io,
                    format!("Directory not found '{}'", self.input_dir),
                )
                .exit();
        }

        Ok(self)
    }

    fn fill_entry_maps(&mut self) -> Result<()> {
        let mut decoded_files = Vec::new();
        let mut rows = Vec::new();

        for file in self.read_csv_files()? {
            decoded_files.push(self.read_mecab_file(&file)?);
        }

        for file in &decoded_files {
            for line in file.lines() {
                rows.push(Row::try_from(line)?);
            }
        }

        for row in rows {
            self.term_map
                .entry(row.surface_form.to_owned())
                .or_default()
                .push(Term::from(&row));

            self.feature_map
                .entry(row.surface_form.to_owned())
                .or_default()
                .push(Feature::try_from(&row)?);
        }

        Ok(())
    }

    fn build_term_fst(&self) -> Result<usize> {
        let handle = self.create_output_file("term.fst")?;
        let mut map_builder = MapBuilder::new(handle)?;
        let mut id = 0u64;

        for (key, terms) in &self.term_map {
            let len = terms.len() as u64;
            let value = id << 5 | len; // encode the offset, assert!(len < 32)
            map_builder.insert(key, value)?;
            id += len;
        }

        let bytes_written = map_builder.bytes_written() as usize;

        map_builder.finish()?;

        Ok(bytes_written)
    }

    fn build_entry_dict(&self) -> Result<usize> {
        let mut terms = Vec::new();
        let mut features = Vec::new();

        for value in self.term_map.values() {
            terms.extend(value.clone());
        }

        for value in self.feature_map.values() {
            features.extend(value.clone());
        }

        let dict = EntryDictionary::new(terms, features);
        let bytes_written = self.write_output_file("dict.bin", dict)?;

        Ok(bytes_written)
    }

    fn build_cost_matrix(&self) -> Result<usize> {
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
        let bytes_written = self.write_output_file("matrix.bin", cost_matrix)?;

        Ok(bytes_written)
    }

    fn build_char_def(&self) -> Result<usize> {
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

        let mut char_category_map = vec![vec![]; 0xFFFF];

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
        let bytes_written = self.write_output_file("char.bin", char_table)?;

        Ok(bytes_written)
    }

    fn build_unk_dict(&self) -> Result<usize> {
        let buffer = self.read_mecab_file("unk.def")?;
        let mut unk_term_map = BTreeMap::new();
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
        let bytes_written = self.write_output_file("unk.bin", unk_dict)?;

        Ok(bytes_written)
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

    fn write_output_file<E: Encode>(&self, filename: &str, data: E) -> Result<usize> {
        let mut handle = self.create_output_file(filename)?;
        let bytes_written = encode(data, &mut handle)?;

        Ok(bytes_written)
    }

    fn profile(&self, report: SizeReport) {
        let sections = [
            ("char.bin", report.char_def.as_file_size()),
            ("unk.bin", report.unk_dict.as_file_size()),
            ("cost.bin", report.cost_matrix.as_file_size()),
            ("dict.bin", report.entry_dict.as_file_size()),
            ("term.fst", report.term_fst.as_file_size()),
            ("Total", report.total().as_file_size()),
        ];

        println!();

        for (label, size) in sections {
            println!("{:<9}: {}", label, size);
        }

        println!(
            "\nBuild complete in {:.2}s",
            (current_time() - self.start_time) as f64 / 1000.
        );
    }
}
