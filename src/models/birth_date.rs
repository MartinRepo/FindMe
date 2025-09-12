use chrono::NaiveDate;
use crate::utils::{Language, i18n};

#[derive(Debug, Clone)]
pub struct BirthDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl BirthDate {
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, String> {
        if let Some(_date) = NaiveDate::from_ymd_opt(year, month, day) {
            Ok(BirthDate { year, month, day })
        } else {
            Err("Invalid date".to_string())
        }
    }
    
    pub fn from_string(date_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return Err("Invalid date format. Use YYYY-MM-DD".to_string());
        }
        
        let year = parts[0].parse::<i32>().map_err(|_| "Invalid year")?;
        let month = parts[1].parse::<u32>().map_err(|_| "Invalid month")?;
        let day = parts[2].parse::<u32>().map_err(|_| "Invalid day")?;
        
        BirthDate::new(year, month, day)
    }
    
    pub fn calculate_life_number(&self) -> u8 {
        let mut sum = self.year + self.month as i32 + self.day as i32;
        while sum > 9 {
            sum = (sum / 10) + (sum % 10);
        }
        sum as u8
    }
    
    pub fn get_zodiac_sign(&self, lang: Language) -> String {
        let zodiac_key = match self.month {
            1 => if self.day <= 19 { "zodiac.capricorn" } else { "zodiac.aquarius" },
            2 => if self.day <= 18 { "zodiac.aquarius" } else { "zodiac.pisces" },
            3 => if self.day <= 20 { "zodiac.pisces" } else { "zodiac.aries" },
            4 => if self.day <= 19 { "zodiac.aries" } else { "zodiac.taurus" },
            5 => if self.day <= 20 { "zodiac.taurus" } else { "zodiac.gemini" },
            6 => if self.day <= 21 { "zodiac.gemini" } else { "zodiac.cancer" },
            7 => if self.day <= 22 { "zodiac.cancer" } else { "zodiac.leo" },
            8 => if self.day <= 22 { "zodiac.leo" } else { "zodiac.virgo" },
            9 => if self.day <= 22 { "zodiac.virgo" } else { "zodiac.libra" },
            10 => if self.day <= 23 { "zodiac.libra" } else { "zodiac.scorpio" },
            11 => if self.day <= 22 { "zodiac.scorpio" } else { "zodiac.sagittarius" },
            12 => if self.day <= 21 { "zodiac.sagittarius" } else { "zodiac.capricorn" },
            _ => "zodiac.unknown",
        };
        i18n(zodiac_key, lang)
    }
}
