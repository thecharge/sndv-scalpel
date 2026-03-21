pub mod app;
pub mod cli;
pub mod commands;
pub mod config;
pub mod constants;
pub mod error;
pub mod lang;
pub mod model;
pub mod parser;
pub mod query;
pub mod transaction;

pub async fn run() -> anyhow::Result<()> {
    app::run().await
}
