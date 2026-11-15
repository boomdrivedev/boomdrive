# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- **The checks.** Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package or reaches the network. A case named in a task is a key of the `nix/tests` file the task names, which `sluice exec main -- moon run nix:test` prints as `nix: <case>: <what>` where it fails.
- **What no task adds or changes.**
  - No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8). Every `.nix` file written is formatted by nixfmt, as treefmt formats it.
  - The runner's code, its file's keys and `nix/tests/files` do not change.
  - No flake input or tag moves, and no option of crew's is declared here.
  - The quick start's words under `site/src/content/` do not change (site.5).
  - Nothing outside the paths `proposal.md`'s Touches lists is changed.
- **What is already written.** This change's own commit holds record 0032's amendment of 2026-11-15, core.57, and core.18's and core.46's details. No task changes them. A task that relies on a choice the design names cites its Decision.

## 1. The accounts give crew each drive's machine and state

- [x] 1.1 `options.nix` declares an account's `state_repository`, an address `<owner>/<name>` or `null` by default, which `fileOf` writes nowhere. `accounts.nix`'s `drivesOf` gives each drive its account's `state_repository` (design Decisions 1 and 2; Task notes 1.1). `nix/tests/file.nix` gains the case. True when the case "A state repository of no address" of `nix/tests/file.nix` holds, with `sluice exec main -- moon run nix:test` passing.
- [x] 1.2 `accounts.nix` gives `crewDrivesOf`. `nix/modules/crew-drives.nix`, its `_file` `services.boomdrive.accounts`, gives `services.crew.settings.drives` each drive's `machine` and `state` from the accounts with crew's module on, at the options' own priority. `darwin.nix` and `nixos.nix` import it (design Decisions 3 and 4; Task notes 1.2). `nix/tests/settings.nix` writes crew's own keys of a drive apart from its machine and state, with the fixtures `writtenDriveOf`, `crewKeys`, `crewAccounts` and `crewDrives` (design Decision 7). `nix/tests/crew-drives.nix` holds the giving module's cases, and `nix/tests/default.nix` takes it. True when `sluice exec main -- moon run nix:test` passes, proving each on nix-darwin and on NixOS:
  - "Each drive's machine and state repository, given to crew";
  - "Given without the runner";
  - "An account naming no state repository gives crew nothing";
  - "A drive crew's settings do not configure";
  - "A machine written twice" and "A state written twice".

  Every case of `nix/tests/tools.nix`, `kept.nix`, `sources.nix` and `boot.nix` still holds, "The tools' activation is their own" among them.
- [ ] 1.3 `site/src/start/nixos/boomdrive.nix` names the account `me`'s `state_repository`, `your-github-name/drive-state`, and crew's drive `home` writes neither `machine` nor `state`. `nix/tests/quick-start.nix` holds the file to that as it stands (design Decision 8; Task notes 1.3). True when the case "quick start: NixOS's drive" holds, with `sluice exec main -- moon run nix:test` passing.

## 2. The runner's crew table from the accounts

- [ ] 2.1 `options.nix` writes the runner's crew table from the accounts' drives of this host under an account that names its `state_repository`, in the accounts' order. The "no drive" failure's words name the accounts' machines, and the `machine` option's description, the table's `defaultText` and `description` and the header comment say so (design Decisions 5 and 6; Task notes 2.1). `settings.nix`'s `crew` account names `state_repository = "example/state"`, and the fixture `elsewhere` and `crewElsewhere` read the accounts. `crewFleet` and `crewOrder` are gone. The crew table's cases in `darwin.nix`, `nixos.nix` and `tools.nix` read the accounts, renamed as design Decision 7 has them. True when `sluice exec main -- moon run nix:test` passes, proving each on nix-darwin and on NixOS:
  - "A host under crew";
  - "The accounts' drives on this host";
  - "A table written out wins";
  - "A hosted control plane beside crew" and "No control plane beside crew";
  - "The accounts' drives in their order";
  - "Under crew with neither" and "No drive of the accounts on this host";
  - "quick start: NixOS's drive".
- [ ] 2.2 Under crew with the runner and crew's module on, `options.nix` fails an account that names a drive and no `state_repository`, naming its option. It fails a drive of `services.crew.settings.drives` that no account names, naming it and `services.boomdrive.accounts` (design Decision 6; Task notes 2.2). `settings.nix` gains `stateless` and `crewStray`, and `tools.nix` the two cases. True when `sluice exec main -- moon run nix:test` passes, proving "An account under crew with no state repository" and "A drive of crew's no account names", each on nix-darwin and on NixOS.

## 3. The words and the specs

- [ ] 3.1 The README's **Installing with nix** writes the account's `state_repository` in its nix block, and neither `state` nor `machine` in crew's drive. Its words say what the module gives crew and writes of the runner's crew table, and what fails (design Decision 9; Task notes 3.1). `tools.nix`'s "The README's crew table" holds that no nix block names `control_plane.crew` or `partitions`, and "The README's machine and state" is new. True when `sluice exec main -- moon run nix:test` passes, proving "The README's machine and state", with "The README's crew table", "The README's crew settings", "The README's words", "The README's accounts" and "The README's lines" still holding; and `devenv shell -- treefmt --ci README.md` passes.
- [ ] 3.2 The nix files this change adds or changes keep to the nix project's format and lint (Task notes 3.2). True when `sluice exec main -- moon run nix:format` and `sluice exec main -- moon run nix:lint` pass.
- [ ] 3.3 The change's deltas archive into specs that pass openspec's strict check. That covers, in `service-modules`, the three requirements modified, the two removed and the eight added, and `quick-start`'s one modified. True when, after `devenv shell -- openspec archive a-drives-machine-and-state-are-named-once --yes`, `sluice exec main -- moon run root:specs` passes.
