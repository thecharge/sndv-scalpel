use std::collections::HashMap;
use std::path::Path;

use crate::config::LanguageConfig;

#[derive(Debug, Clone)]
pub struct LanguageRegistry {
    by_extension: HashMap<String, LanguageConfig>,
}

impl LanguageRegistry {
    pub fn new(languages: &[LanguageConfig]) -> Self {
        let mut by_extension = HashMap::new();
        for language in languages {
            for ext in &language.extensions {
                by_extension.insert(ext.to_ascii_lowercase(), language.clone());
            }
        }
        Self { by_extension }
    }

    pub fn language_for_path(&self, path: &Path) -> Option<LanguageConfig> {
        let ext = path.extension()?.to_string_lossy().to_ascii_lowercase();
        self.by_extension.get(&ext).cloned()
    }
}
