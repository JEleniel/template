---
applyTo: '*.md'
---

# Markdown Style Guide

This document defines formatting and style conventions for Markdown files.

The source of truth for enforcement is `.markdownlint.json`.

## Guidelines

- You MUST use only ATX headings (`#`, `##`, …) and do not indent them.
- You MUST use a single H1.
- You MUST increase heading levels one at a time; do not skip levels.
    * Sibling headings must be unique.
- You MUST NOT use emphasis/strong as the entire heading text.
- You MUST NOT hard-wrap lines in paragraphs.
- You MUST NOT use two spaces together, or leave trailing spaces.
- You MUST end files with exactly one trailing newline.
- You MUST avoid multiple blank lines together.
- You MUST use dashes for first level lists, asterisks for second level, and pluses for third. Repeat this pattern for deeper levels.
- Ordered lists MUST use sequential numbers.
- You MUST use fenced code blocks with backticks.
- You MUST include a language for all code blocks. Use `text` when no specific language applies.
- When documenting a command, include sample output.
- Do not use bare URLs; use Markdown links. Avoid reversed links. Do not create empty links.
- Link fragments (anchors) are allowed.
- Provide alternative text for images.
- You MAY use inline HTML but SHOULD avoid it unless it is necessary.
- You MUST use leading and trailing pipe characters for all table rows and ensure consistent column counts in every row.
- Keep blank lines around tables, code blocks, callouts, and headings.
