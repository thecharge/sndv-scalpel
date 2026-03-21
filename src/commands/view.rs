use std::path::Path;

use crate::config::AppConfig;
use crate::lang::LanguageRegistry;
use crate::parser::parse_path;
use crate::query::Query;

use super::util::select_symbol;

pub async fn run(
    cfg: &AppConfig,
    registry: &LanguageRegistry,
    pattern: &str,
    path: &Path,
    context: usize,
    index: Option<usize>,
) -> anyhow::Result<()> {
    let query = Query::parse(pattern)?;
    let parsed = parse_path(path, cfg, registry).await?;
    let matches: Vec<_> =
        parsed.symbols.into_iter().filter(|s| query.matches(s.kind, &s.name)).collect();

    let selected = select_symbol(pattern, path, &matches, index)?;
    let lines: Vec<&str> = parsed.content.lines().collect();

    let start = selected.start_line.saturating_sub(context + 1);
    let end = (selected.end_line + context).min(lines.len());

    println!("--- {}:{}-{} ---", path.display(), selected.start_line, selected.end_line);

    for (idx, line) in lines[start..end].iter().enumerate() {
        let line_no = start + idx + 1;
        println!("{:>5} | {}", line_no, line);
    }

    Ok(())
}
