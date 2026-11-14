---
number: 47
title: On macOS the runner's service runs a copy signed with the host's own identity, so a grant holds for every build
status: accepted
date: 2026-11-14
decision-makers:
- the construct stage of change/the-runners-macos-grant-outlasts-a-new-build
---

# On macOS the runner's service runs a copy signed with the host's own identity, so a grant holds for every build

Status: accepted. The direction is the brief's. How the runner keeps one identity is the construct stage's own choice, made on the rules as written, and cheap to change: it lives in the nix-darwin module alone.

Deciders: the construct stage of `change/the-runners-macos-grant-outlasts-a-new-build`, from its brief: "A person grants Boomdrive's runner its macOS privacy access once, and every later build of the runner keeps that grant, so a new build asks nothing again."

Sources: the signal `2026-11-11-agentplot-design-34bb0931/02-boomdrive-run-asks-macos-privacy-at-each-build`, in afterthought/crew-state on `agentplot/main`, and the user's words it quotes ("I'm getting messages saying that boomdrive needs access to the disk or something, one of those privacy and policy settings"); proposal 180 there, approved on 2026-11-12, which queues this work "to be built in boomdrive from where it stopped"; the plan of the branch `change/a-new-build-keeps-the-runners-macos-access` at `e3d8962e`, and its construct stage's checks with `codesign` and `security` on deneb on 2026-11-11; `codesign -d` of deneb's running runner on 2026-11-14, and this construct stage's checks that day with macOS 27.0.1's own `openssl`, `security` and `codesign`, in a scratch keychain removed after; nix-darwin's `modules/system/activation-scripts.nix` at the flake's lock; `nix/modules/darwin.nix`; records 0009, 0013, 0020, 0023 and 0034; rules core.4, core.18, core.28, core.32, core.37, core.44, core.46 and core.47; the OpenSpec change `the-runners-macos-grant-outlasts-a-new-build`, whose design gives each point below.

## Context and problem statement

macOS asks a person before a program reads a folder or a volume its privacy settings guard, and remembers the answer for that program. It knows a program without a bundle by its path and its designated requirement. The runner the set builds is signed by the linker alone, ad hoc, with no certificate. On deneb on 2026-11-14 the module's agent, `org.nixos.boomdrive`, ran `/nix/store/dxykdj2d…-boomdrive-0.9.0/bin/boomdrive`, `flags=0x20002(adhoc,linker-signed)`, whose designated requirement was `cdhash H"04f2efaf…"`, the hash of that one build. Every new build has another path and another hash, so macOS takes it for another program and asks again.

The runner is a launchd agent, so it is its own responsible process. A tool it starts, Flume's settle, sluice or git among them, is asked for under the runner's name.

## Considered options

1. **A copy of the runner, at one folder only root writes, signed at each activation with a code-signing identity the host makes once and keeps in its System keychain** (chosen).
2. **The same copy, signed ad hoc with a designated requirement that names only the identifier.**
3. **The runner signed with a Developer ID of Apple's Developer Program, in the workflow that pushes the cache.**
4. **The runner signed in its derivation.**
5. **A small program that never changes as the agent's program, starting the runner as its child.**
6. **The runner never touching what macOS guards.**

## Decision outcome

Option 1.

- **The identity is the host's own.** The nix-darwin module's activation, as root, makes a self-signed certificate for code signing named `Boomdrive runner`, with an RSA key, the first time the System keychain holds none. The key is imported non-extractable, usable by `codesign` alone, and is in no file once imported. Nothing of it is in the repository, an option, a file the module writes or the store (core.4, core.46). It is the keychain of the user the activation runs as, root, as the host's key is the keychain's of the user the runner runs as (core.32). On deneb, macOS's own `codesign` signed with such an identity, untrusted (`CSSMERR_TP_NOT_TRUSTED`), with no prompt and no keychain password. Two different programs signed with it each had `designated => identifier boomdrive and certificate leaf = H"…"`, and both ran; a copy of the runner 0.9.0 signed with it, in a folder whose name holds a space, ran and gave its version. Not taken: an identity a person makes by a command of their own, since activation can make it unattended; the login keychain, which an activation as root reaches only through the person's session.
- **One folder only root writes.** The copy is `/Library/Application Support/Boomdrive/boomdrive`, made by root, its folder writable by no other user, and the activation signs nothing into a folder another user can write. The path never changes, so macOS finds its answer under it. Only root can replace the program there, and only root's `codesign` can sign with the key. So no process of a person's, or of anything running as them, can make code that holds the runner's grant.
- **Signed before the agents load.** nix-darwin runs `extraActivation` before it loads its agents. There the activation copies `<package>/bin/boomdrive`, signs the copy by the identity's SHA-1 under the identifier `boomdrive`, with no timestamp so nothing leaves the host (core.28), and puts it in place by a rename. It names the program it was made from in `boomdrive.source` beside it, removing that file before it replaces the copy and writing it only once the copy is in place, so it never names a program the copy was not made from. It makes a copy only where none is, where the copy was made from another program, or where the copy is not signed with the identity.
- **The agent runs the copy of its own package.** The agent's line runs the copy where `boomdrive.source` names the agent's package, and the package's own `bin/boomdrive` where it does not, by `exec` behind `/bin/wait4path /nix/store` in either case (core.46). The line still names the package and the file, so nix-darwin reloads the agent for a new build or a new file, and for nothing else. A copy that could not be signed leaves the runner running, as it runs today.
- **What the set gives is unchanged.** The set's package, the store paths the cache holds and the paths a host builds from source are the same as before (core.44, core.47). The copy is made from the store path the host's nix substituted or built, which is no release (core.37).

Not taken:

- **Option 2.** Nothing would need a key, but any program signed ad hoc with the identifier `boomdrive` would meet that requirement. The design would then rest on macOS knowing a program without a bundle only by its path.
- **Option 3.** It costs Apple's yearly fee and publishes a name, which is the person's choice. It needs a certificate and its password as the workflow's secrets. A signed build would differ from the one a host builds from source, against core.47. A person may still decide on it, by a record of its own that replaces this identity.
- **Option 4.** A derivation holds no key, so it can only sign ad hoc, which is option 2 in the store.
- **Option 5.** macOS would ask for the small program in place of the runner. A program of the set's built by Nix still changes its hash with each of its own builds, so it needs this record's signing all the same.
- **Option 6.** The runner starts Flume, sluice and git under its own name, so what they read is asked for as the runner's. Whatever macOS guards that they read, the runner needs. The brief asks for the grant to be kept, not to go without one.

### Consequences

- A person grants the runner its access once on each host. Every later build signed with the host's identity keeps the grant. A grant given to a store path's runner does not carry over, so each host is asked once more.
- A person who removes the identity from the System keychain gets a new one at the next activation. The next build signed with it is asked for once.
- What the runner starts still asks under the runner's name, and holds its grant, as before.
- A host whose own agent runs the store path's runner is asked at each build until it runs the module's agent.
- A host Nix does not manage, whose agent `boomdrive setup` writes, runs the runner from the person's profile and is asked at each upgrade. setup runs as the person, with no root to keep a folder only root writes, and nothing runs at a profile's upgrade to sign a new build. This record leaves that host as it is; a record of its own may give setup's agent a signed copy.
- The keychain's own ask before the runner reads a partition's key (core.32) is not covered here. Only a host that enrolls with a hosted control plane meets it.
- A host that turns the module off keeps the folder, the copy and the identity until a person removes them. The identity kept means a host that turns the runner on again keeps its grant.
- This record adds core.60, and amends core.46's details to allow the agent to run that copy. The change `the-runners-macos-grant-outlasts-a-new-build` builds it.
