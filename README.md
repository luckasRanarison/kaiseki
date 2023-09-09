# kaiseki

kaiseki (解析) is a japanese tokenizer and morphological analyzer using [mecab-ipadic](https://taku910.github.io/mecab/), inspired by [this article](https://towardsdatascience.com/how-japanese-tokenizers-work-87ab6b256984) and [kuromoji-rs](https://github.com/fulmicoton/kuromoji-rs).

## Usage

kaiseki currently only supports morpheme tokenization and provides additional informations such as **part of speech**, **conjugation form** and **reading**.

```rust
use kaiseki::{Tokenizer, error:Error};

fn main() -> Result<(), Error> {
    let tokens = tokenizer.tokenize("東京都に住む");
    let morphemes: Vec<_> = tokens.iter().map(|token| &token.text).collect();

    println!("{:?}", morphemes); // ["東京", "都", "に", "住む"]

    Ok(())
}


```

# Build

kaiseki uses pre-compiled binary files, contaning informations extracted from mecab-ipadic.

You can build the binaries with the following commands:

```sh
sh ipadic-install.sh
cargo run --bin kaiseki-build
```
