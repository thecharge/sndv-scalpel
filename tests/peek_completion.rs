use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn peek_defaults_to_first_page() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args(["peek", "tests/fixtures/sample.go"]);
    cmd.assert().success().stdout(contains("sample.go:1-")).stdout(contains("package sample"));
}

#[test]
fn peek_supports_position_range_with_all() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args(["peek", "tests/fixtures/sample.go", "--from-pos", "7", "--to-pos", "10", "--all"]);
    cmd.assert().success().stdout(contains("func CalculateTotal")).stdout(contains("for _, item"));
}

#[test]
fn peek_supports_pagination() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "peek",
        "tests/fixtures/sample.go",
        "--from-line",
        "1",
        "--page-size",
        "3",
        "--page",
        "2",
    ]);
    cmd.assert()
        .success()
        .stdout(contains("sample.go:4-6"))
        .stdout(contains("type Config struct{}"));
}

#[test]
fn bash_completion_generation_outputs_script() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args(["completion", "bash"]);
    cmd.assert().success().stdout(contains("complete -F")).stdout(contains("scalpel"));
}
