use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::Deserialize;

use crate::constants::{
    CONFIG_ENV_KEY, DEFAULT_CONCURRENCY, DEFAULT_CONFIG_PATH, DEFAULT_HOME_CONFIG_RELATIVE,
    DEFAULT_MAX_FILE_BYTES, DEFAULT_XDG_CONFIG_RELATIVE,
};
use crate::model::SymbolKind;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParseStrategy {
    Regex,
    Markdown,
    Yaml,
    Json,
    Jsonl,
    Toml,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegexPattern {
    pub kind: SymbolKind,
    pub regex: String,
    #[serde(default = "default_capture_group")]
    pub capture_group: usize,
    #[serde(default)]
    pub block_scoped: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguageConfig {
    pub id: String,
    pub extensions: Vec<String>,
    pub strategy: ParseStrategy,
    #[serde(default)]
    pub patterns: Vec<RegexPattern>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_concurrency")]
    pub default_concurrency: usize,
    #[serde(default = "default_max_file_bytes")]
    pub max_file_bytes: u64,
    pub languages: Vec<LanguageConfig>,
}

pub fn load_config(path: Option<&Path>) -> anyhow::Result<AppConfig> {
    let config_path = resolve_config_path(path)?;
    let raw = std::fs::read_to_string(&config_path)
        .with_context(|| format!("reading config {}", config_path.display()))?;
    let cfg: AppConfig = serde_yaml::from_str(&raw)
        .with_context(|| format!("parsing config {}", config_path.display()))?;
    Ok(cfg)
}

fn resolve_config_path(explicit: Option<&Path>) -> anyhow::Result<PathBuf> {
    if let Some(path) = explicit {
        return Ok(path.to_path_buf());
    }

    if let Some(path) = read_env_path(CONFIG_ENV_KEY) {
        if path.exists() {
            return Ok(path);
        }
    }

    if let Some(path) = read_env_path("XDG_CONFIG_HOME") {
        let cfg = path.join(DEFAULT_XDG_CONFIG_RELATIVE);
        if cfg.exists() {
            return Ok(cfg);
        }
    }

    if let Some(path) = read_env_path("HOME") {
        let cfg = path.join(DEFAULT_HOME_CONFIG_RELATIVE);
        if cfg.exists() {
            return Ok(cfg);
        }
    }

    Ok(PathBuf::from(DEFAULT_CONFIG_PATH))
}

fn read_env_path(key: &str) -> Option<PathBuf> {
    let Ok(value) = std::env::var(key) else {
        return None;
    };
    if value.is_empty() {
        return None;
    }
    Some(PathBuf::from(value))
}

fn default_capture_group() -> usize {
    1
}

fn default_concurrency() -> usize {
    DEFAULT_CONCURRENCY
}

fn default_max_file_bytes() -> u64 {
    DEFAULT_MAX_FILE_BYTES
}
