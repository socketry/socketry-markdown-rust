# `socketry-markdown`

A CommonMark-compliant Markdown parser for Rust with an AST, extensions, and
HTML and Markdown renderers. It is based on [`markdown-rs`](https://github.com/wooorm/markdown-rs)
and adds Socketry's AST helpers and renderer APIs.

[![Build][badge-build-image]][badge-build-url]
[![Coverage][badge-coverage-image]][badge-coverage-url]

## Install

```sh
cargo add socketry-markdown
```

The library supports Rust 1.73 and later.

## Render HTML

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

## Work with the AST

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

## Extensions

The parser supports CommonMark, GFM, MDX, frontmatter, and math constructs.
HTML output escapes raw HTML and omits MDX expressions by default. See the
[API documentation](https://docs.rs/socketry-markdown/latest/socketry_markdown/)
for parser options, renderer configuration, and AST types.

## Releases

<!-- bake-readme:releases:start -->
See [releases.md](releases.md) for the full release history.

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

- [socketry-markdown](https://github.com/socketry/socketry-markdown-rust) — CommonMark compliant markdown parser in Rust with ASTs and extensions <!-- bake-readme:package -->

- [`markdown-rs`](https://github.com/wooorm/markdown-rs) — the upstream parser.
- [CommonMark](https://commonmark.org/) — the Markdown specification.

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/socketry-markdown-rust).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills
from Cargo dependencies. Read `.agents/context/index.md` and the skills that
apply to your task.

[badge-build-image]: https://github.com/socketry/socketry-markdown-rust/actions/workflows/test.yml/badge.svg
[badge-build-url]: https://github.com/socketry/socketry-markdown-rust/actions
[badge-coverage-image]: https://img.shields.io/codecov/c/github/socketry/socketry-markdown-rust.svg
[badge-coverage-url]: https://codecov.io/github/socketry/socketry-markdown-rust
