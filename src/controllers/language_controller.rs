use crate::utils::{i18n, Language};
use std::io::Write;

pub fn get_language_choice() -> Language {
    println!("{}", i18n("language.choose", Language::Chinese));
    println!("{}", i18n("language.chinese", Language::Chinese));
    println!("{}", i18n("language.english", Language::Chinese));
    println!();

    loop {
        print!("{}", i18n("language.enter_choice", Language::Chinese));
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" | "chinese" => return Language::Chinese,
            "2" | "english" | "English" => return Language::English,
            _ => {
                // Check against i18n translations
                if input.trim() == i18n("lang.chinese", Language::Chinese) {
                    return Language::Chinese;
                } else if input.trim() == i18n("lang.english", Language::Chinese) {
                    return Language::English;
                } else {
                    println!("{}", i18n("language.invalid_choice", Language::Chinese));
                    continue;
                }
            }
        }
    }
}
