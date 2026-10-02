# Codex selection/preflight baseline and treatment

Observation2026-10-02, against baseline79e1ee6 and the staged wave5 unit before independent attack. Explicit MANTLE_CONFIG and MANTLE_STATE_DIR point only to the private synthetic fixture under ~/.cache/mantle-wave5/scratch-integration/claim. The same manifest has agent.kind codex with no explicit auth field, a placeholder source URL and no Claude credential configuration. Neither invocation reaches a provider or uses a credential.

Baseline CLI rejects the agent kind, exit1:

```text
Error: agent.kind "codex" is not served; the slice serves claude-code
```

Treatment CLI accepts selection and reaches the production capture-admission refusal, exit1:

```text
Error: FAILED_CAPABILITY: Codex requires supported non-recording terminal capture; the pinned Substrate SDK cannot provide it
```

Both stdout files are empty. A read-only SQLite query SELECT count(*) FROM sessions returns0 after each. Raw exit statuses, stdout/stderr and query results are retained in the assigned claim directory. This independently verifies recognized Codex selection and early refusal without Claude configuration or session insertion. It does not establish usable authenticated Codex or supported non-recording capture.

The implementation report's public CLI fixture additionally exercises explicit compatible auth and persisted-identity attachment after replacing the manifest. Its native cases exercise actual parser, SQLite migration, production request builders and the admission seam. The pinned-program observation in runtime-sinks.json comes from the implementor's digest-verified Rust harness; it is separate from this coordinator CLI comparison.

implementation.md retains the implementor report with personal home-path prefixes normalized to~/ for repository admission. Raw report/logs remain under the assigned implementor scratch. No output or verdict is changed by normalization.
