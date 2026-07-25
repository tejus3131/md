use crate::terminal::colors::{self, ColorDepth, fg_color};
use crate::theme::style::Theme;

#[must_use]
pub fn render_heading(
    level: u8,
    text: &str,
    theme: &Theme,
    width: usize,
    depth: ColorDepth,
) -> String {
    let style = theme.heading_style(level);

    let mut out = String::new();
    out.push('\n');

    if style.bold {
        out.push_str(colors::BOLD);
    }
    if style.italic {
        out.push_str(colors::ITALIC);
    }
    if style.underline {
        out.push_str(colors::UNDERLINE);
    }
    if let Some(ref fg) = style.fg {
        out.push_str(&fg_color(&theme.resolve_color(fg), depth));
    }

    out.push_str(text);
    out.push_str(colors::RESET);
    out.push('\n');

    if level <= 2 {
        let rule_char = &theme.heading.rule_char;
        if let Some(ref color) = theme.heading.rule_color {
            out.push_str(&fg_color(&theme.resolve_color(color), depth));
        }
        let rule_width = if level == 1 {
            width
        } else {
            width.min(text.len() + 10)
        };
        for _ in 0..rule_width {
            out.push_str(rule_char);
        }
        out.push_str(colors::RESET);
        out.push('\n');
    }

    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme;

    fn test_theme() -> Theme {
        theme::default_theme()
    }

    #[test]
    fn h1_has_underline_rule() {
        let result = render_heading(1, "Title", &test_theme(), 80, ColorDepth::None);
        assert!(result.contains("Title"));
        assert!(result.contains('─'));
    }

    #[test]
    fn h2_has_shorter_rule() {
        let result = render_heading(2, "Sub", &test_theme(), 80, ColorDepth::None);
        assert!(result.contains("Sub"));
        assert!(result.contains('─'));
    }

    #[test]
    fn h3_no_rule() {
        let result = render_heading(3, "Sub", &test_theme(), 80, ColorDepth::None);
        assert!(result.contains("Sub"));
        assert!(!result.contains('─'));
    }

    #[test]
    fn heading_with_color() {
        let result = render_heading(1, "Title", &test_theme(), 80, ColorDepth::TrueColor);
        assert!(result.contains("\x1b[38;2;")); // truecolor fg
    }
}
