# Proposal

## Why

Record 0054, accepted by the user on 2026-11-19, says each account on a host names its own control plane, so one runner serves crew and a cloud side by side, each over its ordinary connection, and each plane is told of every place in its drives' repositories, a place it did not lease going with no lease. The code still serves one control plane, named by `[control_plane]`'s `type`, and still holds record 0048's watch-only observer: an `Observer` in the interface, its fake, the hosted implementation's observers, `[observer]`, `boomdrive enroll --observer` and an observing hello. The user withdrew record 0048 as a hack on 2026-11-19 and, finding it in the code on 2026-11-24, said: "there is no observing." This change builds record 0054 whole and deletes the observer.

## What Changes

- **Each account names its control plane.** `[accounts.<account>]` takes `control_plane`, the word of an implementation, `crew` or `hosted`, whose table `[control_plane]` holds. `[control_plane]`'s `type` is gone, and refused, naming where each account's plane is named now. `[control_plane]` holds `poll_interval` and the tables of the implementations its accounts name, each named by at least one account. An account that names none is served by no control plane. **BREAKING**: a file naming `type` is refused; each host writes `control_plane` in each account its plane serves, which the flake's module does from `services.boomdrive.accounts.<account>.control_plane`.
- **The runner serves every plane its accounts name, side by side.** `boomdrive run` opens each implementation an account names and carries every part of each on its own thread, as it carries one implementation's parts. A part's label is given once on the host.
- **Every plane is told of every place in its drives' repositories.** Each control pass gives the part's share of the report every place Flume's listing holds in a kept repository that the part did not launch, by its place, with its repository and its status as Flume printed it, and no lease. The implementation cuts what it is given to its own repositories: the hosted implementation to each partition's, crew's to its drive's. No plane acts on a place it did not lease (core.10).
- **Version 5 of the protocol.** Version 5 is version 4 but that it holds no observing hello and no observed report, and that a report names `unleased`, each place the partition did not lease. The runner speaks version 5 alone. Version 4's files stay as release `v0.21.0` spoke them (core.42), and `boomdrive_hosted::protocol::v4` holds its files and no type.
- **crew is told too.** crew's implementation writes `unleased.rec` on its runner branch, each place of its drive's repositories it did not lease, under the descriptor this repository proposes for crew.
- **Each repository's forge tokens come from its accounts' plane.** A repository served only by accounts whose plane holds the App takes that plane's tokens; any other is minted from `[forge]`. `[forge]` is refused only where every account names a plane that holds the App (core.33 as amended with record 0054).
- **The observer is deleted.** `Observer`, `Observed` and the fake `Observing` leave `boomdrive-control`; `observer.rs`, the observer table's role, the observing dial and its checks leave `boomdrive-hosted`; `[observer]`, `boomdrive enroll --observer` and the observers of `boomdrive run` leave the runner. Record 0048 is marked superseded by record 0054.
- **The flake's module** writes each account's `control_plane`, writes crew's table where an account names crew, and fails evaluation on `settings.control_plane.type`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `configuration`: each account's `control_plane`; `[control_plane]` without `type`; no `[observer]`; `[forge]` beside the planes.
- `control-plane-interface`: no observer; a report's unleased places.
- `running`: every plane's parts carried side by side; no observer.
- `work-into-flume`: every place in a part's repositories reported to it.
- `hosted-control-plane`: no observer; version 5; unleased places cut to each partition's repositories.
- `runner-protocol`: version 5; version 4 as released.
- `crew-control-plane`: `unleased.rec`.
- `forge-tokens`: tokens by the repository's accounts' plane.
- `service-modules`: each account's `control_plane`.

## Impact

- `crates/boomdrive-control`: `lib.rs`, `fake.rs`, `report.rs`.
- `crates/boomdrive-hosted`: `observer.rs` deleted; `lib.rs`, `table.rs`, `connection.rs`, `check.rs`, `plane.rs`, `enrollment.rs`, `forge_token.rs`, `protocol/`; `protocol/v5/` added; tests.
- `crates/boomdrive-crew`: `write.rs`, `lib.rs`.
- `crates/boomdrive`: `config.rs`, `planes.rs`, `run.rs`, `carry.rs`, `enroll.rs`, `main.rs`, `setup.rs`, `forge.rs`; tests.
- `nix/modules/options.nix`, `accounts.nix`, `nixos.nix`; `nix/tests`.
- `docs/adr/0048-…`, `docs/adr/0054-…`; `docs/architecture/core.md`, core.42's details; `README.md`, `crates/boomdrive-hosted/protocol/README.md`.
- Boomdrive Cloud must speak version 5 to take this runner's reports.
