use std::path::Path;

use crate::config::{AppConfig, LanguageConfig, ParseStrategy};
use crate::lang::LanguageRegistry;
use crate::model::{EngineMode, Symbol};

mod data_parser;
mod regex_parser;
mod stream_io;

pub struct ParsedFile {
    pub language_id: String,
    pub mode: EngineMode,
    pub tier: u8,
    pub content: String,
    pub symbols: Vec<Symbol>,
}

pub async fn parse_path(
    path: &Path,
    cfg: &AppConfig,
    registry: &LanguageRegistry,
) -> anyhow::Result<ParsedFile> {
    let language = registry
        .language_for_path(path)
        .ok_or_else(|| crate::error::ScalpelError::UnsupportedFileType(path.to_path_buf()))?;

    let content = stream_io::read_file_streamed(path, cfg.max_file_bytes).await?;
    let symbols = parse_symbols(&language, path.to_path_buf(), &content)?;
    let mode = strategy_mode(&language.strategy, &language.id);

    Ok(ParsedFile { language_id: language.id, mode, tier: language.tier, content, symbols })
}

pub fn parse_symbols(
    language: &LanguageConfig,
    file: std::path::PathBuf,
    content: &str,
) -> anyhow::Result<Vec<Symbol>> {
    match language.strategy {
        ParseStrategy::Regex => regex_parser::parse_regex(language, file, content),
        ParseStrategy::Markdown => Ok(data_parser::parse_markdown(file, content)),
        ParseStrategy::Yaml => data_parser::parse_yaml(file, content),
        ParseStrategy::Json => data_parser::parse_json(file, content),
        ParseStrategy::Jsonl => data_parser::parse_jsonl(file, content),
        ParseStrategy::Toml => data_parser::parse_toml(file, content),
    }
}

fn strategy_mode(strategy: &ParseStrategy, language_id: &str) -> EngineMode {
    if language_id == "text" {
        return EngineMode::Text;
    }

    match strategy {
        ParseStrategy::Regex
        | ParseStrategy::Markdown
        | ParseStrategy::Yaml
        | ParseStrategy::Json
        | ParseStrategy::Jsonl
        | ParseStrategy::Toml => EngineMode::Structural,
    }
}

#[cfg(test)]
mod tests {
    use crate::config::load_config;

    use super::*;

    #[test]
    fn parses_from_configured_strategies() {
        let cfg = load_config(Some(std::path::Path::new("config/scalpel.yaml"))).expect("load cfg");
        let rust = cfg.languages.iter().find(|l| l.id == "rust").expect("rust language").clone();
        let yaml = cfg.languages.iter().find(|l| l.id == "yaml").expect("yaml language").clone();

        let rust_symbols =
            parse_symbols(&rust, "x.rs".into(), "pub fn run() {}\n").expect("rust parse");
        let yaml_symbols =
            parse_symbols(&yaml, "x.yaml".into(), "name: app\n").expect("yaml parse");

        assert!(!rust_symbols.is_empty());
        assert!(!yaml_symbols.is_empty());
    }
}
