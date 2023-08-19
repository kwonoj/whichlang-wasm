#![no_std]
use wasm_bindgen::prelude::*;
use whichlang::Lang;

#[cfg(all(target_arch = "wasm32", not(target_feature = "atomics")))]
#[global_allocator]
static A: rlsf::SmallGlobalTlsf = rlsf::SmallGlobalTlsf::new();

/// Bindgen-serializable wrapper around `whichlang::Lang`
/// returns iso 6391 language code
#[derive(Clone, Copy, Eq, PartialEq)]
#[wasm_bindgen]
pub enum Language {
    Ara = "AR",
    Cmn = "ZH",
    Deu = "DE",
    Eng = "EN",
    Fra = "FR",
    Hin = "HI",
    Ita = "IT",
    Jpn = "JA",
    Kor = "KO",
    Nld = "NL",
    Por = "PT",
    Rus = "RU",
    Spa = "ES",
    Swe = "SV",
    Tur = "TR",
    Vie = "VI",
}

impl From<Lang> for Language {
    fn from(lang: Lang) -> Self {
        match lang {
            Lang::Ara => Language::Ara,
            Lang::Cmn => Language::Cmn,
            Lang::Deu => Language::Deu,
            Lang::Eng => Language::Eng,
            Lang::Fra => Language::Fra,
            Lang::Hin => Language::Hin,
            Lang::Ita => Language::Ita,
            Lang::Jpn => Language::Jpn,
            Lang::Kor => Language::Kor,
            Lang::Nld => Language::Nld,
            Lang::Por => Language::Por,
            Lang::Rus => Language::Rus,
            Lang::Spa => Language::Spa,
            Lang::Swe => Language::Swe,
            Lang::Tur => Language::Tur,
            Lang::Vie => Language::Vie,
        }
    }
}

#[wasm_bindgen(js_name = "detectLanguage")]
pub fn detect_language(text: &str) -> Language {
    whichlang::detect_language(text).into()
}
