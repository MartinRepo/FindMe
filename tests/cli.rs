use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_basic_fortune_output_english() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--language").arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fortune Score"))
        .stdout(predicate::str::contains("Today's Advice"));

    Ok(())
}

#[test]
fn test_basic_fortune_output_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--language").arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("运势得分"))
        .stdout(predicate::str::contains("今日建议"));

    Ok(())
}

#[test]
fn test_verbose_output_english() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--verbose").arg("--language").arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Findu"))
        .stdout(predicate::str::contains("Today's Tech Work Fortune"));

    Ok(())
}

#[test]
fn test_verbose_output_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--verbose").arg("--language").arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Findu"))
        .stdout(predicate::str::contains("今日技术工作运势"));

    Ok(())
}

#[test]
fn test_language_option_english() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--language").arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fortune Score"))
        .stdout(predicate::str::contains("Today's Advice"));

    Ok(())
}

#[test]
fn test_language_option_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--language").arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("运势得分"))
        .stdout(predicate::str::contains("今日建议"));

    Ok(())
}

#[test]
fn test_invalid_language() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--language").arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid language option"));

    Ok(())
}

#[test]
fn test_help_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("findu"))
        .stdout(predicate::str::contains("Show today's tech work fortune"));

    Ok(())
}

#[test]
fn test_birth_fortune_english() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--birth")
        .arg("--birth-date")
        .arg("1990-05-15")
        .arg("--language")
        .arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Birth Date Fortune Analysis"))
        .stdout(predicate::str::contains("Birth Date"))
        .stdout(predicate::str::contains("Life Number"))
        .stdout(predicate::str::contains("Zodiac Sign"))
        .stdout(predicate::str::contains("Personality Traits"))
        .stdout(predicate::str::contains("Career Advice"))
        .stdout(predicate::str::contains("Lucky Numbers"))
        .stdout(predicate::str::contains("Zodiac Compatibility"));

    Ok(())
}

#[test]
fn test_birth_fortune_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--birth")
        .arg("--birth-date")
        .arg("1990-05-15")
        .arg("--language")
        .arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("出生日期运势分析"))
        .stdout(predicate::str::contains("出生日期"))
        .stdout(predicate::str::contains("生命数字"))
        .stdout(predicate::str::contains("星座"))
        .stdout(predicate::str::contains("性格特征"))
        .stdout(predicate::str::contains("职业建议"))
        .stdout(predicate::str::contains("幸运数字"))
        .stdout(predicate::str::contains("星座配对"));

    Ok(())
}

#[test]
fn test_birth_fortune_invalid_date() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--birth")
        .arg("--birth-date")
        .arg("invalid-date")
        .arg("--language")
        .arg("en");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid birth date format"));

    Ok(())
}