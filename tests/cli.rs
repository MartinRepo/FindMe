use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_basic_fortune_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fortune Score"))
        .stdout(predicate::str::contains("Today's Advice"));

    Ok(())
}

#[test]
fn test_verbose_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--verbose");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Findu"))
        .stdout(predicate::str::contains("版本"));

    Ok(())
}

#[test]
fn test_language_option() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findu")?;
    
    cmd.arg("--language").arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fortune Score"))
        .stdout(predicate::str::contains("Today's Advice"));

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
        .stdout(predicate::str::contains("显示今日技术工作运势"));

    Ok(())
}