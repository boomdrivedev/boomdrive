# Tasks

Work through the tasks in order; each stands on the ones before it. The groups are headings for reading, and none is checked as a whole: the verify stage runs the repository's whole checks once, after every task is done. A task is done once the check it names passes, run exactly as it names it. A test binary a task does not name may fail to build between tasks while a signature it calls is being moved; the task that moves its callers names it.

- A quick check runs as written, `devenv shell -- cargo test -p boomdrive …`; a project's task runs through sluice, `sluice exec main -- moon run <project>:<task>` (CLAUDE.md).
- Every test runs on the scratch host of `crates/boomdrive/tests/common`, or on its crate's unit fixtures, with neutral names (core.41). No test reads the user's configuration or reaches the network.
- Every key, type and function a task adds names what it holds (core.12), as the design's decisions name them.
- No refusal or line quotes a file's text or a value (core.4, core.11).
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. The record and the rules

- [x] 1.1 Record 0009 gains "Amendment, 2026-11-07: how the runner reads its two layers". It is the writing agent's, made within the amendment of 2026-11-06 that the user accepted, and holds design Decisions 1 to 7, each with the path not taken. core.17's details gain what design Decision 11 names. core.5's and core.46's details read "local layer" where they say the managed level of record 0009's first words, citing the record's amendment of 2026-11-06. Done when `sluice exec main -- moon run root:format` passes.

## 2. Each implementation lists its local layer's keys

- [x] 2.1 `boomdrive_crew::config::LOCAL_KEYS` and `boomdrive_hosted::table::LOCAL_KEYS` each list `partitions`. `planes::Entry` gains `local_keys`, the implementation's list, and `read`, its own reading of its table, which opens nothing: crew's `config::read`, and the hosted `Table::read` (design Decision 2). Done when `devenv shell -- cargo test -p boomdrive --test planes each_implementation_lists_partitions_for_the_local_layer` passes. That new test asserts, for every entry of `planes::PLANES`, that its `local_keys` are `["partitions"]` and that its `read` refuses a partition with no `label`.

## 3. The configuration reads its two layers

- [x] 3.1 The configuration reads a local layer over its managed one (design Decisions 1, 3 and 4):
  - `config::local_path(&Env)` gives `config.local.toml` in the default file's directory.
  - `config::read_layers(managed, local, home)` reads both files, and `config::parse_layers` reads both texts. `config::read` and `config::parse` stay the reading of one file, the managed layer alone, its `local` `NoFile`.
  - `Config` gains `local: LocalLayer`, with `LocalLayer::{NoFile, Taken, NotTaken}` and `PinnedKey`.
  - The local layer's `repositories` add to the managed layer's, and its `control_plane.<word>.partitions` replace the managed layer's.
  - The `Config` literals of `carry.rs`'s unit tests and of `tests/pass.rs` name `local: LocalLayer::NoFile`.

  Done when `devenv shell -- cargo test -p boomdrive --lib config::tests` passes, with:
  - a unit test for each of the configuration spec's "No local file", "The local layer's repositories", "A repository both layers name" and "The local layer's partitions";
  - `local_path` under `XDG_CONFIG_HOME` set, unset and not absolute.
- [x] 3.2 A local layer at fault is not taken (design Decision 4, steps 3, 4 and 6):
  - one that sets a key the runner does not list, each named by its dotted path, the shortest holding no listed key;
  - one that is not TOML;
  - one holding a value the runner's reading, or the implementation's `read`, refuses.

  The configuration is then the managed layer's, its `local` `NotTaken` with the keys or the fault, each fault naming the local file. Done when `devenv shell -- cargo test -p boomdrive --lib config::tests` passes, with:
  - a unit test for each of the configuration spec's "A root in the local layer", "A control plane named in the local layer", "A token in the local layer", "A local file that is not TOML", "A local address out of form" and "A partition the implementation refuses";
  - a local layer naming `control_plane.hosted.partitions` under a managed `type = "crew"`, not taken;
  - a local layer naming two entries that are one repository, not taken.
- [x] 3.3 The managed layer's `pin` holds keys against the local layer, a key it holds passed over and named with its pin, and a pin of nothing refused as the managed layer's fault (design Decision 6). Done when `devenv shell -- cargo test -p boomdrive --lib config::tests` passes, with:
  - a unit test for each of the configuration spec's "Repositories pinned", "A table pinned" and "A pin of nothing";
  - a `pin` that is no list of strings, refused;
  - `pin` set in the local layer, not taken.

## 4. Every command reads both layers

- [x] 4.1 `Runner` holds the local layer's path, and its start, every keeping pass, every control pass and every landing read both layers through `config::read_layers`. The start line's count is both layers' repositories (design Decisions 7 and 8). `tests/common/mod.rs` gains `Scratch::local_file()` and `Scratch::configure_local(text)`. Done when `devenv shell -- cargo test -p boomdrive --test running` passes, with:
  - `the_default_file` and `the_file_named` each writing a local layer that keeps a second repository and asserting its clone made, proving the configuration spec's "The default file" and "The file named";
  - `no_local_file` proving "No local file".
- [ ] 4.2 `boomdrive enroll` reads both layers through `config::read_layers`, so no command of the runner's reads through `config::read`, which stays the reading of one file that `tests/module_files.rs` reads the modules' files with (design Decision 8). Done when `devenv shell -- cargo test -p boomdrive --test enroll a_partition_of_the_local_layer` passes. That new test proves the configuration spec's "A partition enrolled from the local layer".

## 5. A keeping pass under the local layer

- [ ] 5.1 A keeping pass says one line of the local layer where its reading differs from the last one said, `Runner` keeping the state said, in design Decision 7's words. `run --once`'s pass says it too. Done when `devenv shell -- cargo test -p boomdrive --test local_layer` passes. That new test file proves the configuration spec's "Taken at the start" and "A fault said once", and a running runner whose pinned `repositories` are passed over saying so once across passes.
- [ ] 5.2 While `config.local` is `NotTaken`, a keeping pass keeps the managed layer's repositories, lets go of none and sets no workspace (design Decision 5). Done when `devenv shell -- cargo test -p boomdrive --test local_layer` passes, proving the configuration spec's:
  - "A local file broken while running", with `code_workspaces = true` and the owner's workspace's `folders` asserted unchanged while the layer is not taken;
  - "A repository the local layer drops";
  - "A local file removed", with the gone line said once.

## 6. The modules leave the local layer

- [ ] 6.1 `nix/tests/activate.sh` gains "A local layer beside it": the activation, run for a home whose `.config/boomdrive` holds a `config.local.toml`, links `config.toml` and leaves that file's text unchanged (design Decision 9). Done when `sluice exec main -- moon run nix:test` passes, proving the service-modules spec's "A local layer beside it".

## 7. README

- [ ] 7.1 README's configuration section says what design Decision 10 lists, and shows a local layer in a second ```toml block after the configuration's. Done when `devenv shell -- cargo test -p boomdrive --test readme` passes, with `the_readme_local_layer_reads` reading that block over the first through `config::parse_layers`, and asserting both layers' repositories kept and the local layer taken.

## 8. The specs

- [ ] 8.1 The change is archived into `openspec/specs`, and the specs it changes pass openspec's strict check. Done when, after `devenv shell -- openspec archive the-runner-reads-a-local-layer-over-its-managed-one --yes`, `sluice exec main -- moon run root:specs` passes.
