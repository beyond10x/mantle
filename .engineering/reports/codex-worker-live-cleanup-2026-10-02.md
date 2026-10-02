# Fresh-worker acceptance cleanup

The coordinator inspected namespace mantle-wave3-acceptance, UID154e7347-5740-4306-aee9-231fb47c3a31, created2026-10-02T12:23:03Z. Its sole VM was the recorded fresh acceptance instance21c01ed2-df0f-4ff1-a79a-0c397c186073, with its launcher pod and root/data PVCs. Both live acceptance outputs had already been retained.

`kubectl --context k3d-mantle delete namespace mantle-wave3-acceptance --wait=false` exited0. Initial observations correctly reported Terminating. A later exact namespace query and exact-namespace vm/vmi/pvc/pod query with --ignore-not-found both exited0 with no resources. Cleanup is verified; no force or finalizer edit was used.

The separate original namespace mantle was not deleted or changed. Read-back observed original VM UID4b378e47-faf8-4c13-b248-5bba94af2c41 Running. Its existing live Claude session preservation is recorded independently in the before/after acceptance report. Private fresh config/state/key files remain under task scratch until scratch cleanup; no credential/login was created on this test VM.
