# Verify: each-drive-opens-from-the-launcher

The work on `change/each-drive-opens-from-the-launcher` was read at 3a2392f. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at 678ccfc. Every `check-fast` task it ran passed.
- The change's deltas add 9 requirements and modify 3, with 22 scenarios. A case that passed in the gate asserts the outcome of each scenario.
- All 13 tasks are ticked. The code stage archived the change at 3a2392f, as task 5.1 says to. This report sits beside the archived change.
- The branch's base is 57b1d1e, where `bolt/each-drive-opens-from-the-launcher` stands at origin. `main` is at 678ccfc, and the branch merges into it with no conflict.

## Checks

- **The gate.** `sluice exec main` at 3a2392f ran the `check-fast` tasks that 3a2392f affects against `main` at 678ccfc. It exited 0, and sluice recorded 35 results under their keys for 3a2392f. moon completed 37 tasks.
  - Passed:
    - `boomdrive`, `boomdrive-bounded`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test` each, with 936 of the crates' tests passed and none failed;
    - `nix:format`, `lint`, `typecheck` and `test`, which said "every case holds";
    - `root:format`, `adrs`, `specs` (23 of 23), `changes`, `releases` and `workflow`;
    - `site:install`, `check`, `prose`, `vocab`, `build`, `test` and `workflow`.
  - `root:toolchain` was not among the tasks sluice found affected. It is task 2.1's own check, so it was run alone, as the task names it: `sluice exec main -- moon run root:toolchain` passed, naming `clang-format is 21.1.8`, the pin `devenv.nix` gives.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install. None of these touched a task's result.
- **The build.** Task 2.4's check, `nix build --no-link .#packages.aarch64-darwin.boomdrive-launcher`, was run on this `aarch64-darwin` host from the place's root, and succeeded. The bundle holds `Contents/Info.plist`, `Contents/MacOS/boomdrive-launcher` and `Contents/MacOS/boomdrive-menu`. The menu is an arm64 Mach-O executable, signed ad hoc, and the `Info.plist` gives `BoomdriveDrives` as an empty array.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 13 tasks of `tasks.md`, groups 1 to 5, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it.

- **The script** (1.1). `nix/packages/launcher.sh` sets `open` for no argument and `open -- "$1"` for one. For more than one it says `boomdrive-launcher: one session at most` on standard error and exits 2, before it runs anything. Its profiles, alert and exit 1 on a refusal are as they were. `nix/tests/way-in.sh` passes each case's arguments to the launcher and holds "A drive's session opened" and "More than one session".
- **clang-format** (2.1). `devenv.nix` pins `clangFormatVersion = "21.1.8"` and asserts it against `pkgs.clang-tools`. `check-toolchain.sh` holds `clang-format --version` to the pin. `treefmt.nix` turns on `programs.clang-format` for `nix/packages/*.m`.
- **The menu** (2.2). `nix/packages/launcher.m` is Objective-C with ARC, on AppKit alone. It sets the accessory activation policy and titles its status item `Boomdrive`. Its menu holds an item for each dictionary of `BoomdriveDrives` whose `drive` and `session` are strings, titled the drive, then `Default stream`, a separator, and `Quit Boomdrive` on `q`. An item starts the bundle's auxiliary executable `boomdrive-launcher` by `NSTask` with its session, or with none, and does not wait. Where the script cannot be started, an alert titled `Boomdrive` gives the error. `root:format`, which runs treefmt, passed over it.
- **The package** (2.3, 2.5). `nix/packages/launcher.nix` takes `drives ? [ ]`.
  - The menu is a derivation of its own, compiled with `-fobjc-arc -Wall -Wextra -Werror -framework AppKit` into `Contents/MacOS/boomdrive-menu`, and takes no drive.
  - `passthru.info` holds design Decision 3's keys, and the `Info.plist` is `lib.generators.toPlist { escape = true; }` of it.
  - On Linux each drive is an action whose id and name are the drive's, and whose `Exec` quotes the session as Decision 4 says. The entry's own `Exec` is the script with no argument.
  - Evaluated for `my $work`, the action's line is `Exec=…/bin/boomdrive-launcher "my \\$work"`. A made plist whose session holds `&` and `<` passes `plutil -lint`.
- **The modules** (3.1, 3.2).
  - `nix/modules/accounts.nix` gives `hereOf`, and `drives.nix` takes its `here` from it.
  - `nix/modules/apps.nix` reads `services.crew.host` as `drives.nix` does, and each stream's `session` from `services.flume.settings.streams`. It sorts the entries by drive name and defaults `launcher.package` to `set.boomdrive-launcher.override { drives = entries; }`.
  - The option's `defaultText` and description say each drive of this host is an entry, and a host's own package is installed as given.
- **The cache** (3.3). `nix/tests/cache.nix` holds "A host's drives in the launcher", which compares `drvPath`s and, on nix-darwin, the menu's `outPath`. "A host's own nixpkgs" takes the same arguments as before.
- **The words** (4.1, 4.2).
  - The README's **Installing with nix** says the launcher is made for each drive of the host: on macOS a menu-bar app with an entry for each drive, then `Default stream` and `Quit Boomdrive`, and on Linux an action for each drive, the entry's own click being `Default stream`. It also says a drive of another machine has no entry, the menu is quit and opened again after a switch that changes the drives, and a host's own launcher gets no entries.
  - Task 4.2's check printed nothing for `CLAUDE.md` and `openspec/config.yaml`.
- **The specs** (5.1). The change is archived at `openspec/changes/archive/2026-11-11-each-drive-opens-from-the-launcher`. Its requirements are in `openspec/specs/way-in`, `service-modules` and `binary-cache`, and `root:specs` passed 23 of 23.

The record and the rules the tasks say were written with the change are in the tree: record 0023's "Amendment, 2026-11-11: each drive opens from the launcher", and the details of core.13, core.44, core.46 and core.47, each citing record 0023 as amended on 2026-11-11.

## Requirements and scenarios

Each scenario's case is named for it, and each passed in the gate. Cases in `nix/tests/apps.nix` and `nix/tests/cache.nix` run on nix-darwin and on NixOS.

| Spec | Requirement | Scenario | Case |
|---|---|---|---|
| way-in | The launcher opens the host's agents at a person's asking (modified) | Opened | `way-in.sh` "Opened" |
| | | A drive's session opened | `way-in.sh` "A drive's session opened" |
| | | The profiles first | `way-in.sh` "The profiles first" |
| way-in | The launcher is an app on macOS and a desktop entry on Linux (modified) | The app on macOS | `way-in.nix` "The app on macOS" |
| | | The desktop entry on Linux | `way-in.nix` "The desktop entry on Linux" |
| way-in | The launcher opens one session at most | More than one session | `way-in.sh` "More than one session" |
| way-in | The menu runs the script for the entry a person picks | The menu's source | `way-in.nix` "The menu's source" |
| way-in | The launcher is made for the drives it is given | No drive | `way-in.nix` "No drive" |
| way-in | On macOS each drive is in the menu's Info.plist | Each drive in the menu on macOS | `way-in.nix` "Each drive in the menu on macOS" |
| way-in | On Linux each drive is an action of the desktop entry | Each drive an action on Linux | `way-in.nix` "Each drive an action on Linux" |
| | | A session quoted in its action | `way-in.nix` "A session quoted in its action" |
| service-modules | The modules install herdr, Ghostty and the launcher, each replaceable or dropped (modified) | The way in with the runner | `apps.nix` "The way in with the runner" |
| | | A host's own herdr | `apps.nix` "A host's own herdr" |
| | | No Ghostty of the set's | `apps.nix` "No Ghostty of the set's" |
| | | A host's own launcher | `apps.nix` "A host's own launcher" |
| | | Not without the runner | `apps.nix` "Not without the runner" |
| service-modules | The launcher has an entry for each drive of this host | Each drive of this host in the launcher | `apps.nix` "Each drive of this host in the launcher" |
| | | A drive of another machine has no entry | `apps.nix` "A drive of another machine has no entry" |
| | | No drive, the set's launcher | `apps.nix` "No drive, the set's launcher" |
| service-modules | A drive's entry opens its stream's own session | A stream's own session named | `apps.nix` "A stream's own session named" |
| service-modules | The README says the launcher opens each drive | The README's launcher | `cache.nix` "The README's launcher" |
| binary-cache | A host's launcher is the flake's own by its override | A host's drives in the launcher | `cache.nix` "A host's drives in the launcher" |

## Scope

The branch's commits since 57b1d1e change only the paths the proposal's Touches lists, and the specs the archive merges into. The runner's code does not change, no crate is added, and the one file type added, `.m`, is formatted by clang-format in the gate (core.7, core.8).

## Left to a person

What a click on the menu does is checked by no task of the gate, as the design's Risks say. On a Mac with the modules switched in, a person opens `Boomdrive` and picks a drive's entry, `Default stream` and `Quit Boomdrive`. A drive's entry and `Default stream` should each show the session they name through `flume open`, and `Quit Boomdrive` should end the menu. This verify did not open the menu.
