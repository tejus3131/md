use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::layout::measure::visible_width;

pub struct InteractiveViewer {
    lines: Vec<String>,
    scroll_offset: usize,
    terminal_height: u16,
    terminal_width: u16,
    needs_redraw: bool,
}

/// Open `/dev/tty` for direct terminal access (bypasses pipe/redirect).
fn open_tty() -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open("/dev/tty")
}

/// Check if we can access the controlling terminal.
pub fn has_tty() -> bool {
    open_tty().is_ok()
}

impl InteractiveViewer {
    #[must_use]
    pub fn new(content: &str) -> Self {
        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        let (width, height) = terminal::size().unwrap_or((80, 24));
        Self {
            lines,
            scroll_offset: 0,
            terminal_height: height,
            terminal_width: width,
            needs_redraw: true,
        }
    }

    fn view_height(&self) -> usize {
        (self.terminal_height as usize).saturating_sub(1)
    }

    fn max_scroll(&self) -> usize {
        self.lines.len().saturating_sub(self.view_height())
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut tty = open_tty()?;

        terminal::enable_raw_mode()?;
        execute!(tty, EnterAlternateScreen, cursor::Hide)?;

        let result = self.event_loop(&mut tty);

        execute!(tty, cursor::Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        result
    }

    fn event_loop(&mut self, tty: &mut File) -> io::Result<()> {
        let mut last_resize = Instant::now();

        loop {
            if self.needs_redraw {
                self.draw(tty)?;
                self.needs_redraw = false;
            }

            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) => {
                        if self.handle_key(key) {
                            break;
                        }
                    }
                    Event::Resize(w, h) => {
                        let now = Instant::now();
                        if now.duration_since(last_resize) > Duration::from_millis(50) {
                            self.terminal_width = w;
                            self.terminal_height = h;
                            if self.scroll_offset > self.max_scroll() {
                                self.scroll_offset = self.max_scroll();
                            }
                            self.needs_redraw = true;
                            last_resize = now;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,

            KeyCode::Down | KeyCode::Char('j') => {
                if self.scroll_offset < self.max_scroll() {
                    self.scroll_offset += 1;
                    self.needs_redraw = true;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                    self.needs_redraw = true;
                }
            }

            KeyCode::PageDown | KeyCode::Char(' ') => {
                let step = self.view_height().saturating_sub(2);
                self.scroll_offset = (self.scroll_offset + step).min(self.max_scroll());
                self.needs_redraw = true;
            }
            KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let step = self.view_height().saturating_sub(2);
                self.scroll_offset = (self.scroll_offset + step).min(self.max_scroll());
                self.needs_redraw = true;
            }

            KeyCode::PageUp => {
                let step = self.view_height().saturating_sub(2);
                self.scroll_offset = self.scroll_offset.saturating_sub(step);
                self.needs_redraw = true;
            }
            KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let step = self.view_height().saturating_sub(2);
                self.scroll_offset = self.scroll_offset.saturating_sub(step);
                self.needs_redraw = true;
            }

            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let step = self.view_height() / 2;
                self.scroll_offset = (self.scroll_offset + step).min(self.max_scroll());
                self.needs_redraw = true;
            }
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let step = self.view_height() / 2;
                self.scroll_offset = self.scroll_offset.saturating_sub(step);
                self.needs_redraw = true;
            }

            KeyCode::Home | KeyCode::Char('g') => {
                self.scroll_offset = 0;
                self.needs_redraw = true;
            }
            KeyCode::End | KeyCode::Char('G') => {
                self.scroll_offset = self.max_scroll();
                self.needs_redraw = true;
            }

            _ => {}
        }
        false
    }

    fn draw(&self, tty: &mut File) -> io::Result<()> {
        execute!(tty, cursor::MoveTo(0, 0))?;

        let view_h = self.view_height();
        let width = self.terminal_width as usize;

        for i in 0..view_h {
            let line_idx = self.scroll_offset + i;
            execute!(tty, cursor::MoveTo(0, i as u16))?;
            execute!(tty, terminal::Clear(ClearType::CurrentLine))?;

            if line_idx < self.lines.len() {
                let line = &self.lines[line_idx];
                let vis_w = visible_width(line);
                if vis_w > width {
                    write!(tty, "{}", truncate_ansi(line, width))?;
                } else {
                    write!(tty, "{line}")?;
                }
            }
        }

        execute!(
            tty,
            cursor::MoveTo(0, self.terminal_height - 1),
            terminal::Clear(ClearType::CurrentLine)
        )?;

        let progress = if self.lines.len() <= view_h {
            "All".to_string()
        } else if self.scroll_offset == 0 {
            "Top".to_string()
        } else if self.scroll_offset >= self.max_scroll() {
            "End".to_string()
        } else {
            let pct = (self.scroll_offset * 100) / self.max_scroll();
            format!("{pct}%")
        };

        let status = format!(
            " \x1b[7m md \x1b[0m  Lines {}-{} of {}  {progress}  q:quit j/k:scroll ",
            self.scroll_offset + 1,
            (self.scroll_offset + view_h).min(self.lines.len()),
            self.lines.len()
        );
        write!(tty, "{status}")?;

        tty.flush()
    }
}

fn truncate_ansi(s: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut vis_width = 0;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            result.push(c);
            match chars.peek() {
                Some(&'[') => {
                    // CSI sequence: \x1b[...X
                    while let Some(&next) = chars.peek() {
                        result.push(chars.next().unwrap());
                        if (0x40..=0x7E).contains(&(next as u32)) {
                            break;
                        }
                    }
                }
                Some(&']') => {
                    // OSC sequence: \x1b]...ST
                    while let Some(next) = chars.next() {
                        result.push(next);
                        if next == '\x07' {
                            break;
                        }
                        if next == '\x1b' && chars.peek() == Some(&'\\') {
                            result.push(chars.next().unwrap());
                            break;
                        }
                    }
                }
                _ => {
                    if let Some(next) = chars.next() {
                        result.push(next);
                    }
                }
            }
        } else {
            let cw = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
            if vis_width + cw > max_width {
                result.push('…');
                break;
            }
            result.push(c);
            vis_width += cw;
        }
    }

    result
}
