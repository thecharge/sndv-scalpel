use std::path::Path;

use similar::TextDiff;

use crate::config::AppConfig;
use crate::lang::LanguageRegistry;
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
    let query = Query::parse(request.pattern)?;
    let parsed = parse_path(request.path, cfg, registry).await?;
    let matches: Vec<_> =
        parsed.symbols.into_iter().filter(|s| query.matches(s.kind, &s.name)).collect();

    let selected = select_symbol(request.pattern, request.path, &matches, request.index)?;
    let changed = build_changed_content(
        &parsed.content,
        selected,
        request.rename,
        request.replace,
        request.body,
        request.body_file,
    )?;

    print_diff(request.path, &parsed.content, &changed);
    if !request.apply {
        println!("{}", dry_run_message());
        return Ok(());
    }

    apply_transactional_patch(request.path, &changed).await?;
    println!("{} {}", applied_message(), request.path.display());
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

fn print_diff(path: &Path, original: &str, changed: &str) {
    let diff = TextDiff::from_lines(original, changed)
        .unified_diff()
        .header(&format!("a/{}", path.display()), &format!("b/{}", path.display()))
        .to_string();
    println!("{diff}");
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
