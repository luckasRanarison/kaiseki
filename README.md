# kaiseki

kaiseki (解析) is a japanese tokenizer and morphological analyzer using [mecab-ipadic](https://taku910.github.io/mecab/), inspired by [this article](https://towardsdatascience.com/how-japanese-tokenizers-work-87ab6b256984) and [kuromoji-rs](https://github.com/fulmicoton/kuromoji-rs).

## Usage

```rust
use kaiseki::{Tokenizer, error:Error};

fn main() -> Result<(), Error> {
    let tokenizer = Tokenizer::new()?;
    let tokens = tokenizer.tokenize("東京都に住む");
    let morphemes: Vec<_> = tokens.iter().map(|token| &token.text).collect();

    println!("{:?}", morphemes); // ["東京", "都", "に", "住む"]

    Ok(())
}
```
