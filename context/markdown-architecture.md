# Markdown Parser Architecture

This crate implements a Markdown state machine that tokenizes source, resolves constructs, and can either compile events directly to HTML or build an mdast syntax tree.

## Main paths

- `to_html` and `to_html_with_options` stream parser events into the built-in HTML compiler. This path does not construct an AST.
- `to_mdast` builds a `mdast::Node` tree. The `Node` enum includes a transparent `Fragment` variant for detached child lists.
- `Renderer` is the AST-to-output interface. `HTMLRenderer` and `MarkdownRenderer` are built in; a custom renderer controls its own output and escaping behavior.
- AST helpers include traversal, `text_content`, code-fence metadata, `extract_children`, and heading lookup and section operations. Keep these helpers working for fragments as well as document nodes.

The Markdown serializer lives in `src/markdown/`. Keep its construct handlers and safety logic separate from the parser's event-to-HTML compiler. Serializer changes should preserve references, extensions, and formatting options when possible.

`MarkdownOptions::line_wrapping` controls soft source line breaks in text. `Preserve` is the default and keeps round-trip behavior; `Unwrap` joins those breaks with spaces. Explicit hard-break nodes, code, and block boundaries are not unwrapped. The serializer does not currently enforce a line width.

## Tests and generated data

Integration tests are organized by Markdown construct in `tests/`; keep regressions close to the behavior they cover. `tests/commonmark.rs` exercises the CommonMark corpus, and `tests/html_renderer.rs` and `tests/markdown_renderer.rs` cover AST rendering. `tests/markdown_serializer_*.rs` contains the Markdown serializer corpus.

Run formatting, linting, and tests from the workspace root. The standard test workflow requires 100% source-region coverage for the published `socketry-markdown` package across all targets and features, then runs the workspace test task. This runs the package's tests again with default features and covers the private Bake and generator packages, which are outside the coverage gate. The crate exposes optional `json`, `log`, and `serde` features. The `generate/` program refreshes generated CommonMark and Unicode data used by the tests.
