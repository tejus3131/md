use crate::terminal::colors::{self, ColorDepth, bg_color, fg_color};
use crate::theme::style::Theme;

#[must_use]
pub fn apply_bold() -> &'static str {
    colors::BOLD
}

#[must_use]
pub fn apply_italic() -> &'static str {
    colors::ITALIC
}

#[must_use]
pub fn apply_strikethrough() -> &'static str {
    colors::STRIKETHROUGH
}

#[must_use]
pub fn render_inline_code(text: &str, theme: &Theme, depth: ColorDepth) -> String {
    let mut out = String::new();

    if let Some(ref bg) = theme.code.inline_bg {
        out.push_str(&bg_color(&theme.resolve_color(bg), depth));
    }
    if let Some(ref fg) = theme.code.inline_fg {
        out.push_str(&fg_color(&theme.resolve_color(fg), depth));
    }

    out.push(' ');
    out.push_str(text);
    out.push(' ');
    out.push_str(colors::RESET);
    out
}

#[must_use]
pub fn render_link(text: &str, url: &str, theme: &Theme, depth: ColorDepth) -> String {
    let mut out = String::new();

    // OSC-8 hyperlink start
    out.push_str(&format!("\x1b]8;;{url}\x1b\\"));

    if let Some(ref fg) = theme.link.text {
        out.push_str(&fg_color(&theme.resolve_color(fg), depth));
    }
    if theme.link.style == "underline" {
        out.push_str(colors::UNDERLINE);
    }

    out.push_str(text);
    out.push_str(colors::RESET);
    out.push_str("\x1b]8;;\x1b\\");

    out
}

#[must_use]
pub fn render_image(alt: &str, theme: &Theme, depth: ColorDepth) -> String {
    if !theme.image.show_alt {
        return String::new();
    }

    let mut out = String::new();
    if let Some(ref fg) = theme.image.style.fg {
        out.push_str(&fg_color(&theme.resolve_color(fg), depth));
    }
    if theme.image.style.italic {
        out.push_str(colors::ITALIC);
    }

    out.push_str(&theme.image.prefix);
    out.push_str(alt);
    out.push_str(colors::RESET);
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
    fn inline_code_has_reset() {
        let result = render_inline_code("foo", &test_theme(), ColorDepth::None);
        assert!(result.contains("foo"));
        assert!(result.ends_with(colors::RESET));
    }

    #[test]
    fn link_has_osc8() {
        let result = render_link(
            "GitHub",
            "https://github.com",
            &test_theme(),
            ColorDepth::None,
        );
        assert!(result.contains("GitHub"));
        assert!(result.contains("\x1b]8;;https://github.com\x1b\\"));
        assert!(result.contains("\x1b]8;;\x1b\\")); // close
    }

    #[test]
    fn image_shows_alt() {
        let result = render_image("photo", &test_theme(), ColorDepth::None);
        assert!(result.contains("photo"));
        assert!(result.contains("🖼"));
    }

    #[test]
    fn image_hidden_when_disabled() {
        let mut t = test_theme();
        t.image.show_alt = false;
        assert_eq!(render_image("photo", &t, ColorDepth::None), "");
    }

    #[test]
    fn bold_italic_strikethrough_are_constants() {
        assert_eq!(apply_bold(), "\x1b[1m");
        assert_eq!(apply_italic(), "\x1b[3m");
        assert_eq!(apply_strikethrough(), "\x1b[9m");
    }
}
