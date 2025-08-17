use crate::traits::lang_trait::LangTrait;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub(crate) enum Lang {
    Zh,
    En,
}

#[cfg(feature = "lang-zh")]
pub(crate) const LANG: Lang = Lang::Zh;

#[cfg(feature = "lang-en")]
pub(crate) const LANG: Lang = Lang::En;

#[cfg(all(not(feature = "lang-zh"), not(feature = "lang-en")))]
pub(crate) const LANG: Lang = Lang::En;

impl LangTrait for Lang {
    fn select<'a>(&self, zh: &'a str, en: &'a str) -> &'a str {
        match self {
            Lang::Zh => zh,
            Lang::En => en,
        }
    }
}
