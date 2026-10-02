# Static worker correction for 0.1.1

Released 0.1.0 failed the documented static musl worker build because libc exposes statfs.f_type with different signedness on GNU and musl. Unit5237b0a converts the positive tmpfs constant to the destination field type without changing filesystem admission. CI now installs the musl target/linker and executes task build-worker. All six workspace package versions are0.1.1; dependency pins and ESS semantics are unchanged.

Integration6560122 passed all seven repository gate steps plus the actual three-package static worker build. The full suite has187 top-level Rust tests and336 native ESS scenarios, zero failures/skips/unsupported/outside/refused. The unit also passed75 launcher tests and49 native launcher scenarios. Static launcher --help executed successfully. Logs here abbreviate personal home prefixes; original logs remain in private wave9 scratch. Native suite/report/results bytes are preserved exactly.

A separate coordinator adversary pass found no findings; new reviewer dispatch and historical reviewer reactivation both failed because the host agent-thread limit was reached. This was a local role fallback, not an independent subagent review. The immutable review-result:musl-worker-adversary-1 records its scope and limitations.

The source release is a corrective0.1.1;0.1.0 is not moved. Common Gates and GitHub required checks must pass on the exact candidate before main promotion and release. No prebuilt release assets are declared. Documentation source was updated in0.1.0; publication is asynchronous. Actual authenticated Codex login/model/tool turns and full dual-agent live acceptance remain unfinished; this build correction does not close those parent stories.
