---
applyTo: '**/*'
---

# IDE Specific Instructions

- You are working within the Visual Studio Code IDE.
- The specified tools, such as `markdownlint-cli2` and `prettier`, are installed globally. If you encounter any issues running them, notify the user.

## Prohibited Actions

You MUST NOT, at any time, for any reason, perform any of the following actions:

- Attempt to write to any folder outside of the workspace. If you need a temporary working space, create a `tmp/` folder in the workspace.
- Use `|| true` or `true ||` or `true` as a command or part of a command, especially in the terminal.
- Use the `gh` command line tool. **It is not installed and will not be.**
- Use the `head` or `tail` commands in the terminal.
