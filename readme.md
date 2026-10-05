# `socketry-markdown`

A CommonMark-compliant Markdown parser for Rust with an AST, extensions, and
HTML and Markdown renderers. It is based on [`markdown-rs`](https://github.com/wooorm/markdown-rs)
and adds Socketry's AST helpers and renderer APIs.

[![Build][badge-build-image]][badge-build-url]
[![Coverage][badge-coverage-image]][badge-coverage-url]

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

Nodes provide traversal, text extraction, code-fence information, heading
lookup, and child or section editing. `extract_children()` returns a `Fragment`
that can be cloned, rendered, or serialized on its own.

Use `HTMLRenderer` or a custom `Renderer` to render an AST. Serialize nodes and
fragments back to Markdown with `Node::to_markdown()`:

```rust
let markdown = document.to_markdown();
assert!(markdown.starts_with("# Introduction"));
```

`MarkdownOptions::line_wrapping` can preserve soft source line breaks or unwrap
them into spaces when serializing Markdown.

### Extensions

The parser supports CommonMark, GFM, MDX, frontmatter, and math constructs.
HTML output escapes raw HTML and omits MDX expressions by default. See the
[API documentation](https://docs.rs/socketry-markdown/latest/socketry_markdown/)
for parser options, renderer configuration, and AST types.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`,
or `bump --version X.Y.Z`), then run `cargo bake cargo:release` and open a
pull request. After review and merge, GitHub Actions publishes the release
when the configured `crates-io` environment approves it. Follow the shared
[Releasing skill](https://github.com/socketry/socketry-project-rust/blob/main/context/releasing.md)
for the standard process.

## Releases

<!-- bake-readme:releases:start -->
See [releases.md](releases.md) for the full release history.

### v0.2.0

- Add an option to unwrap soft line breaks when serializing Markdown.
- Preserve checked and unchecked task list items when serializing Markdown.
- Fix a panic when serializing long inline code or math values containing CRLF
  before unsafe Markdown characters.

- Fix parsing and serialization edge cases for HTML blocks, inline code, and
  Unicode escapes.

### v0.1.1

- Update agent guidance to install dependency context and skills without
  generating `agents.md`.

### v0.1.0

- Add AST renderers for HTML and Markdown, including Markdown serialization on
  nodes and fragments.
- Add fragment extraction, heading helpers, and parser fixes from the Socketry
  fork.
- Adopt shared Socketry testing, documentation, and release automation.
<!-- bake-readme:releases:end -->

## See Also

- [`markdown-rs`](https://github.com/wooorm/markdown-rs) — the upstream parser.
- [CommonMark](https://commonmark.org/) — the Markdown specification.

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/socketry-markdown-rust).

### Agent Context

Before contributing, read `agents.md` and the relevant context files it links.
If `agents.md` is missing or out of date, run `cargo bake agent:context:install`
to install context from dependencies and update the index.

[badge-build-image]: https://github.com/socketry/socketry-markdown-rust/actions/workflows/test.yml/badge.svg
[badge-build-url]: https://github.com/socketry/socketry-markdown-rust/actions
[badge-coverage-image]: https://img.shields.io/codecov/c/github/socketry/socketry-markdown-rust.svg
[badge-coverage-url]: https://codecov.io/github/socketry/socketry-markdown-rust
