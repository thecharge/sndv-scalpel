use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn find_across_required_languages() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args(["find", "fn:*", "tests/fixtures", "--recursive"]);
    cmd.assert()
        .success()
        .stdout(contains("sample.js"))
        .stdout(contains("sample.ts"))
        .stdout(contains("sample.go"))
        .stdout(contains("sample.rs"));
}

#[test]
fn find_markdown_and_yaml_symbols() {
    let mut md = Command::cargo_bin("scalpel").expect("binary");
    md.args(["find", "heading:*", "tests/fixtures/sample.md"]);
    md.assert().success().stdout(contains("heading"));

    let mut yml = Command::cargo_bin("scalpel").expect("binary");
    yml.args(["find", "key:*", "tests/fixtures/sample.yaml"]);
    yml.assert().success().stdout(contains("key"));

    let mut json = Command::cargo_bin("scalpel").expect("binary");
    json.args(["find", "key:*", "tests/fixtures/sample.json"]);
    json.assert().success().stdout(contains("service.name"));

    let mut toml = Command::cargo_bin("scalpel").expect("binary");
    toml.args(["find", "key:*", "tests/fixtures/sample.toml"]);
    toml.assert().success().stdout(contains("service.name"));

    let mut lua = Command::cargo_bin("scalpel").expect("binary");
    lua.args(["find", "fn:*", "tests/fixtures/sample.lua"]);
    lua.assert().success().stdout(contains("calculate_total"));
}

#[test]
fn diff_is_dry_run_by_default() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args(["diff", "fn:CalculateTotal", "tests/fixtures/sample.go", "--rename", "sum=total"]);
    cmd.assert().success().stdout(contains("dry-run only"));
}

#[test]
fn patch_applies_when_enabled() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("sample.rs");
    std::fs::copy("tests/fixtures/sample.rs", &path).expect("copy fixture");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "fn:calculate_total",
        path.to_string_lossy().as_ref(),
        "--rename",
        "sum=total",
        "--apply",
    ]);
    cmd.assert().success().stdout(contains("applied:"));

    let updated = std::fs::read_to_string(path).expect("read patched file");
    assert!(updated.contains("let mut total"));
}

#[test]
fn patch_jsonl_side_flow() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("sample.jsonl");
    std::fs::copy("tests/fixtures/sample.jsonl", &path).expect("copy fixture");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "key:line1.state",
        path.to_string_lossy().as_ref(),
        "--rename",
        "queued=running",
        "--apply",
    ]);
    cmd.assert().success().stdout(contains("applied:"));

    let updated = std::fs::read_to_string(path).expect("read patched jsonl");
    assert!(updated.contains("running"));
}

#[test]
fn ambiguous_match_is_critical_error() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args(["view", "fn:*", "tests/fixtures/sample.rs"]);
    cmd.assert().failure().stderr(contains("ambiguous pattern"));
}

#[test]
fn go_function_body_swap_from_file() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("sample.go");
    let body = temp.path().join("new-body.go");
    std::fs::copy("tests/fixtures/sample.go", &source).expect("copy fixture");
    std::fs::write(
        &body,
        "func CalculateTotal(items []int) int {\n    total := 100\n    return total\n}\n",
    )
    .expect("write body");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "fn:CalculateTotal",
        source.to_string_lossy().as_ref(),
        "--body-file",
        body.to_string_lossy().as_ref(),
        "--apply",
    ]);
    cmd.assert().success().stdout(contains("applied:"));

    let updated = std::fs::read_to_string(source).expect("read patched");
    assert!(updated.contains("total := 100"));
}

#[test]
fn ternary_literal_replacement_scoped() {
    let temp = tempfile::tempdir().expect("tempdir");
    let file = temp.path().join("sample.js");
    std::fs::write(
        &file,
        "function main() {\n  const x = flag ? true : false;\n  const y = flag ? true : false;\n}\n",
    )
    .expect("write js");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "fn:main",
        file.to_string_lossy().as_ref(),
        "--replace",
        "flag ? true : false=>flag ? 1 : 0",
        "--apply",
    ]);
    cmd.assert().success();

    let updated = std::fs::read_to_string(file).expect("read updated");
    assert!(updated.contains("flag ? 1 : 0"));
}

#[test]
fn if_block_literal_replacement_scoped() {
    let temp = tempfile::tempdir().expect("tempdir");
    let file = temp.path().join("sample.ts");
    std::fs::write(
        &file,
        "function run(flag: boolean) {\n  if (flag) { return 1; }\n  return 0;\n}\n",
    )
    .expect("write ts");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "fn:run",
        file.to_string_lossy().as_ref(),
        "--replace",
        "if (flag) { return 1; }=>if (flag) { return 2; }",
        "--apply",
    ]);
    cmd.assert().success();

    let updated = std::fs::read_to_string(file).expect("read ts");
    assert!(updated.contains("return 2"));
}

#[test]
fn function_argument_replacement_scoped() {
    let temp = tempfile::tempdir().expect("tempdir");
    let file = temp.path().join("sample.go");
    std::fs::copy("tests/fixtures/sample.go", &file).expect("copy go");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "fn:CalculateTotal",
        file.to_string_lossy().as_ref(),
        "--replace",
        "items []int=>numbers []int",
        "--apply",
    ]);
    cmd.assert().success();

    let updated = std::fs::read_to_string(file).expect("read go");
    assert!(updated.contains("CalculateTotal(numbers []int)"));
}

#[test]
fn typescript_method_swap_on_complex_fixture() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("sample-complex.ts");
    let body = temp.path().join("method-body.tsfrag");
    std::fs::copy("tests/fixtures/sample-complex.ts", &source).expect("copy fixture");
    std::fs::write(
        &body,
        "  public computeInvoice(lines: InvoiceLine[], discountRate: number): InvoiceSummary {\n    const subtotal = lines.reduce((acc, line) => acc + line.qty * line.unitPrice, 0);\n    const discount = subtotal * discountRate;\n    return { subtotal, discount, total: subtotal - discount, tier: \"basic\" };\n  }\n",
    )
    .expect("write method swap body");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "method:computeInvoice",
        source.to_string_lossy().as_ref(),
        "--body-file",
        body.to_string_lossy().as_ref(),
        "--apply",
    ]);
    cmd.assert().success().stdout(contains("applied:"));

    let updated = std::fs::read_to_string(source).expect("read updated ts");
    assert!(updated.contains("lines.reduce"));
}

#[test]
fn typescript_whole_class_replacement() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("sample-complex.ts");
    let class_file = temp.path().join("replacement-class.tsfrag");
    std::fs::copy("tests/fixtures/sample-complex.ts", &source).expect("copy fixture");
    std::fs::write(
        &class_file,
        "export class InvoiceRepository {\n  public async loadInvoice(id: string): Promise<Invoice> {\n    return { id, currency: \"USD\", lines: [] };\n  }\n\n  public sanitizeLines(lines: InvoiceLine[]): InvoiceLine[] {\n    return lines;\n  }\n}\n",
    )
    .expect("write class replacement");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "class:InvoiceRepository",
        source.to_string_lossy().as_ref(),
        "--body-file",
        class_file.to_string_lossy().as_ref(),
        "--apply",
    ]);
    cmd.assert().success();

    let updated = std::fs::read_to_string(source).expect("read updated class");
    assert!(updated.contains("return lines;"));
    assert!(updated.contains("currency: \"USD\""));
}

#[test]
fn typescript_if_statement_body_replacement_inside_method() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("sample-complex.ts");
    std::fs::copy("tests/fixtures/sample-complex.ts", &source).expect("copy fixture");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "method:chooseTier",
        source.to_string_lossy().as_ref(),
        "--replace",
        "if (amount > 1000) {\n      return \"enterprise\";\n    }=>if (amount > 1000) {\n      return \"platinum\";\n    }",
        "--apply",
    ]);
    cmd.assert().success();

    let updated = std::fs::read_to_string(source).expect("read updated method");
    assert!(updated.contains("return \"platinum\""));
}

#[test]
fn go_import_group_block_swap_from_file() {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("sample-import-groups.go");
    let imports = temp.path().join("imports.go.frag");
    std::fs::copy("tests/fixtures/sample-import-groups.go", &source).expect("copy fixture");
    std::fs::write(&imports, "import (\n    \"strings\"\n    \"fmt\"\n)\n").expect("write imports");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "import:import",
        source.to_string_lossy().as_ref(),
        "--body-file",
        imports.to_string_lossy().as_ref(),
        "--apply",
    ]);
    cmd.assert().success().stdout(contains("applied:"));

    let updated = std::fs::read_to_string(source).expect("read updated imports");
    let strings_pos = updated.find("\"strings\"").expect("strings import");
    let fmt_pos = updated.find("\"fmt\"").expect("fmt import");
    assert!(strings_pos < fmt_pos);
    assert!(updated.contains("func Run(value string) string"));
}

#[test]
fn view_outline_shows_parent_child_symbols() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args(["view", "tests/fixtures/sample.go", "--outline"]);
    cmd.assert().success().stdout(contains("type Config")).stdout(contains("method Run"));
}

#[test]
fn view_lines_supports_explicit_ranges() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args(["view", "tests/fixtures/sample.rs", "--lines", "1:3"]);
    cmd.assert().success().stdout(contains("1 |")).stdout(contains("3 |"));
}

#[test]
fn view_json_outputs_structured_payload() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args(["--json", "view", "fn:calculate_total", "tests/fixtures/sample.rs"]);
    cmd.assert().success().stdout(contains("\"symbol\"")).stdout(contains("\"mode\""));
}

#[test]
fn diff_json_outputs_structured_payload() {
    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "--json",
        "diff",
        "fn:CalculateTotal",
        "tests/fixtures/sample.go",
        "--rename",
        "sum=total",
    ]);
    cmd.assert().success().stdout(contains("\"dry_run\": true")).stdout(contains("\"diff\""));
}

#[test]
fn patch_json_outputs_structured_payload() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("sample.rs");
    std::fs::copy("tests/fixtures/sample.rs", &path).expect("copy fixture");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "--json",
        "patch",
        "fn:calculate_total",
        path.to_string_lossy().as_ref(),
        "--rename",
        "sum=total",
        "--apply",
    ]);
    cmd.assert().success().stdout(contains("\"applied\": true")).stdout(contains("\"changed\""));
}

#[test]
fn patch_supports_direct_line_swap() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("sample.txt");
    std::fs::write(&path, "alpha\nbeta\ngamma\n").expect("write fixture");

    let mut cmd = Command::cargo_bin("scalpel").expect("binary");
    cmd.args([
        "patch",
        "*",
        path.to_string_lossy().as_ref(),
        "--from-line",
        "2",
        "--to-line",
        "2",
        "--body",
        "BETA\n",
        "--apply",
    ]);
    cmd.assert().success();

    let updated = std::fs::read_to_string(path).expect("read updated text");
    assert_eq!(updated, "alpha\nBETA\ngamma\n");
}
