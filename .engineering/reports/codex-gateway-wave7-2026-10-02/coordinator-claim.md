# Gateway default-policy claim

Claim: the production gateway with no --allow override starts with exactly the ten required host:443 entries: the eight prior destinations plus auth.openai.com and chatgpt.com, with no duplicate or extra entry. Run the same binary command and exact-set predicate before and after the policy change.

Both runs used `timeout --signal=TERM --kill-after=2s 1s <binary> --listen 127.0.0.1:0`, a fresh kernel-selected loopback port, no clients and no external network requests. Timeout returned124 in both cases after the gateway reported graceful shutdown. An anchored process check found no remaining baseline or treatment process. The baseline binary was copied from the unchanged gateway source before wave7; its SHA256 is retained in ~/.cache/mantle-wave7/scratch-integration/claim/baseline.sha256. Treatment came from the implementor's handed-off unit binary; test-only review does not change its runtime.

The predicate parses the actual startup allow list, counts entries and unique exact matches against the literal ten-host requirement, and accepts only count=10 and matches=10. The raw stderr/stdout, command exits and predicate outputs remain in the assigned claim scratch. It returned:

```text
baseline: default_count=8 exact_required_matches=8 expected=10
baseline predicate exit:1
treatment: default_count=10 exact_required_matches=10 expected=10
treatment predicate exit:0
```

VERIFIED for the production default configuration. This startup observation does not prove tunnels or real OpenAI traffic. The unit's native GW cases exercise real CONNECT with controlled resolver/dial boundaries, and live authenticated traffic remains parent acceptance. No worker or credential was used.
