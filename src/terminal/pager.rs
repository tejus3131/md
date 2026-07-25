use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum PagerMode {
    Auto,
    Always,
    Never,
}

/// Parse a pager mode from a config-file string (case-insensitive).
#[must_use]
pub fn parse_pager_mode(s: &str) -> Option<PagerMode> {
    use clap::ValueEnum;
    PagerMode::from_str(s, true).ok()
}

pub fn write_stdout(content: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(content.as_bytes())?;
    handle.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pager_mode_value_enum() {
        use clap::ValueEnum;
        assert_eq!(
            PagerMode::from_str("always", true).unwrap(),
            PagerMode::Always
        );
        assert_eq!(
            PagerMode::from_str("never", true).unwrap(),
            PagerMode::Never
        );
        assert_eq!(PagerMode::from_str("auto", true).unwrap(), PagerMode::Auto);
        assert!(PagerMode::from_str("garbage", true).is_err());
    }
}
