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
    write_file(&template_path, "{{ missing + 1 }}");

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

fn teraclio(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_teraclio"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("run teraclio")
}

#[test]
fn include_and_extends_resolve_with_bare_template_name() {
    let temp_dir = unique_temp_dir();
    write_file(&temp_dir.join("data.json"), r#"{"name":"World"}"#);
    write_file(&temp_dir.join("header.txt"), "H:{{ data.name }}");
    write_file(
        &temp_dir.join("base.txt"),
        "[{% block body %}base{% endblock %}]",
    );
    write_file(
        &temp_dir.join("page.txt"),
        r#"{% extends "base.txt" %}{% block body %}{% include "header.txt" %}{% endblock %}"#,
    );

    // Bare file name: Path::parent() is "", which used to skip sibling registration
    let output = teraclio(&temp_dir, &["-s", "data.json", "-t", "page.txt"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr was: {stderr}");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "[H:World]");

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}

#[test]
fn sibling_registration_does_not_depend_on_directory_order() {
    let temp_dir = unique_temp_dir();
    write_file(&temp_dir.join("data.json"), "{}");
    write_file(
        &temp_dir.join("z_base.html"),
        "[{% block b %}base{% endblock %}]",
    );
    // Sorts before its parent, so a one-by-one load would reject it
    write_file(
        &temp_dir.join("a_child.html"),
        r#"{% extends "z_base.html" %}{% block b %}child{% endblock %}"#,
    );
    // Three-level chain: page -> a_child -> z_base
    write_file(
        &temp_dir.join("page.html"),
        r#"{% extends "a_child.html" %}"#,
    );

    let output = teraclio(&temp_dir, &["-s", "data.json", "-t", "./page.html"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr was: {stderr}");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "[child]");

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}

#[test]
fn directory_mode_does_not_recurse_into_destination_inside_templates() {
    let temp_dir = unique_temp_dir();
    write_file(&temp_dir.join("data.json"), r#"{"n":1}"#);
    write_file(&temp_dir.join("tpl/a.txt"), "a={{ data.n }}");
    write_file(&temp_dir.join("tpl/sub/b.txt"), "b={{ data.n }}");

    let output = teraclio(
        &temp_dir,
        &["-s", "data.json", "-t", "tpl", "-d", "tpl/out", "-r", "-q"],
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr was: {stderr}");
    assert_eq!(
        fs::read_to_string(temp_dir.join("tpl/out/sub/b.txt")).expect("b"),
        "b=1"
    );
    assert!(!temp_dir.join("tpl/out/out").exists(), "recursed into dest");

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}

#[test]
fn dest_cannot_overwrite_the_template() {
    let temp_dir = unique_temp_dir();
    write_file(&temp_dir.join("data.json"), "{}");
    write_file(&temp_dir.join("t.txt"), "original");

    let output = teraclio(
        &temp_dir,
        &["-s", "data.json", "-t", "t.txt", "-d", "./t.txt"],
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("must not be the template"),
        "stderr was: {stderr}"
    );
    assert_eq!(
        fs::read_to_string(temp_dir.join("t.txt")).unwrap(),
        "original"
    );

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}

#[test]
fn invalid_date_format_is_an_error_not_a_panic() {
    let temp_dir = unique_temp_dir();
    write_file(&temp_dir.join("data.json"), r#"{"d":"2024-01-15"}"#);
    write_file(
        &temp_dir.join("t.txt"),
        r#"{{ data.d | date_format(format="%Q") }}"#,
    );

    let output = teraclio(&temp_dir, &["-s", "data.json", "-t", "t.txt"]);
    // A panic exits with 101; a handled error with 1
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid date format"),
        "stderr was: {stderr}"
    );

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}

#[test]
fn output_validation_accepts_multi_document_yaml() {
    let temp_dir = unique_temp_dir();
    write_file(&temp_dir.join("data.json"), r#"{"n":"app"}"#);
    write_file(
        &temp_dir.join("k8s.yaml.tera"),
        "kind: Deployment\nname: {{ data.n }}\n---\nkind: Service\nname: {{ data.n }}\n",
    );

    let output = teraclio(
        &temp_dir,
        &[
            "-s",
            "data.json",
            "-t",
            "k8s.yaml.tera",
            "--output-format",
            "yaml",
        ],
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr was: {stderr}");

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}

#[test]
fn watch_rejects_stdin_before_rendering() {
    use std::io::Write;
    use std::process::Stdio;

    let temp_dir = unique_temp_dir();
    write_file(&temp_dir.join("t.txt"), "rendered {{ data.n }}");

    let mut child = Command::new(env!("CARGO_BIN_EXE_teraclio"))
        .current_dir(&temp_dir)
        .args(["-s", "-", "-f", "json", "-t", "t.txt", "--watch"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn teraclio");
    // Valid data: the old code rendered it, then rejected watch + stdin
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(br#"{"n":1}"#)
        .expect("write stdin");
    let output = child.wait_with_output().expect("wait");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot be used with stdin"),
        "stderr was: {stderr}"
    );
    assert!(output.stdout.is_empty(), "rendered before rejecting stdin");

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}
