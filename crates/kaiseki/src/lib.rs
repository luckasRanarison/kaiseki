pub mod error;
pub mod inflection;
pub mod morpheme;
pub mod tokenizer;
pub mod word;

mod consts;
mod fst;
mod lattice;

pub use inflection::Inflection;
pub use morpheme::Morpheme;
pub use tokenizer::Tokenizer;
pub use word::Word;

use error::Result;

pub fn tokenize(input: &str) -> Result<Vec<Morpheme>> {
    Ok(Tokenizer::new()?.tokenize(input))
}

pub fn tokenize_word(input: &str) -> Result<Vec<Word>> {
    Ok(Tokenizer::new()?.tokenize_word(input))
}
