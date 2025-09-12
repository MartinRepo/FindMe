use crate::models::{BirthDate, BirthFortuneModel};
use crate::views::BirthFortuneView;
use crate::utils::{Language, i18n};
use std::io::Write;

pub struct BirthFortuneController;

impl BirthFortuneController {
    pub fn get_birth_date_from_user(lang: Language) -> Result<BirthDate, String> {
        loop {
            print!("{}", i18n("birth.enter_date", lang));
            std::io::stdout().flush().unwrap();
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            if input.is_empty() {
                println!("{}", i18n("birth.invalid_date", lang));
                continue;
            }
            
            match BirthDate::from_string(input) {
                Ok(birth_date) => return Ok(birth_date),
                Err(e) => {
                    println!("{}", i18n("birth.date_format_error", lang).replace("{}", &e));
                    continue;
                }
            }
        }
    }
    
    pub fn process_birth_fortune(lang: Language) -> Result<(), String> {
        let birth_date = Self::get_birth_date_from_user(lang)?;
        let fortune = BirthFortuneModel::calculate_fortune(&birth_date, lang);
        BirthFortuneView::display_birth_fortune(&fortune, &birth_date, lang);
        Ok(())
    }
    
    pub fn validate_birth_date(date_str: &str) -> Result<BirthDate, String> {
        BirthDate::from_string(date_str)
    }
}
