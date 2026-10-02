# Codex prerequisite removal — coordinator observation

VERIFIED for the bounded startup admission claim. The baseline binary from3329356 (SHA2565f6ae61272146dcc88ebded948039753598778d95ad1c173dda633812c5ce4c5) and the source unit treatment binary were run with the same example Codex manifest, empty child environment, synthetic KubeVirt configuration naming not-contacted, isolated empty state directories and five-second process timeout. No credentials or worker were supplied. Command shape: env -i HOME=<isolated-home> MANTLE_CONFIG=<synthetic-config> MANTLE_STATE_DIR=<isolated-state> timeout5 <binary> start --detached <example-manifest>.

Baseline stderr: Error: FAILED_CAPABILITY: Codex requires supported non-recording terminal capture; the pinned Substrate SDK cannot provide it

Treatment stderr: Error: no worker recorded; run `mantle worker up` first

Both exited1 promptly, without timeout. The treatment passes the removed capture policy and reaches the actual worker prerequisite without requesting Claude credentials. This proves admission routing only; no worker deployment, authentication or model turn was attempted. Exact command inputs, binary digest, output logs and exits remain under the assigned private wave8 scratch-integration/claim directory. Independent Rust/native tests cover controlled start flow and actual SDK request construction.
