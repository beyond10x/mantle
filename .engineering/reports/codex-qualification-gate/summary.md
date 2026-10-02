# Wave 2 integration gate

Source integration `e4e866cd989da75d7e6cbb058a682d43f643853d` plus the closing worker proposal
value types in spec/domains/session.yaml. Current compiled specification digest:
`9d516b98e127473571c4fb0e5a285948f2d5a7399a59ba99b681df08ba2263cf`.
All six repository task-check steps executed separately with their own captured exit status.

| Step | Exit | Observed result |
| --- | --- | --- |
| cargo fmt --all --check | 0 | no formatting differences |
| cargo clippy --workspace --all-targets --locked -- -D warnings | 0 | every workspace target checked |
| cargo test --workspace --locked | 0 | 127 top-level Rust tests passed; zero ignored |
| ess specify validate --path spec | 0 | mantle v1 — 6 file(s), valid |
| aep plan artifact validate | 0 | 29 artifacts valid at this gate step; final store validation follows planning-only updates |
| cargo run --locked -p mantle-docs -- build --out website/build --commit e4e866cd989da75d7e6cbb058a682d43f643853d | 0 | documentation built; no deployment/publication claim |

The Rust total is20+10+2+24+6+3+44+9+3+2+4=127. Re-executed nested regression/control children
are excluded. The ESS adapter's one Rust test executed57 generated local scenarios, zero skipped,
zero failures against actual SQLite and default allowlist behavior. The two pre-existing synthesis
refusals for manifest/launcher invariant views remain documented in spec/README.md; they are not
57 passing scenarios or Codex qualification evidence. Empty binary/doc-test lanes are reported as
zero tests, not extra coverage. No gate step skipped itself.

The first test compile was intentionally interrupted at exit130 after concurrent disk usage fell
below the wave's10GiB floor. No test result is claimed from that attempt. The completed rerun put
both target and TMPDIR in the private tmpfs target and disabled sccache disk writes; build jobs2,
debug info0 and incremental off. Earlier clippy used sccache. Full logs retain the interruption and
completed run separately; only personal home-directory prefixes are normalized to $HOME.

This gate validates the qualification harness and incoming local ESS implementation. Device
login, authenticated model/tools, visible approvals, refresh/revocation and full visual usability
remain explicit unobserved CQ obligations; the parity epic remains active.
