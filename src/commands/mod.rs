use crate::cli::Command;
use crate::config::AppConfig;
use crate::lang::LanguageRegistry;

mod completion;
mod find;
mod info;
mod patch;
mod peek;
mod util;
mod view;

use patch::PatchRequest;
use view::ViewRequest;

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
        Command::View { pattern_or_path, path, context, index, outline, lines, all } => {
            view::run(
                cfg,
                registry,
                ViewRequest {
                    pattern_or_path,
                    path: path.as_deref(),
                    context: *context,
                    index: *index,
                    outline: *outline,
                    lines: lines.as_deref(),
                    all: *all,
                    json,
                },
            )
            .await
        }
        Command::Peek { path, from_line, to_line, page_size, page, all } => {
            peek::run(path.as_path(), *from_line, *to_line, *page_size, *page, *all, json)
        }
        Command::Info { path } => info::run(cfg, registry, path.as_path(), json).await,
        Command::Diff {
            pattern,
            path,
            rename,
            replace,
            body,
            body_file,
            index,
            from_line,
            to_line,
        } => {
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
                    from_line: *from_line,
                    to_line: *to_line,
                    json,
                },
            )
            .await
        }
        Command::Patch {
            pattern,
            path,
            rename,
            replace,
            body,
            body_file,
            apply,
            index,
            from_line,
            to_line,
        } => {
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
                    from_line: *from_line,
                    to_line: *to_line,
                    json,
                },
            )
            .await
        }
        Command::Completion { shell } => {
            completion::run(*shell);
            Ok(())
        }
    }
}
