# Design

## Context

**The rulings.**

- agentplot-design, 2026-11-11, on the NixOS cutover: a NixOS VM test, `runNixOSTest`, in Boomdrive's CI on Linux, proves the module, since the merge gate cannot boot a Linux machine.
- agentplot-design, 2026-11-13: the booted check runs on `aarch64-linux`, the system the boxes run. Check first whether GitHub's `ubuntu-24.04-arm` runner gives `/dev/kvm`. If it does not, name where the `aarch64` boot runs and ask, never falling back to `x86_64`.
- Record 0020's amendment of 2026-11-14, boomdrive-design's answer to this change's construct stage (`f261da96`, landing as sluice #202):
  - the boot runs on `ubuntu-24.04-arm` with its guest emulated, `requiredFeatures.kvm` false;
  - it is a job of this repository's workflow, at each push to `main` and each tag `v*`, and its check on the commit is what a box's pin reads;
  - its waits allow for an emulated boot;
  - never on `x86_64-linux`;
  - a bare-metal KVM runner is the user's to choose, and only if the emulated boot proves too slow to keep.
- afterthought/swancloud's record 0059, decision 3: a box's flake input is moved by a change that names the commit.
- boomdrivedev/blueprints `design/principles.md` 15: a tool's checks are tuned from Timber's measurements of their runs. A public repository's runs cost time, not money.

**GitHub's arm runner,** as read on 2026-11-14. actions/runner-images#14062, "Please support KVM on ARM runners", was closed on 2026-08-20: "the ARM64 hosted fleet doesn't surface a nested KVM device ... the only path to a KVM-accelerated arm64 guest is a self-hosted ARM64 runner on bare-metal". community/discussions#160591 gives arm64 nothing as of 2026-10-01. A public repository's `ubuntu-24.04-arm` runner has 4 vCPUs and 16 GB. Its `/` leaves about 106 GiB free, on which the set already builds (the change cache-x86-build-has-room).

**The boxes.** `chuck-herdr-alpha` (`r8g.4xlarge`) and `chuck-sluice-alpha` (`c8gd.4xlarge`) are EC2 Graviton virtual machines of `aarch64-linux`, in Willdan's account, with no KVM (afterthought/swancloud `lib/workspace-boxes.nix`).

**nixpkgs at the lock** (`1d060086`):

- `nixos/lib/testing/run.nix`: a test's `requiredFeatures` are `nixos-test`, and `kvm` on Linux where it has machines. `kvm` "Can be disabled to allow emulated execution". The derivation's `requiredSystemFeatures` are the features set true.
- `nixos/lib/qemu-common.nix`: an `aarch64-linux` guest on an `aarch64-linux` host runs `qemu-system-aarch64 -machine virt,gic-version=max,accel=kvm:tcg -cpu max`, so it falls back to QEMU's own emulation where there is no KVM.
- `nixos/lib/testing/driver.nix`: `globalTimeout` is 60 minutes by default. The driver's `wait_for_unit` and `wait_until_succeeds` wait 15 minutes by default, and take a `timeout` as a `datetime.timedelta`.
- `wait_for_unit(unit, user)` asks the user's manager through `su -l <user>`, which opens a login session.
- The driver logs each action with its time, `(finished: <action>, in <n> seconds)`, and the whole script's, `test script finished in <n>s`.
- `testers.runNixOSTest` gives the test's `config` and `nodes` as attributes of its result, beside its `passthru`, without building it.

**Nix and the job's install.** Nix's default `system-features` hold `nixos-test`, `benchmark` and `big-parallel`, and `kvm` only where `/dev/kvm` can be opened. cachix/install-nix-action at the workflow's pin (`75c75c31`) sets no `system-features`. Its `enable_kvm`, on by default, adds a udev rule for `/dev/kvm`, and says "KVM is not available" where there is none.

**The set on `aarch64-linux`,** as the flake evaluated on 2026-11-14. Each tool's program and version:

| package | program | version |
|---|---|---|
| `boomdrive` | `boomdrive` | `0.0.0` |
| `flume` | `flume` | `0.21.2` |
| `sluice` | `sluice` | `0.86.0` |
| `crew` | `crew` | `0.59.0` |
| `herdr` | `herdr` | `0.9.5` |
| `worktrunk` | `wt` | `0.80.0` |
| `ghostty` | `ghostty` | `1.4.0` |
| `portless` | `portless` | `0.17.0` |
| `pendingyou` | `pendingyou` | `0.41.0` |
| `plannotator` | `plannotator` | `0.28.61` |
| `plannotator-tui` | `plannotator-tui` | `0.9.9` |

Each answers `--version` with a line that holds its version, as the same releases do on `aarch64-darwin`. The exception is crew. crew's `--version` prints its usage, and crew says its version by `crew _release`, one line of JSON whose `version` is its tag, as `v0.59.0` (crew's `tests/contract/test_release.py` at v0.59.0).

**The gate today.** `nix/check.sh` evaluates `nix/tests` against a dummy store. It computes every path and builds nothing. `nix/tests/nixos.nix` evaluates a NixOS system of `x86_64-linux` with the module, including the case "No control plane beside crew on NixOS": crew's module on with `settings.crewModule` and the host `host-a`, the runner's file `nix/tests/files/empty.toml`. `nix/tests/workflow.nix` reads `cache.yml` line by line. It holds its jobs to `build` alone, and reads every line after the first `steps:` as that job's steps.

## Goals / Non-Goals

**Goals:**

- Every commit of `main`, and each tag, is booted as a NixOS system with the module on `aarch64-linux`. In that system the service's user lingers, the runner's user unit starts with no login, and each tool of the set answers.
- The boot runs where it can, on GitHub's arm runner, its guest emulated, and never on `x86_64-linux`.
- A box's pin has one check to read, `boot (aarch64-linux)`.
- The gate holds what the boot is, though it cannot run it.

**Non-Goals:**

- **Moving a box's pin.** That is afterthought/swancloud's, by its record 0059, decision 3, reading this check.
- **A KVM runner.** It is the user's to choose, and only if the emulated boot proves too slow (record 0020 as amended on 2026-11-14).
- **The runner's work in the booted system.** The boot reaches no network, keeps no repository and serves no control plane. The runner's passes are proven by its own tests.
- **The herdr plugins' links, the launcher, Pending You's folder and Claude Code's document.** The gate evaluates each, and the boot runs only what the brief names.
- **A boot on nix-darwin.** No hosted macOS runner boots a nix-darwin system, and the brief names NixOS.

## Decisions

### 1. The boot runs on `ubuntu-24.04-arm`, its guest emulated

This decision is record 0020's amendment of 2026-11-14. The test sets `requiredFeatures.kvm = false`, so its derivation requires `nixos-test` alone, which Nix gives by default. On the arm runner, which has no `/dev/kvm`, QEMU runs the `aarch64-linux` guest under its own emulation. The guest is the boxes' system, built as they build it. The job adds no system feature, and sets no `extra_nix_config`.

**Not taken:**

- `x86_64-linux`, which proves a system no box runs.
- A self-hosted KVM runner, which is the user's to choose.
- `kvm` given to the job's Nix by hand. That would tell Nix a lie, and still run QEMU's emulation.

### 2. The boot is a NixOS test of the flake's, `checks.aarch64-linux.boot`

`nix/boot.nix` takes the flake's own nixpkgs for `aarch64-linux`, the flake's NixOS module and the set for `aarch64-linux`. It gives `pkgs.testers.runNixOSTest` of one machine. `flake.nix` gives it as `checks.aarch64-linux.boot`, and gives `checks` no other system. `flake.nix` binds `nixosModules` in its `let`, so the check and the output are one value. The boot is the flake's own derivation, and the job builds it by the flake's name.

**Not taken:**

- A check for every Linux system, which would give `x86_64-linux` one.
- The boot under `nix/tests`, whose cases the gate evaluates and never builds.
- A NixOS test in the job's own script, which the gate could not evaluate.

### 3. The machine is a box that takes no work

The machine `box` is the gate's evaluated system "No control plane beside crew on NixOS", booted on `aarch64-linux`:

- the flake's `nixosModules.default`, with `services.boomdrive.enable` and `user = "alice"`;
- `alice` a normal user;
- `services.crew` on, with the host `host-a` and `nix/tests/settings.nix`'s `crewModule`, so the modules install crew's companions (record 0044);
- no `accounts` and no `settings`, so the runner's file is `nix/tests/files/empty.toml`'s, with no control plane and no repository.

`environment.systemPackages` holds `pkgs.git`, the host's own (records 0012 and 0023; core.44). The runner refuses to start with no git. The machine has `virtualisation.cores = 4` and `virtualisation.memorySize = 4096`, which the runner's 4 vCPUs and 16 GB hold. QEMU's emulation runs one host thread for each guest core.

This is a box as record 0059, decision 3, has it: a host where no partition's main level runs, which keeps its repositories and takes no work. Here it keeps none, since the guest has no network.

The names are core.41's neutral ones, and `crewModule` is read from the gate's own fixture, so the gate and the boot hold one system.

**Not taken:**

- A crew partition on this host, or a hosted control plane. Either would have the runner reach a state repository or a control plane over a network the guest lacks.
- crew's module off, which installs no companion to answer.

### 4. No KVM, and waits an emulated boot fits

Each wait of the script is given `timeout=wait`, where `wait = datetime.timedelta(minutes=30)`, twice the driver's default. The test's `globalTimeout` is `90 * 60` seconds, half again the default. Under KVM a NixOS boot takes seconds. Under emulation it takes minutes. These are limits on a hang, not a target. The first green runs' times tune them (principle 15).

The job sets no `timeout-minutes`. GitHub's own limit of 360 minutes bounds it, since a job may first build what the cache lacks, the runner's at least, before the test starts. The test's own 90 minutes bound the boot.

### 5. The script proves the linger before anything logs the user in

`su -l alice` opens a session, whose PAM starts `alice`'s manager by itself. So the script proves the linger before any command runs as `alice`, and reads `alice`'s manager as root:

1. `box.start()`, then wait for `multi-user.target`.
2. `loginctl show-user alice --property=Linger` gives `Linger=yes`, waited for.
3. `alice`'s uid by `id -u alice`, which opens no session. Then wait for the system unit `user@<uid>.service`.
4. Wait until `systemctl --user --machine=alice@.host is-active boomdrive.service` succeeds, run as root, which opens no session either.
5. The answers (Decision 6), each `su -l alice -c '<command>'`.
6. `systemctl --user --machine=alice@.host is-active boomdrive.service` succeeds, and `systemctl --user --machine=alice@.host show boomdrive.service --property=NRestarts` gives `NRestarts=0`.

The driver's `wait_for_unit(unit, user)` is not used for `boomdrive.service`, since it runs `su -l`. The unit is started at boot by `default.target` and by the user activation's `keep`, and neither counts as a restart. `NRestarts` counts the restarts systemd made of its own accord.

### 6. Each tool of the set answers, by its program and its version

`nix/boot.nix` makes the answers from the set:

- one for each tool of the set the modules install: `boomdrive`, `flume`, `sluice`, `crew`, `herdr`, `worktrunk`, `ghostty`, `portless`, `pendingyou`, `plannotator` and `plannotator-tui`;
- its command is `<meta.mainProgram> --version`, or `crew _release` for crew;
- its version is the package's `version`.

The answers are a list of `{ command, version }`, given as the test's `passthru.answers` and written into the script as JSON. The script fails where a command fails or its answer does not hold its version. "Holds" is a substring, since each tool words its line its own way: `wt v0.80.0`, `portless`'s bare `0.17.0`, crew's JSON `"version": "v0.59.0"`.

Each runs as `alice`, by a login shell, so it finds the program on the profiles' `PATH`, as a person and the runner do.

Where a tool's `--version` at the set's tag gives no version on a headless machine, the code stage names that tool's own version command in the answers and in this design. It runs no other command in its place.

**Not taken:**

- The lock's tags, which `nix/tests/tags.nix` and `packages.nix` already hold the packages' versions to.
- A version from each tool's own output parsed by its format, which differs by tool.

### 7. The job `boot` in the workflow `cache`

The boot is a job of the workflow the record names, beside `build`. Its steps:

```yaml
  boot:
    name: boot (aarch64-linux)
    runs-on: ubuntu-24.04-arm
    steps:
      - uses: actions/checkout@<build's pin> # <build's tag>
        with:
          persist-credentials: false

      - uses: cachix/install-nix-action@<build's pin> # <build's tag>
        with:
          github_access_token: ${{ secrets.GITHUB_TOKEN }}

      - uses: cachix/cachix-action@<build's pin> # <build's tag>
        with:
          name: boomdrive
          skipPush: true

      - name: Boot NixOS with the module on aarch64-linux
        run: nix build --no-link --print-build-logs .#checks.aarch64-linux.boot
```

- **No `needs`.** The boot substitutes what the cache holds and builds the rest, the same paths `build` builds. A failure or a slow job of another system never holds a box's pin, and the boot does not wait on the slowest system.
- **No environment and no token of the cache's** (core.4, core.47). `cachix-action` with no `authToken` reads the public cache alone. `GITHUB_TOKEN`, GitHub's own, is what `build` gives install-nix-action to fetch the flake's inputs.
- **No push.** The boot is no package, and core.47 has the workflow push the set's paths alone.
- **`--print-build-logs`,** so the driver's log, each wait's time among it, is the job's (Decision 9).
- **A run by hand.** A run by hand on a ref other than `main` or a tag starts the boot alone, since the environment `cachix` admits `main` and tags `v*` alone, and `build` names it.

**Not taken:**

- `needs: build`, which waits on the slowest system and holds a box's pin on another system's failure.
- A step of `build`'s `aarch64-linux` leg, which joins the boot's check to the build's, under the environment that holds the push's token.
- A workflow of its own, where the record names a job of this one, at its triggers.

### 8. A box's pin reads `boot (aarch64-linux)`

The job's name is its interface with the boxes. A commit of `main` or a tag whose `boot (aarch64-linux)` passed is one a box may take.

GitHub's concurrency group, one for each ref, keeps one pending run and cancels an older pending one. So a commit pushed while a newer run was queued may have no boot. No box takes it, and nothing is lost.

The README says so beside its pin of a commit whose `cache` run passed.

### 9. The boot's time

The driver logs each wait's time and the whole script's. The job prints that log. So every run says how long the emulated boot took, with no step of its own.

No run of the boot exists while this change is written or built:

- the gate boots nothing;
- the host that builds the change has no `aarch64-linux` builder, and the boot never runs on `x86_64-linux`;
- the job runs only on GitHub.

The first green run is either a run of `cache` by hand on the bolt, once sluice has pushed it, which starts the boot alone (Decision 7), or the run at the push to `main` that lands it. Its time, read from its log's `test script finished in` line, is written into record 0020's amendment of 2026-11-14, with the limits of Decision 4 tuned from it. If it proves too slow to keep, that is said with its measured time, and the choice of a KVM runner is the user's (record 0020 as amended on 2026-11-14).

### 10. What the gate holds

The gate holds the boot's definition by evaluation, and never builds it (`nix/check.sh`, design Decision 14 of the flake):

- **`nix/tests/boot.nix`,** added to `nix/tests/default.nix`'s `test`, reads `checks.aarch64-linux.boot`'s `config`, `nodes.box` and `answers`, and its script's text with its string's context discarded. Its cases are the booted-system delta's scenarios but "The boot's check", which typecheck holds: "The boot's system", "Never on x86_64-linux", "Emulated where there is no KVM", "Its waits", "The runner's unit from boot", "The runner stays up", "Each tool's answer" and "Every tool installed answers".
- **`nix/tests/typecheck.nix`** gains "The boot": `checks` gives `aarch64-linux` alone, holding `boot` alone, whose `drvPath` evaluates.
- **`nix/tests/workflow.nix`** reads each job's own lines, from `  <job>:` to the next job, and its steps from that job's `steps:`. Its cases of `build` hold what they held, read from `build`'s lines. "Each system has its job" holds the jobs `build` and `boot`. The booted-system delta's job scenarios are its new cases: "The boot's job", "No token for the boot" and "The boot never on x86_64-linux". "Each action pinned by its commit" reads every `uses:` of the file, the boot's among them.

### 11. The rules relied on

- core.4 and core.47: the boot reads no token and pushes nothing, and only the build jobs, under `cachix`, push the set.
- core.7 and core.8: no file type, crate or gate tool is added. The script is Python in a Nix string, formatted with its file by nixfmt.
- core.13: the boot's script runs each tool itself. The runner starts none.
- core.41: `alice`, `box` and `host-a` are neutral names.
- core.44 and core.45: the boot runs the set's own paths. git is the host's own.
- core.46: the machine runs the modules as a host would, and sets nothing the set derives.

The boot departs from no rule, and no rule changes. Record 0020's amendment of 2026-11-14 is the decision, written in its own commit.

### 12. The words

- `CLAUDE.md` and `openspec/config.yaml`: the workflow `cache` also boots a NixOS system with the module on `aarch64-linux`, emulated on GitHub's arm runner, the check a box's pin reads (record 0020 as amended on 2026-11-14).
- The README's **Installing with nix**, after its pin of a commit whose `cache` run passed: that run's job `boot (aarch64-linux)` boots a NixOS system with the module on `aarch64-linux`, and a NixOS host pins a commit whose boot passed.

No check reads these words (principle 16).

## Risks / Trade-offs

- **An emulated boot is slow.** It takes minutes for what KVM does in seconds, and the store reaches the guest over 9p. The limits of Decision 4 allow for it, and the first runs' times tune them. A KVM runner stays the user's choice if the boot proves too slow to keep.
- **A tool may not answer on a headless machine.** The boxes are headless, so that is a finding about the set, and Decision 6 says how the code stage names a tool's own version command.
- **The boot proves the runner idle.** It starts and stays up with no repository and no control plane. Its passes with work are its own tests'.
- **A commit with no boot.** GitHub's concurrency can leave a commit unbooted (Decision 8). No box takes it.

## Migration Plan

- The change lands through sluice's gate, which evaluates the boot and boots nothing.
- The first run of `cache` with the job boots the module. A run by hand on the bolt, once pushed, can give it before `main`.
- afterthought/swancloud's change that moves a box's pin names a commit whose `boot (aarch64-linux)` passed. That is its own, by its record 0059, decision 3.
- Rollback is the commit's revert. A box's pin stays where it was.

## Task notes

- **1.1 `nix/boot.nix`.**
  - It is a function of `{ pkgs, module, set }`, where `pkgs` is `pkgsOf "aarch64-linux"`, `module` the flake's `nixosModules.default` and `set` `packages.aarch64-linux`. It returns `pkgs.testers.runNixOSTest { … }`.
  - The test's `name` is `boomdrive-boot`. It has `nodes.box`, `requiredFeatures.kvm = false`, `globalTimeout = 90 * 60`, `passthru.answers` and `testScript`.
  - The script begins `import datetime, json, shlex`, then `wait = datetime.timedelta(minutes=30)`, then `answers = json.loads(<the answers as a Python string>)`.
  - In `flake.nix`, `nixosModules = modulesOf "nixos" ./nix/modules/nixos.nix;` moves into the `let`, and the outputs give `inherit nixosModules;` and `checks.aarch64-linux.boot = import ./nix/boot.nix { … };`.
- **1.2 The machine.**
  - `crewModule` comes from `import ./tests/settings.nix`.
  - `services.crew` is `{ enable = true; host = "host-a"; settings = settings.crewModule; }`.
  - `services.boomdrive` is `{ enable = true; user = "alice"; }`.
  - `users.users.alice.isNormalUser = true`.
  - `environment.systemPackages = [ pkgs.git ]`.
  - `virtualisation` is `{ cores = 4; memorySize = 4096; }`.
- **1.3 The gate's reading.**
  - The test's attributes `config`, `nodes` and `answers` are read without forcing its derivation. Only typecheck's "The boot" forces `drvPath`.
  - "Its waits" finds every `wait_for_unit(` and `wait_until_succeeds(` call of the script naming `timeout=wait`, and `wait = datetime.timedelta(minutes=30)`.
  - "The runner's unit from boot" finds the steps of Decision 5 by their text, each before the first `su -l alice`.
- **2.1 `workflow.nix`.** `jobOf = name: …` gives a job's lines, and `stepsOf` that job's steps, the existing helpers taking a job's lines in place of the whole block. The build's cases read `jobOf "build"`. The case "The x86_64-linux job has room" still finds the free step the one step of `build` with a condition. The boot has none.
