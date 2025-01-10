use crate::error::Result;

use fst::{raw::Output, Map};
use mecab_types::term::TermId;

pub struct FstSearcher {
    map: Map<&'static [u8]>,
}

impl FstSearcher {
    pub fn load(bytes: &'static [u8]) -> Result<Self> {
        Ok(Self {
            map: Map::new(bytes)?,
        })
    }

    pub fn get_from_prefix(&self, input: &str) -> Vec<(usize, TermId)> {
        let fst = self.map.as_fst();
        let mut node = fst.root();
        let mut output = Output::zero();
        let mut len = 0;
        let mut results = Vec::new();

        for byte in input.bytes() {
            if let Some(index) = node.find_input(byte) {
                let transition = node.transition(index);
                node = fst.node(transition.addr);
                output = output.cat(transition.out);
                len += 1;

                if node.is_final() {
                    let value = output.value();
                    let offset = (value & 0b11111) as usize;
                    let start = value.wrapping_shr(5) as usize;

                    for id in start..(start + offset) {
                        results.push((len, id));
                    }
                }
            } else {
                break;
            }
        }

        results
    }
}
