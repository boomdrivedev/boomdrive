# Tasks

## 1. The set

- [x] 1.1 `flake.nix`'s `flume` input names `github:boomdrivedev/flume/v0.35.0`, and the lock moves the `flume` node alone (Decision 3). True when `sluice exec main -- moon run nix:typecheck nix:test` passes.
- [x] 1.2 A case of the set proves that with the runner on, Flume's module declares `flume-host` on nix-darwin and on NixOS. True when `sluice exec main -- moon run nix:test` passes, proving "Flume's resident runs with the runner".

## 2. The runner

- [x] 2.1 No pass runs `flume settle`; `settle.rs` and `flume::Cli::settle_every` go; each pass ends with its ticks under one lock (Decisions 1 and 2). True when `sluice exec main -- moon run boomdrive:test` passes, proving "No pass runs a settle", "A control pass runs no settle", "Passes back to back share their ticks", "A pass ends while a tick is under way", "A pass on a broken configuration ticks" and "Each pass ticks each inbox".
- [x] 2.2 The tests count passes without a settle (Decision 4). True when `sluice exec main -- moon run boomdrive:lint boomdrive:test` passes.

## 3. The words

- [x] 3.1 `README.md` says how a pass ends and that `flume host` keeps the host's watch; core.13 and the details of core.1, core.49 and core.50 lose the settle (Decision 5). True when `sluice exec main -- moon run root:format` passes.
- [x] 3.2 The decision `the-runner-leaves-flumes-watch-to-flume-host` is proposed to Boom, linted with 0 hard findings.

## 4. The specs

- [x] 4.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
