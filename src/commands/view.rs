use std::path::Path;

use serde::Serialize;

use crate::config::AppConfig;
use crate::lang::LanguageRegistry;
use crate::model::Symbol;
use crate::parser::parse_path;
use crate::query::Query;

use super::util::{kind_label, select_symbol};

const VIEW_LINE_CAP: usize = 200;

pub async fn run(
    cfg: &AppConfig,
    registry: &LanguageRegistry,
    pattern_or_path: &str,
    path: Option<&Path>,
    context: usize,
    index: Option<usize>,
    outline: bool,
    lines: Option<&str>,
    all: bool,
    json: bool,
) -> anyhow::Result<()> {
    if outline {
        return run_outline(cfg, registry, resolve_view_path(pattern_or_path, path)?, json).await;
    }

    if let Some(range) = lines {
        return run_lines(
            cfg,
            registry,
            resolve_view_path(pattern_or_path, path)?,
            range,
            all,
            json,
        )
        .await;
    }

    let target_path = path.ok_or_else(|| anyhow::anyhow!("view requires <pattern> <path>"))?;
    let query = Query::parse(pattern_or_path)?;
    let parsed = parse_path(target_path, cfg, registry).await?;
    let matches: Vec<_> =
        parsed.symbols.into_iter().filter(|s| query.matches(s.kind, &s.name)).collect();

    let selected = select_symbol(pattern_or_path, target_path, &matches, index)?;
    let file_lines: Vec<&str> = parsed.content.lines().collect();

    let start = selected.start_line.saturating_sub(context + 1);
    let end = (selected.end_line + context).min(file_lines.len());

    if json {
        let mut rendered = Vec::new();
        for (idx, line) in file_lines[start..end].iter().enumerate() {
            rendered.push(ViewLine { number: start + idx + 1, text: (*line).to_string() });
        }
        let output = ViewMatchOutput {
            path: target_path.display().to_string(),
            language: parsed.language_id,
            mode: parsed.mode,
            tier: parsed.tier,
            pattern: pattern_or_path.to_string(),
            symbol: selected.clone(),
            lines: rendered,
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    println!("--- {}:{}-{} ---", target_path.display(), selected.start_line, selected.end_line);

    for (idx, line) in file_lines[start..end].iter().enumerate() {
        let line_no = start + idx + 1;
        println!("{:>5} | {}", line_no, line);
    }

    Ok(())
}

async fn run_outline(
    cfg: &AppConfig,
    registry: &LanguageRegistry,
    path: &Path,
    json: bool,
) -> anyhow::Result<()> {
    let parsed = parse_path(path, cfg, registry).await?;
    let nodes = build_outline(&parsed.symbols);

    let output = OutlineOutput {
        path: path.display().to_string(),
        language: parsed.language_id,
        mode: parsed.mode,
        tier: parsed.tier,
        symbols: nodes,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    println!("file: {}", output.path);
    println!("language: {}", output.language);
    for node in &output.symbols {
        print_outline_node(node, 0);
    }
    Ok(())
}

async fn run_lines(
    cfg: &AppConfig,
    registry: &LanguageRegistry,
    path: &Path,
    raw_range: &str,
    all: bool,
    json: bool,
) -> anyhow::Result<()> {
    let (from, to) = parse_line_range(raw_range)?;
    let parsed = parse_path(path, cfg, registry).await?;
    let lines: Vec<&str> = parsed.content.lines().collect();

    if lines.is_empty() {
        anyhow::bail!("file is empty")
    }

    let start = from.max(1).min(lines.len());
    let mut end = to.max(start).min(lines.len());
    let mut truncated = false;
    if !all {
        let cap_end = (start + VIEW_LINE_CAP - 1).min(lines.len());
        if end > cap_end {
            end = cap_end;
            truncated = true;
        }
    }

    if json {
        let mut rendered = Vec::new();
        for (idx, line) in lines[(start - 1)..end].iter().enumerate() {
            rendered.push(ViewLine { number: start + idx, text: (*line).to_string() });
        }

        let output = ViewRangeOutput {
            path: path.display().to_string(),
            language: parsed.language_id,
            mode: parsed.mode,
            tier: parsed.tier,
            from_line: start,
            to_line: end,
            requested_to_line: to,
            truncated,
            lines: rendered,
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    println!("--- {}:{}-{} ---", path.display(), start, end);
    for (idx, line) in lines[(start - 1)..end].iter().enumerate() {
        println!("{:>5} | {}", start + idx, line);
    }
    if truncated {
        println!("(truncated to {} lines; use --all to show full range)", VIEW_LINE_CAP);
    }

    Ok(())
}

fn resolve_view_path<'a>(pattern_or_path: &'a str, path: Option<&'a Path>) -> anyhow::Result<&'a Path> {
    if let Some(value) = path {
        return Ok(value);
    }
    Ok(Path::new(pattern_or_path))
}

fn parse_line_range(value: &str) -> anyhow::Result<(usize, usize)> {
    let (from_raw, to_raw) = value
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("invalid --lines value, expected start:end"))?;
    let from = from_raw.parse::<usize>()?;
    let to = to_raw.parse::<usize>()?;
    Ok((from, to))
}

fn build_outline(symbols: &[Symbol]) -> Vec<OutlineNode> {
    let mut roots: Vec<OutlineNode> = Vec::new();
    let mut children: std::collections::HashMap<String, Vec<OutlineNode>> =
        std::collections::HashMap::new();

    for symbol in symbols {
        let node = OutlineNode::from_symbol(symbol.clone());
        if let Some(parent) = &symbol.parent {
            children.entry(parent.clone()).or_default().push(node);
            continue;
        }
        roots.push(node);
    }

    for root in &mut roots {
        if let Some(mut nested) = children.remove(&root.name) {
            nested.sort_by_key(|n| (n.start_line, n.name.clone()));
            root.children = nested;
        }
    }

    roots.sort_by_key(|n| (n.start_line, n.name.clone()));
    roots
}

#[derive(Debug, Serialize)]
struct ViewLine {
    number: usize,
    text: String,
}

#[derive(Debug, Serialize)]
struct ViewMatchOutput {
    path: String,
    language: String,
    mode: crate::model::EngineMode,
    tier: u8,
    pattern: String,
    symbol: Symbol,
    lines: Vec<ViewLine>,
}

#[derive(Debug, Serialize)]
struct ViewRangeOutput {
    path: String,
    language: String,
    mode: crate::model::EngineMode,
    tier: u8,
    from_line: usize,
    to_line: usize,
    requested_to_line: usize,
    truncated: bool,
    lines: Vec<ViewLine>,
}

#[derive(Debug, Serialize)]
struct OutlineOutput {
    path: String,
    language: String,
    mode: crate::model::EngineMode,
    tier: u8,
    symbols: Vec<OutlineNode>,
}

#[derive(Debug, Clone, Serialize)]
struct OutlineNode {
    name: String,
    kind: crate::model::SymbolKind,
    start_line: usize,
    end_line: usize,
    children: Vec<OutlineNode>,
}

impl OutlineNode {
    fn from_symbol(symbol: Symbol) -> Self {
        Self {
            name: symbol.name,
            kind: symbol.kind,
            start_line: symbol.start_line,
            end_line: symbol.end_line,
            children: Vec::new(),
        }
    }
}

fn print_outline_node(node: &OutlineNode, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{indent}- {} {}:{}-{}", kind_label(node.kind), node.name, node.start_line, node.end_line);
    for child in &node.children {
        print_outline_node(child, depth + 1);
    }
}
