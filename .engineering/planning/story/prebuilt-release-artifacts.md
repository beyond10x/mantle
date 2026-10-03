---
format: aep.planning-md/3
id: story:prebuilt-release-artifacts
kind: story
status: draft
title: Produce verified installable CLI and worker release artifacts
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:worker-doctor
scope:
- confidence: inferred
  path: .github/workflows
- confidence: inferred
  path: Cargo.lock
- confidence: inferred
  path: Cargo.toml
- confidence: inferred
  path: README.md
- confidence: inferred
  path: crates/mantle-conformance
- confidence: inferred
  path: crates/mantle-release
- confidence: inferred
  path: generated/worker-model
- confidence: inferred
  path: spec/components.yaml
- confidence: inferred
  path: spec/domains/operator.yaml
- confidence: inferred
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/cli
- confidence: inferred
  path: website/index.html
revision: 14
---
## Outcome
Build reproducible packaging and publishing support for Linux x86_64 laptop CLI and static x86_64 musl worker binaries, matching the existing supported worker architecture. Do not claim untested macOS/ARM packages. Typed ReleaseManifest/ReleaseArtifact values are declared in spec/domains/operator.yaml; all committed running tooling is Rust with clap.

## Acceptance
Named ESS cases release-package-layout, release-manifest-identities, release-checksum-refusal, release-unsupported-target, release-path-safety and release-install-atomic. A Rust packaging/checking command produces versioned archives, SHA256 checksums and a strict manifest bound to source revision and Substrate pin; archive contains documented installation layout and licenses/notices required by shipped dependencies. Verify extracted version and static worker compatibility in CI. Provide a checksum-verifying install path with no blind pipe-to-shell, atomic replacement and no credential/config overwrite. CI builds and retains candidate artifacts on PR without publication; tag publishing uses bot App authority, immutable action pins and least permissions. Never publish artifacts from a different commit or overwrite an existing immutable version. Test tampered/missing/path-traversal archives and interrupted installation with small executable fixtures. Actual new tagged release is outside this one-PR task.

## Scope
Inferred: new Rust release tooling crate or existing suitable crate; Cargo.toml/lock; .github/workflows release and CI; operator ESS commands/scenarios; generated provenance; README/site. Preserve fixed upstream identity and repository Gates conventions. Inspect current bot-authenticated gh capability for binary asset upload; never use personal gh writes.

## Publication authority

Read-only preflight found Mantle exposes only the B10X_GATES_POLICY repository secret; no bot App publishing credential is configured there. Do not copy private App keys into Mantle or silently publish as github-actions. CI builds and retains immutable candidate/tag artifacts without publishing credentials. Provide an operator-invoked Rust release command which verifies the exact tag/source/version/artifact manifest and invokes the locally installed b10x-gates gh wrapper for release creation/upload; wrapper source confines the bot token to gh's child environment and supports release commands. No clobber/overwrite. The command and documentation complete the supported publication path; no new release is executed in this task. If later CI publishing is enabled, it must use an explicitly configured bot App route with immutable action pins and least permissions.
