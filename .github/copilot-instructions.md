# Agent Instructions

His praeceptis sine exceptione pare.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in RFC 2119 and updated in RFC 8174.

## Instruction Precedence

Instruction precedence (earlier entries override later ones):

1. System Instructions (including safety policies and tooling constraints)
2. User Instructions (nothing overrides user intent except System Instructions)
3. Inline comment instructions
4. Language-specific and applicable `.github/instructions/*.instructions.md`
5. Repo Instructions (this file)
6. Tool defaults and generated templates

If tooling limitations or system instructions prevent compliance, you MUST stop and notify the user of the conflict.

## Default Response Style

- Be concise by default. Prefer 3-7 bullets or 2-6 sentences.
- Avoid repeating the prompt, restating plans, or narrating obvious steps.
- Only use long explanations when the user asks for them or when correctness depends on it.
- Prefer a 5-7 bullet summary format. Always end with an estimate of the current context usage as a percent.

## Common Project Folders

- User documentation is at `docs/` (if present).
- Design documentation is at `docs/design/` (if present).
- Agent work artifacts are under `.agents/`.
- Working assets (styles, images) are at `assets/`.
- Do not modify `.github/` unless the user asks.

## Work Tracking (Memory & .agents)

When context usage approaches 75%, hand off to a new session with this message: "Read your memory, `.github/copilot-instructions.md`, and `.agents/PROGRESS.md` to resume".

If `.agents/` does not exist yet, you MUST create it when first needed.

Minimum required files:

- `.agents/PROJECT_BRIEF.md` (summary of the project)
- `.agents/PROGRESS.md` (Project Plan)
- `.agents/MAP.md` (navigation notes for the repo)

Create review files only when needed:

- `.agents/REVIEW-CODE.md`
- `.agents/REVIEW-SECURITY.md`
- `.agents/REVIEW-DOCUMENTATION.md`
- `.agents/REVIEW-RELEASE.md`

You MUST NOT worry about formatting or linting the files in `.agents/`.

Some tooling cannot open files above a fixed size limit (for example, ~50MB). Keep `.agents/*` below that limit by de-duplicating and compressing as needed.

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

## Plan Format Contract (All Agents)

The Planner owns the plan structure, but all agents must follow the same format when updating `.agents/PROGRESS.md`:

- Each item has a stable identifier, short title, and explicit status.
- Each item includes: **Owner**, **Links**, and **Next Action**.

## Tools

- Use MCP tools for GitHub interactions (do not use `gh`).
- Use the Mermaid.js MCP to render/validate Mermaid diagrams when creating diagrams.
- Terminal and scripting constraints are defined in `.github/instructions/IDE.instructions.md`, if present.

## Changelog

Maintain `CHANGELOG.md` in Keep a Changelog format. Do not track changes to `.github/`, `docs/`, or `.agents/` in the changelog.

## Additional Guidelines

- Do not rely solely on git status/diffs; track your own changes.
- Do not revert changes you did not make.
