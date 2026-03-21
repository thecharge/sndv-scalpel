use std::collections::BTreeMap;
use std::path::Path;

use crate::config::AppConfig;
use crate::lang::LanguageRegistry;
use crate::parser::parse_path;

use super::util::kind_label;

pub async fn run(
    cfg: &AppConfig,
    registry: &LanguageRegistry,
    path: &Path,
    json: bool,
) -> anyhow::Result<()> {
    let parsed = parse_path(path, cfg, registry).await?;

    if json {
        let mut map = BTreeMap::new();
        map.insert("path", path.display().to_string());
        map.insert("language", parsed.language_id.clone());
        map.insert("symbols", parsed.symbols.len().to_string());
        println!("{}", serde_json::to_string_pretty(&map)?);
        return Ok(());
    }

    println!("file: {}", path.display());
    println!("language: {}", parsed.language_id);
    println!("symbols: {}", parsed.symbols.len());

    for symbol in parsed.symbols {
        println!(
            "  - {}:{}-{} {} {}",
            symbol.file.display(),
            symbol.start_line,
            symbol.end_line,
            kind_label(symbol.kind),
            symbol.name
        );
    }

    Ok(())
}
