use crate::terminal::colors::{self, ColorDepth};
use crate::theme::style::Theme;

use super::text;

/// Render an inline HTML fragment by converting known tags to ANSI escapes.
/// Unknown tags are passed through dimmed.
#[must_use]
pub fn render_html_inline(html: &str, theme: &Theme, depth: ColorDepth) -> String {
    let tag = html.trim();

    // Self-closing <br> / <br/>
    if tag_matches(tag, "br") {
        return "\n".to_string();
    }

    // Opening tags → ANSI start sequences
    if let Some(code) = opening_tag_ansi(tag) {
        return code.to_string();
    }

    // Closing tags → RESET
    if is_closing_tag(tag, "em")
        || is_closing_tag(tag, "i")
        || is_closing_tag(tag, "strong")
        || is_closing_tag(tag, "b")
        || is_closing_tag(tag, "del")
        || is_closing_tag(tag, "s")
        || is_closing_tag(tag, "code")
    {
        return colors::RESET.to_string();
    }

    // <a href="..."> → start OSC-8 hyperlink + styling
    if let Some(url) = extract_href(tag) {
        return start_link(&url, theme, depth);
    }

    // </a> → close OSC-8 hyperlink
    if is_closing_tag(tag, "a") {
        return close_link();
    }

    // Unknown tag → dimmed passthrough
    format!("{}{html}{}", colors::DIM, colors::RESET)
}

/// Render a block-level HTML node by converting known tags inline.
/// Falls back to dimmed raw output for unrecognized blocks.
#[must_use]
pub fn render_html_block(html: &str, theme: &Theme, depth: ColorDepth) -> String {
    let trimmed = html.trim();

    if tag_matches(trimmed, "br") {
        return "\n".to_string();
    }
    if tag_matches(trimmed, "hr") {
        return format!("\n{}{}", "─".repeat(40), colors::RESET);
    }

    let mut out = String::new();
    let mut has_known_tags = false;
    for segment in split_html_segments(trimmed) {
        match segment {
            HtmlSegment::Tag(tag) => {
                if is_known_tag(&tag) {
                    has_known_tags = true;
                }
                out.push_str(&render_html_inline(&tag, theme, depth));
            }
            HtmlSegment::Text(t) => out.push_str(t),
        }
    }

    if has_known_tags {
        out
    } else {
        format!("{}{html}{}", colors::DIM, colors::RESET)
    }
}

fn is_known_tag(tag: &str) -> bool {
    let t = tag.trim();
    tag_matches(t, "br")
        || opening_tag_ansi(t).is_some()
        || is_closing_tag(t, "em")
        || is_closing_tag(t, "i")
        || is_closing_tag(t, "strong")
        || is_closing_tag(t, "b")
        || is_closing_tag(t, "del")
        || is_closing_tag(t, "s")
        || is_closing_tag(t, "code")
        || extract_href(t).is_some()
        || is_closing_tag(t, "a")
}

fn opening_tag_ansi(tag: &str) -> Option<&'static str> {
    if is_opening_tag(tag, "em") || is_opening_tag(tag, "i") {
        Some(text::apply_italic())
    } else if is_opening_tag(tag, "strong") || is_opening_tag(tag, "b") {
        Some(text::apply_bold())
    } else if is_opening_tag(tag, "del") || is_opening_tag(tag, "s") {
        Some(text::apply_strikethrough())
    } else if is_opening_tag(tag, "code") {
        Some(colors::DIM)
    } else {
        None
    }
}

fn is_opening_tag(html: &str, name: &str) -> bool {
    let lower = html.to_ascii_lowercase();
    lower == format!("<{name}>") || lower.starts_with(&format!("<{name} "))
}

fn is_closing_tag(html: &str, name: &str) -> bool {
    html.eq_ignore_ascii_case(&format!("</{name}>"))
}

fn tag_matches(html: &str, name: &str) -> bool {
    let lower = html.to_ascii_lowercase();
    lower == format!("<{name}>") || lower == format!("<{name}/>") || lower == format!("<{name} />")
}

fn extract_href(tag: &str) -> Option<String> {
    let lower = tag.to_ascii_lowercase();
    if !lower.starts_with("<a ") {
        return None;
    }
    // Find href="..." or href='...'
    let href_pos = lower.find("href=")?;
    let after = &tag[href_pos + 5..];
    let (quote, rest) = if let Some(stripped) = after.strip_prefix('"') {
        ('"', stripped)
    } else {
        let stripped = after.strip_prefix('\'')?;
        ('\'', stripped)
    };
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

fn start_link(url: &str, theme: &Theme, depth: ColorDepth) -> String {
    let mut out = String::new();
    out.push_str(&format!("\x1b]8;;{url}\x1b\\"));
    if let Some(ref fg) = theme.link.text {
        out.push_str(&crate::terminal::colors::fg_color(
            &theme.resolve_color(fg),
            depth,
        ));
    }
    if theme.link.style == "underline" {
        out.push_str(colors::UNDERLINE);
    }
    out
}

fn close_link() -> String {
    format!("{}\x1b]8;;\x1b\\", colors::RESET)
}

enum HtmlSegment<'a> {
    Tag(String),
    Text(&'a str),
}

fn split_html_segments(html: &str) -> Vec<HtmlSegment<'_>> {
    let mut segments = Vec::new();
    let mut rest = html;
    while let Some(start) = rest.find('<') {
        if start > 0 {
            segments.push(HtmlSegment::Text(&rest[..start]));
        }
        if let Some(end) = rest[start..].find('>') {
            segments.push(HtmlSegment::Tag(rest[start..start + end + 1].to_string()));
            rest = &rest[start + end + 1..];
        } else {
            segments.push(HtmlSegment::Text(&rest[start..]));
            rest = "";
        }
    }
    if !rest.is_empty() {
        segments.push(HtmlSegment::Text(rest));
    }
    segments
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme;

    fn test_theme() -> Theme {
        theme::default_theme()
    }

    #[test]
    fn em_tag_produces_italic() {
        let open = render_html_inline("<em>", &test_theme(), ColorDepth::None);
        let close = render_html_inline("</em>", &test_theme(), ColorDepth::None);
        assert_eq!(open, "\x1b[3m");
        assert_eq!(close, colors::RESET);
    }

    #[test]
    fn i_tag_produces_italic() {
        let open = render_html_inline("<i>", &test_theme(), ColorDepth::None);
        assert_eq!(open, "\x1b[3m");
    }

    #[test]
    fn strong_tag_produces_bold() {
        let open = render_html_inline("<strong>", &test_theme(), ColorDepth::None);
        let close = render_html_inline("</strong>", &test_theme(), ColorDepth::None);
        assert_eq!(open, "\x1b[1m");
        assert_eq!(close, colors::RESET);
    }

    #[test]
    fn b_tag_produces_bold() {
        let open = render_html_inline("<b>", &test_theme(), ColorDepth::None);
        assert_eq!(open, "\x1b[1m");
    }

    #[test]
    fn del_tag_produces_strikethrough() {
        let open = render_html_inline("<del>", &test_theme(), ColorDepth::None);
        assert_eq!(open, "\x1b[9m");
    }

    #[test]
    fn s_tag_produces_strikethrough() {
        let open = render_html_inline("<s>", &test_theme(), ColorDepth::None);
        assert_eq!(open, "\x1b[9m");
    }

    #[test]
    fn code_tag_produces_dim() {
        let open = render_html_inline("<code>", &test_theme(), ColorDepth::None);
        let close = render_html_inline("</code>", &test_theme(), ColorDepth::None);
        assert_eq!(open, colors::DIM);
        assert_eq!(close, colors::RESET);
    }

    #[test]
    fn br_produces_newline() {
        assert_eq!(
            render_html_inline("<br>", &test_theme(), ColorDepth::None),
            "\n"
        );
        assert_eq!(
            render_html_inline("<br/>", &test_theme(), ColorDepth::None),
            "\n"
        );
        assert_eq!(
            render_html_inline("<br />", &test_theme(), ColorDepth::None),
            "\n"
        );
    }

    #[test]
    fn a_tag_with_href() {
        let result = render_html_inline(
            r#"<a href="https://example.com">"#,
            &test_theme(),
            ColorDepth::None,
        );
        assert!(result.contains("\x1b]8;;https://example.com\x1b\\"));
    }

    #[test]
    fn a_closing_tag() {
        let result = render_html_inline("</a>", &test_theme(), ColorDepth::None);
        assert!(result.contains("\x1b]8;;\x1b\\"));
    }

    #[test]
    fn unknown_tag_dimmed() {
        let result = render_html_inline("<div>", &test_theme(), ColorDepth::None);
        assert!(result.contains(colors::DIM));
        assert!(result.contains("<div>"));
    }

    #[test]
    fn extract_href_double_quotes() {
        assert_eq!(
            extract_href(r#"<a href="https://x.com">"#),
            Some("https://x.com".to_string())
        );
    }

    #[test]
    fn extract_href_single_quotes() {
        assert_eq!(
            extract_href("<a href='https://y.com'>"),
            Some("https://y.com".to_string())
        );
    }

    #[test]
    fn extract_href_no_a_tag() {
        assert_eq!(extract_href("<div href='x'>"), None);
    }

    #[test]
    fn html_block_with_known_tags() {
        let result = render_html_block(
            "<p><strong>bold</strong></p>",
            &test_theme(),
            ColorDepth::None,
        );
        assert!(result.contains("\x1b[1m"));
        assert!(result.contains("bold"));
    }
}
