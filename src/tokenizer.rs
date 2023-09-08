use crate::{
    char::CharTable,
    dict::EntryDictionary,
    error::Error,
    fst::FstSearcher,
    lattice::{Lattice, Node},
    matrix::CostMatrix,
    term::ExtratedTerm,
    unk::UnknownDictionary,
};

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
    unk_dict: UnknownDictionary,
    char_table: CharTable,
    matrix: CostMatrix,
}

impl Tokenizer {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            fst: FstSearcher::load()?,
            dict: EntryDictionary::load()?,
            unk_dict: UnknownDictionary::load()?,
            char_table: CharTable::load()?,
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
            let mut extracted = self.get_terms_from_str(substr);
            let found = !extracted.is_empty();
            let unknown = self.get_unkown_terms_from_str(substr, found);

            extracted.extend(unknown);

            for term in extracted {
                lattice.add_node(Node::new(
                    term.id,
                    index,
                    index + term.length,
                    term.value.context_id,
                    term.value.cost,
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

    fn get_terms_from_str(&self, input: &str) -> Vec<ExtratedTerm> {
        let terms = self.fst.get_from_prefix(input);
        let mut extracted = Vec::new();

        for (len, id) in terms {
            if let Some(term) = self.dict.get_term(id) {
                extracted.push(ExtratedTerm::new(id, len, term.clone()));
            }
        }

        extracted
    }

    fn get_unkown_terms_from_str(&self, input: &str, found: bool) -> Vec<ExtratedTerm> {
        let mut unk_terms = Vec::new();
        let mut chars = input.chars().peekable();
        let mut current_len = 0;
        let ch = chars.next().unwrap();
        let char_categories = self.char_table.lookup(ch);

        current_len += ch.len_utf8();

        for category in char_categories {
            if found && !category.invoke {
                continue;
            }

            if category.group {
                while let Some(ch) = chars.next() {
                    if self.char_table.lookup(ch).contains(category) {
                        current_len += ch.len_utf8();
                    } else {
                        break;
                    }
                }
            }

            if let Some(terms) = self.unk_dict.get_terms(&category.name) {
                for (id, value) in terms {
                    unk_terms.push(ExtratedTerm::new(*id, current_len, value.clone()));
                }
            }
        }

        unk_terms
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    Ok(Tokenizer::new()?.tokenize(input))
}

#[cfg(test)]
mod tests {
    use super::Tokenizer;

    #[test]
    fn test_tokenizer() {
        let tokenizer = Tokenizer::new().unwrap();
        let tokens = tokenizer.tokenize("東京都に住む");
        let expected = vec!["東京", "都", "に", "住む"];
        let text: Vec<_> = tokens.iter().map(|token| &token.text).collect();

        assert_eq!(expected, text);
    }

    #[test]
    fn test_tokenizer_unkown() {
        let tokenizer = Tokenizer::new().unwrap();
        let tokens = tokenizer.tokenize("1234個");
        let expected = vec!["1234", "個"];
        let text: Vec<_> = tokens.iter().map(|token| &token.text).collect();

        assert_eq!(expected, text);
    }
}
