---
name: Review
description: Use when you want to review a piece of content, such as an article, code, or document. The AI will provide documents with feedback, suggestions for improvement, actions, and identify any potential issues or areas for enhancement.
agent: agent

---
# Review

Conduct a formal review, per the reviewing skill, and provide actionable feedback.

You MUST NOT do not alter code, documentation, or other files except the review output files.

In order to ensure enough context for the review, break it down into manageable chunks and/or delegate it to subagents as needed. You may also ask for additional information or clarification from the user if necessary.

## Files to Ignore

- Read only the `Compact.json` files under `docs/desing/aurora/` for context on the Aurora model. Do not read any other files in that directory.
- Do not read any files or folders under `docs/design/aurora/` names `MIS-*`. Those are just the human readable version of the model and contain no relevant information not already in the Compact Model.
- Do not read files under `.github/` unless specifically instructed to review them.

## Criterion

- Correctness: code matches intent, performs the function correctly, and wiring is complete.
- Compliance; If an Aurora model is present, code aligns to the model. If there is a divergence, the code or model should be updated to align with the other.
- Reliability: failures are handled intentionally, unhandled errors are logged and/or returned cleanly to the caller.
- Security: apply OWASP guidance (input validation, authz/authn boundaries, secrets handling, least privilege, safe logging).
- Accessibility: WCAG AA (AAA preferred).
- Maintainability: clear names; proper, cohesive modules; minimal complexity.
- Gaps anf 'foot-guns': identify missing edge cases, potential for misuse, and areas where future maintainers may struggle.
- Tests: require tests that prove behavior; no null tests.
- Code conciseness: prefer small functions (~50 lines) and cohesive modules (~500 lines) when practical.
    - For newly written or substantially rewritten code, avoid new functions > 50 lines and new files > 500 lines.
- Documentation conciseness: clear, to the point, easily readable documentation

## Deliverables

- Record findings in the appropriate review file under `docs/design/`:
    - `docs/design/Review-Code.md`
    - `docs/design/Review-Security.md`
    - `docs/design/Review-Documentation.md`
    - `docs/design/Review-Prerelease.md`
- Group findings by severity and include verification guidance.
- Provide actionable feedback: what, why it matters, and the smallest safe fix.
