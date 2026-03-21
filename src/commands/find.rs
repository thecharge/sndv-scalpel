use std::path::PathBuf;

use futures::{stream, StreamExt};

use crate::config::AppConfig;
use crate::constants::NO_MATCHES_MESSAGE;
use crate::lang::LanguageRegistry;
use crate::model::{Confidence, MatchOutput};
use crate::parser::parse_path;
use crate::query::Query;

use super::util::{collect_files, kind_label, mode_label};

pub async fn run(
    cfg: &AppConfig,
    registry: &LanguageRegistry,
    pattern: &str,
    paths: &[PathBuf],
    recursive: bool,
    concurrency: usize,
    json: bool,
) -> anyhow::Result<()> {
    let query = Query::parse(pattern)?;
    let files = collect_files(paths, recursive, registry);

    let entries = stream::iter(files.into_iter().map(|path| {
        let query = query.clone();
        async move {
            let parsed = parse_path(&path, cfg, registry).await;
            (query, parsed)
        }
    }))
    .buffer_unordered(concurrency.max(1))
    .collect::<Vec<_>>()
    .await;

    let mut out = Vec::new();
    for (query, parsed) in entries {
        let Ok(parsed_file) = parsed else {
            continue;
        };

        for symbol in parsed_file.symbols {
            if query.matches(symbol.kind, &symbol.name) {
                out.push(MatchOutput {
                    pattern: query.raw.clone(),
                    language: parsed_file.language_id.clone(),
                    mode: parsed_file.mode,
                    tier: parsed_file.tier,
                    confidence: Confidence::High,
                    symbol,
                });
            }
        }
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        for item in &out {
            println!(
                "{}:{}-{} [{}:{}:{}] {} {}",
                item.symbol.file.display(),
                item.symbol.start_line,
                item.symbol.end_line,
                item.language,
                item.tier,
                mode_label(item.mode),
                kind_label(item.symbol.kind),
                item.symbol.name
            );
        }
    }

    if out.is_empty() {
        anyhow::bail!(NO_MATCHES_MESSAGE);
    }

    Ok(())
}
