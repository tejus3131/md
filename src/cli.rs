use clap::Parser;

use crate::terminal::pager::PagerMode;

#[derive(Parser, Debug)]
#[command(
    name = "md",
    about = "Render Markdown beautifully in the terminal",
    version,
    long_version = concat!(env!("CARGO_PKG_VERSION"), " (", env!("MD_BUILD_INFO"), ")"),
    after_help = "Examples:\n  md README.md\n  cat README.md | md\n  md --theme github-dark README.md\n  md --list-themes"
)]
pub struct Cli {
    /// Markdown file to render (reads from stdin if omitted or -)
    #[arg(value_name = "FILE")]
    pub file: Option<String>,

    /// Theme name or path to .toml theme file
    #[arg(short, long)]
    pub theme: Option<String>,

    /// Max content width in columns
    #[arg(short, long, default_value_t = 100)]
    pub width: usize,

    /// Pager mode
    #[arg(short, long, value_enum, default_value_t = PagerMode::Auto)]
    pub pager: PagerMode,

    /// Hide line numbers in code blocks
    #[arg(long)]
    pub no_line_numbers: bool,

    /// List all available themes
    #[arg(long)]
    pub list_themes: bool,

    /// Disable colors
    #[arg(long)]
    pub no_color: bool,

    /// Left-align content (default is centered)
    #[arg(long)]
    pub left: bool,

    /// Dump current theme as TOML for customization
    #[arg(long)]
    pub dump_theme: bool,

    /// Print to stdout without pager (shorthand for --pager never)
    #[arg(long)]
    pub raw: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_defaults() {
        let cli = Cli::parse_from(["md"]);
        assert!(cli.file.is_none());
        assert!(cli.theme.is_none());
        assert_eq!(cli.width, 100);
        assert_eq!(cli.pager, PagerMode::Auto);
        assert!(!cli.no_line_numbers);
        assert!(!cli.list_themes);
        assert!(!cli.no_color);
        assert!(!cli.left);
        assert!(!cli.dump_theme);
    }

    #[test]
    fn cli_with_file() {
        let cli = Cli::parse_from(["md", "README.md"]);
        assert_eq!(cli.file.as_deref(), Some("README.md"));
    }

    #[test]
    fn cli_with_all_flags() {
        let cli = Cli::parse_from([
            "md",
            "--theme",
            "dracula",
            "--width",
            "120",
            "--pager",
            "never",
            "--no-line-numbers",
            "--no-color",
            "--left",
            "test.md",
        ]);
        assert_eq!(cli.theme.as_deref(), Some("dracula"));
        assert_eq!(cli.width, 120);
        assert_eq!(cli.pager, PagerMode::Never);
        assert!(cli.no_line_numbers);
        assert!(cli.no_color);
        assert!(cli.left);
        assert_eq!(cli.file.as_deref(), Some("test.md"));
    }

    #[test]
    fn cli_rejects_invalid_pager() {
        let result = Cli::try_parse_from(["md", "--pager", "neverr"]);
        assert!(result.is_err());
    }
}
