---
name: architecture
description: Use this skill when you need to create or update the architecture, including Aurora models, of a system, project, or organization.
---

# Architecture Skill

## General guidelines

- Security-First Architecture Mindset: Evaluate architecture with an adversarial mindset and validate trust boundaries, privilege boundaries, and abuse resistance.
- Model-to-Reality Focus: Verify that architecture artifacts represent the implemented system and intended evolution.
- Decision Traceability Focus: Require explicit rationale, constraints, and trade-offs for architecture decisions.
- Operability and Evolvability Focus: Prioritize designs that are observable, resilient, and changeable without cascading rework.

## Principles of Elegant Architecture

- Clarity: Architectural structure and intent are understandable without external explanation.
- Simplicity: The architecture solves requirements with the smallest complete set of mechanisms.
- Composability: Components compose orthogonally without special-case glue.
- Predictability: Similar concerns are handled with consistent patterns and constraints.
- Traceability: Decisions, constraints, and trade-offs are discoverable and auditable.
- Minimal Incidental Complexity: Mechanisms exist for domain value, not accidental implementation convenience.

### Indications of Poor Architecture or Modeling

- God Components: A single component owns too many unrelated responsibilities.
- Leaky Layers: Callers must understand internals across boundaries to use interfaces correctly.
- Inconsistent Vocabulary: Different names are used for the same architecture concepts.
- Model-Implementation Mismatch: Aurora artifacts and architecture docs disagree with observed behavior.
- Unbounded Interfaces: Public interfaces are broader than required capability.
- Decision Orphans: Important architecture decisions lack rationale or traceability.

## Aurora

The Aurora modeling system provides a structured way to capture architectural elements (like missions, drivers, requirements) and their relationships in a formalized, easy to reason about model.

Details on the Aurora modeling system are in the [Aurora instructions](../../aurora/Aurora.instructions.md).

- Do not use the Aurora compact model when using this skill.

### Aurora Model Fidelity Checklist

- Aurora Coverage Is Complete
- No Model Drift
- Constraint Traceability
- Decision-to-Model Traceability
- Terminology Consistency

## Deliverables

- The architecture should be rooted in a `docs/design/Requirements.md` document that captures the requirements and constraints that the architecture is designed to satisfy and i sprovided by the user. This document should be maintained and updated as requirements evolve.
- One or more Aurora models and supporting documentation that capture the detailed architecture of the project. To manage complexity in larger projects, multiple models may be used and may refer to each other.
- Supporting documentation that captures architectural decisions, trade-offs, and rationale. This may include ADRs, hardware specifications, and other relevant documents that provide context and details supporting the model(s).
- Models should be located at the default path of `docs/design/aurora/` and follow the standard Aurora structure and naming conventions.
- Supporting documentation should be located in `docs/design/`, organized in appropriate subdirectories (e.g., `docs/design/adrs/` for Architectural Decision Records).

## Operating Procedure

1. Confirm the task scope, stakeholders, constraints, and success criteria from `docs/design/Requirements.md` and the user request; if critical information is missing, stop and get clarification before modeling.
2. Inspect the existing architecture artifacts under `docs/design/` and `docs/design/aurora/`; update existing documents and models in place unless a new architectural boundary requires a separate model.
3. Capture or update supporting design documentation for major assumptions, constraints, external dependencies, trust boundaries, and architectural decisions before or alongside the model changes.
4. Create or update Aurora models under `docs/design/aurora/` using the full Aurora instructions, canonical card types, and canonical relationships by default; introduce non-canonical elements only when no canonical option is semantically correct.
5. Keep terminology, identifiers, and references consistent across requirements, supporting documentation, and Aurora artifacts so that every important decision and constraint is traceable to a single source of truth.
6. Review the resulting architecture against the deliverables and validation checklists in this skill, confirming that the model and documents are sufficient to guide implementation without guesswork.
7. If the request also includes implementation work, finish the architecture phase first, then switch to the Coding skill for the implementation phase.

## Validation Checklists

### General Architecture Checklist

- ADRs, tradeoffs, and rejected alternatives are captured.
- Authoritative and non-authoritative data sources designed.
- Clean modularization, separation of concerns, and isolation of state.
- Clearly stated assumptions and invariants.
- Data provenance and lineage requirements captured.
- Defined versioning and compatibility strategy.
- Dependencies Are Intentional.
- Deployment and rollback strategy planned.
- Documented inputs and outputs.
- Failure mode and remediation analysis completed.
- Identified stakeholders and external actors.
- Loose coupling and high cohesion.
- Modeled data flow and transformation points.
- NFR Coverage Is Concrete.
- Performance, availability, and degradation behavior expectations.
- Planned horizontal and vertical scaling characteristics.
- Planned observability (what must be measurable).
- Purpose driven APIs.
- Responsibilities Are Cohesive.
- Risks, threat models, security controls, and mitigations.
- System evolution strategy.
- Testability requirements.
- Traceable ownership and responsibility.
- Well defined system, domain, and trust boundaries.

### Architectural Domain Coverage Checklist

- Domains and Capabilities
- Use-Case and Workflow
- Components, Interfaces, and Services
- APIs and Contracts
- Data Architecture
- State Machines and Events
- Security and Trust
- Reliability and Resilience
- Performance and Scalability
- Deployment and Runtime
- Observability and Operations
- Dependencies and Integration
- Migration and Evolution
- Testing and Verification

### External References and Link Integrity Checklist

- Aurora External References Resolve
- Accompanying Document Links Resolve
- Model-to-Document Links are Consistent
- Reference Context Is Explicit

## Cross-skill tasks

- If the request includes implementation work (code changes), treat architecture modeling as its own phase first (produce/confirm the model and supporting documentation), then switch to the Coding skill for implementation.
- If the request is to review an existing architecture or model without changing it, use the Reviewing skill to record findings and recommendations.

## Glossary

See the shared [Skills glossary](../GLOSSARY.md).
