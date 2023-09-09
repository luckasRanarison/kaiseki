# kaiseki

kaiseki (解析) is a japanese tokenizer and morphological analyzer using [mecab-ipadic](https://taku910.github.io/mecab/), insipired by [this article](https://towardsdatascience.com/how-japanese-tokenizers-work-87ab6b256984).

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

## Test 

```sh
cargo test
```

## Credits

- The [Mecab Project](https://taku910.github.io/mecab/) for providing the the dictionary and data used for tokenizing.
 
- [kotori](https://github.com/wanasit/kotori) and [kuromoji-rs](https://github.com/fulmicoton/kuromoji-rs) for some reference. 

## Articles

- [How Japanese Tokenizers Work](https://towardsdatascience.com/how-japanese-tokenizers-work-87ab6b256984).

- [日本語形態素解析の裏側を覗く！MeCab はどのように形態素解析しているか](https://techlife.cookpad.com/entry/2016/05/11/170000).

## License

MIT License.


