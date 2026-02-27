# Security Policy

We take security seriously and appreciate responsible disclosure.

## Reporting a vulnerability

Do **not** report security vulnerabilities through public GitHub issues, pull requests, or discussions.

Use one of the private channels below (customize for your project):

- GitHub Security Advisories: [https://github.com/OWNER/REPO/security/advisories/new](https://github.com/OWNER/REPO/security/advisories/new)
- Email: [security@yourdomain.example](mailto:security@yourdomain.example)

If you use PGP, link your public key here:

- PGP public key: [https://github.com/OWNER.gpg](https://github.com/OWNER.gpg)

### What to include

Please include:

- A clear description of the issue and the affected component.
- Impact assessment (what an attacker can do).
- Steps to reproduce or a proof of concept.
- Version/commit information.
- Any mitigations or workarounds you are aware of.
- Relevant references (for example CVEs).

### What to expect

This is a template. Set expectations for your project.

- Acknowledge receipt within 48 hours.
- Provide a status update within 7 days.
- Coordinate a fix and a disclosure timeline.
- Credit reporters in the advisory if they would like (or keep them anonymous).

## Supported versions

Document which versions receive security fixes.

| Version   | Supported |
| --------- | --------- |
| `main`    | Yes       |
| `< 1.0.0` | No        |

## Scope

In scope (examples):

- Remote code execution, injection, and sandbox escapes.
- Authentication and authorization bypass.
- Sensitive data exposure.
- Privilege escalation.
- Supply chain and dependency integrity issues.

Out of scope (examples):

- Denial of service without a realistic security impact.
- Social engineering.
- Physical attacks.
- Vulnerabilities in third-party services not controlled by the project.

## Security updates

When a vulnerability is confirmed, we will:

- Develop and test a fix.
- Release a patch as soon as reasonably possible.
- Document the change in release notes.
- Publish a GitHub security advisory when appropriate.

## Bug bounty

Unless stated otherwise, this project does not offer a paid bug bounty.

## Customize this template

Before publishing, update the following:

- Replace `OWNER/REPO` and `OWNER` with your GitHub organization/user and repository name.
- Replace the email address with an inbox you actively monitor.
- Replace the supported versions table with your real support policy.
