use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn inline_body_replacement_for_markdown_heading() {
    let temp = tempfile::tempdir().expect("tempdir");
    let file = temp.path().join("sample.md");
    std::fs::copy("tests/fixtures/sample.md", &file).expect("copy md fixture");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "heading:Scalpel Guide",
        file.to_string_lossy().as_ref(),
        "--body",
        "# Scalpel Guide Updated",
        "--apply",
    ]);
    cmd.assert().success().stdout(contains("applied:"));

    let updated = std::fs::read_to_string(file).expect("read md");
    assert!(updated.contains("# Scalpel Guide Updated"));
}

#[test]
fn inline_body_replacement_for_typescript_method() {
    let temp = tempfile::tempdir().expect("tempdir");
    let file = temp.path().join("sample-complex.ts");
    std::fs::copy("tests/fixtures/sample-complex.ts", &file).expect("copy ts fixture");

    let body =
        "public chooseTier(amount: number): \"basic\" | \"enterprise\" { return \"basic\"; }";
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "method:chooseTier",
        file.to_string_lossy().as_ref(),
        "--body",
        body,
        "--apply",
    ]);
    cmd.assert().success();

    let updated = std::fs::read_to_string(file).expect("read ts");
    assert!(updated.contains("return \"basic\";"));
}

#[test]
fn replace_across_text_and_data_files() {
    let temp = tempfile::tempdir().expect("tempdir");

    let yaml = temp.path().join("sample.yaml");
    std::fs::copy("tests/fixtures/sample.yaml", &yaml).expect("copy yaml");
    let mut yaml_cmd = Command::cargo_bin("scalpel").expect("binary");
    yaml_cmd.args([
        "patch",
        "key:mode",
        yaml.to_string_lossy().as_ref(),
        "--replace",
        "safe=>strict",
        "--apply",
    ]);
    yaml_cmd.assert().success();
    assert!(std::fs::read_to_string(&yaml).expect("read yaml").contains("strict"));

    let json = temp.path().join("sample.json");
    std::fs::copy("tests/fixtures/sample.json", &json).expect("copy json");
    let mut json_cmd = Command::cargo_bin("scalpel").expect("binary");
    json_cmd.args([
        "patch",
        "key:service.mode",
        json.to_string_lossy().as_ref(),
        "--replace",
        "safe=>strict",
        "--apply",
    ]);
    json_cmd.assert().success();
    assert!(std::fs::read_to_string(&json).expect("read json").contains("strict"));

    let toml = temp.path().join("sample.toml");
    std::fs::copy("tests/fixtures/sample.toml", &toml).expect("copy toml");
    let mut toml_cmd = Command::cargo_bin("scalpel").expect("binary");
    toml_cmd.args([
        "patch",
        "key:service.mode",
        toml.to_string_lossy().as_ref(),
        "--replace",
        "safe=>strict",
        "--apply",
    ]);
    toml_cmd.assert().success();
    assert!(std::fs::read_to_string(&toml).expect("read toml").contains("strict"));

    let jsonl = temp.path().join("sample.jsonl");
    std::fs::copy("tests/fixtures/sample.jsonl", &jsonl).expect("copy jsonl");
    let mut jsonl_cmd = Command::cargo_bin("scalpel").expect("binary");
    jsonl_cmd.args([
        "patch",
        "key:line1.state",
        jsonl.to_string_lossy().as_ref(),
        "--replace",
        "queued=>running",
        "--apply",
    ]);
    jsonl_cmd.assert().success();
    assert!(std::fs::read_to_string(&jsonl).expect("read jsonl").contains("running"));

    let txt = temp.path().join("sample.txt");
    std::fs::copy("tests/fixtures/sample.txt", &txt).expect("copy text");
    let mut txt_cmd = Command::cargo_bin("scalpel").expect("binary");
    txt_cmd.args([
        "patch",
        "key:status",
        txt.to_string_lossy().as_ref(),
        "--replace",
        "queued=>running",
        "--apply",
    ]);
    txt_cmd.assert().success();
    assert!(std::fs::read_to_string(txt).expect("read text").contains("status: running"));
}
