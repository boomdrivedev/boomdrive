# Verify: a-booted-nixos-system-proves-the-set

The verify stage, 2026-11-13, on `change/a-booted-nixos-system-proves-the-set` at `1a8e898`, cut from `bolt/a-nixos-box-takes-the-set`.

**Verdict: the whole checks pass and the work holds.** The code stage had already archived the change at `1a8e898`, as task 3.3 has it, so this report goes beside the archived change.

## The whole checks

`sluice exec main` ran once, on `1a8e898`, against `main` (`4b2da33`). It exited 0: 8 tasks passed, and sluice recorded their results under their keys for `1a8e898`.

| task | outcome |
|---|---|
| `nix:format` | passed |
| `nix:lint` | passed, actionlint reading `cache.yml` among it |
| `nix:typecheck` | passed, "every case holds", "The boot" among them |
| `nix:test` | passed, "every case holds", `boot.nix`'s and `workflow.nix`'s cases among them |
| `root:format` | passed, 1039 files traversed |
| `root:specs` | passed, 27 of 27 specs, `booted-system` and `binary-cache` among them |
| `root:changes` | passed, with no open change left to validate once the change was archived |
| `boomdrive:test` | passed, every test of the crate |

The branch does not affect the other projects' tasks, so sluice did not run them. moon's warnings that it could not archive task outputs in the sandbox changed no result.

The gate boots nothing. The boot itself is proven only when the workflow `cache` runs on GitHub after the branch lands: `boot (aarch64-linux)` builds `.#checks.aarch64-linux.boot` (design Decision 9). This stage cannot see that run, and the gate holds only the boot's definition.

## The tasks

All 8 tasks, 1.1 to 3.3, are ticked. Each one names `nix:typecheck`, `nix:test`, `nix:lint`, `root:format`, `root:changes` or `root:specs` as its check, and each of those passed above. Task 3.2's strict check of the deltas was run before the archive, at `37095b7`. `root:changes` now finds no open change, and `root:specs` validates the archived result strictly.

## The booted-system delta, requirement by requirement

| requirement | where it is met | proven by |
|---|---|---|
| The flake gives the boot on aarch64-linux alone | `flake.nix`: `checks.aarch64-linux.boot`, imported from `nix/boot.nix` with `pkgsOf "aarch64-linux"`, `nixosModules.default` and `packages.aarch64-linux`. No other system has a check | `typecheck.nix`'s "The boot": `checks` gives `aarch64-linux` alone, holding `boot` alone, whose `drvPath` evaluates. `boot.nix`'s "Never on x86_64-linux": no `x86_64-linux` or `aarch64-darwin`, and both the host's and the machine's nixpkgs are `aarch64-linux` |
| The boot's machine is a box that takes no work | `nodes.box`: the module with `services.boomdrive.enable` and `user = "alice"`, `alice` a normal user, crew's module for `host-a` with `settings.nix`'s `crewModule`, no accounts or settings of the runner's, `pkgs.git` in `environment.systemPackages` | "The boot's system": no failed assertion, `aarch64-linux`, `alice` lingering, crew's drives and hosts equal to `crewModule`'s, accounts `{ }`, the unit running `boomdrive run --config` with `files/empty.toml`'s text (no control plane), git in the profile |
| The boot needs no KVM and waits as long as an emulated boot takes | `requiredFeatures.kvm = false`, `globalTimeout = 90 * 60`, `virtualisation.cores = 4`, `memorySize = 4096`. The script sets `wait = datetime.timedelta(minutes=30)` and passes `timeout=wait` to every wait | "Emulated where there is no KVM": required features `nixos-test` alone, 4 cores, 4096 MiB. "Its waits": 5400 seconds, and every `wait_for_unit` and `wait_until_succeeds` call names `timeout=wait` |
| The booted system's user lingers and the runner's unit starts from boot | the script waits for `multi-user.target`, then waits for `Linger=yes` from `loginctl`, reads `id -u alice`, waits for `user@{uid}.service`, then waits for `systemctl --user --machine=alice@.host is-active boomdrive.service`, all before the first `su -l alice`. After the answers it checks `is-active`, reads `NRestarts` and fails unless it is `NRestarts=0` | "The runner's unit from boot": each step is there and in order, and the first login comes after the unit is found. The only login is `su -l alice`, with no `runuser` and no `user=`. "The runner stays up": the `is-active` check and the `NRestarts=0` failure both come after the last login |
| Each tool of the set answers in the booted system | `passthru.answers` covers the 11 tools. Each command is `<meta.mainProgram> --version` (so `wt --version` for worktrunk), or `crew _release` for crew, with the set's `version`. The script runs each through `su -l alice -c` and fails where the command fails or its output lacks the version | "Each tool's answer": the answers equal the spec's list, built independently with `wt` for worktrunk. The script's `answers` line is the boot's, and the loop runs and checks each answer. "Every tool installed answers": every package of the set in the machine's system profile has an answer, apart from `default` and the launcher |
| The workflow boots every commit a box may take | `cache.yml`: job `boot`, named `boot (aarch64-linux)`, `runs-on: ubuntu-24.04-arm`, with no `needs`, `if`, `strategy` or matrix. cachix-action names `boomdrive` with `skipPush: true`. The last step is `nix build --no-link --print-build-logs .#checks.aarch64-linux.boot` | `workflow.nix`'s "The boot's job" and "The boot never on x86_64-linux": the only system the job names is `aarch64-linux`, and its only runner is `ubuntu-24.04-arm` |
| The boot reads no token and pushes nothing | the job has no `environment`, `authToken`, `op://` or `cachix push`. Its one secret is `GITHUB_TOKEN`, which goes to install-nix-action | "No token for the boot": the secrets are `GITHUB_TOKEN` alone, used in the install step alone. Neither `CACHIX_AUTH_TOKEN` nor `OP_SERVICE_ACCOUNT_TOKEN` appears in the job |

## The binary-cache delta

| requirement | where it is met | proven by |
|---|---|---|
| *(modified)* The workflow builds and pushes every package of every system | the `build` job and its matrix are unchanged. The only other job is `boot` | "Each system has its job": the jobs are exactly `build` and `boot`, and `build` has one entry per system on its runner. "The runner is pushed with its tools", "A package that does not build" and "A branch's push" now read the `build` job's own lines and still hold |
| *(modified)* The push's token is the workflow's alone | only `build` names `environment: cachix`, the service account's token, the 1Password load and the token checks. `boot` reads none of them | "No service account token" and "No token", each with the new check that `boot` is given neither token |

## What the change holds to

- **Paths.** Every path the branch changes since `0ab2632` is in the proposal's Touches. Two paths come from the archive, not from Touches: the change's own folder, moved under `openspec/changes/archive/`, and the specs it writes.
- **Code and tooling.** The runner's code, the modules and the set are unchanged. No crate, gate tool or tracked file type is added (core.7, core.8). The boot's script is Python in a Nix string, and `nix:format` and `nix:lint` pass on it.
- **Records and rules.** The branch changes no rule or record. Record 0020's amendment of 2026-11-14 is the decision the change cites (design Decision 11). It is on `main` at `f261da96`, landed as #202, and not yet on this branch's base `0ab2632`. CLAUDE.md, `openspec/config.yaml` and the README cite it, and they will match it once the bolt lands on `main`.
- **Words.** CLAUDE.md and `openspec/config.yaml` both say that `cache` also boots the module on `aarch64-linux`, emulated on GitHub's arm runner, as the check a box's pin reads. The README's **Installing with nix** says this too, after its pin of a passed `cache` run, and adds that a NixOS host pins a commit whose boot passed.
- **Specs.** `openspec/specs/booted-system/spec.md` is new, with the Purpose task 3.3 names. `binary-cache` carries both modified requirements. `root:specs` validates both strictly.

## Findings

None.
