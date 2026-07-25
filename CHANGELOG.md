# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Bump comrak 0.36 → 0.54, crossterm 0.28 → 0.29, toml 0.8 → 1
- CI: explicit `permissions: contents: read`; bump Actions (checkout v7, artifacts, gh-release v3)
- MSRV job pins via `toolchain: 1.88` on `dtolnay/rust-toolchain@stable` (avoids bad Dependabot bumps)

## [0.2.0] - 2026-07-25

### Added

- GFM-compliant Markdown rendering (tables, task lists, strikethrough, footnotes, alerts)
- Syntax-highlighted code blocks via syntect with GitHub-matching themes
- Six built-in themes: Catppuccin Mocha (default), Catppuccin Latte, GitHub Dark/Light, Dracula, Nord
- Custom TOML themes under `~/.config/md/themes/`
- Interactive fullscreen viewer with vim-style keys and resize support
- Pipe-safe input: `dup(STDIN)` + `/dev/tty` reopen; crossterm `use-dev-tty` for macOS
- Inline HTML: `<em>`, `<i>`, `<strong>`, `<b>`, `<del>`, `<s>`, `<code>`, `<a href>`, `<br>`
- OSC-8 hyperlinks in supported terminals
- Color depth degradation: truecolor → 256 → 16 → none (`NO_COLOR` respected)
- Line numbers in code blocks on by default; `--no-line-numbers` to disable
- `--raw` shorthand for `--pager never`
- Empty argv with TTY stdin shows help instead of hanging
- `--version` long form includes git build info when available

### Fixed

- Viewer PageDown / Ctrl-F / Ctrl-B key guards (bare `f`/`b` no longer page-scroll)
- Global wrap no longer re-wraps code block borders
- `content_width()` clamped to minimum 10 to avoid underflow
- Syntax highlighting respects `ColorDepth`
- OSC sequences handled in viewer truncate
- Invalid `--pager` values rejected at CLI parse time

## [Unreleased]

[0.2.0]: https://github.com/tejus3131/md/releases/tag/v0.2.0
[Unreleased]: https://github.com/tejus3131/md/compare/v0.2.0...HEAD
