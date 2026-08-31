## 1. Archive delta specs to main spec tree

- [x] 1.1 Run `openspec archive "baseline-specs"` to merge all delta specs into `openspec/specs/` — verify the following directories exist under `openspec/specs/`: `conversion/core`, `ui/app`, `integration/clipboard`, `integration/file-io`, `packaging/linux`
- [x] 1.2 Verify each archived spec has a valid `## Purpose` section and at least one requirement with scenarios — read through `openspec/specs/*/spec.md` files to confirm

## 2. Validate baseline

- [x] 2.1 Run `openspec validate` against the archived specs and confirm zero errors
- [x] 2.2 Confirm existing tests still pass: `cargo test`
