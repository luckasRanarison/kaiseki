use crate::{
    char::CharTable,
    dict::EntryDictionary,
    error::Error,
    fst::FstSearcher,
    lattice::{Lattice, Node},
    matrix::CostMatrix,
    morpheme::Morpheme,
    term::ExtratedTerm,
    unk::UnknownDictionary,
    word::Word,
};

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

    pub fn tokenize(&self, input: &str) -> Vec<Morpheme> {
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
                    term.unknown,
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
            let feature = match node.unknown {
                true => self.unk_dict.get_feat(node.term_id),
                false => self.dict.get_feat(node.term_id),
            };
            let feature = feature.cloned().unwrap_or_default();

            tokens.push(Morpheme::new(text, node.start, end, feature));
        }

        tokens
    }

    pub fn tokenize_word(&self, input: &str) -> Vec<Word> {
        let mut words = Vec::new();
        let mut morphemes = self.tokenize(input).into_iter().peekable();

        while let Some(morpheme) = morphemes.next() {
            if morpheme.is_symbol() {
                continue;
            }

            let has_inflections = morpheme.has_inflection();
            let mut word_morphemes = vec![morpheme];

            if has_inflections {
                while let Some(next) = morphemes.peek() {
                    if next.is_inflection() {
                        let next = morphemes.next().unwrap();
                        word_morphemes.push(next);
                    } else {
                        break;
                    }
                }
            }

            let word = Word::from_morphemes(word_morphemes.as_slice());

            if let Some(word) = word {
                words.push(word);
            }
        }

        words
    }

    fn get_terms_from_str(&self, input: &str) -> Vec<ExtratedTerm> {
        let terms = self.fst.get_from_prefix(input);
        let mut extracted = Vec::new();

        for (len, id) in terms {
            if let Some(term) = self.dict.get_term(id) {
                extracted.push(ExtratedTerm::new(id, false, len, term.clone()));
            }
        }

        extracted
    }

    fn get_unkown_terms_from_str(&self, input: &str, found: bool) -> Vec<ExtratedTerm> {
        let mut unk_terms = Vec::new();
        let mut chars = input.chars();
        let mut current_len = 0;
        let ch = chars.next().unwrap();
        let char_categories = self.char_table.lookup(ch);

        current_len += ch.len_utf8();

        for category in char_categories {
            if found && !category.invoke {
                continue;
            }

            if category.group {
                for ch in chars.by_ref() {
                    if self.char_table.lookup(ch).contains(category) {
                        current_len += ch.len_utf8();
                    } else {
                        break;
                    }
                }
            }

            if let Some(terms) = self.unk_dict.get_terms(&category.name) {
                for (id, value) in terms {
                    unk_terms.push(ExtratedTerm::new(*id, true, current_len, value.clone()));
                }
            }
        }

        unk_terms
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Morpheme>, Error> {
    Ok(Tokenizer::new()?.tokenize(input))
}

pub fn tokenize_word(input: &str) -> Result<Vec<Word>, Error> {
    Ok(Tokenizer::new()?.tokenize_word(input))
}

#[cfg(test)]
mod tests {
    use super::Tokenizer;
    use crate::conjugation::ConjugationForm as C;
    use crate::pos::{PartOfSpeech as P, SubPartOfSpeech as S};

    #[test]
    fn test_tokenizer() {
        let tokenizer = Tokenizer::new().unwrap();
        let morphemes = tokenizer.tokenize("東京都に住む");
        let expected = vec!["東京", "都", "に", "住む"];
        let text: Vec<_> = morphemes.iter().map(|token| &token.text).collect();

        assert_eq!(expected, text);
    }

    #[test]
    fn test_tokenizer_unkown() {
        let tokenizer = Tokenizer::new().unwrap();
        let morphemes = tokenizer.tokenize("1234個");
        let expected = vec!["1234", "個"];
        let text: Vec<_> = morphemes.iter().map(|token| &token.text).collect();

        assert_eq!(expected, text);
    }

    #[test]
    fn test_token_feature() {
        let tokenizer = Tokenizer::new().unwrap();
        let morphemes = tokenizer.tokenize("ケーキを食べる");

        assert_eq!(morphemes[0].part_of_speech, P::Noun);
        assert_eq!(Some("ケーキ".to_owned()), morphemes[0].reading);

        assert_eq!(morphemes[1].part_of_speech, P::Particle);
        assert_eq!(Some("ヲ".to_owned()), morphemes[1].reading);

        assert_eq!(morphemes[2].part_of_speech, P::Verb);
        assert_eq!(Some(C::BasicForm), morphemes[2].conjugation_form);
        assert_eq!(Some("タベル".to_owned()), morphemes[2].reading);
    }

    #[test]
    fn test_token_feature_unknown() {
        let tokenizer = Tokenizer::new().unwrap();
        let morphemes = tokenizer.tokenize("100 ");

        assert!(morphemes[0].sub_part_of_speech.contains(&S::Number));
        assert!(morphemes[1].sub_part_of_speech.contains(&S::Space));
    }
}
