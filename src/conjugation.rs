use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq,  Encode, Decode)]
pub enum ConjugationForm {
    ClassicalBasicForm,          // 文語基本形
    IrrealisForm,                // 未然形
    ImperativeE,                 // 命令ｅ
    ImperativeYo,                // 命令ｙｏ
    ContinuativeDeConnection,    // 連用デ接続
    ContinuativeForm,            // 連用形
    NounConnection,              // 体言接続
    BasicForm,                   // 基本形
    ContinuativeTaConnection,    // 連用タ接続
    HypotheticalContraction1,    // 仮定縮約１
    NounConnectionSpecial,       // 体言接続特殊
    NounConnectionSpecial2,      // 体言接続特殊２
    ImperativeI,                 // 命令ｉ
    GalConnection,               // ガル接続
    IrrealisRelConnection,       // 未然レル接続
    IrrealisUConnection,         // 未然ウ接続
    ModernBasicForm,             // 現代基本形
    ContinuativeNiConnection,    // 連用ニ接続
    SoundChangeBasicForm,        // 音便基本形
    ImperativeRo,                // 命令ｒｏ
    IrrealisNuConnection,        // 未然ヌ接続
    ContinuativeTeConnection,    // 連用テ接続
    ContinuativeGozaiConnection, // 連用ゴザイ接続
    HypotheticalForm,            // 仮定形
    BasicFormWithSokuonben,      // 基本形-促音便
    HypotheticalContraction2,    // 仮定縮約２
    IrrealisSpecial,             // 未然特殊
    Unknown,
}

impl From<&str> for ConjugationForm {
    fn from(value: &str) -> Self {
        match value {
            "文語基本形" => Self::ClassicalBasicForm,
            "未然形" => Self::IrrealisForm,
            "命令ｅ" => Self::ImperativeE,
            "命令ｙｏ" => Self::ImperativeYo,
            "連用デ接続" => Self::ContinuativeDeConnection,
            "連用形" => Self::ContinuativeForm,
            "体言接続" => Self::NounConnection,
            "基本形" => Self::BasicForm,
            "連用タ接続" => Self::ContinuativeTaConnection,
            "仮定縮約１" => Self::HypotheticalContraction1,
            "体言接続特殊" => Self::NounConnectionSpecial,
            "体言接続特殊２" => Self::NounConnectionSpecial2,
            "命令ｉ" => Self::ImperativeI,
            "ガル接続" => Self::GalConnection,
            "未然レル接続" => Self::IrrealisRelConnection,
            "未然ウ接続" => Self::IrrealisUConnection,
            "現代基本形" => Self::ModernBasicForm,
            "連用ニ接続" => Self::ContinuativeNiConnection,
            "音便基本形" => Self::SoundChangeBasicForm,
            "命令ｒｏ" => Self::ImperativeRo,
            "未然ヌ接続" => Self::IrrealisNuConnection,
            "連用テ接続" => Self::ContinuativeTeConnection,
            "連用ゴザイ接続" => Self::ContinuativeGozaiConnection,
            "仮定形" => Self::HypotheticalForm,
            "基本形-促音便" => Self::BasicFormWithSokuonben,
            "仮定縮約２" => Self::HypotheticalContraction2,
            "未然特殊" => Self::IrrealisSpecial,
            _ => Self::Unknown,
        }
    }
}

impl ToString for ConjugationForm {
    fn to_string(&self) -> String {
        match self {
            Self::ClassicalBasicForm => "文語基本形",
            Self::IrrealisForm => "未然形",
            Self::ImperativeE => "命令ｅ",
            Self::ImperativeYo => "命令ｙｏ",
            Self::ContinuativeDeConnection => "連用デ接続",
            Self::ContinuativeForm => "連用形",
            Self::NounConnection => "体言接続",
            Self::BasicForm => "基本形",
            Self::ContinuativeTaConnection => "連用タ接続",
            Self::HypotheticalContraction1 => "仮定縮約１",
            Self::NounConnectionSpecial => "体言接続特殊",
            Self::NounConnectionSpecial2 => "体言接続特殊２",
            Self::ImperativeI => "命令ｉ",
            Self::GalConnection => "ガル接続",
            Self::IrrealisRelConnection => "未然レル接続",
            Self::IrrealisUConnection => "未然ウ接続",
            Self::ModernBasicForm => "現代基本形",
            Self::ContinuativeNiConnection => "連用ニ接続",
            Self::SoundChangeBasicForm => "音便基本形",
            Self::ImperativeRo => "命令ｒｏ",
            Self::IrrealisNuConnection => "未然ヌ接続",
            Self::ContinuativeTeConnection => "連用テ接続",
            Self::ContinuativeGozaiConnection => "連用ゴザイ接続",
            Self::HypotheticalForm => "仮定形",
            Self::BasicFormWithSokuonben => "基本形-促音便",
            Self::HypotheticalContraction2 => "仮定縮約２",
            Self::IrrealisSpecial => "未然特殊",
            Self::Unknown => "未知",
        }
        .to_owned()
    }
}
