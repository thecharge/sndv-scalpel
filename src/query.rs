use globset::{Glob, GlobMatcher};

use crate::constants::{
    PREFIX_CLASS, PREFIX_ENUM, PREFIX_FN, PREFIX_HEADING, PREFIX_IMPORT, PREFIX_KEY, PREFIX_METHOD,
    PREFIX_STRUCT, PREFIX_TRAIT, PREFIX_TYPE,
};
use crate::model::SymbolKind;

#[derive(Debug, Clone)]
pub struct Query {
    pub raw: String,
    pub kind: SymbolKind,
    matcher: GlobMatcher,
}

impl Query {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let (kind, pattern) = if let Some((prefix, body)) = input.split_once(':') {
            (map_prefix(prefix), body)
        } else {
            (SymbolKind::Unknown, input)
        };
        let glob = Glob::new(pattern)?;
        Ok(Self { raw: input.to_string(), kind, matcher: glob.compile_matcher() })
    }

    pub fn matches(&self, kind: SymbolKind, name: &str) -> bool {
        let kind_ok = self.kind == SymbolKind::Unknown || self.kind == kind;
        kind_ok && self.matcher.is_match(name)
    }
}

fn map_prefix(prefix: &str) -> SymbolKind {
    let parsed = QueryPrefix::parse(prefix);
    parsed.kind()
}

enum QueryPrefix {
    Function,
    Method,
    Class,
    Type,
    Import,
    Heading,
    Key,
    Unknown,
}

impl QueryPrefix {
    fn parse(raw: &str) -> Self {
        match raw {
            PREFIX_FN => Self::Function,
            PREFIX_METHOD => Self::Method,
            PREFIX_CLASS => Self::Class,
            PREFIX_TYPE | PREFIX_TRAIT | PREFIX_ENUM | PREFIX_STRUCT => Self::Type,
            PREFIX_IMPORT => Self::Import,
            PREFIX_HEADING => Self::Heading,
            PREFIX_KEY => Self::Key,
            _ => Self::Unknown,
        }
    }

    fn kind(&self) -> SymbolKind {
        match self {
            QueryPrefix::Function => SymbolKind::Function,
            QueryPrefix::Method => SymbolKind::Method,
            QueryPrefix::Class => SymbolKind::Class,
            QueryPrefix::Type => SymbolKind::Type,
            QueryPrefix::Import => SymbolKind::Import,
            QueryPrefix::Heading => SymbolKind::Heading,
            QueryPrefix::Key => SymbolKind::Key,
            QueryPrefix::Unknown => SymbolKind::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Query;
    use crate::model::SymbolKind;

    #[test]
    fn happy_path_function_prefix_matches() {
        let query = Query::parse("fn:calc*").expect("parse query");
        assert_eq!(query.kind, SymbolKind::Function);
        assert!(query.matches(SymbolKind::Function, "calculate_total"));
    }

    #[test]
    fn side_path_unknown_prefix_becomes_unknown_kind() {
        let query = Query::parse("other:thing*").expect("parse query");
        assert_eq!(query.kind, SymbolKind::Unknown);
        assert!(query.matches(SymbolKind::Type, "thing_one"));
    }

    #[test]
    fn critical_path_invalid_glob_returns_error() {
        let query = Query::parse("fn:[bad");
        assert!(query.is_err());
    }
}
