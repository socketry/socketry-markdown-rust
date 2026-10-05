# `socketry-markdown`

A CommonMark-compliant Markdown parser for Rust with an AST, extensions, and HTML and Markdown renderers. It is based on [`markdown-rs`](https://github.com/wooorm/markdown-rs) and adds Socketry's AST helpers and renderer APIs.

[![Build][badge-build-image]][badge-build-url] [![Coverage][badge-coverage-image]][badge-coverage-url]

## Motivation

Socketry's documentation tooling needs to inspect, edit, and render Markdown sections as structured data. This fork builds on `markdown-rs`'s CommonMark parser with AST editing helpers, reusable renderers, and optional syntax extensions for richer documentation.

The fork adds three parsing options to `ParseOptions`, all disabled by default so ordinary parsing retains CommonMark behavior:

- `inline_code_info` recognizes a language prefix before inline code, such as `` rust:`code` ``. It removes the prefix from the surrounding text, stores the language on the AST node, and emits a `language-rust` class when rendering HTML.
- `html_block_blank_lines` lets ordinary HTML blocks continue across blank lines when subsequent content maintains consistent indentation. This keeps indented HTML content together instead of ending the block at the first blank line.
- `html_tag_namespaces` recognizes namespace-prefixed HTML tags, such as `<svg:circle />`, in both block and inline HTML.

Beyond parsing, the fork adds AST traversal, text extraction, heading and section editing, and `Fragment` nodes for working with detached content. `HTMLRenderer`, `MarkdownRenderer`, and the custom `Renderer` interface render edited trees; Markdown serialization preserves language metadata and can unwrap soft line breaks. Optional `CompileOptions::heading_ids` generates unique heading anchors for in-page links and tables of contents.

The fork also includes fixes for stale MDX parser errors and parsing and serialization edge cases, including CRLF handling in inline code and math. See [releases.md](releases.md) for changes and the [API documentation](https://docs.rs/socketry-markdown/latest/socketry_markdown/) for configuration details.

## Usage

### Install

```sh
cargo add socketry-markdown
```

The library supports Rust 1.73 and later.

### Render HTML

For direct conversion from Markdown source to HTML:

```rust
let html = socketry_markdown::to_html("## Hello, *world*!");
assert_eq!(html, "<h2>Hello, <em>world</em>!</h2>");
```

Enable GFM constructs with `Options::gfm()`:

```rust
use socketry_markdown::{to_html_with_options, Options};

let html = to_html_with_options("* [x] done", &Options::gfm()).unwrap();
```

### Work with the AST

Parse to a syntax tree when you need to inspect or transform Markdown:

```rust
use socketry_markdown::{to_mdast, ParseOptions};

let document = to_mdast("# Introduction\n\nHello, *world*!", &ParseOptions::default()).unwrap();
assert_eq!(document.text_content(), "IntroductionHello, world!");
```

Nodes provide traversal, text extraction, code-fence information, heading lookup, and child or section editing. `extract_children()` returns a `Fragment` that can be cloned, rendered, or serialized on its own.

Use `HTMLRenderer` or a custom `Renderer` to render an AST. Serialize nodes and fragments back to Markdown with `Node::to_markdown()`:

```rust
let markdown = document.to_markdown();
assert!(markdown.starts_with("# Introduction"));
```

`MarkdownOptions::line_wrapping` can preserve soft source line breaks or unwrap them into spaces when serializing Markdown.

### Extensions

The parser inherits support for CommonMark, GFM, MDX, frontmatter, and math constructs from `markdown-rs`. The fork-specific parsing options are described under [Motivation](#motivation). HTML output escapes raw HTML and omits MDX expressions by default. See the [API documentation](https://docs.rs/socketry-markdown/latest/socketry_markdown/) for parser options, renderer configuration, and AST types.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`, or `bump --version X.Y.Z`), then run `cargo bake cargo:release` and open a pull request. After review and merge, GitHub Actions publishes the release when the configured `crates-io` environment approves it. Follow the shared [Releasing skill](https://github.com/socketry/socketry-project-rust/blob/main/context/releasing.md) for the standard process.

## Releases

<!-- bake-readme:releases:start -->

See [releases.md](releases.md) for the full release history.

### v0.3.2

- Name renderer source files after their public types without changing public import paths.

### v0.3.1

- Adopt `socketry-project` 0.3.7 for shared project tasks and Markdown normalization.
- Require the aggregate test and coverage result for pull request merges.
- Refresh dependency examples and repository-owned agent guidance.

### v0.3.0

- Use hyphens as the default marker for unordered lists in Markdown serialization.

<!-- bake-readme:releases:end -->

## See Also

- [`markdown-rs`](https://github.com/wooorm/markdown-rs) — the upstream parser.
- [CommonMark](https://commonmark.org/) — the Markdown specification.

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/socketry-markdown-rust).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Read `.agents/context/index.md` to find relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.

[badge-build-image]: https://github.com/socketry/socketry-markdown-rust/actions/workflows/test.yml/badge.svg

[badge-build-url]: https://github.com/socketry/socketry-markdown-rust/actions

[badge-coverage-image]: https://github.com/socketry/socketry-markdown-rust/actions/workflows/test.yml/badge.svg

[badge-coverage-url]: https://github.com/socketry/socketry-markdown-rust/actions/workflows/test.yml
