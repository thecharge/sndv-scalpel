use clap::Parser;

use crate::cli::Cli;
use crate::commands;
use crate::config::load_config;
use crate::lang::LanguageRegistry;

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = load_config(cli.config.as_deref())?;
    let registry = LanguageRegistry::new(&cfg.languages);
    let concurrency = cli.concurrency.unwrap_or(cfg.default_concurrency);
    commands::dispatch(&cfg, &registry, &cli.command, cli.json, concurrency).await
}
