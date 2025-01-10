use bincode::{Decode, Encode};
use strum::{Display, EnumString};

#[derive(Debug, Default, Display, Clone, Copy, PartialEq, Encode, Decode, EnumString)]
pub enum PartOfSpeech {
    #[strum(serialize = "名詞")]
    Noun,
    #[strum(serialize = "動詞")]
    Verb,
    #[strum(serialize = "助動詞")]
    AuxiliaryVerb,
    #[strum(serialize = "副詞")]
    Adverb,
    #[strum(serialize = "形容詞")]
    Adjective,
    #[strum(serialize = "連体詞")]
    Adnominal,
    #[strum(serialize = "助詞")]
    Particle,
    #[strum(serialize = "接続詞")]
    Conjunction,
    #[strum(serialize = "接頭詞")]
    Prefix,
    #[strum(serialize = "フィラー")]
    Filler,
    #[strum(serialize = "感動詞")]
    Interjection,
    #[strum(serialize = "記号")]
    Symbol,
    #[default]
    #[strum(serialize = "その他")]
    Other,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Encode, Decode, EnumString)]
pub enum SubPartOfSpeech {
    #[strum(serialize = "一般")]
    General,
    #[strum(serialize = "固有名詞")]
    ProperNoun,
    #[strum(serialize = "副助詞")]
    AdverbialParticle,
    #[strum(serialize = "接尾")]
    Suffix,
    #[strum(serialize = "終助詞")]
    SentenceEndingParticle,
    #[strum(serialize = "地域")]
    Region,
    #[strum(serialize = "連体化")]
    Attributive,
    #[strum(serialize = "形容動詞語幹")]
    AdjectivalNounStem,
    #[strum(serialize = "副詞可能")]
    AdverbPossible,
    #[strum(serialize = "動詞非自立的")]
    NonIndependentVerb,
    #[strum(serialize = "助数詞")]
    Counter,
    #[strum(serialize = "接続助詞")]
    ConjunctiveParticle,
    #[strum(serialize = "格助詞")]
    CaseParticle,
    #[strum(serialize = "括弧閉")]
    CloseBracket,
    #[strum(serialize = "副助詞／並立助詞／終助詞")]
    AdverbialConjunctiveParticle,
    #[strum(serialize = "ナイ形容詞語幹")]
    NaiAdjectivalNounStem,
    #[strum(serialize = "助動詞語幹")]
    AuxiliaryVerbStem,
    #[strum(serialize = "名")]
    Noun,
    #[strum(serialize = "読点")]
    Comma,
    #[strum(serialize = "並立助詞")]
    CoordinatingParticle,
    #[strum(serialize = "副詞化")]
    Adverbialization,
    #[strum(serialize = "自立")]
    Independent,
    #[strum(serialize = "数")]
    Number,
    #[strum(serialize = "アルファベット")]
    Alphabet,
    #[strum(serialize = "括弧開")]
    OpenBracket,
    #[strum(serialize = "縮約")]
    Contraction,
    #[strum(serialize = "非自立")]
    NonIndependent,
    #[strum(serialize = "動詞接続")]
    VerbConnection,
    #[strum(serialize = "特殊")]
    Special,
    #[strum(serialize = "係助詞")]
    DependentParticle,
    #[strum(serialize = "名詞接続")]
    NounConnection,
    #[strum(serialize = "姓")]
    Surname,
    #[strum(serialize = "空白")]
    Space,
    #[strum(serialize = "人名")]
    PersonalName,
    #[strum(serialize = "引用文字列")]
    QuotedString,
    #[strum(serialize = "形容詞接続")]
    AdjectiveConnection,
    #[strum(serialize = "接続詞的")]
    Conjunctional,
    #[strum(serialize = "数接続")]
    NumberConnection,
    #[strum(serialize = "助詞類接続")]
    ParticleConnection,
    #[strum(serialize = "代名詞")]
    Pronoun,
    #[strum(serialize = "国")]
    Country,
    #[strum(serialize = "連語")]
    Phrase,
    #[strum(serialize = "組織")]
    Organization,
    #[strum(serialize = "引用")]
    Quotation,
    #[strum(serialize = "サ変接続")]
    SuruVerbConnection,
    #[strum(serialize = "句点")]
    SentenceEndingMark,
    #[strum(serialize = "間投")]
    Interjection,
    #[strum(serialize = "その他")]
    Other,
}
