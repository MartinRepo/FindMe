use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_basic_fortune_output_english() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--language").arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Overall Score"))
        .stdout(predicate::str::contains("Tech Advice"));

    Ok(())
}

#[test]
fn test_basic_fortune_output_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--language").arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("综合评分"))
        .stdout(predicate::str::contains("技术建议"));

    Ok(())
}

#[test]
fn test_verbose_output_english() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--verbose").arg("--language").arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Findme"))
        .stdout(predicate::str::contains(
            "Developer's Daily Decompression Oracle",
        ));

    Ok(())
}

#[test]
fn test_verbose_output_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--verbose").arg("--language").arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Findme"))
        .stdout(predicate::str::contains("程序员今日解压占卜"));

    Ok(())
}

#[test]
fn test_language_option_english() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--language").arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Overall Score"))
        .stdout(predicate::str::contains("Tech Advice"));

    Ok(())
}

#[test]
fn test_language_option_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--language").arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("综合评分"))
        .stdout(predicate::str::contains("技术建议"));

    Ok(())
}

#[test]
fn test_invalid_language() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--language").arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid language option"));

    Ok(())
}

#[test]
fn test_help_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("findme"))
        .stdout(predicate::str::contains(
            "Developer's Daily Decompression Oracle",
        ));

    Ok(())
}

#[test]
fn test_birthday_personalization_english() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--birthday")
        .arg("1990-05-15")
        .arg("--language")
        .arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Overall Score"))
        .stdout(predicate::str::contains("Tech Advice"));

    Ok(())
}

#[test]
fn test_birthday_personalization_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--birthday")
        .arg("1990-05-15")
        .arg("--language")
        .arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("综合评分"))
        .stdout(predicate::str::contains("技术建议"));

    Ok(())
}

#[test]
fn test_birthday_invalid_date() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;

    cmd.arg("--birthday")
        .arg("invalid-date")
        .arg("--language")
        .arg("en");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid birthday format"));

    Ok(())
}
