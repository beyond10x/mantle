# Operator attachment failure

The user's real 100-column terminal rendered partial Codex art over the prior shell and ended with session.output-backpressure. The existing agent ex_01M3Z04Y9BXGP4VKA5FAA26RS8 remained Running; no workspace was restarted or removed.

A bounded temporary Rust PTY probe spawned the installed mantle attach codex-ready with a 100x50 terminal, continuously drained output, answered only cursor-position requests, discarded screen bytes, and sent Ctrl-] d only at its deadline. It retained counts and protocol-error stderr only. Two independent runs exited1 before the deadline: 65536 bytes/0 cursor queries, then83454 bytes/0 cursor queries; both named session.output-backpressure. No authentication input was sent or login output retained. Immediate stdout-discard and paused pipe-drain controls had stayed attached until their15-second external bound; these do not establish the real TTY path.

Ranked hypotheses, before corrective probes:
1. The launcher replays its256KiB history before the WebSocket client is attached. Substrate queues16 output frames and cancels immediately on Full. Prediction: holding launcher output until an explicit client-ready byte removes the startup error; delaying socket attachment worsens baseline.
2. Replay burst overruns the queue even with the client attached. Prediction: readiness alone fails, and a bounded output/drain flow correction is also necessary; a longer delay before readiness does not change the failure.
3. Local blocking stdout prevents the SDK reader draining. Prediction: failure needs a slow PTY reader and disappears with an actively draining real PTY. Continuous-drain red runs weaken this hypothesis for initial failure, though blocked-output cancellation still needs regression coverage.

The successful earlier local launcher tests bypassed this production Substrate queue seam. The remaining work must test the seam and rerun this live repro, without claiming authenticated parity.

## Follow-up evidence

The baseline SDK probe delayed WebSocket attachment by 500ms and failed with session.not-attachable; the execution observation named Cancelled and session.output-backpressure. The candidate READY/ACK launcher and exact production terminal module completed readiness but still failed after 77,822 output bytes, with the same remote cancellation. This confirms hypothesis 2 in addition to the startup race. Readiness alone is not a completed fix.

Substrate issue https://github.com/beyond10x/substrate/issues/115 tracks the queue defect. Current Substrate 0.7.9 has the same immediate full-queue refusal. The proposed correction reserves a bounded queue slot asynchronously with a one-second stall deadline; queue and output limits remain unchanged. Its real drain regression failed on the unchanged baseline and passed after the reservation change. Independent review, full gates and combined integration evidence remain required.

The Mantle readiness implementation c58e601a3ae86a29787c414eb2a0a39522b233cc passed 123 local tests. Independent adversary tests, committed as 8d1dd6107d8e66745213ca19013f7bee02f94084, add fragmented and malformed ACK, shared ACK/first-input, replacement attachment, resize and signal checks. The launcher suite passed 80 Rust tests and 49 ESS scenarios. These local results do not establish that the deployed worker is fixed.

Only a separately named candidate launcher was copied to the worker. The installed launcher, Substrate daemon, gateway and persistent agent were left unchanged. A combined proof uses a fresh local delegated daemon and synthetic workspace before any production runtime change.
