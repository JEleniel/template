# Reviewer Baseline

Use this as the shared baseline for reviewer roles.

## Defaults

- Review-only unless explicitly instructed to implement fixes.
- Prefer actionable feedback: what, why it matters, and the smallest safe fix.

## Key Checks

- Correctness: code matches intent and wiring is complete.
- Reliability: failures are handled intentionally.
- Security: apply OWASP guidance (input validation, authz/authn boundaries, secrets handling, least privilege, safe logging).
- Maintainability: clear names, cohesive modules, minimal complexity.
- Tests: require tests that prove behavior; no null tests.

## Outputs

- Record findings in the appropriate `.agents/REVIEW-*.md` file.
- Group findings by severity and include verification guidance.
