# Attachment cancellation claim

Claim: a real connected attach client exits within1500ms of SIGTERM while its stdout pipe is unread, without killing its detached server to cause the client exit. The drained-output control must also exit. The same finite producer/probe and command shape ran against the baseline and treatment binaries, with distinct empty per-run directories and otherwise identical inputs.

Probe/source/logs are retained at ~/.cache/mantle-wave6/scratch-integration/claim. Baseline binary was copied from unchanged integration a0f8dd8 and hashed in baseline.sha256; treatment uses /dev/shm/mantle-wave6-target-attach/debug/mantle-launch built from the implementor's handed-off source. Same command: claim/probe <baseline-or-treatment-empty-root> <respective-binary>. No credential, VM or external traffic was used.

Baseline exit1, baseline.log:
```text
drained=true termination_within_1500ms=true
drained=true client_and_server_reaped=true
drained=false termination_within_1500ms=false
drained=false client_and_server_reaped=true
```

Treatment exit0, treatment.log:
```text
drained=true termination_within_1500ms=true
drained=true client_and_server_reaped=true
drained=false termination_within_1500ms=true
drained=false client_and_server_reaped=true
```

VERIFIED for this bounded client-cancellation claim. The probe waits for each child and terminates the server only after observing/reaping the client; final ps -C mantle-launch -C probe returned no rows. This external no-TTY probe does not establish real rendering or authentication. Permanent AB01–AB04 exercise both handled signals, paused input, real PTY/flag restoration, aliases and exact bytes through small pipes.
