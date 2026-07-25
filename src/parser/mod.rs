pub mod extensions;

use comrak::{Arena, Options, nodes::AstNode, parse_document};

#[must_use]
pub fn default_options() -> Options<'static> {
    let mut opts = Options::default();
    opts.extension.strikethrough = true;
    opts.extension.table = true;
    opts.extension.autolink = true;
    opts.extension.tasklist = true;
    opts.extension.footnotes = true;
    opts.extension.header_ids = None;
    opts.parse.smart = true;
    opts
}

pub fn parse<'a>(arena: &'a Arena<AstNode<'a>>, input: &str) -> &'a AstNode<'a> {
    let opts = default_options();
    parse_document(arena, input, &opts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use comrak::nodes::NodeValue;

    #[test]
    fn parse_simple_paragraph() {
        let arena = Arena::new();
        let root = parse(&arena, "Hello world");
        let first_child = root.first_child().expect("should have a child");
        let data = first_child.data.borrow();
        assert!(matches!(data.value, NodeValue::Paragraph));
    }

    #[test]
    fn parse_heading() {
        let arena = Arena::new();
        let root = parse(&arena, "# Title");
        let first_child = root.first_child().expect("should have a child");
        let data = first_child.data.borrow();
        assert!(matches!(data.value, NodeValue::Heading(..)));
    }

    #[test]
    fn parse_code_block() {
        let arena = Arena::new();
        let root = parse(&arena, "```rust\nfn main() {}\n```");
        let first_child = root.first_child().expect("should have a child");
        let data = first_child.data.borrow();
        assert!(matches!(data.value, NodeValue::CodeBlock(..)));
    }

    #[test]
    fn parse_table() {
        let arena = Arena::new();
        let md = "| a | b |\n|---|---|\n| 1 | 2 |";
        let root = parse(&arena, md);
        let first_child = root.first_child().expect("should have a child");
        let data = first_child.data.borrow();
        assert!(matches!(data.value, NodeValue::Table(..)));
    }

    #[test]
    fn parse_tasklist() {
        let arena = Arena::new();
        let md = "- [x] done\n- [ ] todo";
        let root = parse(&arena, md);
        // Should parse without panicking
        let _first = root.first_child().expect("should have list");
    }

    #[test]
    fn default_options_enables_extensions() {
        let opts = default_options();
        assert!(opts.extension.strikethrough);
        assert!(opts.extension.table);
        assert!(opts.extension.autolink);
        assert!(opts.extension.tasklist);
        assert!(opts.extension.footnotes);
        assert!(opts.parse.smart);
    }
}
