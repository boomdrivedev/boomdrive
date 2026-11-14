# Proposal

## Why

The brief: "A person grants Boomdrive's runner its macOS privacy access once, and every later build of the runner keeps that grant, so a new build asks nothing again."

On deneb on 2026-11-11, the runner asked for macOS privacy access again after the herdr session restarted (signal `2026-11-11-agentplot-design-34bb0931/02-boomdrive-run-asks-macos-privacy-at-each-build`). The user said: "I'm getting messages saying that boomdrive needs access to the disk or something, one of those privacy and policy settings."

macOS remembers a person's answer for one program, which it knows by its path and its designated requirement. The runner the set builds is signed by the linker alone, ad hoc. On 2026-11-14 deneb's agent `org.nixos.boomdrive` ran `/nix/store/dxykdj2d…-boomdrive-0.9.0/bin/boomdrive`, `flags=0x20002(adhoc,linker-signed)`, with the designated requirement `cdhash H"04f2efaf…"`, the hash of that one build. Each new build has another store path and another hash, so macOS takes it for a new program and asks again. The runner is a launchd agent, so it is its own responsible process: what it starts, Flume's settle, sluice and git among them, is asked for under its name.

The work is built from the plan of `change/a-new-build-keeps-the-runners-macos-access` at `e3d8962e`, whose first two tasks, the signing step's folder check and its identity, are written there (afterthought/crew-state proposal 180, approved 2026-11-12).

## What Changes

- **The host makes its own code-signing identity once.** At the first nix-darwin activation that finds none, the activation, as root, makes a self-signed certificate for code signing named `Boomdrive runner`, with an RSA key. It keeps both in the System keychain, the key non-extractable and usable by `codesign` alone. The key is in no file once imported, and nothing of it is in the repository, an option or the store.
- **The runner's service runs a signed copy at one fixed path.** Before nix-darwin loads its agents, the activation copies the package's `bin/boomdrive` to `/Library/Application Support/Boomdrive/boomdrive`. It signs the copy with the host's identity under the identifier `boomdrive`, and names the program it was made from in `boomdrive.source` beside it. The copy's designated requirement is then `identifier boomdrive and certificate leaf = H"…"`, the same for every build, at the same path, so a grant given once holds for each later build. The folder is root's, and the activation signs nothing into a folder another user can write.
- **The agent runs the copy of its own package.** The launchd agent's line runs the copy where `boomdrive.source` names the agent's package, and the package's own binary where it does not, by `exec` behind `/bin/wait4path /nix/store` either way. The line still names the package and the file, so a new build or a new file reloads the agent, and nothing else does. A copy that cannot be signed leaves the runner running, asked for as today.
- **The README** says what a person sees: one grant, kept for every later build.
- **Record 0046 and rule core.60** record the decision, and core.46's details allow the agent's new line. They are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: on nix-darwin the runner keeps one code identity across builds. The activation makes the host's identity and signs a copy of the runner at a fixed root-owned path, and the launchd agent runs that copy where it is its package's.

## Impact

- `nix/modules/activate.sh`: a new step, `sign <keychain> <folder> <program>`, run as root. It checks the folder, finds or makes the identity, then copies, signs and records the program.
- `nix/modules/darwin.nix`: `extraActivation` runs the step with `/Library/Keychains/System.keychain`, the folder and the package's runner. The agent's `ProgramArguments` choose the copy or the package's own binary.
- `nix/tests/activate.sh`: the step's cases, over stubs of `security`, `codesign`, `openssl`, `mv` and `id`. No case reaches a keychain.
- `nix/tests/darwin.nix`: the agent's new line, the step before the agents load, the README's words, and the existing cases read through the new line.
- `README.md`: the nix-darwin paragraph says the agent runs the signed copy and the person grants access once.
- `CLAUDE.md` and `openspec/config.yaml`: the modules' description names the signed copy.
- `docs/adr/0046-…` and `docs/architecture/core.md`: record 0046, core.60 and core.46's details, written with this change.
- Nothing else changes. No crate, tool, tracked file type, configuration key, option, field of `report.json` or protocol message changes. NixOS is untouched, since Linux has no such ask. The agent `boomdrive setup` writes on a host Nix does not manage is untouched (design Non-Goals).
- On each Mac that runs the module's agent, the person grants the runner its access once more, to the signed copy. Grants given to store paths' runners do not carry over.

## Touches

```
nix/modules/activate.sh
nix/modules/darwin.nix
nix/tests/activate.sh
nix/tests/darwin.nix
README.md
CLAUDE.md
openspec/config.yaml
docs/adr/0046-on-macos-the-runners-service-runs-a-copy-signed-with-the-hosts-own-identity-so-a-grant-holds-for-every-build.md
docs/architecture/core.md
openspec/specs/service-modules/spec.md
```
