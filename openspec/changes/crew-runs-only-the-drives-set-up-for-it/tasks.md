# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- **The checks.** Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package or reaches the network. A case named in a task is a key of the `nix/tests` file the task names, which `sluice exec main -- moon run nix:test` prints as `nix: <case>: <what>` where it fails.
- **What no task adds or changes.**
  - No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8). Every `.nix` file written is formatted by nixfmt, as treefmt formats it.
  - The runner's code, its file's keys, `nix/tests/files` and `nix/tests/examples` do not change.
  - No flake input or tag moves, and no option crew does not declare is declared here.
  - Nothing outside the paths `proposal.md`'s Touches lists is changed.
- **What is already written.** The change's design holds record 0032's forms of 2026-11-18, core.57, and core.18's and core.46's details. No task changes them. A task that relies on a choice the design names cites its Decision.

## 1. crew is given only the drives its settings set up

- [x] 1.1 `nix/tests/tools.nix`'s case "Each tool's options are its own flake's" fails an option of `services.flume`, `services.sluice`, `services.crew` or `services.timber` with no declaration in its tool's input, and one declared again in any file of this tree but `nix/modules/crew-drives.nix`'s declaration of `services.crew.settings` and its `drives`. It checks every other option of crew's settings, and every option of a drive of crew's, the same way (design Decisions 1 and 5; Task notes 1.1). True when `sluice exec main -- moon run nix:test` passes, proving each on nix-darwin and on NixOS:
  - "Each tool's options are its own flake's";
  - "The tools' modules re-exported".
- [x] 1.2 `nix/modules/crew-drives.nix` adds the giving module to the type of crew's drives, in place of defining `services.crew.settings.drives`. That module gives `machine` and `state` from `crewDrivesOf` only to a drive crew's settings set up, with crew's module on, each definition placed at `services.boomdrive.accounts`. `accounts.nix`'s header comment says so (design Decision 1; Task notes 1.2). `nix/tests/crew-drives.nix` reads each given value's place from `valueMeta` by `placedFor`. Its case "A drive crew's settings do not configure" holds crew's drives to be `home` alone, crew's teams file to evaluate and the system to fail no assertion (design Decision 5). True when `sluice exec main -- moon run nix:test` passes, proving each on nix-darwin and on NixOS:
  - "Each drive's machine and state repository, given to crew";
  - "Given without the runner";
  - "An account naming no state repository gives crew nothing";
  - "A drive crew's settings do not configure";
  - "A machine written twice" and "A state written twice".

  Every case of `nix/tests/tools.nix`, `kept.nix`, `sources.nix`, `quick-start.nix` and `boot.nix` still holds.
- [x] 1.3 The README's **Installing with nix** says that, with crew's module on, each drive crew's settings set up, under an account that names its `state_repository`, is given its `machine` and its account's `state_repository`. It says a drive crew's settings do not set up is given nothing and is kept, with its stream, as any drive. The paragraph on a drive's `machine` says the same (design Decision 6). True when `sluice exec main -- moon run root:format` passes.

## 2. The runner's crew table and the state repository's failure

- [ ] 2.1 `options.nix` writes the runner's crew table from this host's drives that crew's settings set up, under an account that names its `state_repository`. Its "no drive" failure names the machines of the drives crew's settings set up, in Decision 4's words. Its header comment, the `machine` and `state_repository` options' descriptions and the table's `defaultText` and `description` say so (design Decisions 2 and 4; Task notes 2.1). `nix/tests/settings.nix` gains `elsewhereHome`. `nix/tests/tools.nix` gains the cases "Only the drives crew's settings set up" and "No drive crew runs on this host" (design Decision 5). True when `sluice exec main -- moon run nix:test` passes, proving each on nix-darwin and on NixOS:
  - "Only the drives crew's settings set up" and "No drive crew runs on this host";
  - "The accounts' drives on this host", "The accounts' drives in their order" and "A table written out wins";
  - "No drive of the accounts on this host" and "Under crew with neither";
  - "A host under crew";
  - "quick start: NixOS's drive".
- [ ] 2.2 Under crew with the runner and crew's module on, `options.nix` fails an account that names no `state_repository` only where it holds a drive crew's settings set up, in Decision 3's words (design Decision 3; Task notes 2.2). `nix/tests/settings.nix` gains `reading`, and `nix/tests/tools.nix` the case "An account whose drives crew's settings do not set up" (design Decision 5). True when `sluice exec main -- moon run nix:test` passes, proving each on nix-darwin and on NixOS:
  - "An account whose drives crew's settings do not set up";
  - "An account under crew with no state repository";
  - "A drive of crew's no account names".
- [ ] 2.3 The README's **Installing with nix** says the runner's `[control_plane.crew]` holds this host's drives that crew's settings set up. It says that under crew an account needs a `state_repository` only for a drive crew's settings set up, and that a drive of crew's no account names still fails (design Decision 6). True when `sluice exec main -- moon run root:format` passes.

## 3. The format and the specs

- [ ] 3.1 The nix files this change changes keep to the nix project's format and lint (Task notes 3.1). True when `sluice exec main -- moon run nix:format` and `sluice exec main -- moon run nix:lint` pass.
- [ ] 3.2 The change's deltas archive into specs that pass OpenSpec's strict check. In `service-modules`, two requirements are renamed and eight modified (core.61). True when, after `devenv shell -- openspec archive crew-runs-only-the-drives-set-up-for-it --yes`, `sluice exec main -- moon run root:specs root:changes` passes.
