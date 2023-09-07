use crate::term::{Term, TermId};
use anyhow::Error;
use bincode::{config, decode_from_slice};
use fst::{raw::Output, Map};

const FST: &'static [u8] = include_bytes!("../dict/term.fst");
const TERMS: &'static [u8] = include_bytes!("../dict/term.bin");

pub struct FstSearcher {
    map: Map<&'static [u8]>,
    terms: Vec<Term>,
}

impl FstSearcher {
    pub fn default() -> Result<Self, Error> {
        let fst = Map::new(FST)?;
        let config = config::standard();
        let (terms, _) = decode_from_slice(TERMS, config)?;

        Ok(Self { map: fst, terms })
    }

    pub fn get_terms(&self, input: &str) -> Vec<(usize, TermId, Term)> {
        let fst = self.map.as_fst();
        let mut node = fst.root();
        let mut term_id = Vec::new();
        let mut output = Output::zero();
        let mut substr_len = 0;

        for byte in input.bytes() {
            if let Some(index) = node.find_input(byte) {
                let transition = node.transition(index);
                substr_len += 1;
                node = fst.node(transition.addr);
                output = output.cat(transition.out);

                if node.is_final() {
                    term_id.push((substr_len, output.value()));
                }
            } else {
                break;
            }
        }

        let mut terms_value = Vec::new();

        for (len, id) in term_id {
            let offset = (id & 0b11111) as usize;
            let id = id.wrapping_shr(5) as usize;
            let terms = &self.terms[id..id + offset];

            for (i, term) in terms.iter().enumerate() {
                terms_value.push((len, id + i, term.clone()));
            }
        }

        terms_value
    }
}

#[cfg(test)]
mod tests {
    use super::FstSearcher;
    use crate::term::Term;

    #[test]
    fn test_prefix_search() {
        let source = "憂";
        let searcher = FstSearcher::default().unwrap();
        let terms = searcher.get_terms(source);
        let expected = vec![
            (3, 220041, Term::new(11, 5352)),
            (3, 220042, Term::new(1291, 8836)),
        ];

        assert_eq!(terms, expected);
    }
}
