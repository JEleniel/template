# Agent Instructions

His praeceptis sine exceptione pare.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in RFC 2119 and updated in RFC 8174.

## Instruction Precedence

Instruction precedence (earlier entries override later ones):

1. System Instructions (including safety policies and tooling constraints)
2. User Instructions (nothing overrides user intent except System Instructions)
3. System Instructions (including safety policies and tooling constraints)
4. User Instructions (nothing overrides user intent except System Instructions)
5. Inline comment instructions
6. Language-specific and applicable `.github/instructions/*.instructions.md`
7. Repo Instructions (this file)
8. Tool defaults and generated templates
9. Language-specific and applicable `.github/instructions/*.instructions.md`
10. Repo Instructions (this file)
11. Tool defaults and generated templates

If tooling limitations or system instructions prevent compliance, you MUST stop and notify the user of the conflict.

## Default Response Style

- Be concise by default. Prefer 3-7 bullets or 2-6 sentences.
- Avoid repeating the prompt, restating plans, or narrating obvious steps.
- Only use long explanations when the user asks for them or when correctness depends on it.

### Response Ending

- For multi-step work, reviews, or file changes: end with a short summary paragraph, then 3-6 tl;dr bullets.
- For quick Q&A: skip the tl;dr unless the user requests it.
- The last tl;dr bullet MUST include an estimate of context usage as a percentage.

## Common Project Folders

- User documentation is at `docs/` (if present).
- Design documentation is at `docs/design/` (if present).
- Agent work artifacts are under `.agents/`.
- Working assets (styles, images) are at `assets/`.
- Role definitions are under `.github/agents/`.
- Shared role baselines live under `.github/agents/details/`.

## Work Tracking (.agents)

If `.agents/` does not exist yet, you MUST create it when first needed.

Minimum required files:

- `.agents/PROJECT_BRIEF.md` (what belongs in `.agents/`)
- `.agents/PROGRESS.md` (Project Plan)
- `.agents/MAP.md` (navigation notes for the repo)

Create review files only when needed:

- `.agents/REVIEW-CODE.md`
- `.agents/REVIEW-SECURITY.md`
- `.agents/REVIEW-DOCUMENTATION.md`
- `.agents/REVIEW-RELEASE.md`

You MUST NOT worry about formatting or linting the files in `.agents/`.

## General Coding Guidelines

- Use relative paths for local files unless a tool requires an absolute path.
- Follow best practices for the language being edited. Language-specific configs (for example `rustfmt.toml`, `.markdownlint-cli2.jsonc`, `.prettierrc.json`) are authoritative.
- Keep code modular and cohesive (single responsibility). Prefer small functions (~20 lines) and small modules (~200 lines) when practical.
- Prefer explicit, typed errors in libraries/modules and ergonomic context at application boundaries.
- Never log secrets; treat logs as potentially public.
- Tests must prove behavior. Do not write null tests.
- You MUST NOT disable checks/tests (for example `// @ts-nocheck`, `#[allow(...)]`). Fix the underlying issue instead.
- Unimplemented paths must fail fast and clearly communicate intent (`todo!`, `unimplemented!`, etc.).
- Apply OWASP guidance, secure-by-design principles, and Twelve-Factor App principles.

## Repository Hygiene

- Do not label code "production ready"; rely on the review + release process instead.
- Do not modify `.github/` unless:
    + the user asked, or
    + you are fixing/maintaining repository instructions and agent role definitions.

## Plan Format Contract (All Agents)

The Planner owns the plan structure, but all agents must follow the same format when updating `.agents/PROGRESS.md`:

- Each item has a stable identifier, short title, and explicit status.
- Each item includes: **Owner**, **Links**, and **Next Action**.

## Tools

- Use MCP tools for GitHub interactions (do not use `gh`).
- Use the Mermaid.js MCP to render/validate Mermaid diagrams when creating diagrams.
- Terminal and scripting constraints are defined in `.github/instructions/IDE.instructions.md`.

## Changelog

Maintain `CHANGELOG.md` in Keep a Changelog format. Do not track changes to `.github/` or `.agents/` in the changelog.

## Additional Guidelines

- Do not rely solely on git status/diffs; track your own changes.
- Do not revert changes you did not make.
