# md — Terminal Markdown Renderer

[![CI](https://github.com/tejus3131/md/actions/workflows/ci.yml/badge.svg)](https://github.com/tejus3131/md/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.88-orange.svg)](https://www.rust-lang.org/)

Render Markdown beautifully in the terminal — GFM fidelity, configurable themes, and a fullscreen interactive viewer.

> **Note:** Not on crates.io yet (the crate name `md` is contested). Install from GitHub or a release binary.

## Install

### One-liner (no Rust toolchain)

```bash
curl -fsSL https://raw.githubusercontent.com/tejus3131/md/main/install.sh | bash
```

Installs the latest release binary into `~/.local/bin` (ensure that directory is on your `PATH`).

### From GitHub with Cargo

```bash
cargo install --git https://github.com/tejus3131/md --locked
```

### From a local clone

```bash
git clone https://github.com/tejus3131/md.git
cd md
cargo install --path . --locked
```

## Features

- **GFM-compliant** — tables, task lists, strikethrough, autolinks, footnotes, alerts
- **Syntax-highlighted code blocks** — syntect + GitHub-matching themes; **line numbers on by default**
- **Inline HTML** — `<em>`, `<strong>`, `<code>`, `<del>`, `<a href>`, `<br>` rendered natively
- **6 built-in themes** — Catppuccin Mocha (default), Catppuccin Latte, GitHub Dark, GitHub Light, Dracula, Nord
- **Custom themes** — drop a TOML file in `~/.config/md/themes/`
- **Interactive viewer** — fullscreen alt screen; scroll with j/k, arrows, PgUp/PgDn; resize-aware
- **Pipe-safe** — `echo '...' | md` opens the same fullscreen viewer when a TTY is available
- **Color downsampling** — truecolor → 256 → 16 → none; respects `NO_COLOR`
- **OSC-8 hyperlinks** — clickable links in iTerm2, WezTerm, Kitty, and similar terminals

## Usage

```bash
md                              # no file + TTY → print help
md README.md                    # render file (centered, interactive)
cat README.md | md              # pipe → same interactive viewer when possible
md --left README.md             # left-align
md --theme github-dark README.md
md --list-themes
md --dump-theme > my-theme.toml
md --width 120 README.md
md --no-line-numbers README.md  # hide code line numbers (on by default)
md --pager never README.md      # skip interactive viewer
md --raw README.md              # shorthand for --pager never
md --no-color README.md
md --version                    # version + build info
```

Pager modes (`-p` / `--pager`): `auto` (default), `always`, `never`.

## Keybindings (interactive mode)

| Key | Action |
|-----|--------|
| `j` / `↓` | Scroll down |
| `k` / `↑` | Scroll up |
| `Space` / `PgDn` | Page down |
| `Ctrl-f` | Page down |
| `PgUp` / `Ctrl-b` | Page up |
| `g` / `Home` | Top |
| `G` / `End` | Bottom |
| `Ctrl-d` | Half page down |
| `Ctrl-u` | Half page up |
| `q` / `Esc` | Quit |

## Themes

Built-in: `catppuccin-mocha`, `catppuccin-latte`, `github-dark`, `github-light`, `dracula`, `nord`

Custom themes go in `~/.config/md/themes/`. Starter:

```bash
mkdir -p ~/.config/md/themes
md --dump-theme > ~/.config/md/themes/my-theme.toml
```

## Configuration

`~/.config/md/config.toml`:

```toml
theme = "catppuccin-mocha"
max_width = 100
pager = "auto"         # auto | always | never
line_numbers = true
```

Priority: CLI flags > config file > theme defaults.

## HTML support

| Tag | Rendering |
|-----|-----------|
| `<em>`, `<i>` | Italic |
| `<strong>`, `<b>` | Bold |
| `<del>`, `<s>` | Strikethrough |
| `<code>` | Dimmed |
| `<a href="...">` | Clickable link (OSC-8) |
| `<br>`, `<br/>` | Line break |

Unknown tags are shown dimmed.

## Architecture

```
src/
├── main.rs           # Entry, orchestration, pipe/TTY handling
├── cli.rs            # clap CLI
├── config.rs         # ~/.config/md/config.toml
├── error.rs          # thiserror types
├── parser/           # comrak Markdown → AST
├── renderer/         # AST → ANSI
├── layout/           # width, wrap, measure
├── terminal/         # colors, pager, interactive viewer
└── theme/            # TOML theme loader
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). `main` is protected — changes go through PRs; CI (`fmt`, `clippy`, `docs`, `test`, `msrv`) must pass.

Please follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## License

[MIT](LICENSE)
