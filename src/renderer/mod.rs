pub mod block;
pub mod code;
pub mod heading;
pub mod html;
pub mod table;
pub mod text;

use comrak::nodes::{AstNode, ListType, NodeValue};

use crate::layout::LayoutConfig;
use crate::parser::extensions;
use crate::terminal::colors::{self, ColorDepth};
use crate::theme::style::Theme;

/// Immutable document-level context shared during rendering.
pub struct DocumentContext<'a> {
    pub theme: &'a Theme,
    pub layout: &'a LayoutConfig,
    pub depth: ColorDepth,
}

/// Mutable per-render state (list stacks, table accumulation, etc.)
struct RenderState {
    list_stack: Vec<ListContext>,
    blockquote_depth: usize,
    table_rows: Vec<Vec<String>>,
    in_table: bool,
    current_row: Vec<String>,
    current_cell: String,
    in_table_cell: bool,
}

struct ListContext {
    list_type: ListType,
    item_number: usize,
    nesting: usize,
}

impl<'a> DocumentContext<'a> {
    #[must_use]
    pub fn new(theme: &'a Theme, layout: &'a LayoutConfig, depth: ColorDepth) -> Self {
        Self {
            theme,
            layout,
            depth,
        }
    }

    #[must_use]
    pub fn render(&self, root: &'a AstNode<'a>) -> String {
        let mut state = RenderState {
            list_stack: Vec::new(),
            blockquote_depth: 0,
            table_rows: Vec::new(),
            in_table: false,
            current_row: Vec::new(),
            current_cell: String::new(),
            in_table_cell: false,
        };
        let mut output = String::new();
        self.render_node(root, &mut output, &mut state);

        self.layout.center_content(&output)
    }

    fn render_node(&self, node: &'a AstNode<'a>, out: &mut String, state: &mut RenderState) {
        use std::cell::Ref;

        enum Action {
            Children,
            Heading(u8),
            Paragraph,
            Text(String),
            SoftBreak,
            LineBreak,
            InlineCode(String),
            Strong,
            Emph,
            Strike,
            Link { url: String },
            Image,
            CodeBlock { literal: String, lang: String },
            BlockQuote,
            List { list_type: ListType, start: usize },
            Item,
            TaskItem(bool),
            ThematicBreak,
            Table,
            TableRow,
            TableCell,
            FootnoteDef(String),
            FootnoteRef(String),
            HtmlBlock(String),
            HtmlInline(String),
        }

        let action = {
            let data: Ref<'_, _> = node.data.borrow();
            match &data.value {
                NodeValue::Document => Action::Children,
                NodeValue::Heading(h) => Action::Heading(h.level),
                NodeValue::Paragraph => Action::Paragraph,
                NodeValue::Text(t) => Action::Text(t.clone()),
                NodeValue::SoftBreak => Action::SoftBreak,
                NodeValue::LineBreak => Action::LineBreak,
                NodeValue::Code(c) => Action::InlineCode(c.literal.clone()),
                NodeValue::Strong => Action::Strong,
                NodeValue::Emph => Action::Emph,
                NodeValue::Strikethrough => Action::Strike,
                NodeValue::Link(link) => Action::Link {
                    url: link.url.clone(),
                },
                NodeValue::Image(_) => Action::Image,
                NodeValue::CodeBlock(cb) => Action::CodeBlock {
                    literal: cb.literal.clone(),
                    lang: cb.info.split_whitespace().next().unwrap_or("").to_string(),
                },
                NodeValue::BlockQuote => Action::BlockQuote,
                NodeValue::List(nl) => Action::List {
                    list_type: nl.list_type,
                    start: nl.start,
                },
                NodeValue::Item(_) => Action::Item,
                NodeValue::TaskItem(symbol) => Action::TaskItem(symbol.is_some()),
                NodeValue::ThematicBreak => Action::ThematicBreak,
                NodeValue::Table(..) => Action::Table,
                NodeValue::TableRow(..) => Action::TableRow,
                NodeValue::TableCell => Action::TableCell,
                NodeValue::FootnoteDefinition(nfd) => Action::FootnoteDef(nfd.name.clone()),
                NodeValue::FootnoteReference(nfr) => Action::FootnoteRef(nfr.name.clone()),
                NodeValue::HtmlBlock(hb) => Action::HtmlBlock(hb.literal.clone()),
                NodeValue::HtmlInline(h) => Action::HtmlInline(h.clone()),
                _ => Action::Children,
            }
        };

        match action {
            Action::Children => self.render_children(node, out, state),

            Action::Heading(level) => {
                let mut heading_text = String::new();
                self.collect_inline_text(node, &mut heading_text);
                out.push_str(&heading::render_heading(
                    level,
                    &heading_text,
                    self.theme,
                    self.layout.content_width(),
                    self.depth,
                ));
            }

            Action::Paragraph => self.render_paragraph(node, out, state),

            Action::Text(t) => self.render_text(&t, out, state),

            Action::SoftBreak => self.push_to_target(out, state, " "),

            Action::LineBreak => {
                if state.in_table_cell {
                    state.current_cell.push(' ');
                } else {
                    out.push('\n');
                }
            }

            Action::InlineCode(literal) => {
                let rendered = text::render_inline_code(&literal, self.theme, self.depth);
                self.push_to_target(out, state, &rendered);
            }

            Action::Strong => {
                let bold = text::apply_bold();
                self.wrap_inline(node, out, state, bold, colors::RESET);
            }

            Action::Emph => {
                let italic = text::apply_italic();
                self.wrap_inline(node, out, state, italic, colors::RESET);
            }

            Action::Strike => {
                let strike = text::apply_strikethrough();
                self.wrap_inline(node, out, state, strike, colors::RESET);
            }

            Action::Link { url } => {
                let mut link_text = String::new();
                self.collect_plain_text(node, &mut link_text);
                let rendered = text::render_link(&link_text, &url, self.theme, self.depth);
                self.push_to_target(out, state, &rendered);
            }

            Action::Image => {
                let mut alt = String::new();
                self.collect_plain_text(node, &mut alt);
                let rendered = text::render_image(&alt, self.theme, self.depth);
                self.push_to_target(out, state, &rendered);
            }

            Action::CodeBlock { literal, lang } => {
                out.push_str(&code::render_code_block(
                    &literal,
                    &lang,
                    self.theme,
                    self.layout.content_width(),
                    self.depth,
                ));
            }

            Action::BlockQuote => {
                state.blockquote_depth += 1;
                self.render_children(node, out, state);
                state.blockquote_depth -= 1;
            }

            Action::List { list_type, start } => {
                let nesting = state.list_stack.len();
                state.list_stack.push(ListContext {
                    list_type,
                    item_number: start,
                    nesting,
                });
                self.render_children(node, out, state);
                state.list_stack.pop();
                if nesting == 0 {
                    out.push('\n');
                }
            }

            Action::Item => self.render_list_item(node, out, state),

            Action::TaskItem(checked) => self.render_task_item(node, out, state, checked),

            Action::ThematicBreak => {
                out.push_str(&block::render_hr(
                    self.theme,
                    self.layout.content_width(),
                    self.depth,
                ));
            }

            Action::Table => {
                state.in_table = true;
                state.table_rows.clear();
                self.render_children(node, out, state);
                state.in_table = false;
                let rows = std::mem::take(&mut state.table_rows);
                out.push_str(&table::render_table(
                    &rows,
                    self.theme,
                    self.layout.content_width(),
                    self.depth,
                ));
            }

            Action::TableRow => {
                state.current_row.clear();
                self.render_children(node, out, state);
                state
                    .table_rows
                    .push(std::mem::take(&mut state.current_row));
            }

            Action::TableCell => {
                state.current_cell.clear();
                state.in_table_cell = true;
                self.render_children(node, out, state);
                state.in_table_cell = false;
                state
                    .current_row
                    .push(std::mem::take(&mut state.current_cell));
            }

            Action::FootnoteDef(name) => {
                let mut body = String::new();
                self.render_children(node, &mut body, state);
                let trimmed = body.trim();
                let link_color = colors::fg_color(
                    &self
                        .theme
                        .resolve_color(self.theme.link.text.as_deref().unwrap_or("#89b4fa")),
                    self.depth,
                );
                out.push_str(&format!(
                    "\n{link_color}[{name}]{} {trimmed}\n",
                    colors::RESET
                ));
            }

            Action::FootnoteRef(name) => {
                let mut s = String::new();
                if let Some(ref fg) = self.theme.link.text {
                    s.push_str(&colors::fg_color(&self.theme.resolve_color(fg), self.depth));
                }
                s.push_str(&format!("[{name}]"));
                s.push_str(colors::RESET);
                self.push_to_target(out, state, &s);
            }

            Action::HtmlBlock(literal) => {
                let rendered = html::render_html_block(&literal, self.theme, self.depth);
                out.push_str(&rendered);
            }

            Action::HtmlInline(h) => {
                let rendered = html::render_html_inline(&h, self.theme, self.depth);
                self.push_to_target(out, state, &rendered);
            }
        }
    }

    fn render_paragraph(&self, node: &'a AstNode<'a>, out: &mut String, state: &mut RenderState) {
        use crate::layout::wrap::wrap_text;

        if state.in_table_cell {
            self.render_children(node, out, state);
            return;
        }

        let bq_prefix = if state.blockquote_depth > 0 {
            block::blockquote_prefix(self.theme, self.depth, state.blockquote_depth)
        } else {
            String::new()
        };

        let mut para_text = String::new();
        self.render_children(node, &mut para_text, state);

        let wrapped = wrap_text(&para_text, self.layout.content_width());

        if bq_prefix.is_empty() {
            out.push_str(&wrapped);
            out.push('\n');
        } else {
            for line in wrapped.lines() {
                out.push_str(&bq_prefix);
                out.push_str(line);
                out.push('\n');
            }
        }
        out.push('\n');
    }

    fn render_text(&self, t: &str, out: &mut String, state: &mut RenderState) {
        if state.in_table_cell {
            state.current_cell.push_str(t);
            return;
        }

        if state.blockquote_depth > 0 {
            if let Some(alert) = extensions::detect_alert(t) {
                let color = match alert {
                    extensions::AlertKind::Note => &self.theme.alert.note_color,
                    extensions::AlertKind::Tip => &self.theme.alert.tip_color,
                    extensions::AlertKind::Important => &self.theme.alert.important_color,
                    extensions::AlertKind::Warning => &self.theme.alert.warning_color,
                    extensions::AlertKind::Caution => &self.theme.alert.caution_color,
                };
                if let Some(c) = color {
                    out.push_str(&colors::fg_color(&self.theme.resolve_color(c), self.depth));
                }
                out.push_str(colors::BOLD);
                out.push_str(alert.icon());
                out.push(' ');
                out.push_str(alert.label());
                out.push_str(colors::RESET);
                return;
            }
        }

        out.push_str(t);
    }

    fn render_list_item(&self, node: &'a AstNode<'a>, out: &mut String, state: &mut RenderState) {
        let (list_type, number, nesting) = if let Some(ctx) = state.list_stack.last_mut() {
            let n = ctx.item_number;
            ctx.item_number += 1;
            (ctx.list_type, n, ctx.nesting)
        } else {
            (ListType::Bullet, 1, 0)
        };

        match list_type {
            ListType::Bullet => out.push_str(&block::list_bullet(self.theme, self.depth, nesting)),
            ListType::Ordered => {
                out.push_str(&block::list_number(self.theme, self.depth, nesting, number))
            }
        }

        let mut item_text = String::new();
        self.render_children(node, &mut item_text, state);
        out.push_str(item_text.trim_end());
        out.push('\n');
    }

    fn render_task_item(
        &self,
        node: &'a AstNode<'a>,
        out: &mut String,
        state: &mut RenderState,
        checked: bool,
    ) {
        let nesting = state.list_stack.last().map_or(0, |c| c.nesting);
        out.push_str(&block::list_bullet(self.theme, self.depth, nesting));
        out.push_str(&block::task_checkbox(self.theme, self.depth, checked));

        if let Some(ctx) = state.list_stack.last_mut() {
            ctx.item_number += 1;
        }

        let mut item_text = String::new();
        self.render_children(node, &mut item_text, state);
        out.push_str(item_text.trim_end());
        out.push('\n');
    }

    /// Push content to either the table cell or the main output buffer.
    fn push_to_target(&self, out: &mut String, state: &mut RenderState, content: &str) {
        if state.in_table_cell {
            state.current_cell.push_str(content);
        } else {
            out.push_str(content);
        }
    }

    /// Wrap children with a prefix/suffix (e.g. bold start / reset).
    fn wrap_inline(
        &self,
        node: &'a AstNode<'a>,
        out: &mut String,
        state: &mut RenderState,
        prefix: &str,
        suffix: &str,
    ) {
        if state.in_table_cell {
            state.current_cell.push_str(prefix);
            self.render_children(node, out, state);
            state.current_cell.push_str(suffix);
        } else {
            out.push_str(prefix);
            self.render_children(node, out, state);
            out.push_str(suffix);
        }
    }

    fn render_children(&self, node: &'a AstNode<'a>, out: &mut String, state: &mut RenderState) {
        for child in node.children() {
            self.render_node(child, out, state);
        }
    }

    fn collect_inline_text(&self, node: &'a AstNode<'a>, out: &mut String) {
        for child in node.children() {
            enum InlineAction {
                Text(String),
                Code(String),
                Break,
                Bold,
                Italic,
                Other,
            }
            let action = {
                let data = child.data.borrow();
                match &data.value {
                    NodeValue::Text(t) => InlineAction::Text(t.clone()),
                    NodeValue::Code(c) => InlineAction::Code(c.literal.clone()),
                    NodeValue::SoftBreak | NodeValue::LineBreak => InlineAction::Break,
                    NodeValue::Strong => InlineAction::Bold,
                    NodeValue::Emph => InlineAction::Italic,
                    _ => InlineAction::Other,
                }
            };
            match action {
                InlineAction::Text(t) => out.push_str(&t),
                InlineAction::Code(lit) => {
                    out.push_str(&text::render_inline_code(&lit, self.theme, self.depth));
                }
                InlineAction::Break => out.push(' '),
                InlineAction::Bold => {
                    out.push_str(text::apply_bold());
                    self.collect_inline_text(child, out);
                    out.push_str(colors::RESET);
                }
                InlineAction::Italic => {
                    out.push_str(text::apply_italic());
                    self.collect_inline_text(child, out);
                    out.push_str(colors::RESET);
                }
                InlineAction::Other => self.collect_inline_text(child, out),
            }
        }
    }

    fn collect_plain_text(&self, node: &'a AstNode<'a>, out: &mut String) {
        for child in node.children() {
            enum PlainAction {
                Text(String),
                Code(String),
                Break,
                Other,
            }
            let action = {
                let data = child.data.borrow();
                match &data.value {
                    NodeValue::Text(t) => PlainAction::Text(t.clone()),
                    NodeValue::Code(c) => PlainAction::Code(c.literal.clone()),
                    NodeValue::SoftBreak | NodeValue::LineBreak => PlainAction::Break,
                    _ => PlainAction::Other,
                }
            };
            match action {
                PlainAction::Text(t) => out.push_str(&t),
                PlainAction::Code(lit) => out.push_str(&lit),
                PlainAction::Break => out.push(' '),
                PlainAction::Other => self.collect_plain_text(child, out),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutConfig;
    use crate::theme;
    use comrak::Arena;

    fn render_md(md: &str) -> String {
        let arena = Arena::new();
        let root = crate::parser::parse(&arena, md);
        let theme = theme::default_theme();
        let layout = LayoutConfig::new(80, Some(80), false);
        let ctx = DocumentContext::new(&theme, &layout, ColorDepth::None);
        ctx.render(root)
    }

    #[test]
    fn renders_heading() {
        let out = render_md("# Hello");
        assert!(out.contains("Hello"));
    }

    #[test]
    fn renders_paragraph() {
        let out = render_md("Hello world");
        assert!(out.contains("Hello world"));
    }

    #[test]
    fn renders_bold() {
        let out = render_md("**bold**");
        assert!(out.contains("bold"));
    }

    #[test]
    fn renders_italic() {
        let out = render_md("*italic*");
        assert!(out.contains("italic"));
    }

    #[test]
    fn renders_inline_code() {
        let out = render_md("`code`");
        assert!(out.contains("code"));
    }

    #[test]
    fn renders_link() {
        let out = render_md("[GitHub](https://github.com)");
        assert!(out.contains("GitHub"));
        assert!(out.contains("github.com"));
    }

    #[test]
    fn renders_code_block() {
        let out = render_md("```rust\nfn main() {}\n```");
        assert!(out.contains("fn"));
        assert!(out.contains("main"));
    }

    #[test]
    fn renders_list() {
        let out = render_md("- item 1\n- item 2");
        assert!(out.contains("item 1"));
        assert!(out.contains("item 2"));
    }

    #[test]
    fn renders_ordered_list() {
        let out = render_md("1. first\n2. second");
        assert!(out.contains("first"));
        assert!(out.contains("second"));
    }

    #[test]
    fn renders_task_list() {
        let out = render_md("- [x] done\n- [ ] todo");
        assert!(out.contains("done"));
        assert!(out.contains("todo"));
    }

    #[test]
    fn renders_table() {
        let out = render_md("| a | b |\n|---|---|\n| 1 | 2 |");
        assert!(out.contains("a"));
        assert!(out.contains("2"));
    }

    #[test]
    fn renders_blockquote() {
        let out = render_md("> quote");
        assert!(out.contains("quote"));
    }

    #[test]
    fn renders_hr() {
        let out = render_md("---");
        assert!(out.contains('─'));
    }

    #[test]
    fn renders_image() {
        let out = render_md("![alt text](image.png)");
        assert!(out.contains("alt text"));
    }

    #[test]
    fn renders_strikethrough() {
        let out = render_md("~~struck~~");
        assert!(out.contains("struck"));
    }

    #[test]
    fn renders_html_em_as_italic() {
        let out = render_md("This has <em>italic</em> text.");
        assert!(out.contains("\x1b[3m"));
        assert!(out.contains("italic"));
    }

    #[test]
    fn renders_html_strong_as_bold() {
        let out = render_md("This has <strong>bold</strong> text.");
        assert!(out.contains("\x1b[1m"));
        assert!(out.contains("bold"));
    }

    #[test]
    fn renders_html_a_with_osc8() {
        let out = render_md(r#"Click <a href="https://example.com">here</a>."#);
        assert!(out.contains("here"));
        assert!(out.contains("\x1b]8;;https://example.com\x1b\\"));
    }
}
