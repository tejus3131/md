# Heading Level 1

## Heading Level 2

### Heading Level 3

#### Heading Level 4

##### Heading Level 5

###### Heading Level 6

---

## Inline Formatting

This is **bold text**, *italic text*, and ***bold italic***.

This is ~~strikethrough text~~.

This has `inline code` in a sentence.

---

## Links & Images

[GitHub](https://github.com)

[Example with path](https://example.com/path?q=hello&lang=en)

![Placeholder image](https://via.placeholder.com/150)

---

## Blockquotes

> Simple blockquote

> Multi-line blockquote
> that spans two lines.

> > Nested blockquote

---

## GitHub Alerts

> [!NOTE]
> This is a note alert.

> [!TIP]
> This is a tip alert.

> [!IMPORTANT]
> This is an important alert.

> [!WARNING]
> This is a warning alert.

> [!CAUTION]
> This is a caution alert.

---

## Unordered Lists

- Item one
- Item two
  - Nested item A
  - Nested item B
    - Deep nested
- Item three

## Ordered Lists

1. First item
2. Second item
3. Third item
   1. Sub-item
   2. Sub-item

## Task Lists

- [x] Completed task
- [ ] Pending task
- [x] Another done
- [ ] Still to do

---

## Code Blocks

Inline: `let x = 42;`

```rust
fn main() {
    let greeting = "Hello, world!";
    println!("{}", greeting);

    for i in 0..5 {
        println!("Count: {}", i);
    }
}
```

```python
def fibonacci(n):
    """Calculate fibonacci number."""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

for i in range(10):
    print(f"fib({i}) = {fibonacci(i)}")
```

```javascript
const fetchData = async (url) => {
  try {
    const response = await fetch(url);
    const data = await response.json();
    return data;
  } catch (error) {
    console.error("Failed:", error);
  }
};
```

```
Plain code block with no language specified.
Just raw text inside fences.
```

---

## Tables

| Language   | Typing     | Speed   |
|------------|------------|---------|
| Rust       | Static     | Fast    |
| Python     | Dynamic    | Moderate|
| JavaScript | Dynamic    | Fast    |
| Go         | Static     | Fast    |

| Left | Center | Right |
|:-----|:------:|------:|
| 1    |   2    |     3 |
| foo  |  bar   |   baz |

---

## Horizontal Rules

Above the rule.

---

Below the rule.

***

Another rule above.

---

## Footnotes

Here is a sentence with a footnote[^1].

And another[^note].

[^1]: This is the first footnote.
[^note]: This is a named footnote.

---

## HTML Inline

This has <em>italic HTML</em> and <strong>bold HTML</strong>.

This has <i>italic shorthand</i> and <b>bold shorthand</b>.

This is <del>deleted</del> and <s>struck</s>.

This has <code>inline code</code> in HTML.

This has a <a href="https://github.com">GitHub link</a> via HTML.

Line one.<br>Line two after br.

---

## Long Paragraph (Word Wrap Test)

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

---

## Mixed Content

Here is a paragraph with **bold**, *italic*, `code`, ~~struck~~, and a [link](https://example.com) all in one line.

> A blockquote with **bold** and `code` inside.

- List with `inline code`
- List with **bold text**
- List with [a link](https://example.com)

| Feature       | Status |
|---------------|--------|
| `code blocks` | done   |
| **bold**      | done   |
| *italic*      | done   |
