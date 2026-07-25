use super::measure::{strip_ansi, visible_width};

/// ANSI-aware word wrap. Wraps `text` at `width` columns,
/// preserving ANSI escape sequences across line breaks.
#[must_use]
pub fn wrap_text(text: &str, width: usize) -> String {
    if width == 0 {
        return text.to_string();
    }

    let mut result = String::new();
    for line in text.split('\n') {
        if !result.is_empty() {
            result.push('\n');
        }
        wrap_line(line, width, &mut result);
    }
    result
}

fn wrap_line(line: &str, width: usize, out: &mut String) {
    if visible_width(line) <= width {
        out.push_str(line);
        return;
    }

    let words = split_words_preserving_ansi(line);
    let mut current_width: usize = 0;
    let mut line_started = false;
    let mut active_ansi = String::new();

    for word in &words {
        let word_vis_width = visible_width(word);

        if line_started && current_width + word_vis_width > width {
            out.push_str("\x1b[0m");
            out.push('\n');
            if !active_ansi.is_empty() {
                out.push_str(&active_ansi);
            }
            current_width = 0;
        }

        track_ansi_state(word, &mut active_ansi);

        out.push_str(word);
        current_width += word_vis_width;
        line_started = true;
    }
}

fn split_words_preserving_ansi(line: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();
    let mut in_word = false;

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            current.push(c);
            consume_escape(&mut chars, &mut current);
        } else if c == ' ' || c == '\t' {
            if in_word {
                words.push(std::mem::take(&mut current));
                in_word = false;
            }
            current.push(c);
        } else {
            if !in_word && !current.is_empty() {
                let stripped = strip_ansi(&current);
                if !stripped.is_empty() || words.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            current.push(c);
            in_word = true;
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

fn consume_escape(chars: &mut std::iter::Peekable<std::str::Chars>, buf: &mut String) {
    if let Some(&next) = chars.peek() {
        match next {
            '[' => {
                buf.push(chars.next().unwrap());
                while let Some(&c) = chars.peek() {
                    buf.push(chars.next().unwrap());
                    if (0x40..=0x7E).contains(&(c as u32)) {
                        break;
                    }
                }
            }
            ']' => {
                buf.push(chars.next().unwrap());
                while let Some(c) = chars.next() {
                    buf.push(c);
                    if c == '\x07' {
                        break;
                    }
                    if c == '\x1b' && chars.peek() == Some(&'\\') {
                        buf.push(chars.next().unwrap());
                        break;
                    }
                }
            }
            _ => {
                buf.push(chars.next().unwrap());
            }
        }
    }
}

fn track_ansi_state(text: &str, state: &mut String) {
    let mut i = 0;
    let bytes = text.as_bytes();

    while i < bytes.len() {
        if bytes[i] == 0x1b && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            let start = i;
            i += 2;
            while i < bytes.len() && !(bytes[i].is_ascii_alphabetic() || bytes[i] == b'm') {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            let seq = &text[start..i];
            if seq == "\x1b[0m" || seq == "\x1b[m" {
                state.clear();
            } else {
                state.push_str(seq);
            }
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_short_line_unchanged() {
        assert_eq!(wrap_text("hello", 80), "hello");
    }

    #[test]
    fn wrap_long_line_breaks() {
        let result = wrap_text("hello world foo bar", 11);
        assert!(result.contains('\n'));
    }

    #[test]
    fn wrap_preserves_newlines() {
        assert_eq!(wrap_text("a\nb\nc", 80), "a\nb\nc");
    }

    #[test]
    fn wrap_zero_width_passthrough() {
        assert_eq!(wrap_text("hello", 0), "hello");
    }

    #[test]
    fn wrap_with_ansi_preserves_codes() {
        let input = "\x1b[1mhello world this is a long line\x1b[0m";
        let result = wrap_text(input, 15);
        assert!(result.contains('\n'));
        // Ensure reset is emitted at wrap point
        assert!(result.contains("\x1b[0m"));
    }
}
