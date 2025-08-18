use chrono::{Datelike, Local};
use colored::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Chinese,
    English,
}

#[derive(Debug)]
pub struct FinduFortune {
    pub score: u8,
    pub message: String,
    pub advice: String,
    pub lucky_color: String,
    pub lucky_time: String,
}

pub struct FortuneTexts {
    pub title: String,
    pub score_label: String,
    pub message_label: String,
    pub advice_label: String,
    pub lucky_color_label: String,
    pub lucky_time_label: String,
    pub detailed_advice_title: String,
}

impl FortuneTexts {
    pub fn new(lang: Language) -> Self {
        match lang {
            Language::Chinese => FortuneTexts {
                title: "🎯 今日技术工作运势".to_string(),
                score_label: "📊 运势得分".to_string(),
                message_label: "💬 运势评价".to_string(),
                advice_label: "💡 今日建议".to_string(),
                lucky_color_label: "🎨 幸运颜色".to_string(),
                lucky_time_label: "⏰ 幸运时间".to_string(),
                detailed_advice_title: "📋 今日工作建议".to_string(),
            },
            Language::English => FortuneTexts {
                title: "🎯 Today's Tech Work Fortune".to_string(),
                score_label: "📊 Fortune Score".to_string(),
                message_label: "💬 Fortune Message".to_string(),
                advice_label: "💡 Today's Advice".to_string(),
                lucky_color_label: "🎨 Lucky Color".to_string(),
                lucky_time_label: "⏰ Lucky Time".to_string(),
                detailed_advice_title: "📋 Today's Work Advice".to_string(),
            },
        }
    }
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
    match lang {
        Language::Chinese => {
            let messages = HashMap::from([
                (90..=95, ("🌟 今日技术运势极佳！", "适合进行重要的代码重构或系统升级，一切都会很顺利。")),
                (80..=89, ("✨ 今日工作运势很好", "代码质量会很高，bug率很低，适合处理复杂的技术问题。")),
                (70..=79, ("👍 今日运势不错", "编程效率较高，但要注意代码审查，避免小错误。")),
                (60..=69, ("😐 今日运势一般", "写代码时要多测试，避免留下隐患。")),
                (50..=59, ("⚠️ 今日需要小心", "代码容易出错，建议多写单元测试，仔细检查逻辑。")),
                (40..=49, ("😰 今日运势不佳", "容易遇到技术难题，建议多查阅文档，不要急于求成。")),
                (30..=39, ("🚨 今日技术挑战很大", "代码容易出bug，建议专注于简单任务，避免复杂操作。")),
            ]);
            
            for (range, (msg, adv)) in messages {
                if range.contains(&score) {
                    return (msg.to_string(), adv.to_string());
                }
            }
            
            ("🤔 今日运势未知".to_string(), "保持平常心，专注工作即可。".to_string())
        },
        Language::English => {
            let messages = HashMap::from([
                (90..=95, ("🌟 Excellent tech fortune today!", "Perfect for important code refactoring or system upgrades, everything will go smoothly.")),
                (80..=89, ("✨ Great work fortune today", "Code quality will be high, low bug rate, suitable for handling complex technical issues.")),
                (70..=79, ("👍 Good fortune today", "High programming efficiency, but pay attention to code review, avoid small errors.")),
                (60..=69, ("😐 Average fortune today", "Write code carefully, avoid leaving hidden dangers.")),
                (50..=59, ("⚠️ Be careful today", "Code is prone to errors, suggest writing more unit tests, check logic carefully.")),
                (40..=49, ("😰 Poor fortune today", "Easy to encounter technical difficulties, suggest consulting documentation, don't rush.")),
                (30..=39, ("🚨 Major technical challenges today", "Code is prone to bugs, suggest focusing on simple tasks, avoid complex operations.")),
            ]);
            
            for (range, (msg, adv)) in messages {
                if range.contains(&score) {
                    return (msg.to_string(), adv.to_string());
                }
            }
            
            ("🤔 Unknown fortune today".to_string(), "Keep calm and focus on work.".to_string())
        }
    }
}

fn get_lucky_color(rng: &mut StdRng, lang: Language) -> String {
    match lang {
        Language::Chinese => {
            let colors = vec![
                "蓝色", "绿色", "紫色", "橙色", "红色", "黄色", "青色", "粉色"
            ];
            colors[rng.gen_range(0..colors.len())].to_string()
        },
        Language::English => {
            let colors = vec![
                "Blue", "Green", "Purple", "Orange", "Red", "Yellow", "Cyan", "Pink"
            ];
            colors[rng.gen_range(0..colors.len())].to_string()
        }
    }
}

fn get_lucky_time(rng: &mut StdRng, lang: Language) -> String {
    match lang {
        Language::Chinese => {
            let times = vec![
                "上午9-11点", "下午2-4点", "晚上7-9点", "凌晨1-3点", 
                "中午12-1点", "下午5-6点", "晚上10-11点"
            ];
            times[rng.gen_range(0..times.len())].to_string()
        },
        Language::English => {
            let times = vec![
                "9-11 AM", "2-4 PM", "7-9 PM", "1-3 AM", 
                "12-1 PM", "5-6 PM", "10-11 PM"
            ];
            times[rng.gen_range(0..times.len())].to_string()
        }
    }
}

pub fn display_fortune(fortune: &FinduFortune, lang: Language) {
    let texts = FortuneTexts::new(lang);
    
    println!();
    println!("{}", "=".repeat(50).cyan());
    println!("{}", texts.title.bold().yellow());
    println!("{}", "=".repeat(50).cyan());
    println!();
    
    // 显示运势得分
    let score_color = match fortune.score {
        90..=95 => "bright_green",
        80..=89 => "green", 
        70..=79 => "yellow",
        60..=69 => "bright_yellow",
        50..=59 => "bright_red",
        40..=49 => "red",
        _ => "red"
    };
    
    println!("{}: {}", texts.score_label, format!("{}", fortune.score).color(score_color).bold());
    println!("{}: {}", texts.message_label, fortune.message.green());
    println!("{}: {}", texts.advice_label, fortune.advice.cyan());
    println!("{}: {}", texts.lucky_color_label, fortune.lucky_color.magenta());
    println!("{}: {}", texts.lucky_time_label, fortune.lucky_time.blue());
    println!();
    
    // 根据得分给出具体建议
    display_detailed_advice(fortune.score, lang);
    
    println!("{}", "=".repeat(50).cyan());
    println!();
}

fn display_detailed_advice(score: u8, lang: Language) {
    let texts = FortuneTexts::new(lang);
    println!("{}", texts.detailed_advice_title.bold().yellow());
    
    match lang {
        Language::Chinese => {
            match score {
                90..=95 => {
                    println!("✅ 适合进行代码重构和系统优化");
                    println!("✅ 可以尝试新的技术栈或框架");
                    println!("✅ 代码审查会非常顺利");
                    println!("✅ 适合解决复杂的技术难题");
                },
                80..=89 => {
                    println!("✅ 编程效率很高，bug率低");
                    println!("✅ 适合处理重要的功能开发");
                    println!("✅ 代码质量会很好");
                    println!("⚠️ 注意保持代码规范");
                },
                70..=79 => {
                    println!("✅ 整体工作状态不错");
                    println!("⚠️ 写代码时要仔细检查");
                    println!("⚠️ 建议多写注释和文档");
                    println!("⚠️ 避免过度优化");
                },
                60..=69 => {
                    println!("⚠️ 编程时要多加小心");
                    println!("⚠️ 建议多写单元测试");
                    println!("⚠️ 避免复杂的逻辑判断");
                    println!("⚠️ 代码审查要仔细");
                },
                50..=59 => {
                    println!("🚨 今日容易出错，需要格外小心");
                    println!("🚨 建议专注于简单任务");
                    println!("🚨 多查阅文档和参考资料");
                    println!("🚨 避免急于求成");
                },
                40..=49 => {
                    println!("🚨 技术挑战很大，建议谨慎行事");
                    println!("🚨 避免处理复杂的技术问题");
                    println!("🚨 多向同事请教");
                    println!("🚨 保持耐心，不要急躁");
                },
                _ => {
                    println!("🚨 今日技术运势很差，建议");
                    println!("🚨 专注于学习和文档阅读");
                    println!("🚨 避免重要的代码修改");
                    println!("🚨 保持积极心态，明天会更好");
                }
            }
        },
        Language::English => {
            match score {
                90..=95 => {
                    println!("✅ Suitable for code refactoring and system optimization");
                    println!("✅ Can try new tech stacks or frameworks");
                    println!("✅ Code review will be very smooth");
                    println!("✅ Suitable for solving complex technical problems");
                },
                80..=89 => {
                    println!("✅ High programming efficiency, low bug rate");
                    println!("✅ Suitable for important feature development");
                    println!("✅ Code quality will be excellent");
                    println!("⚠️ Pay attention to code standards");
                },
                70..=79 => {
                    println!("✅ Overall work condition is good");
                    println!("⚠️ Check code carefully when writing");
                    println!("⚠️ Suggest writing more comments and documentation");
                    println!("⚠️ Avoid over-optimization");
                },
                60..=69 => {
                    println!("⚠️ Be extra careful when programming");
                    println!("⚠️ Suggest writing more unit tests");
                    println!("⚠️ Avoid complex logic judgments");
                    println!("⚠️ Code review should be thorough");
                },
                50..=59 => {
                    println!("🚨 Easy to make mistakes today, need extra care");
                    println!("🚨 Suggest focusing on simple tasks");
                    println!("🚨 Consult documentation and references more");
                    println!("🚨 Avoid rushing things");
                },
                40..=49 => {
                    println!("🚨 Major technical challenges, suggest proceeding with caution");
                    println!("🚨 Avoid handling complex technical issues");
                    println!("🚨 Ask colleagues for help more");
                    println!("🚨 Keep patience, don't be impatient");
                },
                _ => {
                    println!("🚨 Poor technical fortune today, suggest");
                    println!("🚨 Focus on learning and documentation reading");
                    println!("🚨 Avoid important code modifications");
                    println!("🚨 Keep positive attitude, tomorrow will be better");
                }
            }
        }
    }
}

pub fn get_language_choice() -> Language {
    println!("🌍 请选择语言 / Please select language:");
    println!("1. 中文 (Chinese)");
    println!("2. English");
    println!();
    
    loop {
        print!("请输入选择 (Enter choice): ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" | "中文" | "chinese" => return Language::Chinese,
            "2" | "english" | "English" => return Language::English,
            _ => {
                println!("❌ 无效选择，请输入 1 或 2 / Invalid choice, please enter 1 or 2");
                continue;
            }
        }
    }
}