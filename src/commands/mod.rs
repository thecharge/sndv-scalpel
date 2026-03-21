use crate::cli::Command;
use crate::config::AppConfig;
use crate::lang::LanguageRegistry;

mod find;
mod info;
mod patch;
mod util;
mod view;

use patch::PatchRequest;

pub async fn dispatch(
    cfg: &AppConfig,
    registry: &LanguageRegistry,
    command: &Command,
    json: bool,
    concurrency: usize,
) -> anyhow::Result<()> {
    match command {
        Command::Find { pattern, paths, recursive } => {
            find::run(cfg, registry, pattern, paths, *recursive, concurrency, json).await
        }
        Command::View { pattern, path, context, index } => {
            view::run(cfg, registry, pattern, path.as_path(), *context, *index).await
        }
        Command::Info { path } => info::run(cfg, registry, path.as_path(), json).await,
        Command::Diff { pattern, path, rename, replace, body, body_file, index } => {
            patch::run(
                cfg,
                registry,
                PatchRequest {
                    pattern,
                    path: path.as_path(),
                    rename: rename.as_deref(),
                    replace: replace.as_deref(),
                    body: body.as_deref(),
                    body_file: body_file.as_deref(),
                    apply: false,
                    index: *index,
                },
            )
            .await
        }
        Command::Patch { pattern, path, rename, replace, body, body_file, apply, index } => {
            patch::run(
                cfg,
                registry,
                PatchRequest {
                    pattern,
                    path: path.as_path(),
                    rename: rename.as_deref(),
                    replace: replace.as_deref(),
                    body: body.as_deref(),
                    body_file: body_file.as_deref(),
                    apply: *apply,
                    index: *index,
                },
            )
            .await
        }
    }
}
