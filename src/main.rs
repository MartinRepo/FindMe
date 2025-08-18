use anyhow::Result;
use clap::Parser;
use std::fs;
use findu::{display_fortune, generate_daily_fortune, get_language_choice, Language};

#[derive(Parser)]
#[command(name = "findu")]
#[command(about = "显示今日技术工作运势 / Show today's tech work fortune")]
#[command(version)]
struct Cli {
    /// 显示详细帮助信息 / Show verbose help information
    #[arg(short, long)]
    verbose: bool,
    
    /// 指定日期 (格式: YYYY-MM-DD) / Specify date (format: YYYY-MM-DD)
    #[arg(short, long)]
    date: Option<String>,
    
    /// 语言选择 / Language selection (zh/en)
    #[arg(short, long)]
    language: Option<String>,
    
    /// 设置语言 / Set language
    #[arg(long)]
    set_language: bool,
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
        "zh" | "chinese" | "中文" => Some(Language::Chinese),
        "en" | "english" | "英文" => Some(Language::English),
        _ => None,
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    
    if args.verbose {
        println!("🎯 Findu - 技术工作运势预测工具");
        println!("Findu - Tech Work Fortune Prediction Tool");
        println!("版本 / Version: {}", env!("CARGO_PKG_VERSION"));
        println!("作者 / Author: {}", env!("CARGO_PKG_AUTHORS"));
        println!();
    }
    
    // 处理语言设置
    if args.set_language {
        let lang = get_language_choice();
        save_language(lang)?;
        match lang {
            Language::Chinese => println!("✅ 语言已设置为中文"),
            Language::English => println!("✅ Language set to English"),
        }
        return Ok(());
    }
    
    // 确定使用的语言
    let language = if let Some(lang_str) = args.language {
        parse_language(&lang_str).unwrap_or_else(|| {
            eprintln!("❌ 无效的语言选项 / Invalid language option: {}", lang_str);
            eprintln!("请使用: zh/en / Please use: zh/en");
            std::process::exit(1);
        })
    } else {
        load_language().unwrap_or_else(|| {
            println!("🌍 首次使用，请选择语言 / First time use, please select language:");
            let lang = get_language_choice();
            if let Err(e) = save_language(lang) {
                eprintln!("⚠️ 无法保存语言设置 / Cannot save language setting: {}", e);
            }
            lang
        })
    };
    
    // 生成今日运势
    let fortune = generate_daily_fortune(language);
    
    // 显示运势
    display_fortune(&fortune, language);
    
    Ok(())
}
