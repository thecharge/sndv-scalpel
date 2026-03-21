use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "scalpel",
    version,
    about = "Find and patch code safely across many languages",
    long_about = "Find and patch code safely across many languages.\n\nConfig lookup order:\n1) --config path\n2) SCALPEL_CONFIG\n3) XDG_CONFIG_HOME/scalpel/scalpel.yaml\n4) HOME/.config/scalpel/scalpel.yaml\n5) ./config/scalpel.yaml",
    arg_required_else_help = true,
    subcommand_required = true,
    after_long_help = "Examples:\n  scalpel find 'fn:*' src --recursive\n  scalpel diff 'fn:CalculateTotal' app.go --rename sum=total\n  scalpel diff 'fn:isEnabled' app.js --replace 'flag ? true : false=>flag ? 1 : 0'\n  scalpel patch 'fn:CalculateTotal' app.go --body-file ./new-total.go --apply"
)]
pub struct Cli {
    #[arg(long, global = true)]
    pub json: bool,

    #[arg(long, global = true)]
    pub concurrency: Option<usize>,

    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Find symbols by pattern")]
    Find {
        pattern: String,
        #[arg(required = true)]
        paths: Vec<PathBuf>,
        #[arg(short, long)]
        recursive: bool,
    },
    #[command(about = "Show a matched block with context")]
    View {
        pattern_or_path: String,
        path: Option<PathBuf>,
        #[arg(long, default_value_t = 3)]
        context: usize,
        #[arg(long)]
        index: Option<usize>,
        #[arg(long, help = "Render a structural outline for the file")]
        outline: bool,
        #[arg(long, help = "Show explicit line range in form start:end")]
        lines: Option<String>,
        #[arg(long, help = "Disable line-range safety cap")]
        all: bool,
    },
    #[command(
        about = "Peek file content with pagination or explicit ranges",
        after_long_help = "Examples:\n  scalpel peek src/main.rs\n  scalpel peek src/main.rs --from-line 120 --page-size 40 --page 2\n  scalpel peek src/main.rs --from-pos 300 --to-pos 360 --all"
    )]
    Peek {
        path: PathBuf,
        #[arg(long = "from-line", visible_alias = "from-pos", default_value_t = 1)]
        from_line: usize,
        #[arg(long = "to-line", visible_alias = "to-pos")]
        to_line: Option<usize>,
        #[arg(long, default_value_t = 50)]
        page_size: usize,
        #[arg(long, default_value_t = 1)]
        page: usize,
        #[arg(long, help = "Print all lines in the selected range")]
        all: bool,
    },
    #[command(about = "Show file language and symbol summary")]
    Info { path: PathBuf },
    #[command(
        about = "Preview a patch without writing files",
        long_about = "Preview a patch without writing files.\nExactly one operation is required: --rename, --replace, --body, or --body-file.",
        after_long_help = "Examples:\n  scalpel diff 'fn:CalculateTotal' app.go --rename sum=total\n  scalpel diff 'method:computeInvoice' sample-complex.ts --body-file ./new-method.tsfrag\n  scalpel diff 'method:chooseTier' sample-complex.ts --replace 'if (amount > 1000) { return \"enterprise\"; }=>if (amount > 1000) { return \"platinum\"; }'\n  scalpel diff 'heading:Scalpel Guide' sample.md --body '# Scalpel Guide (Updated)'"
    )]
    Diff {
        pattern: String,
        path: PathBuf,
        #[arg(long)]
        rename: Option<String>,
        #[arg(long, help = "Literal scoped replacement in format old=>new")]
        replace: Option<String>,
        #[arg(long, help = "Replace matched symbol block with this literal body")]
        body: Option<String>,
        #[arg(long, help = "Replace matched symbol block with contents from this file")]
        body_file: Option<PathBuf>,
        #[arg(long)]
        index: Option<usize>,
        #[arg(long, help = "Start line for direct line-range edits")]
        from_line: Option<usize>,
        #[arg(long, help = "End line for direct line-range edits (defaults to --from-line)")]
        to_line: Option<usize>,
    },
    #[command(
        about = "Apply patch transactionally (requires --apply)",
        long_about = "Apply a patch transactionally (requires --apply).\nExactly one operation is required: --rename, --replace, --body, or --body-file.",
        after_long_help = "Examples:\n  scalpel patch 'fn:CalculateTotal' app.go --body-file ./new-total.go --apply\n  scalpel patch 'class:InvoiceRepository' sample-complex.ts --body-file ./replacement-class.tsfrag --apply\n  scalpel patch 'method:chooseTier' sample-complex.ts --replace 'if (amount > 1000) { return \"enterprise\"; }=>if (amount > 1000) { return \"platinum\"; }' --apply\n  scalpel patch 'heading:Scalpel Guide' sample.md --body '# Scalpel Guide (Updated)' --apply"
    )]
    Patch {
        pattern: String,
        path: PathBuf,
        #[arg(long)]
        rename: Option<String>,
        #[arg(long, help = "Literal scoped replacement in format old=>new")]
        replace: Option<String>,
        #[arg(long, help = "Replace matched symbol block with this literal body")]
        body: Option<String>,
        #[arg(long, help = "Replace matched symbol block with contents from this file")]
        body_file: Option<PathBuf>,
        #[arg(long)]
        apply: bool,
        #[arg(long)]
        index: Option<usize>,
        #[arg(long, help = "Start line for direct line-range edits")]
        from_line: Option<usize>,
        #[arg(long, help = "End line for direct line-range edits (defaults to --from-line)")]
        to_line: Option<usize>,
    },
    #[command(about = "Generate shell completion script")]
    Completion {
        #[arg(value_enum, default_value_t = Shell::Bash)]
        shell: Shell,
    },
}
