use std::path::PathBuf;

use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

use crate::model::{Symbol, SymbolKind};

pub fn parse_markdown(file: PathBuf, content: &str) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    let mut byte_cursor = 0usize;

    for (idx, line) in content.lines().enumerate() {
        let line_end = byte_cursor + line.len();
        let line_terminator = usize::from(content.as_bytes().get(line_end) == Some(&b'\n'));
        if !line.starts_with('#') {
            byte_cursor = line_end + line_terminator;
            continue;
        }
        let name = line.trim_start_matches('#').trim();
        if name.is_empty() {
            byte_cursor = line_end + line_terminator;
            continue;
        }
        symbols.push(symbol(
            file.clone(),
            SymbolKind::Heading,
            name,
            idx + 1,
            byte_cursor,
            line_end + line_terminator,
            line,
        ));
        byte_cursor = line_end + line_terminator;
    }

    symbols
}

pub fn parse_yaml(file: PathBuf, content: &str) -> anyhow::Result<Vec<Symbol>> {
    let value: serde_yaml::Value = serde_yaml::from_str(content)?;
    Ok(extract_yaml_keys(file, &value, String::new(), 1))
}

pub fn parse_json(file: PathBuf, content: &str) -> anyhow::Result<Vec<Symbol>> {
    let value: JsonValue = serde_json::from_str(content)?;
    Ok(extract_json_keys(file, &value, String::new(), 1))
}

pub fn parse_jsonl(file: PathBuf, content: &str) -> anyhow::Result<Vec<Symbol>> {
    let mut symbols = Vec::new();
    let mut byte_cursor = 0usize;

    for (index, line) in content.lines().enumerate() {
        let line_start = byte_cursor;
        let line_end = line_start + line.len();
        byte_cursor += line.len() + 1;

        if line.trim().is_empty() {
            continue;
        }

        let value: JsonValue = serde_json::from_str(line)?;
        let mut extracted =
            extract_json_keys(file.clone(), &value, format!("line{}", index + 1), index + 1);
        set_symbol_range(&mut extracted, line_start, line_end);
        symbols.append(&mut extracted);
    }

    Ok(symbols)
}

pub fn parse_toml(file: PathBuf, content: &str) -> anyhow::Result<Vec<Symbol>> {
    let value: TomlValue = toml::from_str(content)?;
    Ok(extract_toml_keys(file, &value, String::new(), 1))
}

fn extract_yaml_keys(
    file: PathBuf,
    value: &serde_yaml::Value,
    path: String,
    line: usize,
) -> Vec<Symbol> {
    let mut out = Vec::new();
    let Some(mapping) = value.as_mapping() else {
        return out;
    };

    for (key, child) in mapping {
        let Some(raw) = key.as_str() else {
            continue;
        };
        let name = dotted_name(&path, raw);
        out.push(symbol(file.clone(), SymbolKind::Key, &name, line, 0, 0, raw));
        out.extend(extract_yaml_keys(file.clone(), child, name, line));
    }
    out
}

fn extract_json_keys(file: PathBuf, value: &JsonValue, path: String, line: usize) -> Vec<Symbol> {
    let Some(map) = value.as_object() else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for (key, child) in map {
        let name = dotted_name(&path, key);
        out.push(symbol(file.clone(), SymbolKind::Key, &name, line, 0, 0, key));
        out.extend(extract_json_keys(file.clone(), child, name, line));
    }
    out
}

fn extract_toml_keys(file: PathBuf, value: &TomlValue, path: String, line: usize) -> Vec<Symbol> {
    let Some(table) = value.as_table() else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for (key, child) in table {
        let name = dotted_name(&path, key);
        out.push(symbol(file.clone(), SymbolKind::Key, &name, line, 0, 0, key));
        out.extend(extract_toml_keys(file.clone(), child, name, line));
    }
    out
}

fn dotted_name(prefix: &str, part: &str) -> String {
    if prefix.is_empty() {
        return part.to_string();
    }
    format!("{prefix}.{part}")
}

fn symbol(
    file: PathBuf,
    kind: SymbolKind,
    name: &str,
    line: usize,
    start_byte: usize,
    end_byte: usize,
    signature: &str,
) -> Symbol {
    Symbol {
        file,
        kind,
        name: name.to_string(),
        start_line: line,
        end_line: line,
        start_byte,
        end_byte,
        signature: signature.to_string(),
    }
}

fn set_symbol_range(symbols: &mut [Symbol], start_byte: usize, end_byte: usize) {
    for symbol in symbols {
        symbol.start_byte = start_byte;
        symbol.end_byte = end_byte;
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_jsonl, parse_markdown};

    #[test]
    fn happy_path_jsonl_extracts_keys() {
        let content = "{\"state\":\"queued\"}\n{\"state\":\"running\"}\n";
        let symbols = parse_jsonl("x.jsonl".into(), content).expect("parse jsonl");
        assert!(symbols.iter().any(|s| s.name == "line1.state"));
        assert!(symbols.iter().any(|s| s.name == "line2.state"));
    }

    #[test]
    fn side_path_jsonl_ranges_are_line_scoped() {
        let content = "{\"state\":\"queued\"}\n{\"state\":\"running\"}\n";
        let symbols = parse_jsonl("x.jsonl".into(), content).expect("parse jsonl");

        let first = symbols.iter().find(|s| s.name == "line1.state").expect("line1 symbol");
        let second = symbols.iter().find(|s| s.name == "line2.state").expect("line2 symbol");

        assert!(first.start_byte < first.end_byte);
        assert!(second.start_byte < second.end_byte);
        assert!(first.end_byte <= second.start_byte);
    }

    #[test]
    fn critical_path_invalid_jsonl_line_fails() {
        let content = "{\"state\":\"ok\"}\n{bad}\n";
        let result = parse_jsonl("x.jsonl".into(), content);
        assert!(result.is_err());
    }

    #[test]
    fn markdown_headings_have_valid_ranges() {
        let content = "# Title\nBody\n";
        let symbols = parse_markdown("x.md".into(), content);
        let heading = symbols.first().expect("heading symbol");
        assert!(heading.start_byte < heading.end_byte);
    }
}
