use crate::models::BirthDate;
use crate::utils::{Language, i18n};

#[derive(Debug)]
pub struct BirthFortune {
    pub life_number: u8,
    pub zodiac_sign: String,
    pub personality_traits: Vec<String>,
    pub career_advice: String,
    pub lucky_numbers: Vec<u8>,
    pub compatibility: Vec<String>,
}

pub struct BirthFortuneModel;

impl BirthFortuneModel {
    pub fn calculate_fortune(birth_date: &BirthDate, lang: Language) -> BirthFortune {
        let life_number = birth_date.calculate_life_number();
        let zodiac_sign = birth_date.get_zodiac_sign(lang);
        let personality_traits = Self::get_personality_traits(life_number, lang);
        let career_advice = Self::get_career_advice(life_number, &zodiac_sign, lang);
        let lucky_numbers = Self::calculate_lucky_numbers(birth_date);
        let compatibility = Self::get_compatibility(&zodiac_sign, lang);
        
        BirthFortune {
            life_number,
            zodiac_sign,
            personality_traits,
            career_advice,
            lucky_numbers,
            compatibility,
        }
    }
    
    fn get_personality_traits(life_number: u8, lang: Language) -> Vec<String> {
        match life_number {
            1 => vec![i18n("personality.leader", lang), i18n("personality.independent", lang), i18n("personality.innovative", lang)],
            2 => vec![i18n("personality.cooperative", lang), i18n("personality.intuitive", lang), i18n("personality.diplomatic", lang)],
            3 => vec![i18n("personality.creative", lang), i18n("personality.expressive", lang), i18n("personality.optimistic", lang)],
            4 => vec![i18n("personality.reliable", lang), i18n("personality.organized", lang), i18n("personality.detail_oriented", lang)],
            5 => vec![i18n("personality.adventurous", lang), i18n("personality.adaptable", lang), i18n("personality.curious", lang)],
            6 => vec![i18n("personality.responsible", lang), i18n("personality.caring", lang), i18n("personality.artistic", lang)],
            7 => vec![i18n("personality.analytical", lang), i18n("personality.intuitive", lang), i18n("personality.truth_seeker", lang)],
            8 => vec![i18n("personality.business_minded", lang), i18n("personality.ambitious", lang), i18n("personality.goal_oriented", lang)],
            9 => vec![i18n("personality.compassionate", lang), i18n("personality.wise", lang), i18n("personality.idealistic", lang)],
            _ => vec![i18n("personality.mysterious", lang), i18n("personality.enigmatic", lang)],
        }
    }
    
    fn get_career_advice(life_number: u8, _zodiac_sign: &str, lang: Language) -> String {
        match life_number {
            1 => i18n("career.leadership", lang),
            2 => i18n("career.teamwork", lang),
            3 => i18n("career.creative", lang),
            4 => i18n("career.technical", lang),
            5 => i18n("career.sales", lang),
            6 => i18n("career.service", lang),
            7 => i18n("career.research", lang),
            8 => i18n("career.finance", lang),
            9 => i18n("career.charity", lang),
            _ => i18n("career.mysticism", lang),
        }
    }
    
    fn calculate_lucky_numbers(birth_date: &BirthDate) -> Vec<u8> {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        
        let mut numbers = vec![
            birth_date.day as u8,
            birth_date.month as u8,
            (birth_date.year % 100) as u8,
        ];
        
        let life_number = birth_date.calculate_life_number();
        numbers.push(life_number);
        
        let mut rng = StdRng::seed_from_u64((birth_date.year * 10000 + birth_date.month as i32 * 100 + birth_date.day as i32) as u64);
        for _ in 0..3 {
            numbers.push(rng.gen_range(1..=99));
        }
        
        numbers.sort();
        numbers.dedup();
        numbers.truncate(6);
        numbers
    }
    
    fn get_compatibility(zodiac_sign: &str, lang: Language) -> Vec<String> {
        let zodiac_key = if zodiac_sign == i18n("zodiac.aries", lang) {
            "aries"
        } else if zodiac_sign == i18n("zodiac.taurus", lang) {
            "taurus"
        } else if zodiac_sign == i18n("zodiac.gemini", lang) {
            "gemini"
        } else if zodiac_sign == i18n("zodiac.cancer", lang) {
            "cancer"
        } else if zodiac_sign == i18n("zodiac.leo", lang) {
            "leo"
        } else if zodiac_sign == i18n("zodiac.virgo", lang) {
            "virgo"
        } else if zodiac_sign == i18n("zodiac.libra", lang) {
            "libra"
        } else if zodiac_sign == i18n("zodiac.scorpio", lang) {
            "scorpio"
        } else if zodiac_sign == i18n("zodiac.sagittarius", lang) {
            "sagittarius"
        } else if zodiac_sign == i18n("zodiac.capricorn", lang) {
            "capricorn"
        } else if zodiac_sign == i18n("zodiac.aquarius", lang) {
            "aquarius"
        } else if zodiac_sign == i18n("zodiac.pisces", lang) {
            "pisces"
        } else {
            "unknown"
        };

        match zodiac_key {
            "aries" => vec![i18n("zodiac.leo", lang), i18n("zodiac.sagittarius", lang), i18n("zodiac.gemini", lang)],
            "taurus" => vec![i18n("zodiac.virgo", lang), i18n("zodiac.capricorn", lang), i18n("zodiac.cancer", lang)],
            "gemini" => vec![i18n("zodiac.libra", lang), i18n("zodiac.aquarius", lang), i18n("zodiac.aries", lang)],
            "cancer" => vec![i18n("zodiac.scorpio", lang), i18n("zodiac.pisces", lang), i18n("zodiac.taurus", lang)],
            "leo" => vec![i18n("zodiac.aries", lang), i18n("zodiac.sagittarius", lang), i18n("zodiac.libra", lang)],
            "virgo" => vec![i18n("zodiac.taurus", lang), i18n("zodiac.capricorn", lang), i18n("zodiac.scorpio", lang)],
            "libra" => vec![i18n("zodiac.gemini", lang), i18n("zodiac.aquarius", lang), i18n("zodiac.leo", lang)],
            "scorpio" => vec![i18n("zodiac.cancer", lang), i18n("zodiac.pisces", lang), i18n("zodiac.virgo", lang)],
            "sagittarius" => vec![i18n("zodiac.aries", lang), i18n("zodiac.leo", lang), i18n("zodiac.aquarius", lang)],
            "capricorn" => vec![i18n("zodiac.taurus", lang), i18n("zodiac.virgo", lang), i18n("zodiac.pisces", lang)],
            "aquarius" => vec![i18n("zodiac.gemini", lang), i18n("zodiac.libra", lang), i18n("zodiac.sagittarius", lang)],
            "pisces" => vec![i18n("zodiac.cancer", lang), i18n("zodiac.scorpio", lang), i18n("zodiac.capricorn", lang)],
            _ => vec![i18n("zodiac.unknown", lang)],
        }
    }
}
