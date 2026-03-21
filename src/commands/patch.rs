use std::path::Path;

use serde::Serialize;
use similar::TextDiff;

use crate::config::AppConfig;
use crate::lang::LanguageRegistry;
use crate::model::{Symbol, SymbolKind};
use crate::parser::parse_path;
use crate::query::Query;
use crate::transaction::Transaction;

use super::util::{
    applied_message, dry_run_message, parse_rename, parse_replace, replace_symbol_block,
    scoped_rename, scoped_replace_literal, select_symbol,
};

pub async fn run(
    cfg: &AppConfig,
    registry: &LanguageRegistry,
    request: PatchRequest<'_>,
) -> anyhow::Result<()> {
    let parsed = parse_path(request.path, cfg, registry).await?;
    let selected = if let Some(from_line) = request.from_line {
        line_range_symbol(request.path, &parsed.content, from_line, request.to_line)?
    } else {
        let query = Query::parse(request.pattern)?;
        let matches: Vec<_> =
            parsed.symbols.into_iter().filter(|s| query.matches(s.kind, &s.name)).collect();
        select_symbol(request.pattern, request.path, &matches, request.index)?.clone()
    };

    let changed = build_changed_content(
        &parsed.content,
        &selected,
        request.rename,
        request.replace,
        request.body,
        request.body_file,
    )?;
    let diff = render_diff(request.path, &parsed.content, &changed);
    let changed_any = parsed.content != changed;

    if !request.json {
        print!("{diff}");
    }
    if !request.apply {
        if request.json {
            let output = PatchOutput {
                path: request.path.display().to_string(),
                applied: false,
                dry_run: true,
                changed: changed_any,
                diff,
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            println!("{}", dry_run_message());
        }
        return Ok(());
    }

    if changed_any {
        apply_transactional_patch(request.path, &changed).await?;
    }

    if request.json {
        let output = PatchOutput {
            path: request.path.display().to_string(),
            applied: true,
            dry_run: false,
            changed: changed_any,
            diff,
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("{} {}", applied_message(), request.path.display());
    }
    Ok(())
}

pub struct PatchRequest<'a> {
    pub pattern: &'a str,
    pub path: &'a Path,
    pub rename: Option<&'a str>,
    pub replace: Option<&'a str>,
    pub body: Option<&'a str>,
    pub body_file: Option<&'a Path>,
    pub apply: bool,
    pub index: Option<usize>,
    pub from_line: Option<usize>,
    pub to_line: Option<usize>,
    pub json: bool,
}

#[derive(Debug, Serialize)]
struct PatchOutput {
    path: String,
    applied: bool,
    dry_run: bool,
    changed: bool,
    diff: String,
}

fn build_changed_content(
    content: &str,
    selected: &crate::model::Symbol,
    rename: Option<&str>,
    replace: Option<&str>,
    body: Option<&str>,
    body_file: Option<&Path>,
) -> anyhow::Result<String> {
    let op_count = usize::from(rename.is_some())
        + usize::from(replace.is_some())
        + usize::from(body.is_some())
        + usize::from(body_file.is_some());
    if op_count != 1 {
        anyhow::bail!("choose exactly one operation: --rename, --replace, --body, or --body-file");
    }

    if let Some(value) = rename {
        let (old_name, new_name) = parse_rename(value)?;
        return scoped_rename(content, selected, old_name, new_name);
    }

    if let Some(value) = replace {
        let (old_lit, new_lit) = parse_replace(value)?;
        return scoped_replace_literal(content, selected, old_lit, new_lit);
    }

    if let Some(value) = body {
        return replace_symbol_block(content, selected, value);
    }

    let Some(path) = body_file else { anyhow::bail!("missing body file") };
    let block = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("reading body file {}: {}", path.display(), e))?;
    replace_symbol_block(content, selected, &block)
}

fn render_diff(path: &Path, original: &str, changed: &str) -> String {
    TextDiff::from_lines(original, changed)
        .unified_diff()
        .header(&format!("a/{}", path.display()), &format!("b/{}", path.display()))
        .to_string()
}

async fn apply_transactional_patch(path: &Path, changed: &str) -> anyhow::Result<()> {
    let tx = Transaction::begin(&[path.to_path_buf()]).await?;
    let write_result = Transaction::atomic_write(path, changed).await;

    if write_result.is_ok() {
        tx.cleanup().await?;
        return Ok(());
    }

    tx.rollback().await?;
    tx.cleanup().await?;
    write_result
}

fn line_range_symbol(
    path: &Path,
    content: &str,
    from_line: usize,
    to_line: Option<usize>,
) -> anyhow::Result<Symbol> {
    if from_line == 0 {
        anyhow::bail!("--from-line must be >= 1");
    }

    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        anyhow::bail!("file is empty");
    }

    let max_line = lines.len();
    let start_line = from_line.min(max_line);
    let end_line = to_line.unwrap_or(from_line).max(start_line).min(max_line);
    let start_byte = line_start_byte(content, start_line);
    let end_byte = line_end_byte(content, end_line);

    Ok(Symbol {
        file: path.to_path_buf(),
        kind: SymbolKind::Unknown,
        name: format!("line-range:{start_line}-{end_line}"),
        start_line,
        end_line,
        start_byte,
        end_byte,
        signature: String::new(),
        parent: None,
    })
}

fn line_start_byte(content: &str, line_number: usize) -> usize {
    if line_number <= 1 {
        return 0;
    }

    let mut current_line = 1usize;
    for (idx, ch) in content.char_indices() {
        if ch == '\n' {
            current_line += 1;
            if current_line == line_number {
                return idx + 1;
            }
        }
    }

    content.len()
}

fn line_end_byte(content: &str, line_number: usize) -> usize {
    let start = line_start_byte(content, line_number);
    let Some(rest) = content.get(start..) else {
        return content.len();
    };
    let Some(offset) = rest.find('\n') else {
        return content.len();
    };
    start + offset + 1
}
