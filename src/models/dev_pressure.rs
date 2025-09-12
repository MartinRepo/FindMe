use crate::utils::i18n::i18n;
use crate::utils::Language;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct DevPressure {
    pub git_diff_lines: i32,
    pub test_success_rate: f32,
    pub has_tests: bool,
    pub build_time_seconds: u32,
    pub pressure_level: PressureLevel,
    pub risk_threshold: String,
    pub patience_advice: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PressureLevel {
    Low,
    Medium,
    High,
    Critical,
}

pub fn analyze_dev_pressure(lang: Language) -> Result<DevPressure, String> {
    let git_diff_lines = get_git_diff_lines()?;
    let (test_success_rate, has_tests) = get_test_success_rate()?;
    let build_time_seconds = get_build_time()?;

    let pressure_level =
        calculate_pressure_level(git_diff_lines, test_success_rate, build_time_seconds);
    let (risk_threshold, patience_advice) = generate_pressure_advice(
        &pressure_level,
        git_diff_lines,
        test_success_rate,
        build_time_seconds,
        lang,
    );

    Ok(DevPressure {
        git_diff_lines,
        test_success_rate,
        has_tests,
        build_time_seconds,
        pressure_level,
        risk_threshold,
        patience_advice,
    })
}

fn get_git_diff_lines() -> Result<i32, String> {
    let since = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - 86400)
        .to_string();

    let output = Command::new("git")
        .args(["log", "--since", &since, "--oneline", "--name-only"])
        .output()
        .map_err(|_| "Git not available or not in a git repository".to_string())?;

    if !output.status.success() {
        return Ok(0);
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let file_count = output_str
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("commit"))
        .count();

    Ok((file_count * 15) as i32)
}

fn get_test_success_rate() -> Result<(f32, bool), String> {
    let output = Command::new("cargo")
        .args(["test", "--quiet"])
        .output()
        .map_err(|_| "Cargo not available".to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let stderr_str = String::from_utf8_lossy(&output.stderr);

    let total_tests = extract_test_count(&output_str, "test result: ");
    let failed_tests = extract_test_count(&stderr_str, "test result: FAILED");

    if total_tests == 0 {
        return Ok((0.0, false));
    }

    let success_rate = (total_tests - failed_tests) as f32 / total_tests as f32;
    Ok((success_rate.clamp(0.0, 1.0), true))
}

fn get_build_time() -> Result<u32, String> {
    let start = SystemTime::now();

    let _output = Command::new("cargo")
        .args(["build", "--quiet"])
        .output()
        .map_err(|_| "Cargo build failed".to_string())?;

    let duration = start.elapsed().unwrap_or_default();
    Ok(duration.as_secs() as u32)
}

fn extract_test_count(text: &str, pattern: &str) -> u32 {
    if let Some(line) = text.lines().find(|line| line.contains(pattern)) {
        for word in line.split_whitespace() {
            if let Ok(num) = word.parse::<u32>() {
                return num;
            }
        }
    }
    0
}

fn calculate_pressure_level(
    git_diff_lines: i32,
    test_success_rate: f32,
    build_time_seconds: u32,
) -> PressureLevel {
    let mut pressure_score = 0.0;

    if git_diff_lines > 200 {
        pressure_score += 40.0;
    } else if git_diff_lines > 100 {
        pressure_score += 30.0;
    } else if git_diff_lines > 50 {
        pressure_score += 20.0;
    } else if git_diff_lines > 20 {
        pressure_score += 10.0;
    }

    if test_success_rate < 0.5 {
        pressure_score += 30.0;
    } else if test_success_rate < 0.7 {
        pressure_score += 20.0;
    } else if test_success_rate < 0.9 {
        pressure_score += 10.0;
    }

    if build_time_seconds > 60 {
        pressure_score += 30.0;
    } else if build_time_seconds > 30 {
        pressure_score += 20.0;
    } else if build_time_seconds > 15 {
        pressure_score += 10.0;
    }

    if pressure_score >= 80.0 {
        PressureLevel::Critical
    } else if pressure_score >= 60.0 {
        PressureLevel::High
    } else if pressure_score >= 30.0 {
        PressureLevel::Medium
    } else {
        PressureLevel::Low
    }
}

fn generate_pressure_advice(
    pressure_level: &PressureLevel,
    _git_diff_lines: i32,
    _test_success_rate: f32,
    _build_time_seconds: u32,
    lang: Language,
) -> (String, String) {
    match pressure_level {
        PressureLevel::Low => (
            i18n("pressure.advice.low.risk", lang),
            i18n("pressure.advice.low.patience", lang),
        ),
        PressureLevel::Medium => (
            i18n("pressure.advice.medium.risk", lang),
            i18n("pressure.advice.medium.patience", lang),
        ),
        PressureLevel::High => (
            i18n("pressure.advice.high.risk", lang),
            i18n("pressure.advice.high.patience", lang),
        ),
        PressureLevel::Critical => (
            i18n("pressure.advice.critical.risk", lang),
            i18n("pressure.advice.critical.patience", lang),
        ),
    }
}
