# Design

## Context

The change a-host-enrolls-from-one-command gave the control plane's page one whole enroll command. The command needs the runner on the host. Its design put the install out of its scope. It gave the script these steps. The script installs Nix where it is missing. It uses the family's cache at the person's asking. It adds the set at its release tag to the person's profile. Then it runs the enroll command.

A release builds and attaches no binary (core.48). The set reaches a host from the family's cache, as the outputs of the flake's derivations (core.47). A store path that Nix gets from the cache is no release, so core.37's check of a release's provenance does not apply to it. The quick start's Mac steps already add the set with one `nix profile add`.

On 2026-11-27 the orchestrator approved this form. It also asked for this: where Nix is missing, the script says what it will install and asks the person first. A flag in the copied command can give the answer yes.

This design relies on core.4, core.13, core.31, core.37, core.39, core.47 and core.48.

## Decisions

### Decision 1: The command

```console
echo <token> | sh -c "$(curl -fsSL https://github.com/boomdrivedev/boomdrive/releases/latest/download/install.sh)" install --yes --control-plane <url> --drive <drive> --repository <owner/name> [--repository <owner/name>…]
```

The script comes as the argument of `sh -c`, and not on the standard input of `sh`. So the standard input is free for the token. The script closes the standard input of each program that could read it, `curl`, Nix's installer, `nix`, `sudo launchctl` and `boomdrive --version`. `sudo tee` reads the lines that the script gives it. So the token goes to `boomdrive enroll` alone, and is in the arguments of no process (core.31). `install` is the script's `$0`.

The script's own flag is `--yes`. The enroll command's flags are `--control-plane`, `--drive` and `--repository`. The script passes them to `boomdrive enroll` in their order. It takes the three together or none of them. Any other flag is a usage error, exit 2. With no enroll flags, the script installs the set and enrolls nothing, as for a drive of crew's.

### Decision 2: Who serves the script

The workflow `release` attaches `install.sh` to each GitHub release, from the release commit (Decision 6). GitHub serves it at two addresses:

- `https://github.com/boomdrivedev/boomdrive/releases/latest/download/install.sh`, the newest release.
- `https://github.com/boomdrivedev/boomdrive/releases/download/v<version>/install.sh`, one release.

The page may name either address. The repository names no other server.

### Decision 3: The systems

The script runs on macOS on Apple silicon. The flake gives no `x86_64-darwin` (record 0020). On Linux, enroll refuses a file that the runner did not get from Nix (the hosted-control-plane spec). So on Linux the script stops with exit 2. It names `services.boomdrive` and the quick start's NixOS steps. On any other system it stops with exit 2.

### Decision 4: Where Nix sets up the runner, the script stops

Where the runner's file, `$XDG_CONFIG_HOME/boomdrive/config.toml` or `~/.config/boomdrive/config.toml`, is a link into `/nix/store`, the host's Nix configuration sets up the runner. The script then stops with exit 2 and installs nothing. With an enroll command, it names that command without the script. That command names the lines to add to the configuration (the change a-host-enrolls-from-one-command, Decision 3).

Where `boomdrive` is on the `PATH` and gives this release's version, the script adds nothing and runs the enroll command. Where it gives another version, the script stops with exit 2 and names both versions. The script does not change a runner that came from another source.

Each stop that comes before the enroll command says that the token is not spent.

### Decision 5: Nix and the cache, at the person's asking

Where the host has no `nix`, the script says what it will do. It will install Nix at the release that the script pins, with Nix's own installer, as root. It will set Nix to use the family's cache, and names the cache's URL and key. Then it asks. `--yes` answers yes. Where `--yes` is not given, it asks on the terminal that is its standard input. Where the standard input is not a terminal, as when the token comes on it, it stops with exit 2. It names `--yes` and gets nothing from the network.

On yes, it downloads Nix's installer from `releases.nixos.org`. It checks the installer against the SHA-256 digest that the script pins, the digest that Nix publishes as `install.sha256`. A different digest stops it with exit 1, and the installer does not run. The installer runs as `--daemon --yes --nix-extra-conf-file`. The file turns on `nix-command` and `flakes`, and names the cache and its key. Nix is the host's own tool and not a tool of the set. Record 0012 keeps git and the multiplexer the host's own the same way. The pinned digest is the check of what runs, and a change of this tree moves it.

Where the host has `nix`, the script reads `nix config show substituters` and `trusted-public-keys`. Where they name the cache and its key, it asks nothing. Where they do not, it names the two lines, the file, and the restart of Nix's daemon, and asks as above. The file is `/etc/nix/nix.conf`, or `/etc/nix/nix.custom.conf` where Determinate's `determinate-nixd` is on the `PATH`. On yes, it adds the lines through `sudo tee -a`, and restarts the daemon with `sudo launchctl kickstart -k`. Where the file cannot be written, as where nix-darwin links it into the store, it stops with exit 2 and names the lines. A host that does not use the cache would build the whole set from source, so the script does not go on without it.

Each `nix` command takes `--extra-experimental-features 'nix-command flakes'`, so a Nix that has not turned them on also works.

### Decision 6: The set at the release's tag

The script holds `version=`, which `set-version.sh` writes in the release commit, as it writes `Cargo.toml` and `Cargo.lock` (core.48). `main` gives `0.0.0`, which `root:versions` holds. The release commit changes the three files alone, which `root:releases` proves. The workflow `release` takes `install.sh` from the release commit with `git cat-file` and attaches it with `gh release create`. It builds nothing.

The script adds `github:boomdrivedev/boomdrive/v<version>`'s `boomdrive`, `flume`, `sluice`, `herdr` and `herdr-plugins`, and git from the flake's own nixpkgs (`--inputs-from`), with one `nix profile add`. The runner needs git 2.48 or later, which a Mac's own git may not be. crew, gh and Node.js are for a drive of crew's, which the quick start adds. The script then puts the profile's `bin` at the end of its `PATH` and runs `boomdrive enroll` from it, with `exec`. So the enroll command's answer and exit code are the script's.

### Decision 7: The tests

`check-install.sh` runs each case of the install spec on a scratch home. The `PATH` holds stubs of `uname`, `curl`, `sudo`, `nix`, `determinate-nixd` and `boomdrive`, and links to the real tools that the script uses, and no other program. Each stub writes its arguments to a log, and `boomdrive` writes its standard input. So a case reads what the script ran, in what order, and that the token reached `boomdrive enroll` alone.

The case that runs past the digest check runs a copy of the script whose pinned digest is that of the stub installer. Every other case runs the script as it is.

Each case runs under the shell's `sh` and under `BOOMDRIVE_MAC_SH --posix`, macOS's `/bin/sh` (core.8). The task is `root:install`, in check-fast. treefmt's shellcheck and shfmt lint and format both files (core.7).

A test does not ask on a terminal, since the gate gives none. The prompt is the five lines of `ask`.

## Out of this change

- **The page.** The control plane's change follows this one. It copies the command of Decision 1, with `--yes`, and cites this change.
- **Linux without NixOS.** The runner on Linux runs from the NixOS module. A systemd user unit for a host without NixOS is a change of its own.
- **An upgrade.** The script does not change a runner of another version (Decision 4). `boomdrive upgrade` (record 0012) is a change of its own.
