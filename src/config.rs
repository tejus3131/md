use serde::Deserialize;
use std::path::Path;

use crate::error::{Error, Result};
use crate::theme;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub theme: Option<String>,
    pub max_width: Option<usize>,
    pub pager: Option<String>,
    pub line_numbers: Option<bool>,
}

impl Config {
    /// Load config from `~/.config/md/config.toml`, returning defaults on
    /// missing file. Returns an error only if the file exists but is malformed.
    pub fn load() -> Result<Self> {
        let Some(dir) = theme::config_dir() else {
            return Ok(Self::default());
        };

        let path = dir.join("config.toml");
        if !path.exists() {
            return Ok(Self::default());
        }

        Self::load_from(&path)
    }

    fn load_from(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| Error::ReadFile {
            path: path.to_path_buf(),
            source: e,
        })?;

        toml::from_str(&content).map_err(|e| Error::ConfigParse {
            path: path.to_path_buf(),
            source: e,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let cfg = Config::default();
        assert!(cfg.theme.is_none());
        assert!(cfg.max_width.is_none());
        assert!(cfg.pager.is_none());
        assert!(cfg.line_numbers.is_none());
    }

    #[test]
    fn load_returns_default_when_no_config() {
        let cfg = Config::load().unwrap();
        assert!(cfg.theme.is_none());
    }

    #[test]
    fn deserialize_partial_config() {
        let toml_str = r#"theme = "dracula""#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.theme.as_deref(), Some("dracula"));
        assert!(cfg.max_width.is_none());
    }

    #[test]
    fn deserialize_full_config() {
        let toml_str = r#"
theme = "nord"
max_width = 120
pager = "always"
line_numbers = false
"#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.theme.as_deref(), Some("nord"));
        assert_eq!(cfg.max_width, Some(120));
        assert_eq!(cfg.pager.as_deref(), Some("always"));
        assert_eq!(cfg.line_numbers, Some(false));
    }
}
