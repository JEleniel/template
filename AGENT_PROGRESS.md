# AGENT_PROGRESS

## Project Brief

This repository is a template for a GitHub project, including contribution policies, documentation scaffolding, and GitHub Issue templates.

## Master Project Plan

- [ ] Customize README placeholders for the target project.
- [ ] Define initial architecture and design docs under docs/design/.
- [ ] Confirm issue templates (Markdown + YAML issue forms) match and are usable.

## Patterns

- Keep GitHub issue forms reporter-focused (avoid machine/agent-oriented fields in templates).
- Prefer a language-neutral root .gitignore with language-specific variants as separate template files.

## Technologies

- GitHub Issue forms (YAML) and legacy issue templates (Markdown)
- Markdownlint (configured via .markdownlint.json)
- Prettier (for YAML/JSON formatting)

## Active Context Summary

Maintained YAML-only GitHub issue forms under .github/ISSUE_TEMPLATE, removed language-specific/hardcoded placeholders, reduced redundant headings, added bug severity/priority, updated test type selection, added a template chooser config.yml, and removed machine/agent-oriented fields (JSON/memory instructions) so forms are reporter-focused.

Revised the pull request template to be conflict-free, repo-agnostic, and aligned with the issue templates' reporter-friendly tone.

Improved .gitignore with additional repo-agnostic excludes (editor cruft, JetBrains, direnv, coverage/test outputs), removed duplicate log sections, and added language-specific gitignore templates as separate files.

<memory>
Session state (2026-01-15): Standardized language-specific gitignore templates to be copies of the root .gitignore with language-specific entries prepended.
Key points: Keep templates repo-agnostic and reporter-friendly; avoid machine/agent instruction sections in issue forms.
</memory>
