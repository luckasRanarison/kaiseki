use mecab_types::conjugation::ConjugationForm;

use crate::morpheme::Morpheme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Inflection {
    Polite,
    Negative,
    Past,
    Te,
    TeIru,
    TeKureru,
    TeMorau,
    TeMiru,
    TeOku,
    TeIku,
    Tekuru,
    TeShimau,
    Chau,
    Volotional,
    Imperative,
    Tara,
    Ba,
    PotentialPassive,
    Passive,
    Causative,
    Tai,
}

impl Inflection {
    pub fn from_morphemes(morphemes: &[Morpheme]) -> Vec<Inflection> {
        let mut inflections = Vec::new();
        let mut morphemes = morphemes.iter().peekable();

        while let Some(morpheme) = morphemes.next() {
            let inflection = match morpheme.base_form.as_deref() {
                Some("ます") => Some(Self::Polite),
                Some("ない") | Some("ん") => Some(Self::Negative),
                Some("た") => morpheme.conjugation_form.as_ref().and_then(|f| match f {
                    ConjugationForm::BasicForm => Some(Self::Past),
                    ConjugationForm::HypotheticalForm => Some(Self::Tara),
                    _ => None,
                }),
                Some("て") | Some("で") => morphemes
                    .peek()
                    .map(|m| match m.base_form.as_deref() {
                        Some("いる") => Self::TeIru,
                        Some("おく") => Self::TeOku,
                        Some("くれる") => Self::TeKureru,
                        Some("もらう") => Self::TeMorau,
                        Some("いく") | Some("行く") => Self::TeIku,
                        Some("くる") | Some("来る") => Self::Tekuru,
                        Some("みる") => Self::TeMiru,
                        Some("しまう") => Self::TeShimau,
                        _ => Self::Te,
                    })
                    .or(Some(Self::Te)),
                Some("ちゃう") => Some(Self::Chau),
                Some("う") => Some(Self::Volotional),
                Some("ば") => Some(Self::Ba),
                Some("れる") => Some(Self::Passive),
                Some("させる") => Some(Self::Causative),
                Some("られる") => Some(Self::PotentialPassive),
                Some("たい") => Some(Self::Tai),
                _ => None,
            };

            if let Some(conjugation_form) = &morpheme.conjugation_form {
                let is_imperative = matches!(
                    conjugation_form,
                    ConjugationForm::ImperativeE
                        | ConjugationForm::ImperativeI
                        | ConjugationForm::ImperativeYo
                        | ConjugationForm::ImperativeRo
                );

                if is_imperative {
                    inflections.push(Self::Imperative);
                }
            }

            if let Some(inflection) = inflection {
                inflections.push(inflection);
            }
        }

        inflections
    }
}

#[cfg(test)]
mod tests {
    use super::Inflection;
    use crate::Tokenizer;

    fn assert_only_contains(inflections: Vec<Inflection>, value: Inflection) {
        assert!(inflections.contains(&value), "{:?}", value);
        assert_eq!(inflections.len(), 1, "{:?}", inflections);
    }

    #[test]
    fn test_base_form() {
        let tokenizer = Tokenizer::new().unwrap();
        let morphemes = tokenizer.tokenize("食べる");
        let inflections = Inflection::from_morphemes(&morphemes);
        assert!(inflections.is_empty());
    }

    #[test]
    fn test_single_inflection() {
        use super::Inflection as I;

        let tokenizer = Tokenizer::new().unwrap();

        let test_cases = vec![
            ("食べます", I::Polite),
            ("食べない", I::Negative),
            ("食べた", I::Past),
            ("食べて", I::Te),
            ("食べている", I::TeIru),
            ("食べてくれる", I::TeKureru),
            ("食べてもらう", I::TeMorau),
            ("食べてみる", I::TeMiru),
            ("食べておく", I::TeOku),
            ("食べていく", I::TeIku),
            ("食べてくる", I::Tekuru),
            ("食べてしまう", I::TeShimau),
            ("食べちゃう", I::Chau),
            ("食べろ", I::Imperative),
            ("食べよう", I::Volotional),
            ("食べたら", I::Tara),
            ("食べれば", I::Ba),
            ("食べられる", I::PotentialPassive),
            ("食べさせる", I::Causative),
            ("食べたい", I::Tai),
            ("飲まれる", I::Passive),
        ];

        for (input, expected) in test_cases {
            let morphemes = tokenizer.tokenize(input);
            let inflections = Inflection::from_morphemes(&morphemes);
            assert_only_contains(inflections, expected);
        }
    }

    #[test]
    fn test_mutiple_inflections() {
        use super::Inflection as I;

        let tokenizer = Tokenizer::new().unwrap();

        #[rustfmt::skip]
        let test_cases = vec![
            ("食べなかった", vec![I::Negative, I::Past]),
            ("遊んでみれば", vec![I::TeMiru, I::Ba]),
            ("寝てしまった", vec![I::TeShimau, I::Past]),
            ("知りたくない", vec![I::Tai, I::Negative]),
            ("持ってください", vec![I::Te, I::Imperative]),
            ("笑っちゃった", vec![I::Chau, I::Past]),
            ("見ませんでした", vec![I::Polite, I::Negative, I::Past]),
            ("考えてみましょう", vec![I::TeMiru, I::Polite, I::Volotional]),
            ("やってみてくれません", vec![I::TeMiru, I::TeKureru, I::Polite, I::Negative]),
            ("見せられたくない", vec![I::PotentialPassive, I::Tai, I::Negative])
        ];

        for (input, expected) in test_cases {
            let morphemes = tokenizer.tokenize(input);
            let inflections = Inflection::from_morphemes(&morphemes);
            assert_eq!(inflections, expected);
        }
    }
}
