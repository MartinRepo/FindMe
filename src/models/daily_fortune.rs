use chrono::{Datelike, Local};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::utils::{Language, i18n};

#[derive(Debug)]
pub struct FinduFortune {
    pub score: u8,
    pub message: String,
    pub advice: String,
    pub lucky_color: String,
    pub lucky_time: String,
}

pub fn generate_daily_fortune(lang: Language) -> FinduFortune {
    let today = Local::now();
    let seed = today.year() as u32 * 10000 + today.month() as u32 * 100 + today.day() as u32;
    
    let mut rng = StdRng::seed_from_u64(seed as u64);
    
    let score = rng.gen_range(30..=95);
    let (message, advice) = get_fortune_message(score, lang);
    let lucky_color = get_lucky_color(&mut rng, lang);
    let lucky_time = get_lucky_time(&mut rng, lang);
    
    FinduFortune {
        score,
        message,
        advice,
        lucky_color,
        lucky_time,
    }
}

fn get_fortune_message(score: u8, lang: Language) -> (String, String) {
    match score {
        90..=95 => (i18n("fortune.excellent", lang), i18n("fortune.advice.excellent", lang)),
        80..=89 => (i18n("fortune.great", lang), i18n("fortune.advice.great", lang)),
        70..=79 => (i18n("fortune.good", lang), i18n("fortune.advice.good", lang)),
        60..=69 => (i18n("fortune.average", lang), i18n("fortune.advice.average", lang)),
        50..=59 => (i18n("fortune.careful", lang), i18n("fortune.advice.careful", lang)),
        40..=49 => (i18n("fortune.poor", lang), i18n("fortune.advice.poor", lang)),
        30..=39 => (i18n("fortune.challenging", lang), i18n("fortune.advice.challenging", lang)),
        _ => (i18n("fortune.unknown", lang), i18n("fortune.advice.unknown", lang)),
    }
}

fn get_lucky_color(rng: &mut StdRng, lang: Language) -> String {
    let color_keys = vec![
        "color.blue", "color.green", "color.purple", "color.orange", 
        "color.red", "color.yellow", "color.cyan", "color.pink"
    ];
    let selected_key = color_keys[rng.gen_range(0..color_keys.len())];
    i18n(selected_key, lang)
}

fn get_lucky_time(rng: &mut StdRng, lang: Language) -> String {
    let time_keys = vec![
        "time.morning_9_11", "time.afternoon_2_4", "time.evening_7_9", "time.night_1_3",
        "time.noon_12_1", "time.afternoon_5_6", "time.evening_10_11"
    ];
    let selected_key = time_keys[rng.gen_range(0..time_keys.len())];
    i18n(selected_key, lang)
}
