# Scan Sets — Configurable Profiles

This document captures the implementation intent for configurable scan sets (Issue #16) so operators can define pipelines, depth/risk metadata, rate limits, and randomization seeds that are referenced by the REST API, CLI, and scheduler.

## Goals

- Provide named, reusable profiles that map to the existing default/complete/deep/fast policies but remain fully tunable.
- Ensure every profile is described by depth, detectability, target/operator risk, and randomization parameters so the scheduler can make safe decisions.
- Expose scan sets via OAuth2-protected REST endpoints and the CLI, propagating their metadata into scheduling, reporting, and telemetry.

## Scan set schema

Each scan set is a JSON document with the following keys (extensions are permitted as long as they stay within the schema):

| Field | Description |
| --- | --- |
| `name` | Unique canonical identifier (e.g., `default`, `complete`, `deep`, `fast`, `custom/audit`). |
| `description` | Human-friendly summary for CLI help text and documentation. |
| `enabled_pipelines` | Ordered list of detection pipelines (memory, protocol, config, session, network, privilege) or plugin modules to include. |
| `allowed_targets` | Optional CIDR list that acts as a filter or scope whitelist for this set. |
| `depth` | Depth rating (enum: `low`, `medium`, `high`, `deep`). Controls how many nested probes are allowed. |
| `detectability` | Detectability rating (enum: `stealth`, `guided`, `aggressive`). Guides jitter/randomization presets. |
| `target_risk` | Target risk rating (enum: `minimal`, `moderate`, `elevated`). Determines safety checks and evidence gating. |
| `operator_risk` | Operator risk rating (enum: `conservative`, `balanced`, `assertive`). Used to warn when a set may trigger defensive countermeasures. |
| `rate_limit` | Max probes per second and concurrent tasks per target (structured object). |
| `evidence_retention` | Artifact retention policy (e.g., `7d`, `30d`, `indefinite`). |
| `randomization` | Seed derivation rules, jitter window, and ordering policy (derived from randomization service). |
| `requires_scope` | OAuth scope required to launch the set (e.g., `hela.scans.deep`). |

### Example scan set

```json
{
  "name": "custom/deep-audit",
  "description": "Deep audit for high-value targets with operator-approved permissions.",
  "enabled_pipelines": ["discovery", "memory", "protocol", "privilege", "reporting"],
  "allowed_targets": ["203.0.113.0/28"],
  "depth": "deep",
  "detectability": "aggressive",
  "target_risk": "elevated",
  "operator_risk": "assertive",
  "rate_limit": {"probes_per_second": 5, "concurrent_targets": 2},
  "evidence_retention": "30d",
  "randomization": {"jitter_ms": 200, "ordering": "per-scan nonce"},
  "requires_scope": "hela.scans.deep"
}
```

## REST endpoints

All scan-set endpoints inherit the same transport/auth constraints as the core API (HTTP/2 + TLS1.3, OAuth Client Credentials). The proposed API surface:

- `GET /v1/scan-sets`
    + Returns the list of registered scan sets and their summary metadata.
    + Requires `hela.scan-sets.read` scope (or the broader `hela.scans.read`).

- `POST /v1/scan-sets`
    + Creates or updates a scan set.
    + Validates `enabled_pipelines`, `rate_limit`, and `randomization` fields before persistence.
    + Requires `hela.scan-sets.write` scope and RBAC checks.

- `POST /v1/scan-sets/preview`
    + Accepts a candidate set payload and returns `applicable_pipelines`, `estimated_duration`, `depth/detectability/risk` ratings, and any policy violations (e.g., conflicting scopes).
    + Useful for CLI `--set` validation before submission.

- `GET /v1/scan-sets/{set_name}`
    + Retrieves the full configuration for a named set, including QoS/risk metadata.
    + Used by the scheduler to hydrate jobs and by CLI `--set` help text.

- `DELETE /v1/scan-sets/{set_name}` (optional)
    + Removes a custom set, ensuring `default/complete/deep/fast` remain immutable.

Scan creation/resolution responses include the depth/detectability/target risk/operator risk fields so downstream components can make decisions.

### Linking scan sets to scan jobs

A scan creation request (`POST /v1/scans`) may include either the `policy` (reserved names) or the `scan_set_name`. When a set name is provided, the API performs:

1. Auth scope validation (ensures token has `requires_scope`).
2. `allowed_targets` filtering (enforces target whitelist or raises `403`).
3. Injects `depth/detectability/target_risk/operator_risk` into the scheduler payload for telemetry.
4. Appends `randomization` hints to the job (e.g., jitter settings, seed overrides) so the worker pool honors detectability requirements.

Jobs launched without a set fall back to `policy=default` and use the default profile stored internally.

## CLI integration

`hela scan` gains a `--set <set_name>` flag:

- CLI requests `/v1/scan-sets/{set_name}` to populate help text and verify existence before posting the scan.
- CLI displays the depth/detectability/risk ratings locally so operators understand the impact of their selection.
- CLI caches approved sets (with `max-age` reflective of policy) to minimize repeated API chatter.
- `hela scan --policy default` and `--set default` are equivalent, but custom sets can expose deeper metadata.

The CLI also exposes a `hela scan-set preview --file <set.json>` command that calls `/v1/scan-sets/preview` so operators validate new profiles without persisting them.

## Scheduler enforcement

Scan sets create deterministic constraints for the scheduler:

- Each job inherits the `enabled_pipelines` list and `rate_limit` object; the scheduler translates these into per-worker task quotas and back-pressure signals.
- Depth/detectability ratings influence which pipeline phases are enabled (`memory` pipeline only runs when depth >= `high`) and how jitter/randomization parameters are applied (`stealth` reduces probe frequency and increases randomization windows).
- `target_risk` and `operator_risk` feed into the approval workflow (e.g., escalate warnings when these fields are `elevated/assertive`).
- The scheduler rejects jobs where `scan_set.required_scope` exceeds the token's granted scopes; this prevents misuse.

Job metadata persisted to Postgres includes the set name, its risk ratings, last applied randomization seed, and the `allowed_targets` set used during execution. This metadata is later surfaced in reports and auditing queries.

## Testing & Validation

- Unit tests should cover schema validation for scan sets (required fields, enum restrictions, `allowed_targets` parsing) and preview endpoint logic.
- Integration tests must authenticate via OAuth Client Credentials, create/preview a custom scan set, and ensure the scheduler receives the correct depth/detectability/risk values.
- CLI tests should simulate `--set` selection, show metadata in the prompt, and verify that the preview call catches invalid combinations.
- Security reviews must confirm that `scan_set.requires_scope` is validated and that unauthorized tokens cannot launch high-impact sets.

## Acceptance criteria

- Operators can define or select named scan sets with explicit depth, detectability, target risk, and operator risk ratings.
- Scan-set metadata propagates to the scheduler, reporting, and telemetry layers so the system honors the configured constraints.
- All scan-set endpoints operate over HTTP/2 + TLS1.3 with OAuth Client Credentials scopes as documented.
- Custom sets can be previewed before submission, and CLI `--set` selection surfaces metadata for operator approval.

This implementation document completes the requirements for Issue #16; refer to `feature-scan-sets.json` and the corresponding memory entry for additional context.
