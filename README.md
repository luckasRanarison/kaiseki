# kaiseki

kaiseki (解析) is a japanese tokenizer and morphological analyzer using [mecab-ipadic](https://taku910.github.io/mecab/), insipired by [this article](https://towardsdatascience.com/how-japanese-tokenizers-work-87ab6b256984).

## Usage

kaiseki supports both morpheme tokenization and word tokenization (inflections included). It also provides additional informations from the mecab dictionary such as part of speech, conjugation form,...

```rust
use kaiseki::{Tokenizer, error:Error};

fn main() -> Result<(), Error> {
    let tokenizer = Tokenizer::new()?;
    let morphemes = tokenizer.tokenize("東京都に住んでいる");
    let morphemes: Vec<_> = morphemes.iter().map(|m| &m.text).collect();

    println!("{:?}", morphemes); // ["東京", "都", "に", "住ん", "で", "いる"]

    let words = tokenizer.tokenize_word("東京都に住んでいる"); 
    let words: Vec<_> = words.iter().map(|w| &w.text).collect();

    println!("{:?}", words); // ["東京", "都", "に", "住んでいる"]

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


