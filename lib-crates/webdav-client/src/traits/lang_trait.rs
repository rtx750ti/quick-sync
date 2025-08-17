#[allow(dead_code)]
pub(crate) trait LangTrait {
    fn select<'a>(&self, zh: &'a str, en: &'a str) -> &'a str;
}