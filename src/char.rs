use anyhow::Error;
use bincode::{config, decode_from_slice, Decode, Encode};
use std::collections::HashMap;

const CHAR_DEF: &'static [u8] = include_bytes!("../dict/char.bin");

#[derive(Encode, Decode)]
pub struct CharLookup {
    boundaries: Vec<(u32, u32, Vec<String>)>,
    categories: HashMap<String, CharCategory>,
}

impl CharLookup {
    pub fn new(
        boundaries: Vec<(u32, u32, Vec<String>)>,
        categories: HashMap<String, CharCategory>,
    ) -> Self {
        Self {
            boundaries,
            categories,
        }
    }

    pub fn load() -> Result<Self, Error> {
        let config = config::standard();
        let (char_def, _) = decode_from_slice(CHAR_DEF, config)?;

        Ok(char_def)
    }

    pub fn lookup(&self, character: char) -> Vec<&CharCategory> {
        let decimal = character as u32;
        let mut categories = Vec::new();

        for (lower, upper, keys) in &self.boundaries {
            if decimal >= *lower && decimal <= *upper {
                for key in keys {
                    if let Some(category) = self.categories.get(key) {
                        categories.push(category);
                    }
                }
            }
        }

        categories
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct CharCategory {
    pub name: String,
    pub invoke: bool,
    pub group: bool,
    pub length: usize,
}

impl CharCategory {
    pub fn new(name: String, invoke: bool, group: bool, length: usize) -> Self {
        Self {
            name,
            invoke,
            group,
            length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        let lookup_table = CharLookup::load().unwrap();
        let categories = lookup_table.lookup('一');
        let cat1 = CharCategory::new("KANJINUMERIC".to_owned(), true, true, 0);
        let cat2 = CharCategory::new("KANJI".to_owned(), false, false, 2);
        let expected: Vec<&CharCategory> = vec![&cat1, &cat2];

        assert_eq!(expected, categories);
    }
}
