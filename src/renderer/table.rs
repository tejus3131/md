use crate::layout::measure::{pad_to_width, visible_width};
use crate::terminal::colors::{self, ColorDepth, fg_color};
use crate::theme::style::Theme;

struct TableBorder {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
    cross: &'static str,
    top_tee: &'static str,
    bottom_tee: &'static str,
    left_tee: &'static str,
    right_tee: &'static str,
}

const ROUNDED_BORDER: TableBorder = TableBorder {
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
    horizontal: "─",
    vertical: "│",
    cross: "┼",
    top_tee: "┬",
    bottom_tee: "┴",
    left_tee: "├",
    right_tee: "┤",
};

const SHARP_BORDER: TableBorder = TableBorder {
    top_left: "┌",
    top_right: "┐",
    bottom_left: "└",
    bottom_right: "┘",
    horizontal: "─",
    vertical: "│",
    cross: "┼",
    top_tee: "┬",
    bottom_tee: "┴",
    left_tee: "├",
    right_tee: "┤",
};

const HEAVY_BORDER: TableBorder = TableBorder {
    top_left: "┏",
    top_right: "┓",
    bottom_left: "┗",
    bottom_right: "┛",
    horizontal: "━",
    vertical: "┃",
    cross: "╋",
    top_tee: "┳",
    bottom_tee: "┻",
    left_tee: "┣",
    right_tee: "┫",
};

const DOUBLE_BORDER: TableBorder = TableBorder {
    top_left: "╔",
    top_right: "╗",
    bottom_left: "╚",
    bottom_right: "╝",
    horizontal: "═",
    vertical: "║",
    cross: "╬",
    top_tee: "╦",
    bottom_tee: "╩",
    left_tee: "╠",
    right_tee: "╣",
};

const ASCII_BORDER: TableBorder = TableBorder {
    top_left: "+",
    top_right: "+",
    bottom_left: "+",
    bottom_right: "+",
    horizontal: "-",
    vertical: "|",
    cross: "+",
    top_tee: "+",
    bottom_tee: "+",
    left_tee: "+",
    right_tee: "+",
};

fn get_border(style: &str) -> &'static TableBorder {
    match style {
        "sharp" => &SHARP_BORDER,
        "heavy" => &HEAVY_BORDER,
        "double" => &DOUBLE_BORDER,
        "ascii" => &ASCII_BORDER,
        _ => &ROUNDED_BORDER,
    }
}

#[must_use]
pub fn render_table(
    rows: &[Vec<String>],
    theme: &Theme,
    _width: usize,
    depth: ColorDepth,
) -> String {
    if rows.is_empty() {
        return String::new();
    }

    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    if num_cols == 0 {
        return String::new();
    }

    let mut col_widths = vec![0usize; num_cols];
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < num_cols {
                col_widths[i] = col_widths[i].max(visible_width(cell));
            }
        }
    }

    for w in &mut col_widths {
        *w = (*w).max(3);
    }

    let border = get_border(&theme.table.style);
    let border_color = theme.resolve_opt(&theme.table.border);

    let bc = |s: &str| -> String {
        if let Some(ref c) = border_color {
            format!("{}{s}{}", fg_color(c, depth), colors::RESET)
        } else {
            s.to_string()
        }
    };

    let mut out = String::new();
    out.push('\n');

    // Top border
    out.push_str(&bc(border.top_left));
    for (i, &w) in col_widths.iter().enumerate() {
        out.push_str(&bc(&border.horizontal.repeat(w + 2)));
        if i < num_cols - 1 {
            out.push_str(&bc(border.top_tee));
        }
    }
    out.push_str(&bc(border.top_right));
    out.push('\n');

    for (row_idx, row) in rows.iter().enumerate() {
        out.push_str(&bc(border.vertical));
        for (i, &w) in col_widths.iter().enumerate() {
            let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");

            out.push(' ');
            if row_idx == 0 {
                if theme.table.header.bold {
                    out.push_str(colors::BOLD);
                }
                if let Some(ref fg) = theme.table.header.fg {
                    out.push_str(&fg_color(&theme.resolve_color(fg), depth));
                }
            }
            out.push_str(cell);
            if row_idx == 0 {
                out.push_str(colors::RESET);
            }
            let padded_tail = pad_to_width("", w.saturating_sub(visible_width(cell)) + 1);
            out.push_str(&padded_tail);
            out.push_str(&bc(border.vertical));
        }
        out.push('\n');

        // Separator after header
        if row_idx == 0 {
            out.push_str(&bc(border.left_tee));
            for (i, &w) in col_widths.iter().enumerate() {
                let sep_color = theme.resolve_opt(&theme.table.separator);
                if let Some(ref c) = sep_color {
                    out.push_str(&fg_color(c, depth));
                    out.push_str(&border.horizontal.repeat(w + 2));
                    out.push_str(colors::RESET);
                } else {
                    out.push_str(&bc(&border.horizontal.repeat(w + 2)));
                }
                if i < num_cols - 1 {
                    out.push_str(&bc(border.cross));
                }
            }
            out.push_str(&bc(border.right_tee));
            out.push('\n');
        }
    }

    // Bottom border
    out.push_str(&bc(border.bottom_left));
    for (i, &w) in col_widths.iter().enumerate() {
        out.push_str(&bc(&border.horizontal.repeat(w + 2)));
        if i < num_cols - 1 {
            out.push_str(&bc(border.bottom_tee));
        }
    }
    out.push_str(&bc(border.bottom_right));
    out.push('\n');
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
    fn empty_table() {
        assert_eq!(render_table(&[], &test_theme(), 80, ColorDepth::None), "");
    }

    #[test]
    fn single_row_table() {
        let rows = vec![vec!["A".to_string(), "B".to_string()]];
        let result = render_table(&rows, &test_theme(), 80, ColorDepth::None);
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        assert!(result.contains('╭'));
    }

    #[test]
    fn table_with_header_and_data() {
        let rows = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
        ];
        let result = render_table(&rows, &test_theme(), 80, ColorDepth::None);
        assert!(result.contains("Name"));
        assert!(result.contains("Alice"));
        assert!(result.contains('├')); // separator
    }

    #[test]
    fn table_border_styles() {
        assert_eq!(get_border("rounded").top_left, "╭");
        assert_eq!(get_border("sharp").top_left, "┌");
        assert_eq!(get_border("heavy").top_left, "┏");
        assert_eq!(get_border("double").top_left, "╔");
        assert_eq!(get_border("ascii").top_left, "+");
        assert_eq!(get_border("unknown").top_left, "╭"); // fallback
    }
}
