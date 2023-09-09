use crate::{error::Error, utils::BINCODE_CONFIG};
use bincode::{decode_from_slice, Decode, Encode};

const CHAR_DEF: &'static [u8] = include_bytes!("../bin/char.bin");

#[derive(Encode, Decode)]
pub struct CharTable {
    map: Vec<Vec<CharCategory>>,
}

impl CharTable {
    pub fn new(map: Vec<Vec<CharCategory>>) -> Self {
        Self { map }
    }

    pub fn load() -> Result<Self, Error> {
        let (char_def, _) = decode_from_slice(CHAR_DEF, *BINCODE_CONFIG)?;

        Ok(char_def)
    }

    pub fn lookup(&self, character: char) -> &Vec<CharCategory> {
        let index = character as usize;

        match index {
            0..=0xFFFF => &self.map[index],
            _ => &self.map[0xFFFF],
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
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
        let lookup_table = CharTable::load().unwrap();
        let categories = lookup_table.lookup('一');
        let expected = vec![
            CharCategory::new("KANJINUMERIC".to_owned(), true, true, 0),
            CharCategory::new("KANJI".to_owned(), false, false, 2),
        ];

        assert_eq!(&expected, categories);

        let categories = lookup_table.lookup('1');
        let expected = vec![CharCategory::new("NUMERIC".to_owned(), true, true, 0)];

        assert_eq!(&expected, categories);
    }
}
