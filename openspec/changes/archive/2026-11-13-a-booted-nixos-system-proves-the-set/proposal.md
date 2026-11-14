# Proposal

## Why

The brief: "Every Boomdrive commit a NixOS box may take is proven by booting a NixOS system with the module on aarch64-linux, the system the boxes run: its service's user lingers, the runner's user unit starts, and each tool of the set answers there; the check runs in Boomdrive's CI on Linux, since the merge gate cannot boot a Linux machine, and a box's pin moves only to a commit whose aarch64-linux boot is green."

Today the module is proven only by evaluation. `nix/tests/nixos.nix` evaluates a NixOS system of `x86_64-linux` with the module and reads its unit key by key, its user's linger, its user activation and its system profile. Nothing starts that system. A unit that never comes up, a linger that starts no manager, or a tool whose binary does not run on `aarch64-linux` all pass the gate. Willdan's boxes, `chuck-herdr-alpha` and `chuck-sluice-alpha`, are EC2 Graviton machines of `aarch64-linux`. afterthought/swancloud's record 0059, decision 3, moves a box's flake input by a change that names the commit. Nothing yet tells that change which commits are safe to name.

agentplot-design ruled on 2026-11-11, on the NixOS cutover, that a NixOS VM test, `runNixOSTest`, in Boomdrive's CI on Linux proves the module. On 2026-11-13 it ruled that the booted check runs on `aarch64-linux`, the system the boxes run, and never falls back to `x86_64-linux`. GitHub's `ubuntu-24.04-arm` runner gives no `/dev/kvm`: actions/runner-images#14062 was closed on 2026-08-20, saying the ARM64 hosted fleet surfaces no nested KVM device. So record 0020's amendment of 2026-11-14 (boomdrive-design, `f261da96`, landing as sluice #202) boots the `aarch64-linux` guest on `ubuntu-24.04-arm`, emulated by QEMU, the test's `requiredFeatures.kvm` false as nixpkgs at the lock allows. It is a job of the workflow `cache`, at each push to `main` and each tag `v*`, and its check on the commit is what a box's pin reads. A bare-metal KVM runner is the user's to choose, and only if the emulated boot proves too slow to keep.

The unit was planned in afterthought/crew-state's `plan.rec` (agentplot/main `03f01311`). This change is the unit `a-booted-nixos-system-proves-the-set` of the bolt `a-nixos-box-takes-the-set`.

## What Changes

- **The boot.** `nix/boot.nix` is a NixOS test, made with the flake's own nixpkgs' `testers.runNixOSTest`, which the flake gives as `checks.aarch64-linux.boot`, for `aarch64-linux` and no other system. Its one machine, `box`, has the Boomdrive module on for its user `alice`, crew's module on, no control plane, and git, the host's own. Its script:
  - boots the machine;
  - finds `alice` lingering, `alice`'s manager started with no login, and the runner's unit, `boomdrive.service`, active in it;
  - runs each tool of the set the modules install, as `alice`, by its program's name, and holds each answer to the set's version of that tool;
  - finds the runner's unit still active, never restarted.
- **Emulated, with room to wait.** The test requires the feature `nixos-test` alone, not `kvm`, so QEMU emulates the guest where the runner has no KVM. Each wait is allowed 30 minutes, and the whole test 90 minutes, where the driver's defaults are 15 and 60.
- **The job.** The workflow `cache` gains the job `boot`, named `boot (aarch64-linux)`, on `ubuntu-24.04-arm`. It runs at every run of the workflow and builds the boot with its log printed. It substitutes what the family's cache holds, waits on no other job, is given no environment, secret or token, and pushes nothing. Its check on a commit is what a box's pin reads.
- **The gate holds the boot's definition.** `nix/tests/boot.nix` evaluates the boot, without building it, in the nix project's test: its system, its required features, its waits, its order and its answers. `nix/tests/workflow.nix` reads each job of the workflow apart, and holds the boot's job. `nix/tests/typecheck.nix` holds that the boot evaluates.
- **The specs.** A new capability, `booted-system`, states the boot and its job. `binary-cache` names the boot as the workflow's one job beside the systems' builds, and has only the build jobs read the tokens.
- **Words.** CLAUDE.md, `openspec/config.yaml` and the README say the workflow boots the module on `aarch64-linux`, and that a NixOS host takes a commit whose `boot (aarch64-linux)` passed.

### What stays

- Every system's build job, its runner, its steps, its tokens and its push.
- `nix/tests/nixos.nix`'s evaluated cases, on `x86_64-linux`.
- The runner's code, the modules and the set.

### Not taken

- **A boot on `x86_64-linux`,** where GitHub's runner gives KVM. It proves a system no box runs (record 0020's amendment of 2026-11-14).
- **A self-hosted runner on a bare-metal `aarch64-linux` machine with KVM.** It costs money and gives ops a machine to keep, so it is the user's to choose, and only if the emulated boot proves too slow to keep.
- **The boot in the gate.** sluice's gate runs moon on the host that lands the change, which boots no Linux machine.

## Capabilities

### New Capabilities

- `booted-system`: the flake's boot of a NixOS system with the module on `aarch64-linux`, emulated on GitHub's arm runner, in which the service's user lingers, the runner's user unit starts and each tool of the set answers, run by the workflow `cache` as the check a box's pin reads.

### Modified Capabilities

- `binary-cache`: the workflow's jobs are each system's build and the boot, and only the build jobs read the push's tokens.

## Impact

- `nix/boot.nix` (added): the boot.
- `flake.nix`: `checks.aarch64-linux.boot`.
- `.github/workflows/cache.yml`: the job `boot`.
- `nix/tests/`: `boot.nix` (added), `default.nix`, `typecheck.nix` and `workflow.nix`.
- `CLAUDE.md`, `openspec/config.yaml` and `README.md`: the words.
- `openspec/specs/booted-system/spec.md` (added) and `openspec/specs/binary-cache/spec.md`, by this change's deltas once archived.
- No crate, tool of the gate or tracked file type is added (core.7, core.8): the boot's script is Python held in a Nix string, which nixfmt formats as the file it is in.
- The proof is a run of `cache` whose `boot (aarch64-linux)` passes. The gate boots nothing. Outside this repository, afterthought/swancloud's change that moves a box's pin names only a commit whose `boot (aarch64-linux)` passed (its record 0059, decision 3).

## Touches

```
flake.nix
nix/**
.github/workflows/cache.yml
README.md
CLAUDE.md
openspec/config.yaml
openspec/changes/a-booted-nixos-system-proves-the-set/**
openspec/specs/booted-system/**
openspec/specs/binary-cache/**
```
