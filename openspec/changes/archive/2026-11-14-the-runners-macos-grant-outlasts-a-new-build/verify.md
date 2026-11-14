# Verify: the-runners-macos-grant-outlasts-a-new-build

The work holds. The gate passes, every task is ticked, and each scenario of the service-modules delta has a case that proves it.

The code stage archived the change under task 3.3, which needs the archive before `root:specs` can prove it. This report sits beside the archive, on `change/the-runners-macos-grant-outlasts-a-new-build` at `a3d3e5d`.

## The whole checks

`sluice exec main` ran once on `a3d3e5d`. It ran the checks the commit affects against `main` (`021a3f5`) and exited 0. All 9 passed, and sluice recorded their results under their keys for `a3d3e5d`:

- `nix`: `format`, `lint`, `typecheck` and `test`;
- `root`: `format`, `adrs`, `changes` and `specs`;
- `boomdrive`: `test`.

`nix:test` said "every case holds". `root:specs` passed 27 items, 0 failed, `spec/service-modules` among them. moon also warned that it could not write its output archives under `.moon/cache/outputs`. Those warnings come from the sandbox, and no check failed because of them.

## Tasks

All nine tasks are ticked, 1.1 to 3.3, and none is open.

- 1.1 to 2.2 and 3.1 rely on `nix:test`.
- 2.3 relies on `nix:lint`.
- 3.2 relies on `root:format`, which also holds `activate.sh` and its tests to shellcheck and shfmt and the `.nix` files to nixfmt.
- 3.3 relies on `root:specs`.

Each of those checks passed in the run above. The diff since the planning commit's parent stays inside proposal.md's Touches. It adds no crate, gate tool, tracked file type, option, configuration key, field of `report.json` or protocol message, and the runner's code is unchanged. Record 0047, core.60 and core.46's amended details were written in the planning commit, `d7867ab5`, as the tasks say. The step keeps the `oneline` helper from `e3d8962e`, which design Decision 8 leaves to the build stage.

## The delta, scenario by scenario

### Modified

| Scenario | Proven by |
|---|---|
| The agent | `nix/tests/darwin.nix`'s case of that name: `commandOf`, read through the line's `else` branch, gives the package's `bin/boomdrive run --config` with the file, and the agent's keys hold |
| No home for the primary user | `darwin.nix`'s case of that name, unchanged |
| The agent waits for the Nix store | `darwin.nix`'s `waits`, for the base system and crew's: `/bin/sh -c`, the line starting `/bin/wait4path /nix/store && `, and an `exec` in each branch |

### Added

| Scenario | Proven by |
|---|---|
| The agent runs the copy of its own package | `darwin.nix`'s case of that name: `/bin/cat` of `boomdrive.source` compared with the package's `bin/boomdrive`, the copy's `exec` with the file in `then`, the package's in `else`, and no `command` |
| The step runs before the agents load | `darwin.nix`'s case of that name: `extraActivation.text` holds the call with `/Library/Keychains/System.keychain`, the quoted folder, the package's runner and `\|\| true`; `postActivation.text` holds no `activate.sh sign` |
| No signing without the runner | `darwin.nix`'s case of that name, a system importing the module with the runner off |
| A new build | `nix/tests/activate.sh`'s case of that name: the signing call by SHA-1 with `--identifier boomdrive`, `--timestamp=none` and the keychain on `boomdrive.new`, the two renames by `mv -f` in order, the source naming the program, one line |
| The same build | `activate.sh`'s case of that name: one `codesign --verify` against the leaf, no signing, the folder's listing and bytes unchanged, nothing said |
| A copy signed with another identity | `activate.sh`'s case of that name: signed anew with the held identity |
| A copy not put in place | `activate.sh`'s case of that name: the rename onto `boomdrive` refused, `boomdrive` as it was, no `boomdrive.source`, nothing else, the rename's words, exit 1. This also proves the source is removed before the rename |
| A signing refused | `activate.sh`'s case of that name: both files as they were, nothing else, `codesign`'s words, exit 1 |
| No identity yet | `activate.sh`'s case of that name: `find-identity -p codesigning`, `openssl req -x509 -newkey rsa:2048 -nodes -days 7300` in a folder of its own with the configuration's five lines, the key imported with `-x -T /usr/bin/codesign` then the certificate, the folder removed, the made line, then the copy signed with it |
| No key in a file | `activate.sh`'s case of that name: the folder `openssl` wrote in is under the case's `TMPDIR`, and no file reads `key` or `cert` |
| An identity that cannot be made | `activate.sh`'s case of that name: the keychain's words on one line, the folder removed, no signing, exit 1 |
| Two identities of the name | `activate.sh`'s case of that name: three lines, two distinct SHA-1s, each named, nothing made or signed, exit 1 |
| A folder another user can write | `activate.sh`'s case of that name: `g+w`, `o+w` and another owner's uid, each refused with one line, no call of a stub, the folder empty |
| The README's signed runner | `darwin.nix`'s case of that name, reading `t.installing` for each phrase |

`activate.sh` also gains "sign's usage": two words or four after `sign` exit 2, make no folder and call no stub.

## The code against the design

- `nix/modules/activate.sh`'s `sign` follows design Decisions 2 to 4:
  - its folder check reads `ls -ldn` and refuses a link, another owner, or write for the group or others;
  - its lookup counts distinct SHA-1s and fails where `security` cannot list;
  - its identity is made in a `mktemp -d` folder under `umask 077`, which a trap removes;
  - its copy is made in Decision 4's order, and each failure leaves the files the design names.
- `nix/modules/darwin.nix` follows Decisions 5 and 6:
  - the agent declares `ProgramArguments` with the line and no `command`;
  - the step runs in `extraActivation` with the operating system's `PATH` alone and `|| true`;
  - `postActivation` is unchanged but for moving into one `system` set with `requiresPrimaryUser`.
- The README's nix-darwin paragraph, CLAUDE.md and `openspec/config.yaml` say what tasks 3.1 and 3.2 ask.

That a grant survives a new build is Apple's to show, on a host after landing (design Risk 1). The gate proves the step's calls and the agent's line.

## Archive

The change is at `openspec/changes/archive/2026-11-14-the-runners-macos-grant-outlasts-a-new-build`, and no change of that name is open. `openspec/specs/service-modules/spec.md` holds the delta, and `root:specs` passes it under the strict check.
