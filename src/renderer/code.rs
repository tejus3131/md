use std::io::Cursor;
use std::sync::LazyLock;

use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

use crate::layout::measure::visible_width;
use crate::terminal::colors::{self, ColorDepth, bg_color, fg_color};
use crate::theme::style::Theme;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(|| {
    let mut ts = ThemeSet::load_defaults();
    // Embed GitHub-matching syntax themes
    let github_dark_bytes = include_bytes!("../../themes/github-dark.tmTheme");
    let github_light_bytes = include_bytes!("../../themes/github-light.tmTheme");
    if let Ok(theme) = ThemeSet::load_from_reader(&mut Cursor::new(&github_dark_bytes[..])) {
        ts.themes.insert("github-dark".to_string(), theme);
    }
    if let Ok(theme) = ThemeSet::load_from_reader(&mut Cursor::new(&github_light_bytes[..])) {
        ts.themes.insert("github-light".to_string(), theme);
    }
    ts
});

struct BoxChars {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
}

const ROUNDED: BoxChars = BoxChars {
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
    horizontal: "─",
    vertical: "│",
};

#[must_use]
pub fn render_code_block(
    code: &str,
    lang: &str,
    theme: &Theme,
    width: usize,
    depth: ColorDepth,
) -> String {
    let border_color = theme.resolve_opt(&theme.code.block_border);
    let bg = theme.resolve_opt(&theme.code.block_bg);
    let chars = &ROUNDED;

    let lines: Vec<&str> = code.lines().collect();
    let show_line_nums = theme.code.line_numbers;
    let num_width = if show_line_nums {
        lines.len().to_string().len()
    } else {
        0
    };
    let gutter = if show_line_nums { num_width + 2 } else { 0 };

    let inner_width = if width > gutter + 4 {
        width - 2 - gutter
    } else {
        width.saturating_sub(2)
    };

    let bc = |s: &str| -> String {
        if let Some(ref c) = border_color {
            format!("{}{s}{}", fg_color(c, depth), colors::RESET)
        } else {
            s.to_string()
        }
    };

    let bg_start = bg.as_ref().map_or(String::new(), |c| bg_color(c, depth));

    let mut out = String::new();
    out.push('\n');

    // Top border with optional language label
    let label = if theme.code.lang_label && !lang.is_empty() {
        format!(" {lang} ")
    } else {
        String::new()
    };
    let label_width = visible_width(&label);
    let top_fill = (inner_width + gutter).saturating_sub(label_width);

    out.push_str(&bc(chars.top_left));
    out.push_str(&bc(&chars.horizontal.repeat(top_fill)));
    if !label.is_empty() {
        if let Some(ref c) = border_color {
            out.push_str(&fg_color(c, depth));
        }
        out.push_str(&label);
        out.push_str(colors::RESET);
    }
    out.push_str(&bc(chars.top_right));
    out.push('\n');

    let highlighted = highlight_code(code, lang, &theme.code.syntax_theme, depth);

    for (i, line) in highlighted.lines().enumerate() {
        out.push_str(&bc(chars.vertical));
        out.push_str(&bg_start);

        if show_line_nums {
            if let Some(ref c) = border_color {
                out.push_str(&fg_color(c, depth));
            }
            out.push_str(&format!(" {:>num_width$} ", i + 1));
            out.push_str(colors::RESET);
            out.push_str(&bg_start);
        }

        let vis_w = visible_width(line);
        out.push_str(line);

        // Re-emit bg after highlighted text (syntect tokens end with RESET)
        out.push_str(&bg_start);
        if vis_w < inner_width {
            out.push_str(&" ".repeat(inner_width - vis_w));
        }

        out.push_str(colors::RESET);
        out.push_str(&bc(chars.vertical));
        out.push('\n');
    }

    // Bottom border
    out.push_str(&bc(chars.bottom_left));
    out.push_str(&bc(&chars.horizontal.repeat(inner_width + gutter)));
    out.push_str(&bc(chars.bottom_right));
    out.push('\n');
    out.push('\n');

    out
}

fn highlight_code(code: &str, lang: &str, theme_name: &str, depth: ColorDepth) -> String {
    use syntect::easy::HighlightLines;
    use syntect::util::LinesWithEndings;

    if depth == ColorDepth::None {
        return code.strip_suffix('\n').unwrap_or(code).to_string();
    }

    let syntax = if lang.is_empty() {
        SYNTAX_SET.find_syntax_plain_text()
    } else {
        SYNTAX_SET
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text())
    };

    let theme = THEME_SET.themes.get(theme_name).unwrap_or_else(|| {
        THEME_SET
            .themes
            .values()
            .next()
            .expect("at least one syntect theme")
    });

    let mut h = HighlightLines::new(syntax, theme);
    let mut result = String::new();

    for line in LinesWithEndings::from(code) {
        match h.highlight_line(line, &SYNTAX_SET) {
            Ok(ranges) => {
                for (style, text) in ranges {
                    let fg = style.foreground;
                    let hex = format!("#{:02x}{:02x}{:02x}", fg.r, fg.g, fg.b);
                    result.push_str(&fg_color(&hex, depth));
                    result.push_str(text.trim_end_matches('\n'));
                }
                result.push_str(colors::RESET);
                if line.ends_with('\n') && !line.ends_with("\n\n") {
                    result.push('\n');
                }
            }
            Err(_) => {
                result.push_str(line);
            }
        }
    }

    if result.ends_with('\n') {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme;

    fn test_theme() -> Theme {
        theme::default_theme()
    }

    #[test]
    fn code_block_has_border() {
        let result = render_code_block("fn main() {}", "rust", &test_theme(), 80, ColorDepth::None);
        assert!(result.contains('╭'));
        assert!(result.contains('╰'));
        assert!(result.contains('│'));
    }

    #[test]
    fn code_block_with_lang_label() {
        let result = render_code_block("x = 1", "python", &test_theme(), 80, ColorDepth::None);
        assert!(result.contains("python"));
    }

    #[test]
    fn code_block_no_lang() {
        let result = render_code_block("plain text", "", &test_theme(), 80, ColorDepth::None);
        assert!(result.contains("plain text"));
    }

    #[test]
    fn code_block_line_numbers() {
        let result = render_code_block(
            "line1\nline2\nline3",
            "txt",
            &test_theme(),
            80,
            ColorDepth::None,
        );
        assert!(result.contains(" 1 "));
        assert!(result.contains(" 3 "));
    }

    #[test]
    fn highlight_unknown_lang_doesnt_panic() {
        let result = highlight_code(
            "hello",
            "zzzzunknown",
            "base16-ocean.dark",
            ColorDepth::TrueColor,
        );
        assert!(result.contains("hello"));
    }
}
