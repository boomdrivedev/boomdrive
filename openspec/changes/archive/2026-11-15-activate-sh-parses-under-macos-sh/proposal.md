# Proposal

## Why

The brief: "nix/modules/activate.sh does not parse under macOS's /bin/sh (bash 3.2.57): '/bin/sh -n nix/modules/activate.sh' on main 23e41810 prints 'nix/modules/activate.sh: line 138: syntax error near unexpected token `)'' at '    ); then'. In plugins(), 'case $line in HERDR_*=*) ... esac' sits inside $( ), and bash 3.2 reads the pattern's bare ')' as closing the substitution, so the whole file fails and every Mac activation's 'activate.sh link', 'pendingyou' and 'plugins' steps have failed silently behind '|| true' since e66a3d72 (2026-11-08); on deneb ~/.config/boomdrive/config.toml stayed a stale regular file. Give each case pattern inside a command substitution its POSIX leading paren ('(HERDR_*=*)'), and add a moon check task that parses activate.sh with macOS's /bin/sh (or bash 3.2) so a script the Macs cannot read fails the gate."

The gate never saw it: `nix/tests/activate.sh` runs the activation's cases by the shell's `sh`, which is nixpkgs' bash 5, and bash 5 reads a `case` inside `$( )` as the case it is.

## What Changes

- `activate.sh`'s `case` leaves the command substitution: it moves into a function of its own, `herdrs`, which prints the names of the environment's `HERDR_*` variables and is defined at the top level, where bash 3.2 reads it, and called inside `$( )`. The plugins step does what it did.
- A check-fast task of the nix project, `nix:mac-sh`, reads every script under `nix/modules` and `nix/packages`, `activate.sh` and `launcher.sh`, with bash 3.2.57 in POSIX mode and `-n`, running none, and fails naming each script it cannot read and bash's words.
- `devenv.nix` pins that bash, built from GNU's release of 3.2.57, and gives it by `BOOMDRIVE_MAC_SH`, on no PATH; `root:toolchain` asserts its version.
- A requirement of its own, "A Mac's /bin/sh reads every script the modules and packages give it", says so, with two scenarios.
- Record 0020 is amended, and core.8's details name the build as not ruled out.

### What stays

- Every step of the activation, its words and its exits; `nix/tests/activate.sh`'s cases, run by the shell's `sh` as before, which also pass with bash 3.2.57 as `sh`; the shell's own `sh` and `bash`, nixpkgs' bash 5.

### Not taken

- **The pattern's leading paren, as the brief asks.** POSIX gives a pattern an optional leading `(`, which bash 3.2 reads. treefmt's shfmt, 3.14.1, which formats every script of the tree (core.7), drops it at every format, with no option to keep it, so `root:format` fails the form and devenv's entry to the shell writes it away. The function needs no paren.
- **The host's /bin/sh.** On a Mac it is the very shell, but a Linux host that runs the gate has none that is bash 3.2, and a tool the task finds outside the tree's pin is what core.8 rules out.
- **nixpkgs' bash.** nixpkgs carries no bash 3, and bash 5, in POSIX mode or not, reads the file main holds.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: a Mac's /bin/sh reads every script the modules and packages give it, and the gate holds them to it.

## Impact

- `nix/modules/activate.sh`: the function `herdrs`.
- `nix/mac-sh.sh` and `nix/moon.yml`: the task `nix:mac-sh`.
- `devenv.nix`: bash 3.2.57 and `BOOMDRIVE_MAC_SH`. `check-toolchain.sh`: its version.
- `CLAUDE.md` and `.moon/workspace.yml`: the fast layer names the task.
- `openspec/specs/service-modules/spec.md`, by this change's delta once archived.
- `docs/adr/0020-*.md`: its amendment of 2026-11-15. `docs/architecture/core.md`: core.8's details.
- A host's first entry to the devenv shell builds bash 3.2.57, a few minutes, as the family's cache holds no path of devenv's.
- The proof on a Mac is its next activation: `~/.config/boomdrive/config.toml` linked to the module's file, Pending You's folder written and the plugins linked.
