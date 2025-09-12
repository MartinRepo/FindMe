use crate::models::dev_pressure::{DevPressure, PressureLevel};
use crate::utils::i18n::i18n;
use crate::utils::Language;
use colored::*;

pub fn display_dev_pressure(pressure: &DevPressure, lang: Language) {
    println!();
    println!("{}", "=".repeat(60).bright_blue());
    println!("{}", i18n("pressure.title", lang).bright_blue().bold());
    println!("{}", "=".repeat(60).bright_blue());

    let pressure_emoji = match pressure.pressure_level {
        PressureLevel::Low => "🟢",
        PressureLevel::Medium => "🟡",
        PressureLevel::High => "🟠",
        PressureLevel::Critical => "🔴",
    };

    let pressure_text = match pressure.pressure_level {
        PressureLevel::Low => i18n("pressure.level.low", lang),
        PressureLevel::Medium => i18n("pressure.level.medium", lang),
        PressureLevel::High => i18n("pressure.level.high", lang),
        PressureLevel::Critical => i18n("pressure.level.critical", lang),
    };

    println!(
        "{} {} {}",
        pressure_emoji,
        i18n("pressure.level_label", lang).bold(),
        pressure_text.bold()
    );
    println!();

    println!("{}", i18n("pressure.metrics_label", lang).bold());
    println!(
        "  {} {} {}",
        "📝".bright_cyan(),
        i18n("pressure.git_diff_label", lang),
        format!(
            "{} {}",
            pressure.git_diff_lines,
            i18n("pressure.lines", lang)
        )
        .bright_white()
    );

    if pressure.has_tests {
        let success_color = if pressure.test_success_rate >= 0.9 {
            "bright_green"
        } else if pressure.test_success_rate >= 0.7 {
            "bright_yellow"
        } else {
            "bright_red"
        };

        println!(
            "  {} {} {}",
            "🧪".bright_cyan(),
            i18n("pressure.test_success_label", lang),
            format!("{:.1}%", pressure.test_success_rate * 100.0).color(success_color)
        );
    } else {
        println!(
            "  {} {} {}",
            "🧪".bright_cyan(),
            i18n("pressure.test_success_label", lang),
            i18n("pressure.no_tests", lang).bright_yellow()
        );
    }

    let build_color = if pressure.build_time_seconds <= 15 {
        "bright_green"
    } else if pressure.build_time_seconds <= 30 {
        "bright_yellow"
    } else {
        "bright_red"
    };

    println!(
        "  {} {} {}",
        "⚡".bright_cyan(),
        i18n("pressure.build_time_label", lang),
        format!("{}s", pressure.build_time_seconds).color(build_color)
    );

    println!();

    println!("{}", i18n("pressure.advice_label", lang).bold());
    println!(
        "  {} {}",
        "⚠️".bright_yellow(),
        pressure.risk_threshold.bright_white()
    );
    println!(
        "  {} {}",
        "💡".bright_blue(),
        pressure.patience_advice.bright_white()
    );

    println!("{}", "=".repeat(60).bright_blue());
}
