pub mod builtin;
pub mod style;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

use directories::ProjectDirs;
use style::Theme;

use crate::error::{Error, Result};

static BUILTINS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("catppuccin-mocha", builtin::CATPPUCCIN_MOCHA),
        ("catppuccin-latte", builtin::CATPPUCCIN_LATTE),
        ("github-dark", builtin::GITHUB_DARK),
        ("github-light", builtin::GITHUB_LIGHT),
        ("dracula", builtin::DRACULA),
        ("nord", builtin::NORD),
    ])
});

#[must_use]
pub fn config_dir() -> Option<PathBuf> {
    ProjectDirs::from("", "", "md").map(|d| d.config_dir().to_path_buf())
}

#[must_use]
pub fn user_themes_dir() -> Option<PathBuf> {
    config_dir().map(|d| d.join("themes"))
}

pub fn load_theme(name: &str) -> Result<Theme> {
    if let Some(toml_str) = BUILTINS.get(name) {
        return toml::from_str(toml_str).map_err(|e| Error::ThemeParse {
            name: name.to_string(),
            source: e,
        });
    }

    if let Some(dir) = user_themes_dir() {
        let path = dir.join(format!("{name}.toml"));
        if path.exists() {
            let content = std::fs::read_to_string(&path).map_err(|e| Error::ReadFile {
                path: path.clone(),
                source: e,
            })?;
            return toml::from_str(&content).map_err(|e| Error::ThemeParse {
                name: name.to_string(),
                source: e,
            });
        }
    }

    let path = PathBuf::from(name);
    if path.exists() {
        let content = std::fs::read_to_string(&path).map_err(|e| Error::ReadFile {
            path: path.clone(),
            source: e,
        })?;
        return toml::from_str(&content).map_err(|e| Error::ThemeParse {
            name: name.to_string(),
            source: e,
        });
    }

    Err(Error::ThemeNotFound {
        name: name.to_string(),
    })
}

#[must_use]
pub fn list_themes() -> Vec<String> {
    let mut names: Vec<String> = BUILTINS.keys().map(|k| k.to_string()).collect();

    if let Some(dir) = user_themes_dir() {
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|e| e == "toml")
                        && let Some(stem) = path.file_stem()
                    {
                        let name = stem.to_string_lossy().to_string();
                        if !names.contains(&name) {
                            names.push(name);
                        }
                    }
                }
            }
        }
    }

    names.sort();
    names
}

pub fn dump_theme(theme: &Theme) -> Result<String> {
    toml::to_string_pretty(theme).map_err(Error::ThemeSerialize)
}

#[cfg(test)]
pub fn default_theme() -> Theme {
    load_theme("catppuccin-mocha").expect("built-in theme must parse")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_builtin_catppuccin() {
        let theme = load_theme("catppuccin-mocha").unwrap();
        assert_eq!(theme.meta.name, "catppuccin-mocha");
    }

    #[test]
    fn load_all_builtins() {
        for name in [
            "catppuccin-mocha",
            "catppuccin-latte",
            "github-dark",
            "github-light",
            "dracula",
            "nord",
        ] {
            let theme = load_theme(name).unwrap_or_else(|e| panic!("failed to load {name}: {e}"));
            assert_eq!(theme.meta.name, name);
        }
    }

    #[test]
    fn load_nonexistent_returns_error() {
        let result = load_theme("nonexistent-theme-xyz");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn list_themes_includes_builtins() {
        let themes = list_themes();
        assert!(themes.contains(&"catppuccin-mocha".to_string()));
        assert!(themes.contains(&"github-dark".to_string()));
        assert!(themes.contains(&"nord".to_string()));
    }

    #[test]
    fn list_themes_sorted() {
        let themes = list_themes();
        let mut sorted = themes.clone();
        sorted.sort();
        assert_eq!(themes, sorted);
    }

    #[test]
    fn dump_theme_roundtrips() {
        let theme = default_theme();
        let dumped = dump_theme(&theme).unwrap();
        let reparsed: Theme = toml::from_str(&dumped).unwrap();
        assert_eq!(reparsed.meta.name, theme.meta.name);
    }

    #[test]
    fn default_theme_is_catppuccin_mocha() {
        let theme = default_theme();
        assert_eq!(theme.meta.name, "catppuccin-mocha");
    }
}
