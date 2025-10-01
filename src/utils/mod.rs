pub mod i18n;
pub mod user;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Chinese,
    English,
}

pub use i18n::i18n;
pub use user::detect_user_name;
