use crate::{
    dict::EntryDictionary,
    fst::FstSearcher,
    lattice::{Lattice, Node},
    matrix::CostMatrix,
    term::{Term, TermId},
};
use anyhow::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn new(text: String, start: usize, end: usize) -> Self {
        Self { text, start, end }
    }
}

pub struct Tokenizer {
    fst: FstSearcher,
    dict: EntryDictionary,
    matrix: CostMatrix,
}

impl Tokenizer {
    pub fn default() -> Result<Self, Error> {
        Ok(Self {
            fst: FstSearcher::load()?,
            dict: EntryDictionary::load()?,
            matrix: CostMatrix::load()?,
        })
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let text_len = input.len();
        let mut lattice = Lattice::new(text_len);

        for index in 0..text_len {
            if !lattice.has_node_ending_at(index) {
                continue;
            }

            let substr = &input[index..];
            let terms = self.get_terms_from_str(substr);

            for (term_id, term) in terms {
                lattice.add_node(Node::new(
                    term_id,
                    index,
                    index + term.length,
                    term.context_id,
                    term.cost,
                ));
            }
        }

        let nodes = lattice.find_path(&self.matrix);
        let mut tokens = Vec::new();

        for node in nodes {
            let end = match node.end > text_len {
                true => text_len,
                false => node.end,
            };
            let text = input[node.start..end].to_owned();

            tokens.push(Token::new(text, node.start, end));
        }

        tokens
    }

    fn get_terms_from_str(&self, input: &str) -> Vec<(TermId, Term)> {
        let term_id = self.fst.get_all(input);
        let mut terms = Vec::new();

        for id in term_id {
            if let Some(term) = self.dict.get_term(id) {
                terms.push((id, term.clone()));
            }
        }

        terms
    }
}

#[cfg(test)]
mod tests {
    use super::Tokenizer;

    #[test]
    fn test_tokenizer() {
        let tokenizer = Tokenizer::default().unwrap();
        let tokens = tokenizer.tokenize("東京都に住む");
        let expected = vec!["東京", "都", "に", "住む"];
        let text: Vec<_> = tokens.iter().map(|token| &token.text).collect();

        assert_eq!(expected, text);
    }
}
