# Vanopticon - Hela

Hela, goddess of the underworld, knows the sins of all; the Hela tool is an active vulnerability scanner and part of the Vanopticon suite.

> **NOTE**: Use of Hela on systems you do not own or have permission to access may be illegal in your jurisdiction- Always ensure you have explicit permission before conducting any security assessments.

## Features

- 100% pure Rust with minimal unsafe code (some unsafe code is required for low level operations)
- Asynchronous scanning using Tokio for high performance
- Modular architecture for easy extension and customization
- Support for a wide range of vulnerability checks
- Detailed reporting and logging
- Integration with other Vanopticon tools

## Core Discovery & Enumeration

- **Adaptive Host and Service Discovery**
   Dynamic fingerprinting across IPv4/IPv6, leveraging parallel TCP/UDP probing, SYN/ACK heuristics, banner analysis, and protocol-specific negotiation- Handles ephemeral ports, port-knocking, and rate-limited services.

- **Protocol-Aware Enumeration Modules**
   Rich dissectors for HTTP(S), SSH, RDP, SMB, LDAP, SNMPv3, MQTT, AMQP, industrial protocols, OT/ICS stacks, and custom enterprise protocols- Precision negotiation to avoid false positives.

- **Authenticated Scanning**
   Support for credentialed Windows (WinRM, WMI, SMB RPC), Linux (SSH PAM/keys), and hybrid cloud IAM roles- Correct privilege boundary handling, token renewal, and session confinement.

## Vulnerability Detection

- **Exploit-Independent Detection**
   Semantic version diffing, function/patch presence tests, and behavioral validation without exploitation- Algorithmic mapping to CVE, KEV, CISA directives, vendor advisories.

- **Safe Proof-of-Vulnerability (PoV) Checks**
   Non-destructive exploit simulation using input perturbation, stub exploitation, environment sandboxing, and side-effect detection- Prevents service disruption.

- **Application-Layer Weakness Testing**
   Authenticated/unauthenticated web testing: injection, path traversal, misconfigurations, weak crypto, broken access control, deserialization issues, SSRF, RCE indicators, etc- Support for OpenAPI/GraphQL introspection.

- **Configuration & Hardening Validation**
   CIS, DISA STIG, NIST benchmarks, AWS Well-Architected, Azure Security Benchmark, Kubernetes CIS- Delta analysis against baselines and drift detection.

## Coverage Expansion

- **Cloud-Native Resource Scanning**
   Real-time enumeration of cloud assets, ephemeral nodes, serverless, containers, images, registries, IaC templates, and agentless VM snapshots.

- **Container & Image Security**
   Layer-diff scanning, package manifest reconciliation, syscall profile analysis, vulnerable capabilities, exposed secrets, insecure Dockerfile instructions.

- **OT/ICS-Safe Mode**
    Passive-first handshake, rate-limited probing for fragile PLCs, SCADA systems, and proprietary field controllers- Avoids unintended device state changes.

## Performance & Safety

- **Distributed Scan Orchestration**
    Horizontal scaling, sharded workloads, checkpointing, result consistency, adaptive throttling- Cloud/hybrid scheduling with network locality awareness.

- **Network Impact Mitigation**
    Probe pacing, router/firewall congestion modeling, smart retries, token-bucket rate controls, low-impact modes for production networks.

- **Service Conditioning Detection**
    Identifies WAF/IPS interference, honeypots, tarpits, throttling, deception systems; adjusts probe strategies accordingly.

## Intelligence Integration

- **Contextual Risk Scoring**
    Multi-factor evaluation: exploit maturity, weaponization, KEV status, exposure path, business criticality, compensating controls, lateral movement potential.

- **Threat Intelligence Fusion**
    Ingest STIX/TAXII, MISP, CTI feeds, vendor advisories- Automated correlation between scanner findings and observed threat actor TTPs.

- **Anomaly-Driven Targeting**
    Feed from netflow, passive DNS, EDR telemetry, and SIEM events to prioritize next scans based on suspicious behavior.

## Accuracy & Analysis

- **False Positive Suppression**
    Behavioral confirmation loops, cross-module correlation, version fingerprint triangulation, service capability diffs.

- **Temporal Drift Tracking**
    Change detection across scans with precise deltas: newly opened/closed ports, changed banners, patch regressions, config drift.

- **Dependency & SBOM Correlation**
    Scan package manifests, libraries, JAR/WAR/NuGet/npm/pip artifacts, binaries- Link to SBOM sources and correlate transitive vulnerabilities.

- **Detailed Remediation Pathing**
    Vendor-specific patch instructions, configuration corrections, compensating hardening, rollback-safe guidance, validation steps.

## Automation, Extensibility & Integration

- **Plugin Architecture**
    Hot-reloadable modules in multiple languages (e.g., Lua, Python, Rust)- Contract-driven API for new detectors.

- **Policy-Driven Scan Plans**
    Declarative policies: scope blocks, priority tiers, scheduling windows, credential rotation, cloud account discovery.

- **CI/CD and DevSecOps Integration**
    Automated pipeline scanning for IaC, containers, and service endpoints- GitOps-aligned diff-based testing.

- **Comprehensive Reporting APIs**
    REST, gRPC, and event-driven outputs- Structured evidence payloads (JSON, CBOR)- Integration hooks for SOAR/SIEM.

- **Chain-of-Custody Logging**
    Cryptographically verifiable logs, time-stamped evidence, traceable scan actions, tamper-evident audit trails.

## Types of Scans and Tests

- Buffer Overflow
- Buffer Underflow
- UAF- Use After Free
- Deserialization Vulnerabilities
- Unsafe Command Invocation
- Template Injection
- Sanbox Escape
- Incorrect Permission Assignment
- Kernel or Code Vulnerabilities
- SUID / SGID Issues
- Process Injection
- ACL Misconfigurations
- Insecure Defaults
- Logic Flaws
- Insecure Session Management
- Insecure Cryptographic Storage
- Insecure Communications
- Failure to Restrict URL Access
- Insufficient Transport Layer Protection
- Hardcoded Credentials
- Flawed Cryptography
- Flawed Token Validation
- And lots more...

## Technologies

- 100% Pure Rust
- Asynchronous Programming with Tokio

## Status

- CI: See the repository's Actions tab for the latest build status.
- Documentation: See the `docs/` and `docs/design/` directories for architecture and design notes.

## Prerequisites

- Rust toolchain (stable). Install via rustup if you don't already have it:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable
rustup default stable
```

- A few scanners and integrations may require native libraries (for example OpenSSL). On Debian/Ubuntu these are typically provided by:

```bash
sudo apt-get update && sudo apt-get install -y pkg-config build-essential libssl-dev
```

## Quick start

Clone the repository (replace the URL with the project's remote):

```bash
git clone https://github.com/<OWNER>/<REPO>.git
cd <repo>
```

Build in debug mode:

```bash
cargo build
```

Build an optimized release:

```bash
cargo build --release
```

Run the binary (show help to discover available subcommands and options):

```bash
cargo run -- --help
# or run the release build
cargo run --release -- --help
```

## Testing

Run the full test suite:

```bash
cargo test --workspace
```

## Scan set preview

Before adding a new scan set profile, validate it with the built-in preview command:

```bash
cargo run -- scan-set preview --file docs/design/scan-sets/example.json
```

The CLI outputs the pipelines, estimated duration, and depth/detectability/risk metadata that will be applied to the scheduler.

Run a single crate's tests (replace <crate-name>):

```bash
cargo test -p <crate-name>
```

## Formatting & linting

This repository includes a `rustfmt.toml`. Please format changes before submitting PRs:

```bash
cargo fmt --all
```

Lint with Clippy (recommended as part of CI):

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Documentation

Generate and open API docs locally:

```bash
cargo doc --open
```

Design and architecture documents live in `docs/design/` — please consult them for high-level decisions and protocol handling.

## Contributing

We welcome contributions. Please read `CONTRIBUTING.md` and follow the repository's contribution workflow. Note these key points:

- Create branches for individual features or fixes.
- Write tests for new behavior and ensure the test suite passes.
- Run `cargo fmt` and `cargo clippy` before submitting a pull request.
- Sign your commits if the repository requires DCO compliance (see `DCO.md`).

Also review `CODE_OF_CONDUCT.md`.

## Security

If you discover a security vulnerability, follow the disclosure guidelines in `SECURITY.md` rather than opening a public issue.

## License

This repository contains license text files in the project root. See `LICENSE-Apache.md`, `LICENSE-MIT.md`, and `LICENSE-GPL.md` for details about licensing and any multi-license options.

## Support

See `SUPPORT.md` for information about commercial support and how to get help.

## Directory layout

- `src/` — primary Rust source code
- `docs/` — user and design documentation
- `docker/` — helper scripts and Dockerfiles used for development/testing

## Where to go next

- Read `docs/design/` for architecture and feature cards.
- Open issues or PRs to request features or report problems.

---

If you'd like a quick example or a tailored quickstart for your platform (macOS, Windows, or specific Linux distribution), tell me which OS and I'll add platform-specific setup steps.
