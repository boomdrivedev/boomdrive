# Tasks

## 1. The observer goes

- [x] 1.1 `Observer`, `Observed` and `fake::Observing` leave `boomdrive-control`; `observer.rs`, `table::Role`, the observing dial, its checks and refusals leave `boomdrive-hosted`; `[observer]`, `boomdrive enroll --observer`, the runner's observers and `observed_pass` leave `boomdrive`, with their tests (Decision 7). True when `sluice exec main -- moon run boomdrive-control:test boomdrive-hosted:test boomdrive:test` passes and no code of a crate's `src` holds an observer; `protocol/v5.rs` names the observing forms only as what version 5 lacks.
- [x] 1.2 Record 0048 reads superseded by record 0054; core.42's details name version 5.

## 2. Version 5

- [x] 2.1 `protocol/v5/` is version 4 but for the observing forms and `unleased`, with its examples; `protocol/v5.rs` holds the runner's types, and `v4.rs` version 4's files alone; every message and request is of version 5 (Decision 4). True when `sluice exec main -- moon run boomdrive-hosted:test` passes, proving "Version 5 is version 4 whole…", "Version 5's report names each place…", "The runner speaks version 5 alone" and "Version 4 stays as a release spoke it".

## 3. Every place in a part's repositories

- [x] 3.1 `Report` holds `unleased`, never written to the file; the fake holds what it is reported (Decision 3). True when `boomdrive-control:test` passes.
- [x] 3.2 Each control pass gives the part's share every place in a kept repository it did not launch (Decision 3). True when `boomdrive:test` passes, proving the work-into-flume scenarios.
- [x] 3.3 The hosted implementation cuts `unleased` to each partition's repositories (Decision 3). True when `boomdrive-hosted:test` passes.
- [x] 3.4 crew's implementation writes `unleased.rec` (Decision 5). True when `boomdrive-crew:test` passes.

## 4. Each account's plane

- [x] 4.1 The configuration reads each account's `control_plane` and `[control_plane]` without `type` (Decision 1). True when `boomdrive:test` passes, proving the configuration scenarios.
- [x] 4.2 `boomdrive run` opens every plane its accounts name, side by side; `enroll` and `setup` follow (Decisions 1 and 2). True when `boomdrive:test` passes, proving the running and setup scenarios.
- [x] 4.3 Each repository's forge tokens follow its accounts' plane (Decision 6). True when `boomdrive:test` passes, proving the forge-tokens scenarios.
- [x] 4.4 The flake's module writes each account's `control_plane` (Decision 8). True when `sluice exec main -- moon run nix:test nix:typecheck` passes.

## 5. The gate

- [x] 5.1 The fast layer passes. True when `sluice exec main` passes.
