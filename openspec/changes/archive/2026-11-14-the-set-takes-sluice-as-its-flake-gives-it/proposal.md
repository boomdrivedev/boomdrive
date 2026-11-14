# Proposal

## Why

The brief: "The Boomdrive set pins sluice v0.89.0 or later from sluice's own flake, flake.lock moved to match, and takes sluice's package as that flake gives it: nix/packages/sluice.nix's version given over it, and its tests, go".

From `v0.89.0` on, a sluice release's tag names a release commit that writes the release's version into the tree, and sluice's flake builds that tree with no version given (sluice's record 0111). The set pins `v0.86.0`, whose tree says `0.0.0`, so `nix/packages/sluice.nix` gives the package its tag's version and `SLUICE_VERSION` over sluice's flake's. Once the set pins a tag whose tree carries its version, sluice is taken as Flume, crew and timber are: its own flake's package, unchanged (core.44).

## What Changes

- `flake.nix` pins sluice at `v0.94.8`, and `flake.lock` locks it.
- The set's `sluice` is sluice's flake's `packages.<system>.default`, unchanged. `nix/packages/sluice.nix`, which gave it its tag's version, goes.
- The nix project's case "sluice through its own flake" holds the set's sluice to sluice's flake's own derivation and to the locked tag's version, as Flume's case does. The case "sluice's version from its tree", and its tree `nix/tests/tree-9.9.9`, go.
- The README's override of sluice's version goes: an overridden sluice says its tag's version as an overridden Flume or crew does.
- The family-packages spec's requirement reads each tool's version from its tagged tree alone, and its scenario "sluice through its own flake" names a tag whose tree carries its version.

### What stays

- `nix/packages/tag.nix`, which worktrunk's package reads its tag from; the tags check, `nix/tests/tags.nix`, which holds sluice's input to a release tag; sluice's herdr plugin, from the same tag; the worktrunk the set pins, the `v0.80.0` sluice's tree pins at `v0.94.8`.

### Not taken

- **sluice `v0.95.0` or later, the newest releases.** sluice's record 0120, released in `v0.95.0`, has `sluice release` of a worktree sluice holds nothing for run the default branch's release command, making the session's scratch under `/private/tmp/sluice-<uid>` first, which sluice resolves from `/tmp` and no variable. Flume's teardown of a landed place runs that release. In the gate, the runner's test "From crew's launch to main" (`quick_start.rs`) runs under sluice's `sandbox` executor, which cannot write there, so the place's teardown fails, Flume keeps the place, and the test fails. The same test passes with `v0.94.8`, the newest release before record 0120. The move past it waits on sluice, which makes the scratch only where the policy names a release command, or reads it from the gate's own folders.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `family-packages`: sluice through its own flake, unchanged; each tool's version read from its tagged tree.

## Impact

- `flake.nix`, `flake.lock`: sluice at `v0.94.8`.
- `nix/packages/sluice.nix`: removed.
- `nix/packages/tag.nix`: its comment names worktrunk's package as its reader.
- `nix/tests/packages.nix`, `nix/tests/tree-9.9.9`: the cases above.
- `README.md`: the override of sluice's version.
- `openspec/specs/family-packages/spec.md`, by this change's delta once archived.
- No record or rule changes: core.44 already takes a tool from its own flake, unchanged, where its tagged tree carries its version, and record 0020 foresaw sluice's tree carrying it.
