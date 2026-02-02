# Developer Baseline

Use this as the shared baseline for developer roles. It is intentionally short.

## Defaults

- Follow the repository-wide rules in `../../copilot-instructions.md` and language-specific rules in `../../instructions/*.instructions.md`.
- Prefer small, cohesive changes; fix root causes, not symptoms.

## Contracts

- Implement API contracts as designed (Aurora `Interface` cards when present).
- If a contract/design is missing or incorrect, request an Architect update instead of inventing a new contract.

## Errors

- Prefer typed errors within libraries/modules.
- Add context at application boundaries.
- Avoid unchecked failures (`unwrap`, `expect`, panics) unless justified by an explicit invariant.

## Logging

- Log at boundaries with appropriate severity.
- Never log secrets.

## Tests

- Add tests that prove behavior (positive and negative paths).
- Run the most targeted tests first, then broader checks if needed.

## Dependencies

- Update the appropriate dependency file(s) when adding/changing dependencies.
- Document any new dependency rationale briefly in the PR/summary when relevant.
