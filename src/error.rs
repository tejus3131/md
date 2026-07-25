use std::path::PathBuf;

/// All errors that `md` can produce.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not read '{path}': {source}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("could not read from stdin: {0}")]
    ReadStdin(std::io::Error),

    #[error("could not write output: {0}")]
    WriteOutput(std::io::Error),

    #[error("theme '{name}' not found (use --list-themes to see available themes)")]
    ThemeNotFound { name: String },

    #[error("failed to parse theme '{name}': {source}")]
    ThemeParse {
        name: String,
        source: toml::de::Error,
    },

    #[error("failed to serialize theme: {0}")]
    ThemeSerialize(toml::ser::Error),

    #[error("failed to parse config '{path}': {source}")]
    ConfigParse {
        path: PathBuf,
        source: toml::de::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
