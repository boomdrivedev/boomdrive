# Design

## Context

See proposal.md for why. There is no area page for `nix/`, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.4: no token, key or credential is in the repository, and the runner reads each from the host. The identity is made on the host and kept in its keychain (Decision 2);
- core.28: nothing leaves a host from the runner but its protocol, git's calls and an OTLP export. The signature asks no timestamp server (Decision 4);
- core.32: the host's key is kept only in the host's credential store, never in a file in the clear. The identity's key is kept the same way, in the keychain of the user the activation runs as (Decision 2);
- core.37: what a host runs comes from a verified source. The copy is made only from the store path the host's nix substituted or built (Decision 4);
- core.44 and core.47: the set's packages are the flake's own derivations, and the cache holds the paths a host would build. Neither changes (Decision 1);
- core.46: the modules hold no credential in an option, a file or the store, and the agent waits for `/nix/store`, then `exec`s the runner. The agent's new line keeps both (Decision 5);
- core.7 and core.8: every file is owned by a formatter the gate runs, and every tool a task runs is pinned. The change adds no file type and no tool to the gate: its tests stub `security`, `codesign` and `openssl` (Decision 7).

It departs from no rule. It adds core.59 and record 0043, and amends core.46's details (Decision 8).

**What macOS asks, and why again.** macOS remembers a person's answer about a guarded folder or volume for one program. It knows a program without a bundle by its path and its designated requirement. On deneb on 2026-11-11, `codesign -d -r-` of the running runner, `/nix/store/v6ppis4r…-boomdrive-0.0.0/bin/boomdrive`, gave `designated => cdhash H"241d42f3…"`, and `codesign -dv` gave `flags=0x20002(adhoc,linker-signed)`, `TeamIdentifier=not set`. The requirement is the hash of one build, at one store path. A new build changes both, so macOS asks again.

The runner is a launchd agent, its own responsible process. Flume's settle, sluice and git, which it starts, are asked for under its name. Flume's, sluice's and herdr's binaries in the set are signed the same way, ad hoc by the linker (`codesign -dv` on deneb, 2026-11-11).

**What the construct stage checked on deneb.** These checks ran in a scratch keychain, removed after, with the host's own `security` and `codesign`:

- a self-signed certificate for code signing, made by `openssl req -x509` with `extendedKeyUsage = critical,codeSigning`, its key and certificate imported as PEM files, the key with `-x -T /usr/bin/codesign`, formed an identity that `security find-identity -p codesigning` lists as `CSSMERR_TP_NOT_TRUSTED`;
- `codesign --force --sign <SHA-1> --keychain <keychain> --identifier boomdrive` signed with it, with no prompt, no keychain password and no `set-key-partition-list`;
- two different programs signed with it each had `designated => identifier boomdrive and certificate leaf = H"1939e9c0…"`, and both ran;
- `codesign --verify -R '=certificate leaf = H"<SHA-1>"'` passed for a program signed with the identity, and failed for one signed with another.

**When nix-darwin loads agents.** nix-darwin's activation script, at the flake's lock, runs as root. It runs `preActivation`, `checks`, `createRun`, `extraActivation`, then later `launchd` and `userLaunchd`, and `postActivation` last. Its first lines are `set -e` and `set -o pipefail`.

**The family.** No tool of the family signs its binaries. The set is this flake's (records 0020 and 0023), and the runner's agent is the only service it declares on macOS. So the module owns how the set's service keeps one code identity, and a tool that later needs the same cites this design.

## Goals / Non-Goals

**Goals:**

- A person grants the runner's service its macOS privacy access once on a host, and every later build keeps the grant.
- Nothing of the identity is in the repository, an option, the store, a file or a log, and no process of the person's can make code that holds the runner's grant.
- The set's packages and the cache's paths are unchanged, and the runner keeps running where its copy cannot be signed.

**Non-Goals:**

- A Developer ID of Apple's Developer Program, which costs money and publishes a name. That is the person's to decide (record 0043, option 3).
- The keychain's own ask before the runner reads a partition's key (core.32), which only a host enrolled with a hosted control plane meets (Risk 4).
- Signing herdr, Flume or sluice. herdr's servers run as launchd agents on some hosts. Where macOS asks under herdr's name, the cause is the same, and this module's identity is where the fix belongs (Risk 5).
- NixOS, where nothing asks.
- A host's own agent that runs the store path's runner, such as afterthought/swancloud's (Risk 3).

## Decisions

### 1. The service runs a copy signed with the host's own identity

The nix-darwin module keeps a copy of `<package>/bin/boomdrive` at a fixed path, signed with a code-signing identity the host makes once. The copy's designated requirement is then `identifier boomdrive and certificate leaf = H"<identity>"`, the same for every build. The path is the same too, so macOS finds the person's answer for each later build. The set's package is untouched, and the cache's paths and those a host builds from source stay the same (core.44, core.47).

Not taken, as record 0043 has it:

- **An ad hoc signature with a requirement that names only the identifier.** Any program signed ad hoc as `boomdrive` would meet it.
- **A Developer ID, in the cache's workflow.** It costs money, publishes a name and needs secrets, and a signed build would differ from a host's own build.
- **Signing in the derivation.** A derivation holds no key.
- **A small program that never changes, as the agent's program.** It is built by Nix, so its own hash still changes.
- **Never touching what macOS guards.** The runner's children are asked for under its name.

### 2. The identity is made by the activation and kept in the System keychain

The step looks for the identity in `security find-identity -p codesigning <keychain>`'s output. It counts the distinct SHA-1s on lines naming `"Boomdrive runner"`. With none, it makes one:

- in a folder of its own, made by `mktemp -d` under `umask 077`, which a trap removes whatever happens, it writes an OpenSSL configuration and runs `openssl req -x509 -newkey rsa:2048 -nodes -days 7300 -config <it> -keyout key.pem -out cert.pem`. The configuration sets `prompt = no`, `CN = Boomdrive runner`, `basicConstraints = critical,CA:false`, `keyUsage = critical,digitalSignature` and `extendedKeyUsage = critical,codeSigning`;
- it runs `security import key.pem -k <keychain> -x -T /usr/bin/codesign`, then `security import cert.pem -k <keychain>`;
- it removes the folder and says `boomdrive: made the code-signing identity Boomdrive runner, <SHA-1>, in <keychain>`.

With two or more, it signs nothing and says `boomdrive: <keychain> holds <n> identities named Boomdrive runner, <SHA-1>…: signed nothing; remove all but one`, then exits 1.

The keychain is `/Library/Keychains/System.keychain`, the credential store of root, as whom nix-darwin's activation runs. The key is non-extractable, and `codesign` alone may use it without a prompt. An identity macOS does not trust is enough: `codesign` signs with it, and the copy's requirement names its leaf, not a chain. The tools are the operating system's own, `/usr/bin/openssl`, `/usr/bin/security` and `/usr/bin/codesign`, so the step adds nothing to the system's closure (Decision 6). Twenty years outlives any host. An RSA key of 2048 bits is what Apple's own code-signing certificates use.

Not taken:

- **A command a person runs once.** The activation can make the identity unattended, and a person's step can be forgotten.
- **The login keychain.** An activation as root reaches it only through the person's session.
- **A key in a file of root's**, or a keychain of the module's own whose password is in a file. Each puts a key, or what unlocks one, in a file in the clear (core.32).
- **A trust setting for the certificate.** macOS asks a person to change trust settings, even as root, and signing needs none.

### 3. One folder, only root's

The copy lives in `/Library/Application Support/Boomdrive`. The step makes the folder with `mkdir -m 0755` where it is not there. It then reads the folder's owner and mode from `ls -ldn`, which reads the same on macOS and in the gate's Linux sandbox. The folder must be owned by the user the step runs as (`id -u`), root under nix-darwin, and its mode must give no write to its group or to others. Otherwise the step makes no identity and signs nothing. It says `boomdrive: <folder> is writable by a user other than <user>: signed nothing`, then exits 1. It changes no owner and no mode, since a folder made by anyone else is no folder to trust.

Only root can replace the program there, and only root's `codesign` can sign with the key. So no process of the person's, or of anything running as them, can put code that holds the runner's grant where macOS looks for it.

### 4. A copy is made only for a new build, by a rename

The step is `activate.sh sign <keychain> <folder> <program>`. After the folder's check and the identity's lookup, it makes a copy where any of these holds:

- `<folder>/boomdrive` is not there;
- `<folder>/boomdrive.source` does not hold `<program>`;
- `codesign --verify -R '=certificate leaf = H"<SHA-1>"' <folder>/boomdrive` fails.

To make a copy, it copies `<program>` to `<folder>/boomdrive.new` and gives it mode 0755. It runs `codesign --force --sign <SHA-1> --keychain <keychain> --identifier boomdrive --timestamp=none <folder>/boomdrive.new`, and moves it to `<folder>/boomdrive` by `mv -f`. It writes `<program>` to `<folder>/boomdrive.source.new` and moves that to `boomdrive.source` the same way. It then says `boomdrive: signed <folder>/boomdrive, made from <program>, with Boomdrive runner, <SHA-1>`.

Where `cp` or `codesign` fails, it removes `boomdrive.new` and leaves `boomdrive` and `boomdrive.source` as they were. It says `boomdrive: could not sign <program> into <folder>: <its words on one line>`, then exits 1. Where nothing is to be made, it says nothing and exits 0. A running runner keeps the program it started from, since the rename replaces the name and not the file.

`--timestamp=none` keeps the signing from asking Apple's timestamp server, so nothing leaves the host (core.28). The copy is made only from the `<program>` the module names, the store path the host's nix substituted against the cache's key or built from the lock's source (core.37). Not taken: the hardened runtime, `--options runtime`. It changes what the runner and the loader may do, and macOS's grant does not turn on it.

### 5. The agent runs the copy of its own package, and the package's own otherwise

The agent's `serviceConfig.ProgramArguments` are `/bin/sh`, `-c` and one line:

```
/bin/wait4path /nix/store && if [ "$(cat '<folder>/boomdrive.source' 2>/dev/null)" = <package>/bin/boomdrive ]; then exec '<folder>/boomdrive' run --config <file>; else exec <package>/bin/boomdrive run --config <file>; fi
```

Each path in the line is quoted by `lib.escapeShellArg`, the folder's space among them. The module declares the line in `ProgramArguments` itself, not through `command`. nix-darwin turns a `command` into `/bin/wait4path /nix/store && exec <command>`, and that leaves no room for a choice. The line keeps what core.46 asks: it waits for the store's volume, and the runner holds the job's process by `exec` in either branch.

The line names the package and the file. So nix-darwin, which reloads an agent whose plist changed, still reloads it for a new build or a new file and for nothing else. The requirement "A changed file or package restarts the runner, and nothing else does" holds unchanged. The step signs the copy in `extraActivation`, before `userLaunchd` loads the reloaded agent, so a new build starts from its signed copy. A build whose copy could not be signed runs the package's own binary, asked for as today, and the runner keeps working.

Not taken: a fixed program line with the activation restarting the agent when the copy changed. That would take the reload from nix-darwin, and a failed signing would leave the agent running a copy of another build.

### 6. The step runs as root in `extraActivation`

`darwin.nix` adds to `system.activationScripts.extraActivation.text`:

```
/usr/bin/env PATH=/usr/bin:/bin:/usr/sbin:/sbin /bin/sh ${./activate.sh} sign /Library/Keychains/System.keychain '/Library/Application Support/Boomdrive' ${cfg.package}/bin/boomdrive || true
```

It runs as root, as nix-darwin's activation does, with no `sudo --user`. `|| true` keeps nix-darwin's `set -e` from stopping the activation on a refusal, since the step has said why. The `PATH` holds the operating system's folders alone, so the step runs Apple's `security`, `codesign` and `openssl`. `postActivation` keeps the link of the file and of the plugins as the primary user, unchanged.

### 7. The tests stub the keychain and the signer

`nix/tests/activate.sh` gains the step's cases, named for the spec's scenarios, each in a scratch folder of its own, as the link's and the plugins' cases are. Its stubs, first on `PATH`, record their calls in the case's `calls`:

- `security`: `find-identity` prints the lines of the case's `identities` in `security`'s own form, `  1) <SHA-1> "Boomdrive runner" (CSSMERR_TP_NOT_TRUSTED)`. `import` records its arguments, and adds the identity to `identities` once both files are imported. It refuses, with the words of the case's `refuse-import`, where that file is there;
- `codesign`: `--verify -R '=certificate leaf = H"<SHA-1>"' <file>` passes where `<file>` holds `signed <SHA-1>`. `--force --sign <SHA-1> … <file>` appends `signed <SHA-1>` to `<file>`, or refuses, with the words of the case's `refuse-sign`, where that file is there;
- `openssl`: writes `key` to its `-keyout` file and `cert` to its `-out` file, and records their folder;
- `id`: answers `-u` with the case's own uid, by the system's `id -u`, and `-un` with the case's `user`, as today.

The keychain is a word the stubs take, never a real keychain, so no case touches a keychain of the gate's host. The case "No key in a file" asserts that neither the case's folder nor the folder `openssl` wrote in holds a file reading `key` or `cert` once the step ends.

`nix/tests/darwin.nix` reads the agent's new line. The cases "The agent" and "The agent waits for the Nix store" check both branches, the source compared with the package's `bin/boomdrive`, and the wait. "The step runs before the agents load" checks `extraActivation.text` holds the step's call and `postActivation.text` holds no `sign`. The helpers that read the line, `commandOf` and `fileOf`, read it through the `else` branch, so "A new file on nix-darwin", "A new package on nix-darwin" and the crew cases still read the file and the package.

### 8. Record 0043, core.59, and core.46's details

Record 0043 holds the decision and its options. core.59 is the rule: the copy, its folder, the identity and its keychain, and the package's own binary where no copy of it is signed. core.46's details gain, among what they do not rule out, the agent's line running that copy or the package's own binary behind the same wait. All three are written with this change.

## Risks / Trade-offs

1. **What macOS remembers is Apple's.** The design rests on macOS knowing a program without a bundle by its path and its designated requirement. The gate proves the step's calls and the agent's line. That a grant survives a new build is seen on a host after landing. The check on deneb shows that two builds signed with one identity share one requirement.
2. **One more grant on each host.** A grant given to a store path's runner does not carry over to the signed copy. A person who removes the identity is asked once more after the next build.
3. **A host's own agent.** afterthought/swancloud's `swancloud.boomdrive` agent on deneb runs the store path's runner. It keeps asking at each build until swancloud runs the module's agent, as its record 0059 plans. That is swancloud's move, told to the orchestrator.
4. **The keychain's ask for a partition's key.** The runner reads a partition's key from the keychain with the keyring crate, in its own name (core.32). The keychain may ask again for a new build. No such host is affected until one enrolls with a hosted control plane.
5. **herdr's servers.** On deneb, herdr's servers are launchd agents of the host's own, ad hoc signed. Where macOS asks under herdr's name, this module's identity is the one owner of the fix.
6. **The System keychain as root.** The checks ran in a user's keychain. Where root's import or signing fails on a host, the step says so, and the agent runs the package's own binary.
7. **The grant is the runner's children's too.** What the runner starts holds its grant, as it does today.
8. **A copy left behind.** A host that turns the module off keeps the folder and the copy until a person removes them.

## Migration Plan

Nothing moves by hand. At the first activation of a release with this change, the step makes the identity and the copy, and the agent runs the copy. macOS then asks the person once for what the runner reads. A rollback runs the earlier package's own binary, since its line names no copy.
