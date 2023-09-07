use crate::{
    fst::FstSearcher,
    lattice::{Lattice, Node},
    matrix::CostMatrix,
};
use anyhow::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

pub struct Tokenizer {
    fst: FstSearcher,
    matrix: CostMatrix,
}

impl Tokenizer {
    pub fn default() -> Result<Self, Error> {
        Ok(Self {
            fst: FstSearcher::default()?,
            matrix: CostMatrix::default()?,
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
            let terms = self.fst.get_terms(substr);

            for (len, term_id, term) in terms {
                lattice.add_node(Node::new(
                    term_id,
                    index,
                    index + len,
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

            tokens.push(Token {
                text,
                start: node.start,
                end,
            })
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::Tokenizer;
    use crate::tokenizer::Token;

    #[test]
    fn test_tokenizer() {
        let tokenizer = Tokenizer::default().unwrap();
        let tokens = tokenizer
            .tokenize("上記の処理は次のスライドを見てもらうとイメージが付きやすいと思います");
        let expected: Vec<Token> = Vec::new();

        assert_eq!(expected, tokens);
    }
}
