use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
pub enum Language {
    ArAe,
    DeDe,
    EnUs,
    EsEs,
    EsMx,
    FrFr,
    IdId,
    ItIt,
    JaJp,
    KoKr,
    PlPl,
    PtBr,
    RuRu,
    ThTh,
    TrTr,
    ViVn,
    ZhCn,
    ZhTw,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::ArAe => write!(f, "ar-AE"),
            Language::DeDe => write!(f, "de-DE"),
            Language::EnUs => write!(f, "en-US"),
            Language::EsEs => write!(f, "es-ES"),
            Language::EsMx => write!(f, "es-MX"),
            Language::FrFr => write!(f, "fr-FR"),
            Language::IdId => write!(f, "id-ID"),
            Language::ItIt => write!(f, "it-IT"),
            Language::JaJp => write!(f, "ja-JP"),
            Language::KoKr => write!(f, "ko-KR"),
            Language::PlPl => write!(f, "pl-PL"),
            Language::PtBr => write!(f, "pt-BR"),
            Language::RuRu => write!(f, "ru-RU"),
            Language::ThTh => write!(f, "th-TH"),
            Language::TrTr => write!(f, "tr-TR"),
            Language::ViVn => write!(f, "vi-VN"),
            Language::ZhCn => write!(f, "zh-CN"),
            Language::ZhTw => write!(f, "zh-TW"),
        }
    }
}
