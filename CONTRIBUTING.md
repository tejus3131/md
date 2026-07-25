# Contributing to `md`

Thanks for helping improve the terminal Markdown renderer.

## Development setup

Requirements:

- Rust **1.85+** (MSRV) — [rustup](https://rustup.rs/) or [mise](https://mise.jdx.dev/)
- Optional: [pre-commit](https://pre-commit.com/)

```bash
git clone https://github.com/tejus3131/md.git
cd md
cargo build
cargo test --all-targets
```

With mise (see `mise.toml`):

```bash
mise install
```

Install git hooks (recommended):

```bash
brew install pre-commit   # or pipx install pre-commit
pre-commit install
```

## Branch policy

`main` is **protected**:

- No direct pushes — open a pull request
- CI must pass before merge (see below)
- Prefer small, focused PRs

### Branch protection checklist (maintainers)

After the first push of `main`, configure GitHub → **Settings → Branches → Add rule** for `main`:

- [ ] Require a pull request before merging
- [ ] Require status checks to pass: `fmt`, `clippy`, `docs`, `test (ubuntu-latest)`, `test (macos-latest)`, `msrv`
- [ ] Require branches to be up to date before merging
- [ ] Do not allow force pushes
- [ ] Do not allow deletions

## What CI runs

| Check | Command |
|-------|---------|
| `fmt` | `cargo fmt --all -- --check` |
| `clippy` | `cargo clippy --all-targets --all-features -- -D warnings` |
| `docs` | `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` |
| `test` | `cargo test --all-targets` (Ubuntu + macOS) |
| `msrv` | `cargo test --all-targets` on Rust 1.85 |

Run the same locally before pushing.

## Code style

- `cargo fmt` before commit
- No new clippy warnings (`-D warnings`)
- Prefer small modules; match existing patterns in `src/`
- Add unit tests next to the code you change

## Themes

Built-in themes live in `themes/`. User themes go in `~/.config/md/themes/`.
Export a starter with `md --dump-theme`.

## Pull requests

1. Fork / branch from latest `main`
2. Make changes + tests
3. Ensure fmt, clippy, docs, and tests pass
4. Open a PR using the template

By contributing, you agree that your contributions will be licensed under the MIT License.
