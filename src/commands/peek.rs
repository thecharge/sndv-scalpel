use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde::Serialize;

#[derive(Debug, Serialize)]
struct PeekLine {
    line: usize,
    text: String,
}

#[derive(Debug, Serialize)]
struct PeekOutput {
    path: String,
    start_line: usize,
    end_line: usize,
    total_lines: usize,
    page: usize,
    page_size: usize,
    has_next_page: bool,
    lines: Vec<PeekLine>,
}

pub fn run(
    path: &Path,
    from_line: usize,
    to_line: Option<usize>,
    page_size: usize,
    page: usize,
    all: bool,
    json: bool,
) -> anyhow::Result<()> {
    if from_line == 0 {
        anyhow::bail!("--from-line must be >= 1");
    }
    if page_size == 0 {
        anyhow::bail!("--page-size must be >= 1");
    }
    if page == 0 {
        anyhow::bail!("--page must be >= 1");
    }
    if let Some(to) = to_line {
        if to < from_line {
            anyhow::bail!("--to-line must be >= --from-line");
        }
    }

    let (start_line, end_line_hint) = if all {
        (from_line, to_line)
    } else {
        let start = from_line + (page - 1) * page_size;
        let requested_end = start + page_size - 1;
        let end = to_line.map_or(requested_end, |to| to.min(requested_end));
        (start, Some(end))
    };

    let file =
        File::open(path).map_err(|e| anyhow::anyhow!("opening {}: {}", path.display(), e))?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut total_lines = 0usize;

    for (index, line_result) in reader.lines().enumerate() {
        let line_no = index + 1;
        total_lines = line_no;
        let line = line_result.map_err(|e| anyhow::anyhow!("reading {}: {}", path.display(), e))?;

        if line_no < start_line {
            continue;
        }

        if let Some(end_line) = end_line_hint {
            if line_no > end_line {
                continue;
            }
        }

        lines.push(PeekLine { line: line_no, text: line });
    }

    if lines.is_empty() {
        anyhow::bail!("no lines found in requested range");
    }

    let end_line = lines.last().map(|l| l.line).unwrap_or(start_line);
    let has_next_page = end_line < total_lines && (all || end_line_hint.is_some());

    if json {
        let out = PeekOutput {
            path: path.display().to_string(),
            start_line,
            end_line,
            total_lines,
            page,
            page_size,
            has_next_page,
            lines,
        };
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!("--- {}:{}-{} (total: {}) ---", path.display(), start_line, end_line, total_lines);

    for entry in &lines {
        println!("{:>5} | {}", entry.line, entry.text);
    }

    if has_next_page && !all {
        println!("next page: --page {}", page + 1);
    }

    Ok(())
}
