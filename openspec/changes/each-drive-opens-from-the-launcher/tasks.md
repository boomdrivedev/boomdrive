# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. Every sh case runs stubs of Flume, `osascript`, `uname` and `id` in scratch folders, as `nix/tests/way-in.sh` does. No case runs a real Flume or `osascript`.
- One task builds: 2.4 compiles the menu with `nix build` on an `aarch64-darwin` host, outside the gate's sandbox, since the gate never builds (design Decision 7).
- The runner's code does not change. No crate is added. The one file type added is `.m`, formatted by clang-format, which task 2.1 adds to the gate (core.7, core.8). Nothing outside the paths proposal.md's Touches lists is changed.
- Record 0023's amendment of 2026-11-11 and the details of core.13, core.44, core.46 and core.47 were written in the commit that wrote this change, and no task changes them. A task that relies on a choice the design names cites its Decision.

## 1. The script

- [x] 1.1 `nix/packages/launcher.sh` runs `flume open -- <session>` given one argument and `flume open` given none. Given more than one, it runs nothing, says `boomdrive-launcher: one session at most` on standard error and exits 2. Its profiles, its alert and its exit on a refusal stay as they are. `nix/tests/way-in.sh` passes a case's arguments to the launcher, and gains "A drive's session opened" and "More than one session" (design Decision 2; Task notes 1.1). True when `devenv shell -- sh nix/tests/way-in.sh` prints nothing, proving the way-in delta's "Opened", "A drive's session opened", "More than one session" and "The profiles first", and the refusals' cases as they stand.

## 2. The launcher

- [x] 2.1 clang-format joins the gate: `devenv.nix` pins `clangFormatVersion` to its nixpkgs' `clang-tools` and asserts it, `check-toolchain.sh` holds `clang-format --version` to the pin, and `treefmt.nix` formats `nix/packages/*.m` with it (design Decision 6; Task notes 2.1). True when `sluice exec main -- moon run root:toolchain` passes, naming clang-format at its pin.
- [x] 2.2 `nix/packages/launcher.m` is the menu-bar app of design Decision 3: a status item `Boomdrive`, an item for each dictionary of `BoomdriveDrives`, `Default stream` and `Quit Boomdrive`, each drive's item running the bundle's `boomdrive-launcher` with its session, not waited on (Task notes 2.2). True when `devenv shell -- treefmt --ci nix/packages/launcher.m` passes.
- [x] 2.3 `nix/packages/launcher.nix` takes `drives ? [ ]`. On `aarch64-darwin` its menu part compiles `launcher.m` into `Contents/MacOS/boomdrive-menu`, taking no drive. Its `passthru.info` gives Decision 3's keys with `BoomdriveDrives` the drives, and its `Info.plist` is `lib.generators.toPlist { escape = true; }` of it. The script stays at `Contents/MacOS/boomdrive-launcher`. `nix/tests/way-in.nix`'s "The app on macOS" reads the new bundle, and the file gains "Each drive in the menu on macOS", "No drive" and "The menu's source" (design Decisions 3 and 5; Task notes 2.3). True when `sluice exec main -- moon run nix:test` passes, proving the way-in delta's "The app on macOS", "Each drive in the menu on macOS", "No drive" and "The menu's source".
- [x] 2.4 The `aarch64-darwin` launcher builds: the menu compiles with `-Werror` and the bundle is assembled (design Decision 7; Task notes 2.4). True when `nix build --no-link .#packages.aarch64-darwin.boomdrive-launcher` succeeds on an `aarch64-darwin` host, run from the place's root.
- [x] 2.5 On Linux, the desktop entry gains one action for each drive, its id and name the drive's, running the script with the drive's session quoted as design Decision 4 says. The entry's own `Exec` stays the script with no argument. `nix/tests/way-in.nix` gains "Each drive an action on Linux" and "A session quoted in its action" (Task notes 2.5). True when `sluice exec main -- moon run nix:test` passes, proving the way-in delta's "Each drive an action on Linux", "A session quoted in its action" and "The desktop entry on Linux", and "No drive" on Linux.

## 3. The modules

- [x] 3.1 `nix/modules/accounts.nix` gives `hereOf`, the drives of this host, and `nix/modules/drives.nix` takes its `here` from it (design Decision 5; Task notes 3.1). True when `sluice exec main -- moon run nix:test` passes, with the service-modules spec's "A drive of another machine" and "A drive of this machine" holding on nix-darwin and on NixOS.
- [ ] 3.2 `nix/modules/apps.nix`'s `services.boomdrive.launcher.package` defaults to the set's launcher made for this host's entries: each drive of this host in name order, its session the stream's `session` in Flume's settings, else the stream's name. A host's own package is installed as given. Its `defaultText` and description say so. `nix/tests/apps.nix` gains the delta's cases (design Decisions 1 and 5; Task notes 3.2). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "Each drive of this host in the launcher", "A drive of another machine has no entry", "A stream's own session named", "No drive, the set's launcher", "A host's own launcher" and "The way in with the runner", on nix-darwin and on NixOS.
- [ ] 3.3 `nix/tests/cache.nix` gains "A host's drives in the launcher", and "A host's own nixpkgs" holds unchanged (design Decision 5; Task notes 3.3). True when `sluice exec main -- moon run nix:test` passes, proving the binary-cache delta's "A host's drives in the launcher" and "A host's own nixpkgs" on nix-darwin and on NixOS.

## 4. The words

- [ ] 4.1 The README's **Installing with nix** says the set's launcher is a menu-bar app on macOS and a desktop entry on Linux. It says the modules install it made for each drive of the host, with `Default stream`, that a drive of another machine has no entry, that the menu is quit and opened again after a switch that changes the drives, and that a host's own package gets no entries. `nix/tests/cache.nix` gains "The README's launcher" (Task notes 4.1). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "The README's launcher".
- [ ] 4.2 CLAUDE.md's paragraph on the flake and `openspec/config.yaml`'s stack line name the launcher a menu-bar app on macOS and a desktop entry on Linux, with an entry for each drive (Task notes 4.2). True when `for f in CLAUDE.md openspec/config.yaml; do tr -s ' \n' '  ' < "$f" | grep -q 'with an entry for each drive' || echo "$f"; done` prints nothing.

## 5. The specs

- [ ] 5.1 The change's deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive each-drive-opens-from-the-launcher --yes`, `sluice exec main -- moon run root:specs` passes.
