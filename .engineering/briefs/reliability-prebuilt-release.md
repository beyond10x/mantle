# Unit 4: verified prebuilt releases

Read [shared invariants](reliability-invariants.md) and
[the story](../planning/story/prebuilt-release-artifacts.md). Dispatch begins only after the doctor
unit passes its integrated gate; the execution record supplies the exact base commit.

- Managed id and branch: `mantle-prebuilt-release`, `unit/mantle-prebuilt-release`.
- Checkout: `$HOME/.local/state/worktree/trees/b10x/mantle/mantle-prebuilt-release`.
- Implementor lease: `codex-prebuilt-release`.
- Target: `/dev/shm/mantle-prebuilt-release-target`.
- Temporary files: `/dev/shm/mantle-prebuilt-release-tmp`.
- Scratch: `$HOME/.cache/mantle-reliability/prebuilt-release`.

Build one coherent Rust library/CLI with build, verify, install and operator publication commands.
Keep verification independent of Mantle application/worker dependencies so the next unit can
reuse it. Supported artifacts are Linux x86_64 GNU CLI and static x86_64 musl worker binaries.
Ship the Rust verifier/installer executable with the GNU CLI artifact and document the initial
trusted-manifest checksum check, so using the prebuilt installation path does not require a
source build of its installer.
Update the existing ESS release types and named scenarios before runtime implementation.

Bind artifacts to a verified exact source revision. Prefer building an isolated export of the
verified clean Git tree with a fresh target. Version equality alone is insufficient. Retain exact
Substrate revision/version, toolchain identity and target provenance in the strict manifest.
Make archives deterministic in ordering, permissions, ownership and timestamps. Document the
GNU runtime baseline actually built and tested; do not promise untested host compatibility.

Verify checksums, sizes, payload inventory, target and strict path whitelist before bounded
execution of version probes. Reject duplicate members, links, traversal, oversized expansion,
unsupported targets and nonstatic workers. Reuse established format parsers. Checksums establish
integrity against the supplied trusted manifest, not independent publisher authentication.

Install through a private same-filesystem stage and bounded exclusive lock. Verify every
destination collision before activation; refuse unmanaged files or links. Atomic activation
must preserve the prior installation on interruption and concurrent attempts. Use owned unique
stage names, immutable verified generations and one activation point. Configuration and
credentials stay outside this transaction. Keep worker filenames compatible with delivery.

The source story records exact missing-license and runtime inputs, including their hashes.
Root LICENSE is currently absent. Use cargo-about with a generated-model clarification rather
than hand-editing the generated manifest. Include complete runtime and composite dependency
notices. License generation/checking is Rust; no committed scripts. Actual musl runtime was
identified as 1.2.5; the matching notice is retained in release-preflight scratch. Preserve its
provenance and fail on an unrecognized runtime instead of silently reusing unrelated text.

CI builds, verifies and retains artifacts without publishing credentials. Pin action identities
and use least permissions. Operator publication uses the installed `b10x-gates gh` bot wrapper:
exact tag, source, manifest and assets must agree. Existing immutable releases/assets refuse;
no clobber and no personal gh writes. No release is executed during this unit.

Tests exercise the actual Rust/CLI seams with small Rust fixtures: wrong source with same
version, dirty source, malformed manifest, tampered/missing archives, unsafe members, static
worker checks, concurrent install, unmanaged collisions and interrupted activation. Verify
publication refusal/failure paths through a controlled command fixture. Record source and
package/conformance evidence, then hand to a separate adversary. The root runs the full gate.
