---
applyTo: '**/*'
---

# IDE Specific Instructions

- You are working within the Visual Studio Code IDE.
- The specified tools, such as `markdownlint` and `prettier`, are installed globally. If you encounter any issues running them, notify the user.

## Memory Management

When context usage approaches 75%, hand off to a new session with this message: "Read your memory, `.github/copilot-instructions.md`, and `.agents/PROGRESS.md` to resume".

Note: Some tooling cannot open files above a fixed size limit (for example, ~50MB). Keep `.agents/PROGRESS.md` below that limit by de-duplicating and compressing it as needed.

Note: If `.agents/PROGRESS.md` does not exist yet, create the `.agents/` folder and the required files as described in `.github/copilot-instructions.md`.

## Prohibited Actions

You MUST NOT, at any time, for any reason, perform any of the following actions:

- Attempt to write to any folder outside of the workspace. If you need a temporary working space, create a `tmp/` folder in the workspace.
- Use `|| true` or `true ||` or `true` as a command or part of a command, especially in the terminal.
- Use the `gh` command line tool. **It is not installed and will not be.**
- Use the `head` or `tail` commands in the terminal.

## Memory

- You are equipped with a local memory capability.
- You MUST begin every session by reading your memory, no exceptions.
- If your memory exceeds 500 lines or 10% (whichever is smaller) of the total context window, compress it by summarizing key points and removing redundancies.

## Additional Instructions

- The `tools/` folder is not guaranteed to exist in this template. Do not assume repo-specific tooling paths unless the user explicitly points you at them.
