# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- **The checks.** Every case of the gate's evaluates against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds the boot, boots a machine or reaches the network. The boot itself is proven only by the workflow's run on GitHub (design Decision 9).
- **What no task adds or changes.**
  - No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8). The boot's script is Python in a Nix string.
  - The runner's code, the modules and the set do not change.
  - Every `.nix` file written is formatted by nixfmt, as treefmt formats it, and passes statix and deadnix.
  - Nothing outside the paths `proposal.md`'s Touches lists is changed.
  - No rule or record changes. Record 0020's amendment of 2026-11-14 is the decision, in its own commit (design Decision 11).
- **Choices.** A task that relies on a choice the design names cites its Decision.

## 1. The boot

- [x] 1.1 `nix/boot.nix` is added: a function of `{ pkgs, module, set }` giving `pkgs.testers.runNixOSTest` of one machine, `box`, named `boomdrive-boot` (design Decision 2; Task notes 1.1). `flake.nix` binds `nixosModules` in its `let`, and gives `checks.aarch64-linux.boot` from `pkgsOf "aarch64-linux"`, its `nixosModules.default` and `packages.aarch64-linux`, and no check for another system. `nix/tests/typecheck.nix` gains the case "The boot": `checks` gives `aarch64-linux` alone, holding `boot` alone, whose `drvPath` evaluates. True when `sluice exec main -- moon run nix:typecheck` passes, proving the booted-system delta's "The boot's check".
- [x] 1.2 The machine `box` has (design Decision 3; Task notes 1.2):
  - the flake's NixOS module, with `services.boomdrive.enable` and `user = "alice"`, and `alice` a normal user;
  - crew's module on, with the host `host-a` and `nix/tests/settings.nix`'s `crewModule`;
  - no `accounts` and no `settings` of the runner's;
  - git in its system profile;
  - 4 cores and 4096 MiB.

  `nix/tests/boot.nix` is added to `nix/tests/default.nix`'s `test`, reading the boot's `config` and `nodes.box` (design Decision 10; Task notes 1.3). True when `sluice exec main -- moon run nix:test` passes, proving the booted-system delta's "The boot's system" and "Never on x86_64-linux".
- [x] 1.3 The boot sets `requiredFeatures.kvm = false` and `globalTimeout = 90 * 60`. Its script defines `wait = datetime.timedelta(minutes=30)` and gives every `wait_for_unit` and `wait_until_succeeds` `timeout=wait` (design Decisions 1 and 4). `nix/tests/boot.nix` gains a case for each scenario it proves. True when `sluice exec main -- moon run nix:test` passes, proving the booted-system delta's "Emulated where there is no KVM" and "Its waits".
- [x] 1.4 The script boots the machine, then, before any command as `alice` (design Decision 5):
  - waits for `multi-user.target`;
  - waits for `alice`'s `Linger=yes` from `loginctl`;
  - waits for the system unit `user@<uid>.service`;
  - waits until `systemctl --user --machine=alice@.host is-active boomdrive.service` succeeds.

  After the answers, it finds the unit active with `NRestarts=0`. `nix/tests/boot.nix` gains a case for each scenario it proves. True when `sluice exec main -- moon run nix:test` passes, proving the booted-system delta's "The runner's unit from boot" and "The runner stays up".
- [x] 1.5 The boot's `passthru.answers` are made from the set, one for each of `boomdrive`, `flume`, `sluice`, `crew`, `herdr`, `worktrunk`, `ghostty`, `portless`, `pendingyou`, `plannotator` and `plannotator-tui` (design Decision 6):
  - its command is `<meta.mainProgram> --version`, or `crew _release` for crew;
  - its version is the package's `version`.

  The script runs each command by `su -l alice -c`, between the waits of 1.4 and the last check, and fails where the command fails or its answer does not hold its version. `nix/tests/boot.nix` gains a case for each scenario it proves. True when `sluice exec main -- moon run nix:test` passes, proving the booted-system delta's "Each tool's answer" and "Every tool installed answers".

## 2. The workflow

- [x] 2.1 `.github/workflows/cache.yml` gains the job `boot`, as design Decision 7 gives it, and `nix/tests/workflow.nix` reads each job's own lines and steps (design Decision 10; Task notes 2.1):
  - the build's cases read `build`'s, and hold what they held;
  - "Each system has its job" holds the jobs `build` and `boot`;
  - the cases "The boot's job", "No token for the boot" and "The boot never on x86_64-linux" are added.

  True when `sluice exec main -- moon run nix:test` passes and `sluice exec main -- moon run nix:lint` passes, actionlint reading the workflow. That proves:
  - the binary-cache delta's "Each system has its job", "The runner is pushed with its tools", "A package that does not build", "A branch's push", "No service account token" and "No token";
  - the booted-system delta's "The boot's job", "No token for the boot" and "The boot never on x86_64-linux".

## 3. The words and the specs

- [x] 3.1 CLAUDE.md and `openspec/config.yaml` say that the workflow `cache` also boots a NixOS system with the module on `aarch64-linux`, its guest emulated on GitHub's arm runner, the check a box's pin reads (record 0020 as amended on 2026-11-14). The README's **Installing with nix**, after its pin of a commit whose `cache` run passed, says that the run's job `boot (aarch64-linux)` boots a NixOS system with the module on `aarch64-linux`, and that a NixOS host pins a commit whose boot passed (design Decision 12). No check reads these words. True when `sluice exec main -- moon run root:format` passes.
- [x] 3.2 The change's deltas pass openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [x] 3.3 The deltas archive into specs that pass openspec's strict check. The new `openspec/specs/booted-system/spec.md`'s Purpose says it is the flake's boot of a NixOS system with the module on `aarch64-linux`, emulated on GitHub's arm runner, in which the service's user lingers, the runner's user unit starts and each tool of the set answers, run by the workflow `cache` as the check a box's pin reads (record 0020 as amended on 2026-11-14). True when, after `devenv shell -- openspec archive a-booted-nixos-system-proves-the-set --yes`, `sluice exec main -- moon run root:specs` passes.
