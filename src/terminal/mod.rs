pub mod colors;
pub mod pager;
pub mod viewer;

use crossterm::terminal;

pub struct TerminalInfo {
    pub width: u16,
    pub color_depth: colors::ColorDepth,
}

impl TerminalInfo {
    #[must_use]
    pub fn detect() -> Self {
        let (width, _height) = terminal::size().unwrap_or((80, 24));
        let color_depth = colors::detect_color_depth();

        Self { width, color_depth }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_returns_reasonable_values() {
        let info = TerminalInfo::detect();
        assert!(info.width > 0);
    }
}
