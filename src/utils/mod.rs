pub mod i18n;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Chinese,
    English,
}

pub use i18n::i18n;
