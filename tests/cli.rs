use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::io::Write;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::{thread::sleep, time::Duration}; // Used for writing assertions

use thqm::utils::create_full_url;
use thqm::utils::get_ip;

fn start(cmd: &mut std::process::Command) -> Result<Child> {
    cmd.stdin(Stdio::piped());
    let mut child = cmd.spawn().expect("Failed to spawnd command");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all("Option 1\nOption 2".as_bytes())
            .expect("Failed to write to stdin");
    });
    // wait a bit
    sleep(Duration::from_millis(100));
    Ok(child)
}

fn stop(child: &mut std::process::Child) -> Result<()> {
    // then kill it to just get the output
    child.kill().expect("Failed to kill child");
    Ok(())
}

fn start_then_stop(cmd: &mut std::process::Command) -> Result<Child> {
    let mut child = start(cmd).unwrap();
    stop(&mut child).unwrap();
    Ok(child)
}

#[test]
fn help_msg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("thqm")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("thqm"));
    Ok(())
}

#[test]
fn list_styles() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("thqm")?;
    cmd.arg("--list-styles");
    cmd.assert()
        .success()
        .stdout(predicate::eq("base\nfa-grid\ndefault\n"));
    Ok(())
}

#[test]
fn stdout_url_basic() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("thqm")?;
    cmd.arg("-U");
    cmd.stdout(Stdio::piped());

    let child = start_then_stop(&mut cmd).unwrap();
    let output = child.wait_with_output().expect("Failed to read stdout");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("{}\n", create_full_url(&get_ip(None)?, 8000, None, None))
    );
    Ok(())
}

#[test]
fn stdout_url_auth() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("thqm")?;
    cmd.arg("-U");
    cmd.arg("--username");
    cmd.arg("loiccoyle");
    cmd.arg("--password");
    cmd.arg("test");
    cmd.arg("--port");
    cmd.arg("8001");
    cmd.stdout(Stdio::piped());

    let child = start_then_stop(&mut cmd).unwrap();
    let output = child.wait_with_output().expect("Failed to read stdout");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!(
            "{}\n",
            create_full_url(&get_ip(None)?, 8001, Some("loiccoyle"), Some("test"))
        )
    );
    Ok(())
}

#[test]
fn save_qrcode() -> Result<(), Box<dyn std::error::Error>> {
    let image_file = assert_fs::NamedTempFile::new("qrcode.png").unwrap();
    let mut cmd = Command::cargo_bin("thqm")?;
    cmd.arg("--save-qrcode");
    cmd.arg(image_file.to_str().unwrap());
    cmd.arg("--port");
    cmd.arg("8002");
    start_then_stop(&mut cmd).unwrap();

    assert!(image_file.is_file());
    Ok(())
}

#[test]
fn basic_selection() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("thqm")?;
    cmd.arg("--port");
    cmd.arg("8003");
    cmd.stdout(Stdio::piped());
    let mut child = start(&mut cmd).unwrap();

    let url = create_full_url(&get_ip(None)?, 8003, None, None);
    let url = format!("{}/select/test", url);
    reqwest::blocking::get(url).unwrap();

    stop(&mut child).unwrap();
    let output = child.wait_with_output().expect("Failed to read stdout");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "test\n");
    Ok(())
}

#[test]
fn basic_selection_oneshot() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("thqm")?;
    cmd.arg("--port");
    cmd.arg("8004");
    cmd.arg("--oneshot");
    cmd.stdout(Stdio::piped());
    let mut child = start(&mut cmd).unwrap();

    let url = create_full_url(&get_ip(None)?, 8004, None, None);
    let url = format!("{}/select/test", url);
    let _ = reqwest::blocking::get(url);
    let code = child.try_wait().unwrap();
    assert!(code.is_some());
    Ok(())
}

#[test]
fn cmd_shutdown() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("thqm")?;
    cmd.arg("--port");
    cmd.arg("8005");
    cmd.stdout(Stdio::piped());
    let mut child = start(&mut cmd).unwrap();

    let url = create_full_url(&get_ip(None)?, 8005, None, None);
    let url = format!("{}/cmd/shutdown", url);
    let _ = reqwest::blocking::get(url);
    let code = child.try_wait().unwrap();
    assert!(code.is_some());
    Ok(())
}
