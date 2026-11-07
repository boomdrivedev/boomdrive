# Proposal

## Why

The brief, 2026-11-07: "The runner reads its local layer, config.local.toml, over its managed one, taking from it only the keys it lists for it, its repositories added to the managed layer's, and a key the managed layer pins kept."

Record 0009's amendment of 2026-11-06, accepted by the user that day, gives every tool of the family two layers in Flume's words (design/principles.md 10; flume record 0032). The managed layer, `config.toml`, is written by Nix, a fleet or a person. The local layer, `config.local.toml` beside it, is written by the operator or a control plane, and read over it. In the user's words: "Flume's: managed + local", "Flume's rule for all", and for repositories under the cloud, "Add to Nix's". Rules core.17 and core.18 were amended with it.

The runner still reads one file. A hosted control plane has nowhere to write the repositories of the drives a host serves without rewriting the file Nix wrote, which core.18 rules out.

## What Changes

- **Two layers.** The runner reads its managed layer from the file `--config` names, or its default `config.toml`, as today. It reads its local layer from `config.local.toml` in its own configuration directory, `$XDG_CONFIG_HOME/boomdrive/` or `~/.config/boomdrive/`, whatever `--config` names. A host with no local file reads the managed layer alone, as today.
- **What the local layer may set.** It may set `repositories`, and, in a control plane's table, the keys that implementation lists: `partitions`, for crew's and for the hosted one's. Each is one key, taken whole.
- **Repositories add.** The runner keeps every repository either layer names: the managed layer's first, in their order, then the local layer's. A repository both name is kept once. A local layer's `partitions` replace the managed layer's.
- **Pins.** The managed layer's `pin`, a list of dotted keys such as `"repositories"` or `"control_plane.hosted"`, holds each key it names against the local layer. A pinned key the local layer sets is passed over and named, and the rest is taken. A pin of nothing the local layer may set is the managed layer's fault.
- **A local layer at fault is not taken.** This covers a local file that sets any other key, is not TOML, or holds a value the runner's reading or the implementation's refuses. The managed layer then runs alone. While such a file stands, a pass lets go of no repository and sets no workspace, since the layer it could not take may name them.
- **Said once each way.** A pass says one line naming the local file when its reading changes: taken, with each key passed over and its pin; not taken, with each key or the fault; or gone.
- **Every command reads both layers.** That covers the keeping pass, the control pass, a landing and the start's `[control_plane]` table, so `boomdrive enroll` enrolls a partition the local layer names.
- **The modules leave the local layer.** Each module's activation links `config.toml` alone and leaves `config.local.toml` as it is. A host pins through `settings.pin`, which the freeform `settings` already writes.
- **Record 0009** gains an amendment of 2026-11-07 holding this design's choices, the writing agent's. core.17's details follow it. core.5's and core.46's details read "local layer" where they still say the managed level of record 0009's first words.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `configuration`: the configuration is a managed file and a local one over it. The local layer sets the repositories and a control plane's partitions alone, its repositories adding to the managed layer's. The managed layer may pin. A local layer at fault is not taken and lets go of nothing. Its reading is said once each way.
- `service-modules`: the activation leaves the local layer as it is.

## Impact

- `crates/boomdrive`:
  - `src/config.rs`: the local layer's path, the layered reading, `pin`, and the local layer's state on `Config`.
  - `src/planes.rs`: each implementation's entry gives the keys of its table the local layer may set, and its reading of the table.
  - `src/run.rs` and `src/enroll.rs`: both layers at every reading, and the local layer's line once each way.
  - `src/pass.rs`: no let-go and no workspace set while the local layer is not taken.
  - `src/carry.rs`: its unit tests' `Config`.
  - Tests: `tests/common/mod.rs`, `tests/running.rs`, `tests/enroll.rs`, `tests/planes.rs`, `tests/pass.rs`, `tests/readme.rs`, and a new `tests/local_layer.rs`.
- `crates/boomdrive-crew/src/config.rs` and `crates/boomdrive-hosted/src/table.rs`: each gives the keys of its own table that a local layer may set, `partitions` (core.2).
- `nix/tests/activate.sh`: the activation leaves a local layer beside the link.
- `README.md`: the two layers, with an example of a local layer.
- `docs/adr/0009-…`, its amendment of 2026-11-07, and `docs/architecture/core.md`: the details of core.5, core.17 and core.46.
- Nothing else changes:
  - no protocol message, report field or interface (core.2, core.42);
  - no crate, tool or tracked file type (core.7, core.8);
  - no module option, since `settings` is freeform at its top.
- A host with no `config.local.toml` reads exactly as today.

## Touches

```
docs/adr/0009-a-host-is-set-up-through-each-tools-own-commands-and-its-repositories-follow-its-drives.md
docs/architecture/core.md
crates/boomdrive/src/config.rs
crates/boomdrive/src/planes.rs
crates/boomdrive/src/run.rs
crates/boomdrive/src/enroll.rs
crates/boomdrive/src/pass.rs
crates/boomdrive/src/carry.rs
crates/boomdrive-crew/src/config.rs
crates/boomdrive-hosted/src/table.rs
crates/boomdrive/tests/common/mod.rs
crates/boomdrive/tests/running.rs
crates/boomdrive/tests/enroll.rs
crates/boomdrive/tests/planes.rs
crates/boomdrive/tests/pass.rs
crates/boomdrive/tests/local_layer.rs
crates/boomdrive/tests/readme.rs
nix/tests/activate.sh
README.md
```
