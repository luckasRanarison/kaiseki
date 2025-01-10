use bincode::{Decode, Encode};
use strum::{Display, EnumString};

#[derive(Debug, Display, Clone, Copy, PartialEq, Encode, Decode, EnumString)]
pub enum ConjugationType {
    #[strum(serialize = "文語・ケリ")]
    BungoKeri,
    #[strum(serialize = "一段・得ル")]
    IchidanEru,
    #[strum(serialize = "特殊・デス")]
    SpecialDesu,
    #[strum(serialize = "五段・ラ行")]
    GodanRaRow,
    #[strum(serialize = "特殊・ダ")]
    SpecialDa,
    #[strum(serialize = "サ変・スル")]
    IrregularSuru,
    #[strum(serialize = "形容詞・イイ")]
    AdjectiveIi,
    #[strum(serialize = "下二・タ行")]
    NidanTaRow,
    #[strum(serialize = "一段・クレル")]
    IchidanKureru,
    #[strum(serialize = "下二・ハ行")]
    NidanHaRow,
    #[strum(serialize = "下二・ガ行")]
    NidanGaRow,
    #[strum(serialize = "五段・ナ行")]
    GodanNaRow,
    #[strum(serialize = "特殊・タイ")]
    SpecialTai,
    #[strum(serialize = "五段・カ行促音便")]
    GodanKaRowSokuonben,
    #[strum(serialize = "四段・ハ行")]
    YondanHaRow,
    #[strum(serialize = "五段・サ行")]
    GodanSaRow,
    #[strum(serialize = "五段・ラ行特殊")]
    GodanRaRowSpecial,
    #[strum(serialize = "文語・キ")]
    BungoKi,
    #[strum(serialize = "五段・タ行")]
    GodanTaRow,
    #[strum(serialize = "カ変・クル")]
    KakanKuru,
    #[strum(serialize = "特殊・マス")]
    SpecialMasu,
    #[strum(serialize = "不変化型")]
    Invariable,
    #[strum(serialize = "下二・マ行")]
    NidanMaRow,
    #[strum(serialize = "四段・サ行")]
    YondanSaRow,
    #[strum(serialize = "サ変・－スル")]
    IrregularSuru2,
    #[strum(serialize = "文語・ル")]
    BungoRu,
    #[strum(serialize = "四段・バ行")]
    YondanBaRow,
    #[strum(serialize = "五段・マ行")]
    GodanMaRow,
    #[strum(serialize = "下二・得")]
    NidanToku,
    #[strum(serialize = "文語・ナリ")]
    BungoNari,
    #[strum(serialize = "五段・バ行")]
    GodanBaRow,
    #[strum(serialize = "文語・ゴトシ")]
    BungoGotoshi,
    #[strum(serialize = "形容詞・イ段")]
    AdjectiveI,
    #[strum(serialize = "一段")]
    Ichidan,
    #[strum(serialize = "五段・カ行促音便ユク")]
    GodanKaRowSokuonbenYuku,
    #[strum(serialize = "四段・タ行")]
    YondanTaRow,
    #[strum(serialize = "特殊・ヌ")]
    SpecialNu,
    #[strum(serialize = "形容詞・アウオ段")]
    AdjectiveAuo,
    #[strum(serialize = "五段・ラ行アル")]
    GodanRaRowAru,
    #[strum(serialize = "五段・カ行イ音便")]
    GodanKaRowIOnbin,
    #[strum(serialize = "文語・マジ")]
    BungoMaji,
    #[strum(serialize = "特殊・ヤ")]
    SpecialYa,
    #[strum(serialize = "五段・ワ行ウ音便")]
    GodanWaRowUOnbin,
    #[strum(serialize = "上二・ハ行")]
    JodanHaRow,
    #[strum(serialize = "上二・ダ行")]
    JodanDaRow,
    #[strum(serialize = "五段・ワ行促音便")]
    GodanWaRowSokuonben,
    #[strum(serialize = "特殊・ナイ")]
    SpecialNai,
    #[strum(serialize = "サ変・－ズル")]
    IrregularZuru,
    #[strum(serialize = "下二・カ行")]
    NidanKaRow,
    #[strum(serialize = "下二・ダ行")]
    NidanDaRow,
    #[strum(serialize = "文語・ベシ")]
    BungoBeshi,
    #[strum(serialize = "カ変・来ル")]
    KakanKuru2,
    #[strum(serialize = "ラ変")]
    Rahan,
    #[strum(serialize = "特殊・ジャ")]
    SpecialJa,
    #[strum(serialize = "文語・リ")]
    BungoRi,
    #[strum(serialize = "特殊・タ")]
    SpecialTa,
    #[strum(serialize = "五段・ガ行")]
    GodanGaRow,
    #[strum(serialize = "未知")]
    Unknown,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Encode, Decode, EnumString)]
pub enum ConjugationForm {
    #[strum(serialize = "文語基本形")]
    ClassicalBasicForm,
    #[strum(serialize = "未然形")]
    IrrealisForm,
    #[strum(serialize = "命令ｅ")]
    ImperativeE,
    #[strum(serialize = "命令ｙｏ")]
    ImperativeYo,
    #[strum(serialize = "連用デ接続")]
    ContinuativeDeConnection,
    #[strum(serialize = "連用形")]
    ContinuativeForm,
    #[strum(serialize = "体言接続")]
    NounConnection,
    #[strum(serialize = "基本形")]
    BasicForm,
    #[strum(serialize = "連用タ接続")]
    ContinuativeTaConnection,
    #[strum(serialize = "仮定縮約１")]
    HypotheticalContraction1,
    #[strum(serialize = "体言接続特殊")]
    NounConnectionSpecial,
    #[strum(serialize = "体言接続特殊２")]
    NounConnectionSpecial2,
    #[strum(serialize = "命令ｉ")]
    ImperativeI,
    #[strum(serialize = "ガル接続")]
    GalConnection,
    #[strum(serialize = "未然レル接続")]
    IrrealisRelConnection,
    #[strum(serialize = "未然ウ接続")]
    IrrealisUConnection,
    #[strum(serialize = "現代基本形")]
    ModernBasicForm,
    #[strum(serialize = "連用ニ接続")]
    ContinuativeNiConnection,
    #[strum(serialize = "音便基本形")]
    SoundChangeBasicForm,
    #[strum(serialize = "命令ｒｏ")]
    ImperativeRo,
    #[strum(serialize = "未然ヌ接続")]
    IrrealisNuConnection,
    #[strum(serialize = "連用テ接続")]
    ContinuativeTeConnection,
    #[strum(serialize = "連用ゴザイ接続")]
    ContinuativeGozaiConnection,
    #[strum(serialize = "仮定形")]
    HypotheticalForm,
    #[strum(serialize = "基本形-促音便")]
    BasicFormWithSokuonben,
    #[strum(serialize = "仮定縮約２")]
    HypotheticalContraction2,
    #[strum(serialize = "未然特殊")]
    IrrealisSpecial,
    #[strum(serialize = "未知")]
    Unknown,
}
