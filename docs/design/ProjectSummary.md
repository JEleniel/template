# Project Summary

This is a template repository that contains files necessary to initialize a new repository, including the utility configurations (`.markdownlint-cli2.jsonc`, `.prettierrc`, etc.), a set of templates for public repository root documentation files, and configuration and `.gitignore` templates for a variety of languages.

## Using this Repo

This repository can be used in one of two ways:

1. Use it directly as a "template repo" in GitHub when initializing a new repository, move the language specific files to the root, and delete the unnecessary folders.
2. Copy the elements you want into another repository.

## Repo Structure

- `.github/`:
    - `aurora`: The Aurora modeling system, including Agent instructions, schemas, and reference files.
    - `instructions/`, `skills/`, and `copilot-instrcutions.md`: A complete set of the instructions, prompts, and skills I use to make GitHub Copilot a useful tool instead of a random noise generator.
    - `ISSUE_TEMPLATE`, and `PULL_REQUEST_TEMPLATE.md`: GitHub form templates
- `.vscode/`: A copy of my settings to make setting up a VSCode environment easier.
- `assets`: Templates for various assets, including my favorite CSS reset stylesheet, GitHub banners, and cross platform icons.
- `docs/design/ProjectSummary.md`: this file, a summary of the project the repo holds and key information for working with it, applicable to both human and agent users.
- `Frameworks`: A set of framework specific templates
- `Languages/`: A set of language specific templatesand `.gitignore` files tuned to the language.
    - `Deno`
    - `Go`
    - `Java`
    - `Node`
    - `Python`
    - `Rust`
- `mcp`: Configurations for various MCP servers I regularly use.
- `.gitattributes`: Git attributes to help agents ignore Aurora generated files.
- `.markdownlint-cli2.jsonc`, `.prettierrc.json`: linting and formatting configurations I use in every repo; Prettier is my default formatter and Markdownlint handles all the `*.md` files in repos.
- `*.md`: A set of the top level repo documentation for public repos.
