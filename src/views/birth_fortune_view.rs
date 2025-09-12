use crate::models::{BirthFortune, BirthDate};
use crate::utils::{Language, i18n};
use colored::*;

pub struct BirthFortuneView;

impl BirthFortuneView {
    pub fn display_birth_fortune(fortune: &BirthFortune, birth_date: &BirthDate, lang: Language) {
        println!();
        println!("{}", "=".repeat(60).cyan());
        println!("{}", i18n("app.birth_title", lang).bold().yellow());
        println!("{}", "=".repeat(60).cyan());
        println!();
        
        println!("{}: {}-{}-{}", i18n("birth.birth_date_label", lang), birth_date.year, birth_date.month, birth_date.day);
        println!("{}: {}", i18n("birth.life_number_label", lang), format!("{}", fortune.life_number).color("bright_blue").bold());
        println!("{}: {}", i18n("birth.zodiac_sign_label", lang), fortune.zodiac_sign.color("magenta"));
        println!();
        
        println!("{}", i18n("birth.personality_title", lang).bold().yellow());
        for trait_item in &fortune.personality_traits {
            println!("  • {}", trait_item.green());
        }
        println!();
        
        println!("{}", i18n("birth.career_title", lang).bold().yellow());
        println!("  {}", fortune.career_advice.cyan());
        println!();
        
        println!("{}", i18n("birth.lucky_numbers_title", lang).bold().yellow());
        let numbers_str = fortune.lucky_numbers.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("  {}", numbers_str.color("bright_yellow"));
        println!();
        
        println!("{}", i18n("birth.compatibility_title", lang).bold().yellow());
        let compat_str = fortune.compatibility.join(", ");
        println!("  {}", compat_str.color("bright_green"));
        println!();
        
        println!("{}", "=".repeat(60).cyan());
        println!();
    }
}

