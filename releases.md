# Releases

## Unreleased

- Add an option to unwrap soft line breaks when serializing Markdown.
- Preserve checked and unchecked task list items when serializing Markdown.
- Fix a panic when serializing long inline code or math values containing CRLF
  before unsafe Markdown characters.

- Fix parsing and serialization edge cases for HTML blocks, inline code, and Unicode escapes.

## v0.1.1

- Update agent guidance to install dependency context and skills without
  generating `agents.md`.

## v0.1.0

- Add AST renderers for HTML and Markdown, including Markdown serialization on
  nodes and fragments.
- Add fragment extraction, heading helpers, and parser fixes from the Socketry
  fork.
- Adopt shared Socketry testing, documentation, and release automation.
