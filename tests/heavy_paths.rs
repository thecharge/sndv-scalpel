use assert_cmd::Command;
use predicates::str::contains;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn parses_10k_loc_file_happy_flow() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("large.rs");

    let mut content = String::new();
    for index in 0..10_000usize {
        content.push_str(&format!("pub fn f_{index}() -> usize {{ {index} }}\n"));
    }
    std::fs::write(&path, content).expect("write large fixture");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args(["find", "fn:f_9999", path.to_string_lossy().as_ref()]);
    cmd.assert().success().stdout(contains("f_9999"));
}

#[test]
fn one_line_json_critical_patch() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("flat.json");
    std::fs::write(&path, "{\"env\":{\"mode\":\"safe\"},\"count\":1}").expect("write json");

    let mut diff = Command::cargo_bin("scalpel").expect("binary");
    diff.args(["diff", "key:env.mode", path.to_string_lossy().as_ref(), "--rename", "safe=strict"]);
    diff.assert().success().stdout(contains("dry-run only"));

    let mut patch = Command::cargo_bin("scalpel").expect("binary");
    patch.args([
        "patch",
        "key:env.mode",
        path.to_string_lossy().as_ref(),
        "--rename",
        "safe=strict",
        "--apply",
    ]);
    patch.assert().success();

    let updated = std::fs::read_to_string(path).expect("read json");
    assert!(updated.contains("strict"));
}

#[test]
fn huge_jsonl_surgical_patch_single_line_only() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("big.jsonl");

    let huge_payload = "x".repeat(20_000);
    let line1 = format!("{{\"id\":1,\"state\":\"queued\",\"payload\":\"{}\"}}", huge_payload);
    let line2 = format!("{{\"id\":2,\"state\":\"queued\",\"payload\":\"{}\"}}", huge_payload);
    let line3 = format!("{{\"id\":3,\"state\":\"queued\",\"payload\":\"{}\"}}", huge_payload);
    std::fs::write(&path, format!("{line1}\n{line2}\n{line3}\n")).expect("write jsonl");

    let mut patch = Command::cargo_bin("scalpel").expect("binary");
    patch.args([
        "patch",
        "key:line2.state",
        path.to_string_lossy().as_ref(),
        "--rename",
        "queued=running",
        "--apply",
    ]);
    patch.assert().success();

    let updated = std::fs::read_to_string(path).expect("read updated jsonl");
    let lines: Vec<&str> = updated.lines().collect();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].contains("\"state\":\"queued\""));
    assert!(lines[1].contains("\"state\":\"running\""));
    assert!(lines[2].contains("\"state\":\"queued\""));
}

#[test]
fn huge_jsonl_diff_does_not_modify_file() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("big-diff.jsonl");

    let line1 = "{\"id\":1,\"state\":\"queued\",\"payload\":\"a\"}";
    let line2 = "{\"id\":2,\"state\":\"queued\",\"payload\":\"b\"}";
    let baseline = format!("{line1}\n{line2}\n");
    std::fs::write(&path, &baseline).expect("write baseline");

    let mut diff = Command::cargo_bin("scalpel").expect("binary");
    diff.args([
        "diff",
        "key:line2.state",
        path.to_string_lossy().as_ref(),
        "--rename",
        "queued=running",
    ]);
    diff.assert().success().stdout(contains("dry-run only"));

    let current = std::fs::read_to_string(path).expect("read current");
    assert_eq!(current, baseline);
}

#[test]
fn deep_line_target_jsonl_patch() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("deep.jsonl");

    let mut content = String::new();
    for idx in 1..=5000usize {
        content.push_str(&format!("{{\"id\":{idx},\"state\":\"queued\"}}\n"));
    }
    std::fs::write(&path, content).expect("write deep jsonl");

    let baseline_lines: Vec<String> = std::fs::read_to_string(&path)
        .expect("read baseline")
        .lines()
        .map(ToString::to_string)
        .collect();
    let baseline_fingerprint = fingerprint_except(&baseline_lines, 4320);

    let mut patch = Command::cargo_bin("scalpel").expect("binary");
    patch.args([
        "patch",
        "key:line4321.state",
        path.to_string_lossy().as_ref(),
        "--rename",
        "queued=running",
        "--apply",
    ]);
    patch.assert().success();

    let lines: Vec<String> = std::fs::read_to_string(path)
        .expect("read deep jsonl")
        .lines()
        .map(ToString::to_string)
        .collect();

    assert!(lines[4320].contains("\"state\":\"running\""));
    assert!(lines[4319].contains("\"state\":\"queued\""));
    assert!(lines[4321].contains("\"state\":\"queued\""));

    let updated_fingerprint = fingerprint_except(&lines, 4320);
    assert_eq!(baseline_fingerprint, updated_fingerprint);
}

#[test]
fn fixture_big_jsonl_precise_line_patch() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("events-10k.jsonl");
    std::fs::copy("tests/fixtures/big/events-10k.jsonl", &path).expect("copy big fixture");

    let baseline_lines: Vec<String> = std::fs::read_to_string(&path)
        .expect("read baseline")
        .lines()
        .map(ToString::to_string)
        .collect();
    let target_index = 9000usize;
    let baseline_fingerprint = fingerprint_except(&baseline_lines, target_index);

    let mut patch = Command::cargo_bin("scalpel").expect("binary");
    patch.args([
        "patch",
        "key:line9001.state",
        path.to_string_lossy().as_ref(),
        "--rename",
        "queued=running",
        "--apply",
    ]);
    patch.assert().success();

    let updated_lines: Vec<String> = std::fs::read_to_string(path)
        .expect("read updated")
        .lines()
        .map(ToString::to_string)
        .collect();

    assert!(updated_lines[target_index].contains("\"state\":\"running\""));
    let updated_fingerprint = fingerprint_except(&updated_lines, target_index);
    assert_eq!(baseline_fingerprint, updated_fingerprint);
}

fn fingerprint_except(lines: &[String], skip_index: usize) -> u64 {
    let mut hasher = DefaultHasher::new();
    for (idx, line) in lines.iter().enumerate() {
        if idx == skip_index {
            continue;
        }
        idx.hash(&mut hasher);
        line.hash(&mut hasher);
    }
    hasher.finish()
}
