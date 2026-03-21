use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SymbolKind {
    Function,
    Method,
    Class,
    Type,
    Import,
    Heading,
    Key,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct Symbol {
    pub file: PathBuf,
    pub kind: SymbolKind,
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_byte: usize,
    pub end_byte: usize,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchOutput {
    pub pattern: String,
    pub language: String,
    pub symbol: Symbol,
}
