use crate::terminal::colors::{self, ColorDepth, fg_color};
use crate::theme::style::Theme;

#[must_use]
pub fn blockquote_prefix(theme: &Theme, depth: ColorDepth, nesting: usize) -> String {
    let mut out = String::new();
    for _ in 0..nesting {
        if let Some(ref color) = theme.blockquote.border {
            out.push_str(&fg_color(&theme.resolve_color(color), depth));
        }
        out.push_str(&theme.blockquote.border_char);
        out.push_str(colors::RESET);
        out.push(' ');
    }
    out
}

#[must_use]
pub fn render_hr(theme: &Theme, width: usize, depth: ColorDepth) -> String {
    let mut out = String::new();
    out.push('\n');

    if let Some(ref color) = theme.hr.color {
        out.push_str(&fg_color(&theme.resolve_color(color), depth));
    }

    match theme.hr.style.as_str() {
        "dashed" => {
            let pattern = format!("{} ", theme.hr.char);
            let repeated: String = pattern.chars().cycle().take(width).collect();
            out.push_str(&repeated);
        }
        "center" => {
            let rule_width = width * 2 / 3;
            let pad = (width - rule_width) / 2;
            out.push_str(&" ".repeat(pad));
            for _ in 0..rule_width {
                out.push_str(&theme.hr.char);
            }
        }
        _ => {
            for _ in 0..width {
                out.push_str(&theme.hr.char);
            }
        }
    }

    out.push_str(colors::RESET);
    out.push('\n');
    out.push('\n');
    out
}

#[must_use]
pub fn list_bullet(theme: &Theme, depth: ColorDepth, nesting: usize) -> String {
    let chars = &theme.list.bullet_chars;
    let bullet = &chars[nesting % chars.len()];
    let indent = "  ".repeat(nesting);

    let mut out = String::new();
    if let Some(ref fg) = theme.list.number_style.fg {
        out.push_str(&fg_color(&theme.resolve_color(fg), depth));
    }
    out.push_str(&indent);
    out.push_str(bullet);
    out.push_str(colors::RESET);
    out.push(' ');
    out
}

#[must_use]
pub fn list_number(theme: &Theme, depth: ColorDepth, nesting: usize, number: usize) -> String {
    let indent = "  ".repeat(nesting);

    let mut out = String::new();
    if let Some(ref fg) = theme.list.number_style.fg {
        out.push_str(&fg_color(&theme.resolve_color(fg), depth));
    }
    out.push_str(&indent);
    out.push_str(&format!("{number}."));
    out.push_str(colors::RESET);
    out.push(' ');
    out
}

#[must_use]
pub fn task_checkbox(theme: &Theme, depth: ColorDepth, checked: bool) -> String {
    let mut out = String::new();
    if checked {
        if let Some(ref color) = theme.list.task_done_color {
            out.push_str(&fg_color(&theme.resolve_color(color), depth));
        }
        out.push_str(&theme.list.task_done);
    } else {
        if let Some(ref color) = theme.list.task_pending_color {
            out.push_str(&fg_color(&theme.resolve_color(color), depth));
        }
        out.push_str(&theme.list.task_pending);
    }
    out.push_str(colors::RESET);
    out.push(' ');
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
    fn blockquote_prefix_nesting() {
        let p1 = blockquote_prefix(&test_theme(), ColorDepth::None, 1);
        let p2 = blockquote_prefix(&test_theme(), ColorDepth::None, 2);
        assert!(p1.contains('▌'));
        assert_eq!(p2.matches('▌').count(), 2);
    }

    #[test]
    fn hr_full_width() {
        let hr = render_hr(&test_theme(), 20, ColorDepth::None);
        assert!(hr.contains('─'));
    }

    #[test]
    fn list_bullet_cycling() {
        let t = test_theme();
        let b0 = list_bullet(&t, ColorDepth::None, 0);
        let b1 = list_bullet(&t, ColorDepth::None, 1);
        assert!(b0.contains('●'));
        assert!(b1.contains('○'));
    }

    #[test]
    fn list_number_formatting() {
        let n = list_number(&test_theme(), ColorDepth::None, 0, 3);
        assert!(n.contains("3."));
    }

    #[test]
    fn task_checkbox_done() {
        let c = task_checkbox(&test_theme(), ColorDepth::None, true);
        assert!(c.contains('✓'));
    }

    #[test]
    fn task_checkbox_pending() {
        let c = task_checkbox(&test_theme(), ColorDepth::None, false);
        assert!(c.contains('○'));
    }
}
