use crate::models::FinduFortune;
use crate::utils::{Language, i18n};
use colored::*;


pub fn display_fortune(fortune: &FinduFortune, lang: Language) {
    println!();
    println!("{}", "=".repeat(50).cyan());
    println!("{}", i18n("app.title", lang).bold().yellow());
    println!("{}", "=".repeat(50).cyan());
    println!();
    
    let score_color = match fortune.score {
        90..=95 => "bright_green",
        80..=89 => "green", 
        70..=79 => "yellow",
        60..=69 => "bright_yellow",
        50..=59 => "bright_red",
        40..=49 => "red",
        _ => "red"
    };
    
    println!("{}: {}", i18n("fortune.score_label", lang), format!("{}", fortune.score).color(score_color).bold());
    println!("{}: {}", i18n("fortune.message_label", lang), fortune.message.green());
    println!("{}: {}", i18n("fortune.advice_label", lang), fortune.advice.cyan());
    println!("{}: {}", i18n("fortune.lucky_color_label", lang), fortune.lucky_color.magenta());
    println!("{}: {}", i18n("fortune.lucky_time_label", lang), fortune.lucky_time.blue());
    println!();
    
    display_detailed_advice(fortune.score, lang);
    
    println!("{}", "=".repeat(50).cyan());
    println!();
}

fn display_detailed_advice(score: u8, lang: Language) {
    println!("{}", i18n("fortune.detailed_advice_title", lang).bold().yellow());
    
    match score {
        90..=95 => {
            println!("✅ {}", i18n("advice.excellent.refactor", lang));
            println!("✅ {}", i18n("advice.excellent.framework", lang));
            println!("✅ {}", i18n("advice.excellent.review", lang));
            println!("✅ {}", i18n("advice.excellent.problem", lang));
        },
        80..=89 => {
            println!("✅ {}", i18n("advice.great.efficiency", lang));
            println!("✅ {}", i18n("advice.great.development", lang));
            println!("✅ {}", i18n("advice.great.quality", lang));
            println!("⚠️ {}", i18n("advice.great.standards", lang));
        },
        70..=79 => {
            println!("✅ {}", i18n("advice.good.condition", lang));
            println!("⚠️ {}", i18n("advice.good.check", lang));
            println!("⚠️ {}", i18n("advice.good.documentation", lang));
            println!("⚠️ {}", i18n("advice.good.optimization", lang));
        },
        60..=69 => {
            println!("⚠️ {}", i18n("advice.average.careful", lang));
            println!("⚠️ {}", i18n("advice.average.tests", lang));
            println!("⚠️ {}", i18n("advice.average.logic", lang));
            println!("⚠️ {}", i18n("advice.average.review", lang));
        },
        50..=59 => {
            println!("🚨 {}", i18n("advice.careful.mistakes", lang));
            println!("🚨 {}", i18n("advice.careful.simple", lang));
            println!("🚨 {}", i18n("advice.careful.documentation", lang));
            println!("🚨 {}", i18n("advice.careful.rush", lang));
        },
        40..=49 => {
            println!("🚨 {}", i18n("advice.poor.challenges", lang));
            println!("🚨 {}", i18n("advice.poor.complex", lang));
            println!("🚨 {}", i18n("advice.poor.help", lang));
            println!("🚨 {}", i18n("advice.poor.patience", lang));
        },
        _ => {
            println!("🚨 {}", i18n("advice.terrible.fortune", lang));
            println!("🚨 {}", i18n("advice.terrible.learning", lang));
            println!("🚨 {}", i18n("advice.terrible.avoid", lang));
            println!("🚨 {}", i18n("advice.terrible.attitude", lang));
        }
    }
}
