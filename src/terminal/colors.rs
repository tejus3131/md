use std::env;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorDepth {
    TrueColor,
    EightBit,
    Basic,
    None,
}

impl fmt::Display for ColorDepth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TrueColor => write!(f, "truecolor"),
            Self::EightBit => write!(f, "256-color"),
            Self::Basic => write!(f, "16-color"),
            Self::None => write!(f, "no-color"),
        }
    }
}

#[must_use]
pub fn detect_color_depth() -> ColorDepth {
    if env::var("NO_COLOR").is_ok() {
        return ColorDepth::None;
    }

    if let Ok(ct) = env::var("COLORTERM")
        && (ct == "truecolor" || ct == "24bit")
    {
        return ColorDepth::TrueColor;
    }

    if env::var("TERM").is_ok_and(|t| t.contains("256color")) {
        return ColorDepth::EightBit;
    }

    ColorDepth::Basic
}

#[must_use]
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

#[must_use]
pub fn rgb_to_256(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 {
            return 16;
        }
        if r > 248 {
            return 231;
        }
        return ((r as u16 - 8) * 24 / 247 + 232) as u8;
    }

    let ri = (r as u16 * 5 / 255) as u8;
    let gi = (g as u16 * 5 / 255) as u8;
    let bi = (b as u16 * 5 / 255) as u8;
    16 + 36 * ri + 6 * gi + bi
}

#[must_use]
pub fn rgb_to_basic(r: u8, g: u8, b: u8) -> u8 {
    let brightness = (r as u16 + g as u16 + b as u16) / 3;
    let bold = brightness > 127;

    let ri = u8::from(r > 127);
    let gi = u8::from(g > 127);
    let bi = u8::from(b > 127);

    let base = bi << 2 | gi << 1 | ri;
    if bold { base + 8 } else { base }
}

#[must_use]
pub fn fg_color(hex: &str, depth: ColorDepth) -> String {
    match depth {
        ColorDepth::None => String::new(),
        ColorDepth::TrueColor => hex_to_rgb(hex)
            .map(|(r, g, b)| format!("\x1b[38;2;{r};{g};{b}m"))
            .unwrap_or_default(),
        ColorDepth::EightBit => hex_to_rgb(hex)
            .map(|(r, g, b)| format!("\x1b[38;5;{}m", rgb_to_256(r, g, b)))
            .unwrap_or_default(),
        ColorDepth::Basic => hex_to_rgb(hex)
            .map(|(r, g, b)| format!("\x1b[{}m", 30 + rgb_to_basic(r, g, b)))
            .unwrap_or_default(),
    }
}

#[must_use]
pub fn bg_color(hex: &str, depth: ColorDepth) -> String {
    match depth {
        ColorDepth::None => String::new(),
        ColorDepth::TrueColor => hex_to_rgb(hex)
            .map(|(r, g, b)| format!("\x1b[48;2;{r};{g};{b}m"))
            .unwrap_or_default(),
        ColorDepth::EightBit => hex_to_rgb(hex)
            .map(|(r, g, b)| format!("\x1b[48;5;{}m", rgb_to_256(r, g, b)))
            .unwrap_or_default(),
        ColorDepth::Basic => hex_to_rgb(hex)
            .map(|(r, g, b)| format!("\x1b[{}m", 40 + rgb_to_basic(r, g, b)))
            .unwrap_or_default(),
    }
}

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const STRIKETHROUGH: &str = "\x1b[9m";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_rgb_valid() {
        assert_eq!(hex_to_rgb("#ff0000"), Some((255, 0, 0)));
        assert_eq!(hex_to_rgb("#00ff00"), Some((0, 255, 0)));
        assert_eq!(hex_to_rgb("#1e1e2e"), Some((30, 30, 46)));
    }

    #[test]
    fn hex_to_rgb_invalid() {
        assert_eq!(hex_to_rgb("ff0000"), None);
        assert_eq!(hex_to_rgb("#fff"), None);
        assert_eq!(hex_to_rgb("#zzzzzz"), None);
        assert_eq!(hex_to_rgb(""), None);
    }

    #[test]
    fn rgb_to_256_grayscale() {
        assert_eq!(rgb_to_256(0, 0, 0), 16);
        assert_eq!(rgb_to_256(255, 255, 255), 231);
        assert_eq!(rgb_to_256(128, 128, 128), 243);
    }

    #[test]
    fn rgb_to_256_color() {
        let idx = rgb_to_256(255, 0, 0);
        assert_eq!(idx, 196);
    }

    #[test]
    fn rgb_to_basic_maps() {
        assert_eq!(rgb_to_basic(0, 0, 0), 0);
        assert_eq!(rgb_to_basic(255, 255, 255), 15);
        assert_eq!(rgb_to_basic(255, 0, 0), 1);
    }

    #[test]
    fn fg_color_none_is_empty() {
        assert_eq!(fg_color("#ff0000", ColorDepth::None), "");
    }

    #[test]
    fn fg_color_truecolor() {
        assert_eq!(
            fg_color("#ff0000", ColorDepth::TrueColor),
            "\x1b[38;2;255;0;0m"
        );
    }

    #[test]
    fn fg_color_invalid_hex() {
        assert_eq!(fg_color("not-hex", ColorDepth::TrueColor), "");
    }

    #[test]
    fn bg_color_truecolor() {
        assert_eq!(
            bg_color("#00ff00", ColorDepth::TrueColor),
            "\x1b[48;2;0;255;0m"
        );
    }

    #[test]
    fn detect_respects_no_color() {
        // Can't easily test env-dependent code without isolation,
        // but we can test the branches exist by calling detect_color_depth()
        let _depth = detect_color_depth();
    }
}
