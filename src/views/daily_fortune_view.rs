use crate::models::Fortune;
use crate::utils::{detect_user_name, i18n, Language};
use chrono::Local;
use colored::*;

pub fn display_fortune(fortune: &Fortune, lang: Language) {
    println!();
    println!("{}", "=".repeat(60).cyan());
    let today = Local::now().format("%Y-%m-%d");
    println!(
        "{}",
        format!("{} · {}", today, i18n("app.title", lang))
            .bold()
            .yellow()
    );

    let user_name = detect_user_name();
    println!(
        "{}",
        i18n("app.welcome", lang)
            .replace("{}", &user_name)
            .color("bright_blue")
    );

    println!("{}", "=".repeat(60).cyan());
    println!();

    let score_color = match fortune.overall_score {
        90..=100 => "bright_green",
        80..=89 => "green",
        70..=79 => "yellow",
        60..=69 => "bright_yellow",
        50..=59 => "bright_red",
        40..=49 => "red",
        _ => "red",
    };

    println!(
        "{}: {}",
        i18n("fortune.overall_score_label", lang),
        format!("{}", fortune.overall_score)
            .color(score_color)
            .bold()
    );
    println!();

    println!("{}", i18n("fortune.dimensions_label", lang).bold().yellow());
    display_dimension_bar(
        i18n("fortune.focus_label", lang),
        fortune.dimensions.focus,
        lang,
    );
    display_dimension_bar(
        i18n("fortune.creativity_label", lang),
        fortune.dimensions.creativity,
        lang,
    );
    display_dimension_bar(
        i18n("fortune.debugging_label", lang),
        fortune.dimensions.debugging,
        lang,
    );
    display_dimension_bar(
        i18n("fortune.collaboration_label", lang),
        fortune.dimensions.collaboration,
        lang,
    );
    display_dimension_bar(
        i18n("fortune.risk_label", lang),
        fortune.dimensions.risk_tolerance,
        lang,
    );
    println!();

    println!(
        "{}: {}",
        i18n("fortune.message_label", lang),
        fortune.message.green()
    );
    println!(
        "{}: {}",
        i18n("fortune.advice_label", lang),
        fortune.advice.cyan()
    );
    println!();

    println!(
        "{}: {}",
        i18n("fortune.lucky_color_label", lang),
        fortune.lucky_color.magenta()
    );
    println!(
        "{}: {}",
        i18n("fortune.lucky_time_label", lang),
        fortune.lucky_time.blue()
    );
    println!();

    println!("{}", "=".repeat(60).cyan());
    println!();
}

fn display_dimension_bar(label: String, value: u8, _lang: Language) {
    let bar_length = 20;
    let filled_length = (value as f32 / 100.0 * bar_length as f32) as usize;
    let empty_length = bar_length - filled_length;

    let bar_color = match value {
        90..=100 => "bright_green",
        80..=89 => "green",
        70..=79 => "yellow",
        60..=69 => "bright_yellow",
        50..=59 => "bright_red",
        40..=49 => "red",
        _ => "red",
    };

    let filled_bar = "█".repeat(filled_length).color(bar_color);
    let empty_bar = "░".repeat(empty_length).color("bright_black");

    println!(
        "  {}: {}{} {}",
        label,
        filled_bar,
        empty_bar,
        format!("{:3}", value).color(bar_color)
    );
}
