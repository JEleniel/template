---
name: SessionStart
description: Use when starting a coding session that must stay strictly within the user-assigned task and existing instructions.
agent: agent
---
# Strict Session

- You MUST treat your system, user, and workspace instructions, and user preferences as binding.
- You MUST keep the session narrowly scoped.
- You MUST NOT expand scope beyond the exact task the user assigned.
- You MUST NOT change other files, modules, or behaviors unless the task explicitly requires it.
- You MUST NOT add code whose main purpose is to silence warnings.
- You MUST NOT add code that does not add value, is non-functional, or is decorative, including one-line helper functions; you MUST ask whether a helper adds value before introducing it.
- You MUST only read source, documentation, and tests that are relevant to the task.
- You MUST NOT attempt to address issues outside the assigned scope. Include them in your summary and take no further action on them.
- You MUST keep comments concise and relevant, and comments MUST explain why rather than how or what.
- You MUST include links in module-level comments when, and only when, they are relevant to the module.
- If a change would require leaving the assigned scope, you MUST stop and ask once.
- You MUST validate only the code and files you directly touched.

## Anti-Patterns

Red flags will get your code rejected immediately, no questions asked. Yellow flags indicate areas of concern.

### Red Flags

- Violating the One Source of Truth:
  - Creating multiple functions and multiple paths through the code when a single direct path would work.
  - A `#[cfg(...)]` gate outside a function.
  - Multiple functions whose names only vary by environment or other configuration, for example `read_non_volatile_memory_host()` and `read_non_volatile_memory_esp()`, rather than a single `read_non_volatile_memory()` that handles both cases.
- Defining something that is only used once, could be implemented more simply, or is otherwise not a meaningful abstraction.
- Tests that are directly in the same module as the implementation code, rather than in a separate test module.
- Putting functions outside the narrowest scope possible, for example putting a function outside the only structure that calls it.
- The `let _ =` anti-pattern, which is a strong sign of writing code just to silence warnings rather than to add value.
- Use of `unwrap`, `expect`, `map_err`, `panic`, or similar bypass constructs instead of properly typing errors and `#[from]` conversions.

### Yellow Flags

- Adding a module or file without a valid, clear reason that is explicitly required by the task.
- Failing to read the documentation and reading the source code first.
- Overly broad commands, for example using `create_dir_all()` when `create_dir()` would work, which is a strong sign of not caring about the exact scope of the change.
- Overly verbose comments to explain what the code does. If the code is unclear enough to require a comment, it is a strong sign that the code should be rewritten to be clearer rather than adding a comment to explain it.
- Responses or errors that only include a number and convey no useful information.
- Using external constants, such as `CARGO_MANIFEST_DIR`, to establish the current working folder.
- Writing a bespoke function when an existing library or function already provides the same functionality, or a related dependency (e.g. for `serde` there are `toml` and `serde_json`) is available that provides it.

## Follow-up

- You MUST acknowledge these instructions and ask any questions now; otherwise, you MUST wait for the user to assign a task.
