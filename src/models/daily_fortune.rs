use crate::utils::{i18n, Language};
use chrono::{Datelike, Local, Weekday};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum WorkScenario {
    Workday,
    Weekend,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechDimensions {
    pub focus: u8,
    pub creativity: u8,
    pub debugging: u8,
    pub collaboration: u8,
    pub risk_tolerance: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FortuneTemplate {
    pub condition: String,
    pub message: String,
    pub advice: String,
    pub color: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FortunePack {
    pub name: String,
    pub version: String,
    pub templates: HashMap<String, FortuneTemplate>,
    pub dimension_weights: HashMap<String, f32>,
}

#[derive(Debug)]
pub struct Fortune {
    pub scenario: WorkScenario,
    pub dimensions: TechDimensions,
    pub overall_score: u8,
    pub message: String,
    pub advice: String,
    pub lucky_color: String,
    pub lucky_time: String,
}

pub fn generate_daily_fortune(lang: Language) -> Fortune {
    let today = Local::now();
    let today_str = format!("{:04}{:02}{:02}", today.year(), today.month(), today.day());
    generate_fortune_with_seed(&today_str, None, lang)
}

pub fn generate_daily_fortune_with_birthday(birthday: &str, lang: Language) -> Fortune {
    let today = Local::now();
    let today_str = format!("{:04}{:02}{:02}", today.year(), today.month(), today.day());
    generate_fortune_with_seed(&today_str, Some(birthday), lang)
}

fn generate_fortune_with_seed(today: &str, birthday: Option<&str>, lang: Language) -> Fortune {
    let today_date = chrono::NaiveDate::parse_from_str(today, "%Y%m%d").unwrap();
    let scenario = match today_date.weekday() {
        Weekday::Sat | Weekday::Sun => WorkScenario::Weekend,
        _ => WorkScenario::Workday,
    };

    let mut hasher = Sha256::new();
    hasher.update(today.as_bytes());
    if let Some(bday) = birthday {
        hasher.update(bday.as_bytes());
    }
    hasher.update(b"findme_v0.1.0");
    hasher.update(b"dev_salt");

    let hash = hasher.finalize();
    let seed = u64::from_le_bytes([
        hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
    ]);

    let mut rng = ChaCha20Rng::seed_from_u64(seed);

    let dimensions = generate_scenario_biased_dimensions(&scenario, &mut rng);

    let overall_score = calculate_scenario_weighted_score(&dimensions, &scenario);

    let (message, advice, color, time) =
        generate_fortune_content(&dimensions, &scenario, &mut rng, lang);

    Fortune {
        scenario,
        dimensions,
        overall_score,
        message,
        advice,
        lucky_color: color,
        lucky_time: time,
    }
}

fn generate_scenario_biased_dimensions(
    scenario: &WorkScenario,
    rng: &mut ChaCha20Rng,
) -> TechDimensions {
    match scenario {
        WorkScenario::Workday => TechDimensions {
            focus: rng.gen_range(40..=100),
            creativity: rng.gen_range(20..=80),
            debugging: rng.gen_range(50..=100),
            collaboration: rng.gen_range(60..=100),
            risk_tolerance: rng.gen_range(10..=60),
        },
        WorkScenario::Weekend => TechDimensions {
            focus: rng.gen_range(20..=70),
            creativity: rng.gen_range(60..=100),
            debugging: rng.gen_range(30..=80),
            collaboration: rng.gen_range(10..=50),
            risk_tolerance: rng.gen_range(50..=100),
        },
    }
}

fn calculate_scenario_weighted_score(dimensions: &TechDimensions, scenario: &WorkScenario) -> u8 {
    match scenario {
        WorkScenario::Workday => {
            let weighted_sum = (dimensions.focus as f32 * 0.35)
                + (dimensions.debugging as f32 * 0.30)
                + (dimensions.collaboration as f32 * 0.25)
                + (dimensions.creativity as f32 * 0.07)
                + (dimensions.risk_tolerance as f32 * 0.03);
            weighted_sum.round() as u8
        }
        WorkScenario::Weekend => {
            let weighted_sum = (dimensions.creativity as f32 * 0.40)
                + (dimensions.risk_tolerance as f32 * 0.30)
                + (dimensions.focus as f32 * 0.20)
                + (dimensions.debugging as f32 * 0.07)
                + (dimensions.collaboration as f32 * 0.03);
            weighted_sum.round() as u8
        }
    }
}

fn generate_fortune_content(
    dimensions: &TechDimensions,
    scenario: &WorkScenario,
    rng: &mut ChaCha20Rng,
    lang: Language,
) -> (String, String, String, String) {
    let primary_dim = if dimensions.focus >= dimensions.creativity
        && dimensions.focus >= dimensions.debugging
        && dimensions.focus >= dimensions.collaboration
        && dimensions.focus >= dimensions.risk_tolerance
    {
        "focus"
    } else if dimensions.creativity >= dimensions.debugging
        && dimensions.creativity >= dimensions.collaboration
        && dimensions.creativity >= dimensions.risk_tolerance
    {
        "creativity"
    } else if dimensions.debugging >= dimensions.collaboration
        && dimensions.debugging >= dimensions.risk_tolerance
    {
        "debugging"
    } else if dimensions.collaboration >= dimensions.risk_tolerance {
        "collaboration"
    } else {
        "risk_tolerance"
    };

    let message = generate_message(dimensions, primary_dim, scenario, lang);
    let advice = generate_advice(dimensions, primary_dim, scenario, lang);
    let color = generate_color(dimensions, scenario, rng, lang);
    let time = generate_time(dimensions, scenario, rng, lang);

    (message, advice, color, time)
}

fn generate_message(
    dimensions: &TechDimensions,
    primary_dim: &str,
    scenario: &WorkScenario,
    lang: Language,
) -> String {
    let score = calculate_scenario_weighted_score(dimensions, scenario);

    match score {
        90..=100 => match primary_dim {
            "focus" => i18n("fortune.excellent.focus", lang),
            "creativity" => i18n("fortune.excellent.creativity", lang),
            "debugging" => i18n("fortune.excellent.debugging", lang),
            "collaboration" => i18n("fortune.excellent.collaboration", lang),
            "risk_tolerance" => i18n("fortune.excellent.risk", lang),
            _ => i18n("fortune.excellent", lang),
        },
        80..=89 => match primary_dim {
            "focus" => i18n("fortune.great.focus", lang),
            "creativity" => i18n("fortune.great.creativity", lang),
            "debugging" => i18n("fortune.great.debugging", lang),
            "collaboration" => i18n("fortune.great.collaboration", lang),
            "risk_tolerance" => i18n("fortune.great.risk", lang),
            _ => i18n("fortune.great", lang),
        },
        70..=79 => i18n("fortune.good", lang),
        60..=69 => i18n("fortune.average", lang),
        50..=59 => i18n("fortune.careful", lang),
        40..=49 => i18n("fortune.poor", lang),
        30..=39 => i18n("fortune.challenging", lang),
        _ => i18n("fortune.unknown", lang),
    }
}

fn generate_advice(
    dimensions: &TechDimensions,
    primary_dim: &str,
    scenario: &WorkScenario,
    lang: Language,
) -> String {
    let score = calculate_scenario_weighted_score(dimensions, scenario);

    match score {
        90..=100 => match primary_dim {
            "focus" => i18n("advice.excellent.focus", lang),
            "creativity" => i18n("advice.excellent.creativity", lang),
            "debugging" => i18n("advice.excellent.debugging", lang),
            "collaboration" => i18n("advice.excellent.collaboration", lang),
            "risk_tolerance" => i18n("advice.excellent.risk", lang),
            _ => i18n("advice.excellent", lang),
        },
        80..=89 => match primary_dim {
            "focus" => i18n("advice.great.focus", lang),
            "creativity" => i18n("advice.great.creativity", lang),
            "debugging" => i18n("advice.great.debugging", lang),
            "collaboration" => i18n("advice.great.collaboration", lang),
            "risk_tolerance" => i18n("advice.great.risk", lang),
            _ => i18n("advice.great", lang),
        },
        70..=79 => i18n("advice.good", lang),
        60..=69 => i18n("advice.average", lang),
        50..=59 => i18n("advice.careful", lang),
        40..=49 => i18n("advice.poor", lang),
        30..=39 => i18n("advice.challenging", lang),
        _ => i18n("advice.unknown", lang),
    }
}

fn generate_color(
    dimensions: &TechDimensions,
    scenario: &WorkScenario,
    rng: &mut ChaCha20Rng,
    lang: Language,
) -> String {
    let color_keys = match scenario {
        WorkScenario::Workday => {
            if dimensions.focus > 70 {
                vec!["color.blue", "color.cyan", "color.green"]
            } else if dimensions.debugging > 70 {
                vec!["color.blue", "color.green", "color.cyan"]
            } else {
                vec!["color.blue", "color.green", "color.cyan", "color.yellow"]
            }
        }
        WorkScenario::Weekend => {
            if dimensions.creativity > 70 {
                vec!["color.purple", "color.pink", "color.orange"]
            } else if dimensions.risk_tolerance > 70 {
                vec!["color.red", "color.orange", "color.pink"]
            } else {
                vec!["color.purple", "color.pink", "color.orange", "color.red"]
            }
        }
    };

    let selected_key = color_keys[rng.gen_range(0..color_keys.len())];
    i18n(selected_key, lang)
}

fn generate_time(
    dimensions: &TechDimensions,
    scenario: &WorkScenario,
    rng: &mut ChaCha20Rng,
    lang: Language,
) -> String {
    let time_keys = match scenario {
        WorkScenario::Workday => {
            if dimensions.focus > 80 {
                vec!["time.morning_9_11", "time.afternoon_2_4"]
            } else if dimensions.collaboration > 70 {
                vec!["time.afternoon_2_4", "time.afternoon_5_6"]
            } else if dimensions.debugging > 70 {
                vec!["time.morning_9_11", "time.afternoon_2_4"]
            } else {
                vec![
                    "time.morning_9_11",
                    "time.afternoon_2_4",
                    "time.afternoon_5_6",
                ]
            }
        }
        WorkScenario::Weekend => {
            if dimensions.creativity > 70 {
                vec!["time.evening_7_9", "time.night_1_3"]
            } else if dimensions.risk_tolerance > 70 {
                vec!["time.evening_10_11", "time.night_1_3"]
            } else {
                vec!["time.evening_7_9", "time.evening_10_11", "time.night_1_3"]
            }
        }
    };

    let selected_key = time_keys[rng.gen_range(0..time_keys.len())];
    i18n(selected_key, lang)
}
