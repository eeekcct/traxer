use serde_json::Value as JsonValue;
use std::process::Command;

fn run_example(example_name: &str, rust_log: Option<&str>, args: &[&str]) -> String {
    let mut command = Command::new("cargo");
    command
        .args(["run", "--quiet", "--example", example_name])
        .current_dir(env!("CARGO_MANIFEST_DIR"));
    if !args.is_empty() {
        command.arg("--").args(args);
    }

    match rust_log {
        Some(value) => {
            command.env("RUST_LOG", value);
        }
        None => {
            command.env_remove("RUST_LOG");
        }
    }

    let output = command
        .output()
        .expect("failed to execute cargo run for example");

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    assert!(
        output.status.success(),
        "example `{example_name}` failed.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );

    format!("{stdout}{stderr}")
}

fn strip_ansi(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.peek() == Some(&'[') {
                let _ = chars.next();
                for next in chars.by_ref() {
                    if ('@'..='~').contains(&next) {
                        break;
                    }
                }
            } else {
                output.push(ch);
            }
        } else {
            output.push(ch);
        }
    }

    output
}

#[test]
fn minimal_example_emits_ansi_levels_with_trace_filter() {
    let output = run_example("minimal", Some("trace"), &[]);

    assert!(output.contains("[traxer-example] "), "output:\n{output}");

    assert!(output.contains("\x1b[31mERROR\x1b[0m"), "output:\n{output}");
    assert!(output.contains("\x1b[33mWARN\x1b[0m"), "output:\n{output}");
    assert!(output.contains("\x1b[32mINFO\x1b[0m"), "output:\n{output}");
    assert!(output.contains("\x1b[34mDEBUG\x1b[0m"), "output:\n{output}");
    assert!(output.contains("\x1b[90mTRACE\x1b[0m"), "output:\n{output}");

    assert!(output.contains("error message"), "output:\n{output}");
    assert!(output.contains("warn message"), "output:\n{output}");
    assert!(output.contains("info message"), "output:\n{output}");
    assert!(output.contains("debug message"), "output:\n{output}");
    assert!(output.contains("trace message"), "output:\n{output}");
}

#[test]
fn minimal_example_defaults_to_info_filter() {
    let output = run_example("minimal", None, &[]);

    assert!(output.contains("error message"), "output:\n{output}");
    assert!(output.contains("warn message"), "output:\n{output}");
    assert!(output.contains("info message"), "output:\n{output}");

    assert!(!output.contains("debug message"), "output:\n{output}");
    assert!(!output.contains("trace message"), "output:\n{output}");
}

#[test]
fn fields_example_emits_structured_fields() {
    let output = run_example("fields", Some("trace"), &["--plain"]);
    let plain = strip_ansi(&output);

    assert!(plain.contains("[traxer-fields] "), "output:\n{output}");
    assert!(plain.contains("user_id=42"), "output:\n{output}");
    assert!(plain.contains("elapsed_ms=128"), "output:\n{output}");
    assert!(plain.contains("action=\"login\""), "output:\n{output}");
    assert!(plain.contains("user action processed"), "output:\n{output}");
}

#[test]
fn fields_example_defaults_to_plain_output() {
    let output = run_example("fields", Some("trace"), &[]);
    let plain = strip_ansi(&output);

    assert!(plain.contains("[traxer-fields] "), "output:\n{output}");
}

#[test]
fn fields_example_json_emits_typed_values() {
    let output = run_example("fields", Some("trace"), &["--json"]);
    let line = output
        .lines()
        .find(|l| l.trim_start().starts_with('{'))
        .expect("json log line not found");

    let parsed: JsonValue = serde_json::from_str(line).expect("failed to parse json log line");
    let obj = parsed.as_object().expect("json log is not an object");

    assert_eq!(
        obj.get("action"),
        Some(&JsonValue::String("login".to_string()))
    );
    assert_eq!(obj.get("elapsed_ms"), Some(&JsonValue::Number(128.into())));
    assert_eq!(obj.get("user_id"), Some(&JsonValue::Number(42.into())));
    assert_eq!(
        obj.get("message"),
        Some(&JsonValue::String("user action processed".to_string()))
    );
}

#[test]
fn try_init_example_is_idempotent() {
    let output = run_example("try_init", None, &[]);

    assert!(output.contains("try_init ok"), "output:\n{output}");
}
