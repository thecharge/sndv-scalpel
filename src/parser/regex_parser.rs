use regex::Regex;
use std::path::PathBuf;

use crate::config::{LanguageConfig, RegexPattern};
use crate::model::Symbol;

pub fn parse_regex(
    language: &LanguageConfig,
    file: PathBuf,
    content: &str,
) -> anyhow::Result<Vec<Symbol>> {
    let line_starts = compute_line_starts(content);
    let mut symbols = Vec::new();

    for pattern in &language.patterns {
        let re = Regex::new(&pattern.regex)?;
        capture_pattern(&mut symbols, &re, pattern, file.clone(), content, &line_starts);
    }

    symbols.sort_by_key(|s| (s.start_line, s.start_byte));
    Ok(symbols)
}

fn capture_pattern(
    symbols: &mut Vec<Symbol>,
    re: &Regex,
    pattern: &RegexPattern,
    file: PathBuf,
    content: &str,
    line_starts: &[usize],
) {
    for caps in re.captures_iter(content) {
        let Some(m) = caps.get(0) else {
            continue;
        };

        let name = capture_name(&caps, pattern.capture_group);
        let parent = capture_parent(&caps, pattern.parent_capture_group);
        let start_byte = m.start();
        let mut end_byte = line_end_byte(content, start_byte);

        if pattern.block_scoped {
            if let Some(block_end) = find_block_end(content, start_byte) {
                end_byte = block_end;
            } else if is_import_group(m.as_str()) {
                if let Some(group_end) = find_group_end(content, start_byte) {
                    end_byte = group_end;
                }
            }
        }

        symbols.push(Symbol {
            file: file.clone(),
            kind: pattern.kind,
            name,
            start_line: byte_to_line(line_starts, start_byte),
            end_line: byte_to_line(line_starts, end_byte),
            start_byte,
            end_byte,
            signature: m.as_str().trim().to_string(),
            parent,
        });
    }
}

fn capture_name(caps: &regex::Captures<'_>, group: usize) -> String {
    let Some(name_match) = caps.get(group) else {
        return "<symbol>".to_string();
    };
    name_match.as_str().to_string()
}

fn capture_parent(caps: &regex::Captures<'_>, group: Option<usize>) -> Option<String> {
    let idx = group?;
    let value = caps.get(idx)?.as_str().trim();
    if value.is_empty() {
        return None;
    }
    Some(value.to_string())
}

fn compute_line_starts(content: &str) -> Vec<usize> {
    let mut starts = vec![0usize];
    for (idx, ch) in content.char_indices() {
        if ch == '\n' {
            starts.push(idx + 1);
        }
    }
    starts
}

fn byte_to_line(line_starts: &[usize], byte: usize) -> usize {
    let pos = line_starts.partition_point(|&s| s <= byte);
    pos.max(1)
}

fn line_end_byte(content: &str, start: usize) -> usize {
    let Some(slice) = content.get(start..) else {
        return content.len();
    };
    let Some(offset) = slice.find('\n') else {
        return content.len();
    };
    start + offset
}

fn find_block_end(content: &str, start: usize) -> Option<usize> {
    let open_rel = content.get(start..)?.find('{')?;
    let mut depth = 0usize;
    let mut in_str = false;
    let mut prev_escape = false;

    for (offset, ch) in content[start + open_rel..].char_indices() {
        if in_str {
            if ch == '"' && !prev_escape {
                in_str = false;
            }
            prev_escape = ch == '\\' && !prev_escape;
            continue;
        }

        if ch == '"' {
            in_str = true;
            prev_escape = false;
            continue;
        }

        if ch == '{' {
            depth += 1;
        }

        if ch != '}' {
            continue;
        }

        if depth == 0 {
            continue;
        }

        depth -= 1;
        if depth == 0 {
            return Some(start + open_rel + offset + 1);
        }
    }

    None
}

fn is_import_group(signature: &str) -> bool {
    let trimmed = signature.trim_start();
    trimmed.starts_with("import") && trimmed.contains('(')
}

fn find_group_end(content: &str, start: usize) -> Option<usize> {
    let open_rel = content.get(start..)?.find('(')?;
    let mut depth = 0usize;
    let mut in_str = false;
    let mut prev_escape = false;

    for (offset, ch) in content[start + open_rel..].char_indices() {
        if in_str {
            if ch == '"' && !prev_escape {
                in_str = false;
            }
            prev_escape = ch == '\\' && !prev_escape;
            continue;
        }

        if ch == '"' {
            in_str = true;
            prev_escape = false;
            continue;
        }

        if ch == '(' {
            depth += 1;
            continue;
        }

        if ch != ')' {
            continue;
        }

        if depth == 0 {
            continue;
        }

        depth -= 1;
        if depth == 0 {
            return Some(start + open_rel + offset + 1);
        }
    }

    None
}
