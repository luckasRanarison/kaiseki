use crate::{ConjugationForm, Morpheme};

#[derive(Debug, Clone, PartialEq)]
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
    use super::Inflection::{self, *};
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
        let tokenizer = Tokenizer::new().unwrap();

        let test_cases = vec![
            ("食べます", Polite),
            ("食べない", Negative),
            ("食べた", Past),
            ("食べて", Te),
            ("食べている", TeIru),
            ("食べてくれる", TeKureru),
            ("食べてもらう", TeMorau),
            ("食べてみる", TeMiru),
            ("食べておく", TeOku),
            ("食べていく", TeIku),
            ("食べてくる", Tekuru),
            ("食べてしまう", TeShimau),
            ("食べちゃう", Chau),
            ("食べろ", Imperative),
            ("食べよう", Volotional),
            ("食べたら", Tara),
            ("食べれば", Ba),
            ("食べられる", PotentialPassive),
            ("食べさせる", Causative),
            ("食べたい", Tai),
        ];

        for (input, expected) in test_cases {
            let morphemes = tokenizer.tokenize(input);
            let inflections = Inflection::from_morphemes(&morphemes);
            assert_only_contains(inflections, expected);
        }
    }

    #[test]
    fn test_mutiple_inflections() {
        let tokenizer = Tokenizer::new().unwrap();

        #[rustfmt::skip]
        let test_cases = vec![
            ("食べなかった", vec![Negative, Past]),
            ("遊んでみれば", vec![TeMiru, Ba]),
            ("寝てしまった", vec![TeShimau, Past]),
            ("知りたくない", vec![Tai, Negative]),
            ("持ってください", vec![Te, Imperative]),
            ("笑っちゃった", vec![Chau, Past]),
            ("見ませんでした", vec![Polite, Negative, Past]),
            ("考えてみましょう", vec![TeMiru, Polite, Volotional]),
            ("やってみてくれません", vec![TeMiru, TeKureru, Polite, Negative]),
            ("見せられたくない", vec![PotentialPassive, Tai, Negative])
        ];

        for (input, expected) in test_cases {
            let morphemes = tokenizer.tokenize(input);
            let inflections = Inflection::from_morphemes(&morphemes);
            assert_eq!(inflections, expected);
        }
    }
}
