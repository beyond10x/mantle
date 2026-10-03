---
format: aep.planning-md/3
id: story:prebuilt-release-artifacts
kind: story
status: proposed
title: Produce verified installable CLI and worker release artifacts
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:worker-doctor
scope:
- confidence: inferred
  path: .github/workflows
- confidence: cited
  path: .github/workflows/ci.yml
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: inferred
  path: LICENSE
- confidence: inferred
  path: README.md
- confidence: inferred
  path: THIRD_PARTY_LICENSES.html
- confidence: inferred
  path: about.toml
- confidence: inferred
  path: crates/mantle-conformance
- confidence: inferred
  path: crates/mantle-release
- confidence: inferred
  path: generated/worker-model
- confidence: inferred
  path: spec/components.yaml
- confidence: cited
  path: spec/domains/operator.yaml
- confidence: inferred
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/cli
- confidence: inferred
  path: website/index.html
revision: 25
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:57:34Z", actor: "human:timo", revision: 19}
---
## Outcome
Build reproducible packaging and publishing support for Linux x86_64 laptop CLI and static x86_64 musl worker binaries, matching the existing supported worker architecture. Do not claim untested macOS/ARM packages. Typed ReleaseManifest/ReleaseArtifact values are declared in spec/domains/operator.yaml; all committed running tooling is Rust with clap.

## Acceptance
Named ESS cases release-package-layout, release-manifest-identities, release-checksum-refusal, release-unsupported-target, release-path-safety and release-install-atomic. A Rust packaging/checking command produces versioned archives, SHA256 checksums and a strict manifest bound to source revision and Substrate pin; archive contains documented installation layout and licenses/notices required by shipped dependencies. Verify extracted version and static worker compatibility in CI. Provide a checksum-verifying install path with no blind pipe-to-shell, atomic replacement and no credential/config overwrite. CI builds and retains candidate artifacts on PR without publication; tag publishing uses bot App authority, immutable action pins and least permissions. Never publish artifacts from a different commit or overwrite an existing immutable version. Test tampered/missing/path-traversal archives and interrupted installation with small executable fixtures. Actual new tagged release is outside this one-PR task.

## Scope
Inferred: new Rust release tooling crate or existing suitable crate; Cargo.toml/lock; .github/workflows release and CI; operator ESS commands/scenarios; generated provenance; README/site. Preserve fixed upstream identity and repository Gates conventions. Inspect current bot-authenticated gh capability for binary asset upload; never use personal gh writes.

## Publication authority

Read-only preflight found Mantle exposes only the B10X_GATES_POLICY repository secret; no bot App publishing credential is configured there. Do not copy private App keys into Mantle or silently publish as github-actions. CI builds and retains immutable candidate/tag artifacts without publishing credentials. Provide an operator-invoked Rust release command which verifies the exact tag/source/version/artifact manifest and invokes the locally installed b10x-gates gh wrapper for release creation/upload; wrapper source confines the bot token to gh's child environment and supports release commands. No clobber/overwrite. The command and documentation complete the supported publication path; no new release is executed in this task. If later CI publishing is enabled, it must use an explicitly configured bot App route with immutable action pins and least permissions.

## Packaging and license inputs

Read-only preflight by attach_readiness_impl found root LICENSE absent although workspace metadata declares Apache-2.0. cargo-about 0.9.1 using Substrate's existing configuration, --workspace --locked --offline --fail and both supported Linux targets refused three entries: generated mantle-worker-model has no license metadata/file, minicbor 2.3.0 needs BlueOak-1.0.0, and webpki-roots 1.0.9 needs CDLA-Permissive-2.0. Add the root license and inspect actual dependency texts. Preserve generated-model ownership: ESS 0.50 has no license generation flag; use a checked cargo-about clarification referring to ../../LICENSE from the generated crate instead of hand-editing generated Cargo.toml. Substrate's standard Apache LICENSE hashes cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30; verify exact bytes before using that clarification.

Use the Rust notice approach in substrate/xtask/src/licenses.rs as a reference, including composite aws-lc-sys/ring notices. Registry archives for base64-simd/vsimd 0.8.0 omit their license files; their recorded upstream commit is d74c030d9dc4f3cae02146d1f497ff62726ef09a. Obtain and pin the actual upstream text rather than silently relying on a generic fallback. Account for distributed Rust runtime and musl texts too. License generation/checking is Rust; CI may invoke pinned cargo-about but must not commit scripts.

Prefer one library/CLI crate for deterministic packaging, strict verification, installation and operator publication. Archive path whitelist, bounded expansion, duplicate/link refusal, exact version and static ELF checks are implementation boundaries. Source archives and license inputs do not authorize publishing an actual release in this task.

## Verified upstream license input

The omitted SIMD license is available from the exact published source revision: https://raw.githubusercontent.com/Nugine/simd/d74c030d9dc4f3cae02146d1f497ff62726ef09a/LICENSE. Retrieved bytes have SHA256 71674605ec4c087fe9eb534e3e4f9e26eb2e4aabcd76a29fd156c6a844d44b3d and identify the MIT grant with Copyright (c) 2021 Nugine. Retained local input: $HOME/.cache/mantle-reliability/release-preflight/simd-d74c030-LICENSE. Include the actual grant and attribution in distribution notices, with source/hash provenance; do not change generated crates or registry files.

## Source identity verification

Version equality alone cannot bind an artifact to a source commit: current development and release binaries can both report 0.1.4. The packaging path must either build from the verified clean exact checkout itself and retain that provenance, or verify embedded build source identity in each supplied binary. Do not accept arbitrary supplied binaries and label them with the current Git HEAD merely because --version matches. Include a same-version/wrong-source fixture. Publication verifies manifest source against the exact tag and all payload checksums; it must never relabel an existing version or overwrite assets. Keep the shared artifact verifier below worker and release CLI dependencies, without creating a Mantle-worker-release dependency cycle.

## Runtime license inputs

The pinned Rust 1.97 installation includes share/doc/rust/COPYRIGHT-library.html (279302 bytes) under the sysroot reported by rustc --print sysroot. This is a concrete runtime notice input, separate from cargo-about's dependency graph; use the matching build toolchain's copy. The local musl package also provides /usr/share/licenses/musl/COPYRIGHT, but a host package text alone does not prove the version bundled in Rust's self-contained musl target. Resolve that target's actual musl/runtime provenance before claiming matching notices. Keep runtime notice source/version metadata with produced artifacts.
