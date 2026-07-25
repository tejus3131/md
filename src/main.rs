mod cli;
mod config;
mod error;
mod layout;
mod parser;
mod renderer;
mod terminal;
mod theme;

use std::io::Read;
use std::process;

use clap::Parser;
use comrak::Arena;

use cli::Cli;
use config::Config;
use error::Error;
use layout::LayoutConfig;
use renderer::DocumentContext;
use terminal::TerminalInfo;
use terminal::pager::{self, PagerMode, write_stdout};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run() -> error::Result<()> {
    let args = Cli::parse();

    // No file and stdin is a tty → show help
    if args.file.is_none()
        && !args.list_themes
        && !args.dump_theme
        && std::io::IsTerminal::is_terminal(&std::io::stdin())
    {
        use clap::CommandFactory;
        Cli::command().print_help().ok();
        println!();
        return Ok(());
    }

    let config = Config::load()?;
    let term = TerminalInfo::detect();

    // --list-themes
    if args.list_themes {
        let themes = theme::list_themes();
        println!("Available themes:\n");
        for name in &themes {
            println!("  {name}");
        }
        if let Some(dir) = theme::user_themes_dir() {
            println!("\nCustom themes directory: {}", dir.display());
        }
        return Ok(());
    }

    // Resolve theme
    let theme_name = args
        .theme
        .as_deref()
        .or(config.theme.as_deref())
        .unwrap_or("catppuccin-mocha");

    let mut active_theme = theme::load_theme(theme_name)?;

    // Line numbers on by default; --no-line-numbers or config overrides
    if args.no_line_numbers {
        active_theme.code.line_numbers = false;
    } else if let Some(ln) = config.line_numbers {
        active_theme.code.line_numbers = ln;
    } else {
        active_theme.code.line_numbers = true;
    }

    // --dump-theme
    if args.dump_theme {
        let s = theme::dump_theme(&active_theme)?;
        print!("{s}");
        return Ok(());
    }

    // If reading from stdin pipe, swap fd 0 to /dev/tty BEFORE any
    // library touches stdin. Read pipe content from dup'd fd instead.
    #[cfg(unix)]
    let piped_stdin: Option<std::fs::File> = {
        if args.file.is_none() && !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
            use std::os::unix::io::FromRawFd;
            let pipe_fd = unsafe { libc::dup(libc::STDIN_FILENO) };
            if pipe_fd >= 0 {
                reopen_stdin_tty();
                Some(unsafe { std::fs::File::from_raw_fd(pipe_fd) })
            } else {
                None
            }
        } else {
            None
        }
    };
    #[cfg(not(unix))]
    let piped_stdin: Option<std::fs::File> = None;

    // Read input
    let input = if let Some(mut pipe) = piped_stdin {
        let mut buf = String::new();
        pipe.read_to_string(&mut buf).map_err(Error::ReadStdin)?;
        Ok(buf)
    } else {
        read_input(&args.file)
    }?;

    // Color depth
    let color_depth = if args.no_color {
        terminal::colors::ColorDepth::None
    } else {
        term.color_depth
    };

    // Layout
    let max_width = if args.width != 100 {
        Some(args.width)
    } else {
        config.max_width.or(Some(100))
    };
    let layout = LayoutConfig::new(term.width, max_width, !args.left);

    // Parse
    let arena = Arena::new();
    let root = parser::parse(&arena, &input);

    // Render
    let ctx = DocumentContext::new(&active_theme, &layout, color_depth);
    let output = ctx.render(root);

    // Output
    let pager_mode = if args.raw {
        PagerMode::Never
    } else if args.pager != PagerMode::Auto {
        args.pager
    } else {
        config
            .pager
            .as_deref()
            .and_then(pager::parse_pager_mode)
            .unwrap_or(PagerMode::Auto)
    };

    let can_view = terminal::viewer::has_tty();
    let should_interactive = match pager_mode {
        PagerMode::Always => can_view,
        PagerMode::Never => false,
        PagerMode::Auto => can_view,
    };

    if should_interactive {
        let mut viewer = terminal::viewer::InteractiveViewer::new(&output);
        match viewer.run() {
            Ok(()) => return Ok(()),
            Err(e) => {
                eprintln!("md: viewer failed ({e}), falling back to stdout");
            }
        }
    }

    // Ensure trailing newline to avoid zsh `%` marker
    let out = if output.ends_with('\n') {
        output
    } else {
        format!("{output}\n")
    };

    write_stdout(&out).map_err(|e| {
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            process::exit(0);
        }
        Error::WriteOutput(e)
    })
}

/// Reopen stdin (fd 0) from `/dev/tty` so crossterm can read keyboard
/// events after the original stdin was consumed by a pipe.
#[cfg(unix)]
fn reopen_stdin_tty() {
    use std::os::unix::io::AsRawFd;
    if let Ok(tty) = std::fs::File::open("/dev/tty") {
        unsafe {
            libc::dup2(tty.as_raw_fd(), libc::STDIN_FILENO);
        }
    }
}

fn read_input(file: &Option<String>) -> error::Result<String> {
    match file.as_deref() {
        None | Some("-") => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .map_err(Error::ReadStdin)?;
            Ok(buf)
        }
        Some(path) => std::fs::read_to_string(path).map_err(|e| Error::ReadFile {
            path: path.into(),
            source: e,
        }),
    }
}
