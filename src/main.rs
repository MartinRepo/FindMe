use anyhow::Result;
use clap::Parser;
use std::fs;
use findu::{display_fortune, generate_daily_fortune, get_language_choice, Language, BirthFortuneController, i18n};

#[derive(Parser)]
#[command(name = "findu")]
#[command(about = "Show today's tech work fortune")]
#[command(version)]
struct Cli {
    /// Show verbose help information
    #[arg(short, long)]
    verbose: bool,
    
    /// Specify date (format: YYYY-MM-DD)
    #[arg(short, long)]
    date: Option<String>,
    
    /// Language selection (zh/en)
    #[arg(short, long)]
    language: Option<String>,
    
    /// Set language
    #[arg(long)]
    set_language: bool,
    
    /// Birth date fortune analysis
    #[arg(short, long)]
    birth: bool,
    
    /// Birth date (format: YYYY-MM-DD)
    #[arg(long)]
    birth_date: Option<String>,
}

fn get_config_dir() -> std::path::PathBuf {
    let mut config_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    config_dir.push(".findu");
    config_dir
}

fn get_config_file() -> std::path::PathBuf {
    let mut config_file = get_config_dir();
    config_file.push("config.txt");
    config_file
}

fn load_language() -> Option<Language> {
    let config_file = get_config_file();
    if let Ok(content) = fs::read_to_string(config_file) {
        match content.trim() {
            "zh" | "chinese" => Some(Language::Chinese),
            "en" | "english" => Some(Language::English),
            _ => None,
        }
    } else {
        None
    }
}

fn save_language(lang: Language) -> Result<()> {
    let config_dir = get_config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }
    
    let config_file = get_config_file();
    let content = match lang {
        Language::Chinese => "zh",
        Language::English => "en",
    };
    fs::write(config_file, content)?;
    Ok(())
}

fn parse_language(lang_str: &str) -> Option<Language> {
    match lang_str.to_lowercase().as_str() {
        "zh" | "chinese" => Some(Language::Chinese),
        "en" | "english" => Some(Language::English),
        _ => {
            // Check against i18n translations
            if lang_str == i18n("lang.chinese", Language::Chinese) {
                Some(Language::Chinese)
            } else if lang_str == i18n("lang.english", Language::Chinese) {
                Some(Language::English)
            } else {
                None
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    
    // Handle language setting
    if args.set_language {
        let lang = get_language_choice();
        save_language(lang)?;
        match lang {
            Language::Chinese => println!("{}", i18n("language.set_chinese", lang)),
            Language::English => println!("{}", i18n("language.set_english", lang)),
        }
        return Ok(());
    }
    
    // Determine language to use
    let language = if let Some(lang_str) = args.language {
        parse_language(&lang_str).unwrap_or_else(|| {
            eprintln!("{}", i18n("language.invalid_option", Language::Chinese).replace("{}", &lang_str));
            eprintln!("{}", i18n("language.use_zh_en", Language::Chinese));
            std::process::exit(1);
        })
    } else {
        load_language().unwrap_or_else(|| {
            println!("{}", i18n("language.first_time", Language::Chinese));
            let lang = get_language_choice();
            if let Err(e) = save_language(lang) {
                eprintln!("{}", i18n("language.cannot_save", Language::Chinese).replace("{}", &e.to_string()));
            }
            lang
        })
    };
    
    if args.verbose {
        println!("🎯 Findu - {}", i18n("app.title", language));
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
        println!();
    }
    
    // Handle birth date fortune analysis
    if args.birth {
        if let Some(birth_date_str) = args.birth_date {
            match BirthFortuneController::validate_birth_date(&birth_date_str) {
                Ok(birth_date) => {
                    let fortune = findu::BirthFortuneModel::calculate_fortune(&birth_date, language);
                    findu::BirthFortuneView::display_birth_fortune(&fortune, &birth_date, language);
                },
                Err(e) => {
                    eprintln!("{}", i18n("birth.date_format_error_cli", language).replace("{}", &e));
                    std::process::exit(1);
                }
            }
        } else {
            match BirthFortuneController::process_birth_fortune(language) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{}", i18n("birth.processing_error", language).replace("{}", &e));
                    std::process::exit(1);
                }
            }
        }
        return Ok(());
    }
    
    // Generate daily fortune
    let fortune = generate_daily_fortune(language);
    
    // Display fortune
    display_fortune(&fortune, language);
    
    Ok(())
}
