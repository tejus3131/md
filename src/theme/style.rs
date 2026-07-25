use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextStyle {
    pub fg: Option<String>,
    pub bg: Option<String>,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub strikethrough: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMeta {
    pub name: String,
    #[serde(default = "default_variant")]
    pub variant: String,
}

fn default_variant() -> String {
    "dark".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingTheme {
    #[serde(default)]
    pub h1: TextStyle,
    #[serde(default)]
    pub h2: TextStyle,
    #[serde(default)]
    pub h3: TextStyle,
    #[serde(default)]
    pub h4: TextStyle,
    #[serde(default)]
    pub h5: TextStyle,
    #[serde(default)]
    pub h6: TextStyle,
    #[serde(default = "default_rule_char")]
    pub rule_char: String,
    pub rule_color: Option<String>,
}

fn default_rule_char() -> String {
    "─".to_string()
}

impl Default for HeadingTheme {
    fn default() -> Self {
        Self {
            h1: TextStyle {
                bold: true,
                ..Default::default()
            },
            h2: TextStyle {
                bold: true,
                ..Default::default()
            },
            h3: TextStyle {
                bold: true,
                ..Default::default()
            },
            h4: TextStyle {
                bold: true,
                ..Default::default()
            },
            h5: TextStyle {
                bold: true,
                ..Default::default()
            },
            h6: TextStyle {
                bold: true,
                ..Default::default()
            },
            rule_char: default_rule_char(),
            rule_color: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTheme {
    pub inline_fg: Option<String>,
    pub inline_bg: Option<String>,
    pub block_border: Option<String>,
    pub block_bg: Option<String>,
    #[serde(default = "default_syntax_theme")]
    pub syntax_theme: String,
    #[serde(default = "default_true")]
    pub line_numbers: bool,
    #[serde(default = "default_true")]
    pub lang_label: bool,
}

fn default_syntax_theme() -> String {
    "base16-ocean.dark".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for CodeTheme {
    fn default() -> Self {
        Self {
            inline_fg: None,
            inline_bg: None,
            block_border: None,
            block_bg: None,
            syntax_theme: default_syntax_theme(),
            line_numbers: true,
            lang_label: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InlineTheme {
    #[serde(default)]
    pub bold: TextStyle,
    #[serde(default)]
    pub italic: TextStyle,
    #[serde(default)]
    pub strikethrough: TextStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTheme {
    pub text: Option<String>,
    pub url: Option<String>,
    #[serde(default = "default_link_style")]
    pub style: String,
}

fn default_link_style() -> String {
    "underline".to_string()
}

impl Default for LinkTheme {
    fn default() -> Self {
        Self {
            text: None,
            url: None,
            style: default_link_style(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockquoteTheme {
    pub border: Option<String>,
    pub text: Option<String>,
    #[serde(default = "default_border_char")]
    pub border_char: String,
}

fn default_border_char() -> String {
    "▌".to_string()
}

impl Default for BlockquoteTheme {
    fn default() -> Self {
        Self {
            border: None,
            text: None,
            border_char: default_border_char(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTheme {
    #[serde(default = "default_bullets")]
    pub bullet_chars: Vec<String>,
    #[serde(default)]
    pub number_style: TextStyle,
    #[serde(default = "default_task_done")]
    pub task_done: String,
    #[serde(default = "default_task_pending")]
    pub task_pending: String,
    pub task_done_color: Option<String>,
    pub task_pending_color: Option<String>,
}

fn default_bullets() -> Vec<String> {
    vec!["●".into(), "○".into(), "■".into(), "□".into()]
}
fn default_task_done() -> String {
    "✓".to_string()
}
fn default_task_pending() -> String {
    "○".to_string()
}

impl Default for ListTheme {
    fn default() -> Self {
        Self {
            bullet_chars: default_bullets(),
            number_style: TextStyle::default(),
            task_done: default_task_done(),
            task_pending: default_task_pending(),
            task_done_color: None,
            task_pending_color: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableTheme {
    pub border: Option<String>,
    #[serde(default)]
    pub header: TextStyle,
    pub separator: Option<String>,
    #[serde(default = "default_table_style")]
    pub style: String,
}

fn default_table_style() -> String {
    "rounded".to_string()
}

impl Default for TableTheme {
    fn default() -> Self {
        Self {
            border: None,
            header: TextStyle {
                bold: true,
                ..Default::default()
            },
            separator: None,
            style: default_table_style(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HrTheme {
    #[serde(default = "default_rule_char")]
    pub char: String,
    pub color: Option<String>,
    #[serde(default = "default_hr_style")]
    pub style: String,
}

fn default_hr_style() -> String {
    "full".to_string()
}

impl Default for HrTheme {
    fn default() -> Self {
        Self {
            char: default_rule_char(),
            color: None,
            style: default_hr_style(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTheme {
    #[serde(default = "default_true")]
    pub show_alt: bool,
    #[serde(default)]
    pub style: TextStyle,
    #[serde(default = "default_image_prefix")]
    pub prefix: String,
}

fn default_image_prefix() -> String {
    "🖼 ".to_string()
}

impl Default for ImageTheme {
    fn default() -> Self {
        Self {
            show_alt: true,
            style: TextStyle {
                italic: true,
                ..Default::default()
            },
            prefix: default_image_prefix(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertTheme {
    pub note_color: Option<String>,
    pub tip_color: Option<String>,
    pub important_color: Option<String>,
    pub warning_color: Option<String>,
    pub caution_color: Option<String>,
}

impl Default for AlertTheme {
    fn default() -> Self {
        Self {
            note_color: Some("#89b4fa".to_string()),
            tip_color: Some("#a6e3a1".to_string()),
            important_color: Some("#cba6f7".to_string()),
            warning_color: Some("#f9e2af".to_string()),
            caution_color: Some("#f38ba8".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub meta: ThemeMeta,
    #[serde(default)]
    pub palette: HashMap<String, String>,
    #[serde(default)]
    pub heading: HeadingTheme,
    #[serde(default)]
    pub code: CodeTheme,
    #[serde(default)]
    pub text: InlineTheme,
    #[serde(default)]
    pub link: LinkTheme,
    #[serde(default)]
    pub blockquote: BlockquoteTheme,
    #[serde(default)]
    pub list: ListTheme,
    #[serde(default)]
    pub table: TableTheme,
    #[serde(default)]
    pub hr: HrTheme,
    #[serde(default)]
    pub image: ImageTheme,
    #[serde(default)]
    pub alert: AlertTheme,
}

impl Theme {
    /// Resolve a color reference: palette name → hex, or return as-is if already hex.
    #[must_use]
    pub fn resolve_color(&self, color: &str) -> String {
        if color.starts_with('#') {
            color.to_string()
        } else {
            self.palette
                .get(color)
                .cloned()
                .unwrap_or_else(|| color.to_string())
        }
    }

    #[must_use]
    pub fn resolve_opt(&self, color: &Option<String>) -> Option<String> {
        color.as_ref().map(|c| self.resolve_color(c))
    }

    /// Get heading style by level (1-6).
    #[must_use]
    pub fn heading_style(&self, level: u8) -> &TextStyle {
        match level {
            1 => &self.heading.h1,
            2 => &self.heading.h2,
            3 => &self.heading.h3,
            4 => &self.heading.h4,
            5 => &self.heading.h5,
            6 => &self.heading.h6,
            _ => &self.heading.h1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_theme() -> Theme {
        let toml_str = include_str!("../../themes/catppuccin-mocha.toml");
        toml::from_str(toml_str).unwrap()
    }

    #[test]
    fn resolve_palette_color() {
        let theme = test_theme();
        assert_eq!(theme.resolve_color("blue"), "#89b4fa");
    }

    #[test]
    fn resolve_hex_passthrough() {
        let theme = test_theme();
        assert_eq!(theme.resolve_color("#ff0000"), "#ff0000");
    }

    #[test]
    fn resolve_unknown_palette() {
        let theme = test_theme();
        assert_eq!(theme.resolve_color("nonexistent"), "nonexistent");
    }

    #[test]
    fn resolve_opt_some() {
        let theme = test_theme();
        assert_eq!(
            theme.resolve_opt(&Some("blue".to_string())),
            Some("#89b4fa".to_string())
        );
    }

    #[test]
    fn resolve_opt_none() {
        let theme = test_theme();
        assert_eq!(theme.resolve_opt(&None), None);
    }

    #[test]
    fn heading_style_levels() {
        let theme = test_theme();
        assert!(theme.heading_style(1).bold);
        assert!(theme.heading_style(2).bold);
        assert!(theme.heading_style(7).bold); // fallback to h1
    }

    #[test]
    fn deserialize_catppuccin_mocha() {
        let theme = test_theme();
        assert_eq!(theme.meta.name, "catppuccin-mocha");
        assert_eq!(theme.meta.variant, "dark");
        assert!(!theme.palette.is_empty());
    }
}
