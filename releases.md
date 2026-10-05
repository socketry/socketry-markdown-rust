# Releases

## v0.3.2

- Name renderer source files after their public types without changing public import paths.

## v0.3.1

- Adopt `socketry-project` 0.3.7 for shared project tasks and Markdown normalization.
- Require the aggregate test and coverage result for pull request merges.
- Refresh dependency examples and repository-owned agent guidance.

## v0.3.0

- Use hyphens as the default marker for unordered lists in Markdown serialization.

## v0.2.0

- Add an option to unwrap soft line breaks when serializing Markdown.

- Preserve checked and unchecked task list items when serializing Markdown.

- Fix a panic when serializing long inline code or math values containing CRLF before unsafe Markdown characters.

- Fix parsing and serialization edge cases for HTML blocks, inline code, and Unicode escapes.

## v0.1.1

- Update agent guidance to install dependency context and skills without generating `agents.md`.

## v0.1.0

- Add AST renderers for HTML and Markdown, including Markdown serialization on nodes and fragments.
- Add fragment extraction, heading helpers, and parser fixes from the Socketry fork.
- Adopt shared Socketry testing, documentation, and release automation.
