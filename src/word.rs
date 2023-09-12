use crate::{inflection::Inflection, morpheme::Morpheme, pos::PartOfSpeech};

#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub base_form: String,
    pub part_of_speech: PartOfSpeech,
    pub morphemes: Vec<Morpheme>,
    pub inflections: Vec<Inflection>,
}

impl Word {
    pub fn from_morphemes(morphemes: &[Morpheme]) -> Option<Self> {
        morphemes.first().map(|main| {
            let start = main.start;
            let end = morphemes.last().map_or(main.end, |l| l.end);
            let text = morphemes.iter().map(|m| m.text.to_owned()).collect();
            let base_form = main.base_form.as_ref().unwrap_or(&main.text).to_owned();
            let part_of_speech = main.part_of_speech;
            let inflections = Inflection::from_morphemes(morphemes);
            let morphemes = morphemes.to_vec();

            Self {
                text,
                start,
                end,
                base_form,
                part_of_speech,
                morphemes,
                inflections,
            }
        })
    }
}
