use crate::{ConjugationForm, Morpheme};

#[derive(Debug, Clone, PartialEq)]
pub struct Verb {
    pub verb_type: VerbType,
    pub inflections: Vec<VerbInflection>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerbType {
    Ichidan,
    Godan,
    Suru,
    Kuru,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerbInflection {
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
    Volotional,
    Imperative,
    Tara,
    Ba,
    PotentialPassive,
    Causative,
    Tai,
}

impl VerbInflection {
    pub fn from_morphemes(morphemes: &[Morpheme]) -> Vec<VerbInflection> {
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
                Some("て") => morphemes
                    .peek()
                    .and_then(|m| match m.base_form.as_deref() {
                        Some("いる") => Some(Self::TeIru),
                        Some("おく") => Some(Self::TeOku),
                        Some("くれる") => Some(Self::TeKureru),
                        Some("もらう") => Some(Self::TeMorau),
                        Some("いく") | Some("行く") => Some(Self::TeIku),
                        Some("くる") | Some("来る") => Some(Self::Tekuru),
                        Some("みる") => Some(Self::TeMiru),
                        _ => Some(Self::Te),
                    })
                    .or(Some(Self::Te)),
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
    use super::VerbInflection;
    use crate::Tokenizer;

    fn assert_only_contains(inflections: Vec<VerbInflection>, value: VerbInflection) {
        assert!(inflections.contains(&value), "{:?}", value);
        assert_eq!(inflections.len(), 1, "{:?}", inflections);
    }

    #[test]
    fn test_single_inflection() {
        let tokenizer = Tokenizer::new().unwrap();

        let morphemes = tokenizer.tokenize("食べる");
        let inflections = VerbInflection::from_morphemes(&morphemes);
        assert!(inflections.is_empty());

        let test_cases = vec![
            ("食べます", VerbInflection::Polite),
            ("食べない", VerbInflection::Negative),
            ("食べた", VerbInflection::Past),
            ("食べて", VerbInflection::Te),
            ("食べている", VerbInflection::TeIru),
            ("食べてくれる", VerbInflection::TeKureru),
            ("食べてもらう", VerbInflection::TeMorau),
            ("食べてみる", VerbInflection::TeMiru),
            ("食べておく", VerbInflection::TeOku),
            ("食べていく", VerbInflection::TeIku),
            ("食べてくる", VerbInflection::Tekuru),
            ("食べろ", VerbInflection::Imperative),
            ("食べよう", VerbInflection::Volotional),
            ("食べたら", VerbInflection::Tara),
            ("食べれば", VerbInflection::Ba),
            ("食べられる", VerbInflection::PotentialPassive),
            ("食べさせる", VerbInflection::Causative),
            ("食べたい", VerbInflection::Tai),
        ];

        for (input, expected) in test_cases {
            let morphemes = tokenizer.tokenize(input);
            let inflections = VerbInflection::from_morphemes(&morphemes);
            assert_only_contains(inflections, expected);
        }
    }
}
