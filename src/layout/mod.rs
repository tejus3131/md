pub mod measure;
pub mod wrap;

/// Pure layout configuration — no terminal dependency.
/// Constructed from terminal dimensions but usable independently.
pub struct LayoutConfig {
    pub max_width: usize,
    pub terminal_width: usize,
    pub center: bool,
}

impl LayoutConfig {
    #[must_use]
    pub fn new(terminal_width: u16, max_width: Option<usize>, center: bool) -> Self {
        let tw = terminal_width as usize;
        let max = max_width.unwrap_or(100);
        Self {
            max_width: max,
            terminal_width: tw,
            center,
        }
    }

    #[must_use]
    pub fn content_width(&self) -> usize {
        self.terminal_width.min(self.max_width).max(10)
    }

    #[must_use]
    pub fn left_padding(&self) -> usize {
        if !self.center {
            return 0;
        }
        if self.terminal_width > self.content_width() {
            (self.terminal_width - self.content_width()) / 2
        } else {
            0
        }
    }

    #[must_use]
    pub fn padding_str(&self) -> String {
        " ".repeat(self.left_padding())
    }

    #[must_use]
    pub fn center_content(&self, content: &str) -> String {
        let pad = self.padding_str();
        if pad.is_empty() {
            return content.to_string();
        }
        content
            .lines()
            .map(|line| format!("{pad}{line}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_width_capped() {
        let lc = LayoutConfig::new(200, Some(100), true);
        assert_eq!(lc.content_width(), 100);
    }

    #[test]
    fn content_width_narrow_terminal() {
        let lc = LayoutConfig::new(60, Some(100), true);
        assert_eq!(lc.content_width(), 60);
    }

    #[test]
    fn left_padding_centered() {
        let lc = LayoutConfig::new(200, Some(100), true);
        assert_eq!(lc.left_padding(), 50);
    }

    #[test]
    fn left_padding_zero_when_no_center() {
        let lc = LayoutConfig::new(200, Some(100), false);
        assert_eq!(lc.left_padding(), 0);
    }

    #[test]
    fn left_padding_zero_when_narrow() {
        let lc = LayoutConfig::new(60, Some(100), true);
        assert_eq!(lc.left_padding(), 0);
    }

    #[test]
    fn center_content_adds_padding() {
        let lc = LayoutConfig::new(110, Some(100), true);
        let centered = lc.center_content("hello\nworld");
        assert!(centered.starts_with("     "));
    }

    #[test]
    fn center_content_no_padding_when_disabled() {
        let lc = LayoutConfig::new(200, Some(100), false);
        let centered = lc.center_content("hello");
        assert_eq!(centered, "hello");
    }

    #[test]
    fn center_content_no_padding_when_narrow() {
        let lc = LayoutConfig::new(80, Some(100), true);
        let centered = lc.center_content("hello");
        assert_eq!(centered, "hello");
    }

    #[test]
    fn default_max_width() {
        let lc = LayoutConfig::new(200, None, false);
        assert_eq!(lc.content_width(), 100);
    }
}
