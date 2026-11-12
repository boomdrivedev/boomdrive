# Proposal

## Why

The brief: "A person grants Boomdrive's runner its macOS privacy access once, and every later build of the runner keeps that grant, so a new build asks nothing again."

On deneb on 2026-11-11, the runner asked for macOS privacy access again after the herdr session restarted (signal `2026-11-11-agentplot-design-34bb0931/02-boomdrive-run-asks-macos-privacy-at-each-build`). The user said: "I'm getting messages saying that boomdrive needs access to the disk or something, one of those privacy and policy settings."

macOS remembers a person's answer for one program, which it knows by its path and its designated requirement. The runner the set builds is signed by the linker alone, ad hoc. deneb's runner was `/nix/store/v6ppis4r…-boomdrive-0.0.0/bin/boomdrive`, `flags=0x20002(adhoc,linker-signed)`, with the designated requirement `cdhash H"241d42f3…"`, the hash of that one build. Each new build has another store path and another hash, so macOS takes it for a new program and asks again. The runner is a launchd agent, so it is its own responsible process: what it starts, Flume's settle, sluice and git among them, is asked for under its name.

## What Changes

- **The host makes its own code-signing identity once.** At the first nix-darwin activation that finds none, the activation, as root, makes a self-signed certificate for code signing named `Boomdrive runner`, with an RSA key. It keeps both in the System keychain, the key non-extractable and usable by `codesign` alone. The key is in no file once imported, and nothing of it is in the repository, an option or the store.
- **The runner's service runs a signed copy at one fixed path.** Before nix-darwin loads its agents, the activation copies the package's `bin/boomdrive` to `/Library/Application Support/Boomdrive/boomdrive`. It signs the copy with the host's identity under the identifier `boomdrive`, and names the program it was made from in `boomdrive.source` beside it. The copy's designated requirement is then `identifier boomdrive and certificate leaf = H"…"`, the same for every build, at the same path, so a grant given once holds for each later build. The folder is root's, and the activation signs nothing into a folder another user can write.
- **The agent runs the copy of its own package.** The launchd agent's line runs the copy where `boomdrive.source` names the agent's package, and the package's own binary where it does not, by `exec` behind `/bin/wait4path /nix/store` either way. The line still names the package and the file, so a new build or a new file reloads the agent, and nothing else does. A copy that cannot be signed leaves the runner running, asked for as today.
- **Record 0043 and rule core.59** record the decision, and core.46's details allow the agent's new line. They are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: on nix-darwin the runner keeps one code identity across builds. The activation makes the host's identity and signs a copy of the runner at a fixed root-owned path, and the launchd agent runs that copy where it is its package's.

## Impact

- `nix/modules/activate.sh`: a new step, `sign <keychain> <folder> <program>`, run as root. It checks the folder, finds or makes the identity, then copies, signs and records the program.
- `nix/modules/darwin.nix`: `extraActivation` runs the step with `/Library/Keychains/System.keychain`, the folder and the package's runner. The agent's `ProgramArguments` choose the copy or the package's own binary.
- `nix/tests/activate.sh`: the step's cases, over stubs of `security`, `codesign`, `openssl` and `id`. No case reaches a keychain.
- `nix/tests/darwin.nix`: the agent's new line, the step before the agents load, and the existing cases read through the new line.
- `README.md`: the nix-darwin paragraph says the agent runs the signed copy and the person grants access once.
- `docs/adr/0043-…` and `docs/architecture/core.md`: record 0043, core.59 and core.46's details, written with this change.
- Nothing else changes. No crate, tool, tracked file type, configuration key, option, field of `report.json` or protocol message changes. NixOS is untouched, since Linux has no such ask.
- On each Mac that runs the module's agent, the person grants the runner its access once more, to the signed copy. Grants given to store paths' runners do not carry over.
- Outside this repository: afterthought/swancloud runs its own agent, `swancloud.boomdrive`, on deneb, with the store path's runner. That agent keeps asking at each build until swancloud runs the module's agent, as its record 0059 plans. That is swancloud's move, told to the orchestrator.

## Touches

```
nix/modules/activate.sh
nix/modules/darwin.nix
nix/tests/activate.sh
nix/tests/darwin.nix
README.md
docs/adr/0043-on-macos-the-runners-service-runs-a-copy-signed-with-the-hosts-own-identity-so-a-grant-holds-for-every-build.md
docs/architecture/core.md
openspec/specs/service-modules/spec.md
```
