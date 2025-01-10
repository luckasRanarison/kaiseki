use std::collections::HashMap;

use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub struct CharTable {
    map: HashMap<u16, Vec<CharCategory>>,
}

impl CharTable {
    pub fn new(map: HashMap<u16, Vec<CharCategory>>) -> Self {
        Self { map }
    }

    pub fn lookup(&self, character: char) -> &[CharCategory] {
        let index = character as usize;

        match index {
            0..=0xFFFF => &self.map[&(index as u16)],
            _ => &self.map[&0xFFFF],
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
