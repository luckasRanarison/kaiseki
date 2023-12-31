use bincode::{Decode, Encode};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    AuxiliaryVerb,
    Adverb,
    Adjective,
    Adnominal,
    Particle,
    Conjunction,
    Prefix,
    Filler,
    Interjection,
    Symbol,
    Other,
}

impl Default for PartOfSpeech {
    fn default() -> Self {
        Self::Other
    }
}

impl From<&str> for PartOfSpeech {
    fn from(value: &str) -> Self {
        match value {
            "名詞" => Self::Noun,
            "動詞" => Self::Verb,
            "助動詞" => Self::AuxiliaryVerb,
            "副詞" => Self::Adverb,
            "形容詞" => Self::Adjective,
            "連体詞" => Self::Adnominal,
            "助詞" => Self::Particle,
            "接続詞" => Self::Conjunction,
            "接頭詞" => Self::Prefix,
            "フィラー" => Self::Filler,
            "感動詞" => Self::Interjection,
            "記号" => Self::Symbol,
            _ => Self::Other,
        }
    }
}

impl fmt::Display for PartOfSpeech {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Noun => "名詞",
            Self::Verb => "動詞",
            Self::AuxiliaryVerb => "助動詞",
            Self::Adverb => "副詞",
            Self::Adjective => "形容詞",
            Self::Adnominal => "連体詞",
            Self::Particle => "助詞",
            Self::Conjunction => "接続詞",
            Self::Prefix => "接頭詞",
            Self::Filler => "フィラー",
            Self::Interjection => "感動詞",
            Self::Symbol => "記号",
            Self::Other => "その他",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
pub enum SubPartOfSpeech {
    General,
    ProperNoun,
    AdverbialParticle,
    Suffix,
    SentenceEndingParticle,
    Region,
    Attributive,
    AdjectivalNounStem,
    AdverbPossible,
    NonIndependentVerb,
    Counter,
    ConjunctiveParticle,
    CaseParticle,
    CloseBracket,
    AdverbialConjunctiveParticle,
    NaiAdjectivalNounStem,
    AuxiliaryVerbStem,
    Noun,
    Comma,
    CoordinatingParticle,
    Adverbialization,
    Independent,
    Number,
    Alphabet,
    OpenBracket,
    Contraction,
    NonIndependent,
    VerbConnection,
    Special,
    DependentParticle,
    NounConnection,
    Surname,
    Space,
    PersonalName,
    QuotedString,
    AdjectiveConnection,
    Conjunctional,
    NumberConnection,
    ParticleConnection,
    Pronoun,
    Country,
    Phrase,
    Organization,
    Quotation,
    SuruVerbConnection,
    SentenceEndingMark,
    Interjection,
    Other,
}

impl From<&str> for SubPartOfSpeech {
    fn from(value: &str) -> Self {
        match value {
            "一般" => Self::General,
            "固有名詞" => Self::ProperNoun,
            "副助詞" => Self::AdverbialParticle,
            "接尾" => Self::Suffix,
            "終助詞" => Self::SentenceEndingParticle,
            "地域" => Self::Region,
            "連体化" => Self::Attributive,
            "形容動詞語幹" => Self::AdjectivalNounStem,
            "副詞可能" => Self::AdverbPossible,
            "動詞非自立的" => Self::NonIndependentVerb,
            "助数詞" => Self::Counter,
            "接続助詞" => Self::ConjunctiveParticle,
            "格助詞" => Self::CaseParticle,
            "括弧閉" => Self::CloseBracket,
            "副助詞／並立助詞／終助詞" => Self::AdverbialConjunctiveParticle,
            "ナイ形容詞語幹" => Self::NaiAdjectivalNounStem,
            "助動詞語幹" => Self::AuxiliaryVerbStem,
            "名" => Self::Noun,
            "読点" => Self::Comma,
            "並立助詞" => Self::CoordinatingParticle,
            "副詞化" => Self::Adverbialization,
            "自立" => Self::Independent,
            "数" => Self::Number,
            "アルファベット" => Self::Alphabet,
            "括弧開" => Self::OpenBracket,
            "縮約" => Self::Contraction,
            "非自立" => Self::NonIndependent,
            "動詞接続" => Self::VerbConnection,
            "特殊" => Self::Special,
            "係助詞" => Self::DependentParticle,
            "名詞接続" => Self::NounConnection,
            "姓" => Self::Surname,
            "空白" => Self::Space,
            "人名" => Self::PersonalName,
            "引用文字列" => Self::QuotedString,
            "形容詞接続" => Self::AdjectiveConnection,
            "接続詞的" => Self::Conjunctional,
            "数接続" => Self::NumberConnection,
            "助詞類接続" => Self::ParticleConnection,
            "代名詞" => Self::Pronoun,
            "国" => Self::Country,
            "連語" => Self::Phrase,
            "組織" => Self::Organization,
            "引用" => Self::Quotation,
            "サ変接続" => Self::SuruVerbConnection,
            "句点" => Self::SentenceEndingMark,
            "間投" => Self::Interjection,
            _ => Self::Other,
        }
    }
}

impl fmt::Display for SubPartOfSpeech {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::General => "一般",
            Self::ProperNoun => "固有名詞",
            Self::AdverbialParticle => "副助詞",
            Self::Suffix => "接尾",
            Self::SentenceEndingParticle => "終助詞",
            Self::Region => "地域",
            Self::Attributive => "連体化",
            Self::AdjectivalNounStem => "形容動詞語幹",
            Self::AdverbPossible => "副詞可能",
            Self::NonIndependentVerb => "動詞非自立的",
            Self::Counter => "助数詞",
            Self::ConjunctiveParticle => "接続助詞",
            Self::CaseParticle => "格助詞",
            Self::CloseBracket => "括弧閉",
            Self::AdverbialConjunctiveParticle => "副助詞／並立助詞／終助詞",
            Self::NaiAdjectivalNounStem => "ナイ形容詞語幹",
            Self::AuxiliaryVerbStem => "助動詞語幹",
            Self::Noun => "名",
            Self::Comma => "読点",
            Self::CoordinatingParticle => "並立助詞",
            Self::Adverbialization => "副詞化",
            Self::Independent => "自立",
            Self::Number => "数",
            Self::Alphabet => "アルファベット",
            Self::OpenBracket => "括弧開",
            Self::Contraction => "縮約",
            Self::NonIndependent => "非自立",
            Self::VerbConnection => "動詞接続",
            Self::Special => "特殊",
            Self::DependentParticle => "係助詞",
            Self::NounConnection => "名詞接続",
            Self::Surname => "姓",
            Self::Space => "空白",
            Self::PersonalName => "人名",
            Self::QuotedString => "引用文字列",
            Self::AdjectiveConnection => "形容詞接続",
            Self::Conjunctional => "接続詞的",
            Self::NumberConnection => "数接続",
            Self::ParticleConnection => "助詞類接続",
            Self::Pronoun => "代名詞",
            Self::Country => "国",
            Self::Phrase => "連語",
            Self::Organization => "組織",
            Self::Quotation => "引用",
            Self::SuruVerbConnection => "サ変接続",
            Self::SentenceEndingMark => "句点",
            Self::Interjection => "間投",
            Self::Other => "その他",
        };

        write!(f, "{s}")
    }
}
