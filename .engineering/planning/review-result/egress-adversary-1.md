---
format: aep.planning-md/3
id: review-result:egress-adversary-1
kind: review-result
status: active
title: Adversary review of mantle-egress, pass 1
relations:
- reviews: story:egress-gateway
revision: 1
---
needs-revision

# Adversary review of mantle-egress, pass 1

Reviewer: `aep:adversary`, against the uncommitted tree on `main` at `2c5003f`, 2026-10-02.
Cases executed 25 → 31; 6 red, in `crates/mantle-egress/tests/adversary.rs`.
Command: `CARGO_TARGET_DIR=$HOME/.cache/b10x-target/mantle cargo test -p mantle-egress --no-fail-fast` → exit 101 (lib 22/0, adversary 0/6, refusal 3/0).

| # | Severity | Verdict | What | file:line | Test |
|---|---|---|---|---|---|
| 1 | warning | CONFIRMED | a slot is taken at accept before any head byte, so one session's silent sockets on the shared per-worker gateway make every other session's requests get 503 | `proxy.rs:51` | `silent_sockets_do_not_starve_a_well_formed_request` |
| 2 | warning | INFEASIBLE | IPv4 special-purpose blocks 192.0.0.0/24, 198.18.0.0/15 and the documentation ranges are classed public | `addr.rs:22` | `ipv4_ietf_protocol_block_is_refused`, `ipv4_benchmarking_and_documentation_ranges_are_refused` |
| 3 | warning | INFEASIBLE | 64:ff9b:1::/48 and ::ffff:0:0:0/96 embed IPv4 addresses such as 169.254.169.254 but are not judged by them | `addr.rs:42` | `ipv6_local_use_nat64_prefix_embedding_metadata_is_refused`, `ipv6_siit_translated_form_embedding_metadata_is_refused` |
| 4 | note | INFEASIBLE | IPv6 outside 2000::/3 and 100::/64, 2001:db8::/32, 2001:2::/48 fall through to public | `addr.rs:56` | `ipv6_non_global_unicast_space_is_refused` |
| 5 | note | INFEASIBLE | over-capacity refusals spawn unbounded tasks holding an fd; the unit sets no LimitNOFILE | `proxy.rs:60` | argued, no test |
| 6 | note | INFEASIBLE | resolving the dot-stripped name lets the resolver search list expand an allowed name | `proxy.rs:229` | argued, no test |
| 7 | note | CONFIRMED | no read_head test splits the terminator across reads; connect_without_headers asserts only is_ok | `head.rs:51` | argued, no test |

```findings
- file: crates/mantle-egress/src/proxy.rs
  line: 51
  category: concurrency
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: a slot is taken at accept before any head byte, so one session's silent sockets on the shared per-worker gateway make every other session's requests get 503
- file: crates/mantle-egress/src/addr.rs
  line: 22
  category: contract-drift
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: IPv4 special-purpose blocks 192.0.0.0/24, 198.18.0.0/15 and the documentation ranges are classed public, contrary to lib.rs "refuses to dial non-public addresses"
- file: crates/mantle-egress/src/addr.rs
  line: 42
  category: contract-drift
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: 64:ff9b:1::/48 and ::ffff:0:0:0/96 embed IPv4 addresses such as 169.254.169.254 but are not judged by them, contrary to the comment at addr.rs:34
- file: crates/mantle-egress/src/addr.rs
  line: 56
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: IPv6 addresses outside 2000::/3 and the 100::/64, 2001:db8::/32 and 2001:2::/48 blocks fall through to public
- file: crates/mantle-egress/src/proxy.rs
  line: 60
  category: concurrency
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: over-capacity refusals spawn unbounded tasks that each hold an fd, and the unit sets no LimitNOFILE, so a flood can exhaust descriptors for all sessions
- file: crates/mantle-egress/src/proxy.rs
  line: 229
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: resolving the dot-stripped name lets the resolver search list expand an allowed name to a different host
- file: crates/mantle-egress/src/head.rs
  line: 51
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: no read_head test splits the terminator across reads, so dropping the 3-byte overlap survives the suite, and connect_without_headers asserts only is_ok
```
