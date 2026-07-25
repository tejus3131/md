use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertKind {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

impl AlertKind {
    #[must_use]
    pub fn icon(self) -> &'static str {
        match self {
            Self::Note => "ℹ",
            Self::Tip => "💡",
            Self::Important => "❗",
            Self::Warning => "⚠",
            Self::Caution => "🔴",
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Note => "Note",
            Self::Tip => "Tip",
            Self::Important => "Important",
            Self::Warning => "Warning",
            Self::Caution => "Caution",
        }
    }
}

impl fmt::Display for AlertKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Detect a GFM alert from a blockquote's first-line text.
/// GitHub format: `[!NOTE]`, `[!TIP]`, `[!IMPORTANT]`, `[!WARNING]`, `[!CAUTION]`
#[must_use]
pub fn detect_alert(text: &str) -> Option<AlertKind> {
    match text.trim() {
        "[!NOTE]" => Some(AlertKind::Note),
        "[!TIP]" => Some(AlertKind::Tip),
        "[!IMPORTANT]" => Some(AlertKind::Important),
        "[!WARNING]" => Some(AlertKind::Warning),
        "[!CAUTION]" => Some(AlertKind::Caution),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_all_alert_kinds() {
        assert_eq!(detect_alert("[!NOTE]"), Some(AlertKind::Note));
        assert_eq!(detect_alert("[!TIP]"), Some(AlertKind::Tip));
        assert_eq!(detect_alert("[!IMPORTANT]"), Some(AlertKind::Important));
        assert_eq!(detect_alert("[!WARNING]"), Some(AlertKind::Warning));
        assert_eq!(detect_alert("[!CAUTION]"), Some(AlertKind::Caution));
    }

    #[test]
    fn detect_with_whitespace() {
        assert_eq!(detect_alert("  [!NOTE]  "), Some(AlertKind::Note));
    }

    #[test]
    fn detect_unknown_returns_none() {
        assert_eq!(detect_alert("[!UNKNOWN]"), None);
        assert_eq!(detect_alert("hello"), None);
        assert_eq!(detect_alert(""), None);
    }

    #[test]
    fn icon_and_label() {
        assert_eq!(AlertKind::Note.icon(), "ℹ");
        assert_eq!(AlertKind::Warning.label(), "Warning");
    }

    #[test]
    fn display_impl() {
        assert_eq!(format!("{}", AlertKind::Caution), "Caution");
    }
}
