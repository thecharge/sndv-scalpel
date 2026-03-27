use std::path::{Path, PathBuf};

use regex::Regex;

use crate::constants::{APPLIED_MESSAGE, DRY_RUN_MESSAGE};
use crate::error::ScalpelError;
use crate::lang::LanguageRegistry;
use crate::model::{EngineMode, Symbol, SymbolKind};

pub fn collect_files(
    paths: &[PathBuf],
    recursive: bool,
    registry: &LanguageRegistry,
) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_file() {
            if registry.language_for_path(path).is_some() {
                files.push(path.clone());
            }
            continue;
        }

        if !recursive || !path.is_dir() {
            continue;
        }

        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_entry(|e| e.file_name() != ".git")
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let candidate = entry.path().to_path_buf();
            if registry.language_for_path(&candidate).is_some() {
                files.push(candidate);
            }
        }
    }
    files
}

pub fn select_symbol<'a>(
    pattern: &str,
    path: &Path,
    matches: &'a [Symbol],
    index: Option<usize>,
) -> anyhow::Result<&'a Symbol> {
    if matches.is_empty() {
        return Err(ScalpelError::NoMatch {
            pattern: pattern.to_string(),
            path: path.to_path_buf(),
        }
        .into());
    }

    if matches.len() == 1 {
        return Ok(&matches[0]);
    }

    if let Some(raw_index) = index {
        let selected = raw_index.saturating_sub(1);
        if let Some(symbol) = matches.get(selected) {
            return Ok(symbol);
        }
    }

    eprintln!("ambiguous pattern '{}':", pattern);
    for (idx, symbol) in matches.iter().enumerate() {
        eprintln!(
            "  {}. {} {}:{}-{}",
            idx + 1,
            symbol.name,
            symbol.file.display(),
            symbol.start_line,
            symbol.end_line
        );
    }

    Err(ScalpelError::Ambiguous {
        pattern: pattern.to_string(),
        path: path.to_path_buf(),
        count: matches.len(),
    }
    .into())
}

pub fn parse_rename(rename: &str) -> anyhow::Result<(&str, &str)> {
    rename.split_once('=').ok_or_else(|| ScalpelError::InvalidRename.into())
}

pub fn parse_replace(replace: &str) -> anyhow::Result<(&str, &str)> {
    replace.split_once("=>").ok_or_else(|| ScalpelError::InvalidReplace.into())
}

pub fn scoped_rename(
    content: &str,
    symbol: &Symbol,
    old_name: &str,
    new_name: &str,
) -> anyhow::Result<String> {
    if symbol.end_byte <= symbol.start_byte || symbol.end_byte > content.len() {
        return replace_whole_content(content, old_name, new_name);
    }

    let prefix = &content[..symbol.start_byte];
    let scope = &content[symbol.start_byte..symbol.end_byte];
    let suffix = &content[symbol.end_byte..];

    let escaped = regex::escape(old_name);
    let boundary = Regex::new(&format!(r"\b{}\b", escaped))?;
    let rewritten = boundary.replace_all(scope, new_name).to_string();
    Ok(format!("{prefix}{rewritten}{suffix}"))
}

fn replace_whole_content(content: &str, old_name: &str, new_name: &str) -> anyhow::Result<String> {
    let escaped = regex::escape(old_name);
    let boundary = Regex::new(&format!(r"\b{}\b", escaped))?;
    Ok(boundary.replace_all(content, new_name).to_string())
}

pub fn scoped_replace_literal(
    content: &str,
    symbol: &Symbol,
    old_literal: &str,
    new_literal: &str,
) -> anyhow::Result<String> {
    if symbol.end_byte <= symbol.start_byte || symbol.end_byte > content.len() {
        return Ok(content.replacen(old_literal, new_literal, 1));
    }

    let prefix = &content[..symbol.start_byte];
    let scope = &content[symbol.start_byte..symbol.end_byte];
    let suffix = &content[symbol.end_byte..];
    let rewritten = scope.replacen(old_literal, new_literal, 1);
    Ok(format!("{prefix}{rewritten}{suffix}"))
}

pub fn replace_symbol_block(content: &str, symbol: &Symbol, block: &str) -> anyhow::Result<String> {
    if symbol.end_byte <= symbol.start_byte || symbol.end_byte > content.len() {
        anyhow::bail!("invalid symbol byte range")
    }

    let prefix = &content[..symbol.start_byte];
    let suffix = &content[symbol.end_byte..];
    Ok(format!("{prefix}{block}{suffix}"))
}

pub fn kind_label(kind: SymbolKind) -> &'static str {
    match kind {
        SymbolKind::Function => "function",
        SymbolKind::Method => "method",
        SymbolKind::Class => "class",
        SymbolKind::Type => "type",
        SymbolKind::Import => "import",
        SymbolKind::Heading => "heading",
        SymbolKind::Key => "key",
        SymbolKind::Unknown => "unknown",
    }
}

pub fn mode_label(mode: EngineMode) -> &'static str {
    match mode {
        EngineMode::Structural => "structural",
        EngineMode::Text => "text",
    }
}

pub fn dry_run_message() -> &'static str {
    DRY_RUN_MESSAGE
}

pub fn applied_message() -> &'static str {
    APPLIED_MESSAGE
}

#[cfg(test)]
mod tests {
    use super::{replace_symbol_block, scoped_rename, scoped_replace_literal};
    use crate::model::{Symbol, SymbolKind};

    #[test]
    fn happy_path_scoped_rename_only_changes_range() {
        let content = "line1 queued\nline2 queued\nline3 queued\n";
        let symbol = Symbol {
            file: "x.txt".into(),
            kind: SymbolKind::Key,
            name: "line2.state".to_string(),
            start_line: 2,
            end_line: 2,
            start_byte: 13,
            end_byte: 25,
            signature: "line2 queued".to_string(),
            parent: None,
        };

        let updated = scoped_rename(content, &symbol, "queued", "running").expect("rename");
        let lines: Vec<&str> = updated.lines().collect();

        assert_eq!(lines[0], "line1 queued");
        assert_eq!(lines[1], "line2 running");
        assert_eq!(lines[2], "line3 queued");
    }

    #[test]
    fn side_path_invalid_range_falls_back_to_full_content() {
        let content = "queued one\nqueued two\n";
        let symbol = Symbol {
            file: "x.txt".into(),
            kind: SymbolKind::Key,
            name: "line1.state".to_string(),
            start_line: 1,
            end_line: 1,
            start_byte: 0,
            end_byte: 0,
            signature: "line1".to_string(),
            parent: None,
        };

        let updated = scoped_rename(content, &symbol, "queued", "running").expect("rename");
        assert!(updated.contains("running one"));
        assert!(updated.contains("running two"));
    }

    #[test]
    fn critical_path_invalid_regex_word_boundary_input_does_not_fail() {
        let content = "a+b\na+b\n";
        let symbol = Symbol {
            file: "x.txt".into(),
            kind: SymbolKind::Key,
            name: "line1.key".to_string(),
            start_line: 1,
            end_line: 1,
            start_byte: 0,
            end_byte: 3,
            signature: "a+b".to_string(),
            parent: None,
        };

        let updated = scoped_rename(content, &symbol, "a+b", "c").expect("rename");
        assert!(updated.starts_with("c"));
    }

    #[test]
    fn literal_replace_changes_single_scope_occurrence() {
        let content = "a\nflag ? true : false\nflag ? true : false\n";
        let symbol = Symbol {
            file: "x.js".into(),
            kind: SymbolKind::Function,
            name: "f".to_string(),
            start_line: 2,
            end_line: 2,
            start_byte: 2,
            end_byte: 21,
            signature: "line".to_string(),
            parent: None,
        };

        let updated = scoped_replace_literal(content, &symbol, "true", "1").expect("replace");
        assert!(updated.contains("flag ? 1 : false"));
        let unchanged_count = updated.matches("flag ? true : false").count();
        assert_eq!(unchanged_count, 1);
    }

    #[test]
    fn block_replace_swaps_exact_symbol_range() {
        let content = "func A() {}\nfunc B() {}\n";
        let symbol = Symbol {
            file: "x.go".into(),
            kind: SymbolKind::Function,
            name: "A".to_string(),
            start_line: 1,
            end_line: 1,
            start_byte: 0,
            end_byte: 11,
            signature: "func A() {}".to_string(),
            parent: None,
        };

        let updated =
            replace_symbol_block(content, &symbol, "func A() { return }\n").expect("block replace");
        assert!(updated.starts_with("func A() { return }"));
        assert!(updated.contains("func B() {}"));
    }
}
