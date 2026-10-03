# Native conformance evidence

The suite, run, result, report and no-op audit JSON files in this directory form one coordinated
snapshot from the final reliability integration gate. The closing record in
[the reliability wave ledger](../reliability/waves.md) identifies the exact tested source, command,
direct exit status and measured counts. Component reports describe their own selected inventory;
the aggregate report reconciles every component against the complete specification inventory.
A passed local scenario does not establish authenticated Claude or Codex qualification.

`mutations.json` is historical evidence from source
`a7b26668a20dce232c5e068989b6818bdefa4171` on 2026-10-02, retained unchanged. Its mutations and
scenario identities are not a claim that those experiments were rerun on the current source.
The later, bounded reliability hardening audit is recorded separately in
[hardening-mutations-evidence.json](../reliability/hardening-mutations-evidence.json) and
[hardening-design-evidence.json](../reliability/hardening-design-evidence.json).
Five implementation mutations were exercised there; the checksum mutation initially survived
and a same-length archive-corruption test now detects it. Nine broader ESS observation gaps remain
proposed follow-up in `task:reliability-conformance-boundary-expansion`.

The no-op audit is freshly produced from the same complete suite as the current aggregate run.
It is distinct from historical mutation evidence: accepted commands with empty observations must
not satisfy authored assertions. Generated outcome-only passes are listed explicitly by the audit.
