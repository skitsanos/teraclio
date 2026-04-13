use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir() -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time before unix epoch")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("teraclio-cli-test-{now}"));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(path, contents).expect("write test file");
}

#[test]
fn check_mode_fails_when_render_would_fail() {
    let temp_dir = unique_temp_dir();
    let data_path = temp_dir.join("data.json");
    let template_path = temp_dir.join("template.txt");

    write_file(&data_path, "{}");
    write_file(&template_path, "{{ data.missing }}");

    let output = Command::new(env!("CARGO_BIN_EXE_teraclio"))
        .args([
            "-s",
            data_path.to_str().expect("utf8 path"),
            "-t",
            template_path.to_str().expect("utf8 path"),
            "--check",
        ])
        .output()
        .expect("run teraclio");

    assert!(!output.status.success(), "expected command to fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Template error"), "stderr was: {stderr}");

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}

#[test]
fn directory_check_mode_does_not_require_dest() {
    let temp_dir = unique_temp_dir();
    let data_path = temp_dir.join("data.json");
    let template_dir = temp_dir.join("templates");
    let template_path = template_dir.join("report.txt");

    write_file(&data_path, r#"{"name":"World"}"#);
    write_file(&template_path, "Hello {{ data.name }}");

    let output = Command::new(env!("CARGO_BIN_EXE_teraclio"))
        .args([
            "-s",
            data_path.to_str().expect("utf8 path"),
            "-t",
            template_dir.to_str().expect("utf8 path"),
            "--check",
        ])
        .output()
        .expect("run teraclio");

    assert!(output.status.success(), "expected command to succeed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Template render check passed."),
        "stderr was: {stderr}"
    );

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}

#[test]
fn check_mode_validates_output_format() {
    let temp_dir = unique_temp_dir();
    let data_path = temp_dir.join("data.json");
    let template_path = temp_dir.join("template.txt");

    write_file(&data_path, r#"{"name":"World"}"#);
    write_file(&template_path, "Hello {{ data.name }}");

    let output = Command::new(env!("CARGO_BIN_EXE_teraclio"))
        .args([
            "-s",
            data_path.to_str().expect("utf8 path"),
            "-t",
            template_path.to_str().expect("utf8 path"),
            "--check",
            "--output-format",
            "json",
        ])
        .output()
        .expect("run teraclio");

    assert!(!output.status.success(), "expected command to fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Output is not valid JSON"),
        "stderr was: {stderr}"
    );

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}
