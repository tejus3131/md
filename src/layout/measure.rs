use unicode_width::UnicodeWidthStr;

/// Measure the visible width of a string, ignoring ANSI escape sequences.
#[must_use]
pub fn visible_width(s: &str) -> usize {
    let stripped = strip_ansi(s);
    UnicodeWidthStr::width(stripped.as_str())
}

/// Strip ANSI escape sequences from a string.
#[must_use]
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if let Some(&next) = chars.peek() {
                match next {
                    '[' => {
                        chars.next();
                        while let Some(&c2) = chars.peek() {
                            chars.next();
                            if (0x40..=0x7E).contains(&(c2 as u32)) {
                                break;
                            }
                        }
                    }
                    ']' => {
                        chars.next();
                        while let Some(c2) = chars.next() {
                            if c2 == '\x07' {
                                break;
                            }
                            if c2 == '\x1b' && chars.peek() == Some(&'\\') {
                                chars.next();
                                break;
                            }
                        }
                    }
                    _ => {
                        chars.next();
                    }
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Pad string to target visible width with spaces.
#[must_use]
pub fn pad_to_width(s: &str, target: usize) -> String {
    let w = visible_width(s);
    if w >= target {
        s.to_string()
    } else {
        format!("{s}{}", " ".repeat(target - w))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_width_plain() {
        assert_eq!(visible_width("hello"), 5);
        assert_eq!(visible_width(""), 0);
    }

    #[test]
    fn visible_width_with_ansi() {
        assert_eq!(visible_width("\x1b[1mhello\x1b[0m"), 5);
        assert_eq!(visible_width("\x1b[38;2;255;0;0mred\x1b[0m"), 3);
    }

    #[test]
    fn visible_width_osc8_links() {
        let link = "\x1b]8;;https://example.com\x1b\\click\x1b]8;;\x1b\\";
        assert_eq!(visible_width(link), 5);
    }

    #[test]
    fn strip_ansi_removes_csi() {
        assert_eq!(strip_ansi("\x1b[1mbold\x1b[0m"), "bold");
    }

    #[test]
    fn strip_ansi_removes_osc() {
        let osc = "\x1b]8;;url\x1b\\text\x1b]8;;\x1b\\";
        assert_eq!(strip_ansi(osc), "text");
    }

    #[test]
    fn strip_ansi_plain_text_unchanged() {
        assert_eq!(strip_ansi("hello world"), "hello world");
    }

    #[test]
    fn pad_to_width_pads() {
        assert_eq!(pad_to_width("hi", 5), "hi   ");
    }

    #[test]
    fn pad_to_width_no_op_when_wider() {
        assert_eq!(pad_to_width("hello", 3), "hello");
    }

    #[test]
    fn pad_to_width_with_ansi() {
        let s = "\x1b[1mhi\x1b[0m";
        let padded = pad_to_width(s, 5);
        assert_eq!(visible_width(&padded), 5);
    }
}
