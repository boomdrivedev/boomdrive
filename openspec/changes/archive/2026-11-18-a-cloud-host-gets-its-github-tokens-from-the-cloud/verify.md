# Verify

The verify stage of a-cloud-host-gets-its-github-tokens-from-the-cloud,
2026-11-18, on `change/a-cloud-host-gets-its-github-tokens-from-the-cloud`
at `27805b09`.

**Verdict: archived.** The whole checks pass, every task is done, the
change passes OpenSpec's strict check, and every requirement and scenario
of its four specs is met by the work.

## The whole checks

`sluice exec main`, run once against `main` at `f954b77`: 22 tasks
completed, 0 failed, in 3m 24s. sluice recorded 22 results for `27805b0`.
The Rust tests ran 1,197 tests in 78 binaries, and none failed.

| task | outcome |
|---|---|
| `boomdrive:format`, `:lint`, `:typecheck`, `:test` | passed |
| `boomdrive-control:format`, `:lint`, `:typecheck`, `:test` | passed |
| `boomdrive-crew:format`, `:lint`, `:typecheck`, `:test` | passed |
| `boomdrive-hosted:format`, `:lint`, `:typecheck`, `:test` | passed |
| `nix:typecheck`, `nix:test` | passed |
| `root:format`, `root:changes`, `root:releases`, `root:versions` | passed |

The previous verify found two failures. Both now pass:

- `boomdrive-hosted:lint` (clippy's `type_complexity` in
  `tests/common/fake.rs`) passes after task 3.7, `7fb2e38d`.
- `boomdrive:test`'s `tests/control.rs` `a_keeping_pass_under_way` passes
  after task 4.6, `e519220f`. All of `tests/control.rs` ran.

`tests/forge_tokens.rs` and `tests/clones.rs` ran in full, which the
earlier stop left out. Every cloud-host test passed:
`a_get_on_a_cloud_host`, `each_get_on_a_cloud_host_asks`,
`a_repository_no_drive_serves_on_a_cloud_host`,
`an_erase_on_a_cloud_host`, `the_app_not_installed_on_a_cloud_host`,
`a_cloud_hosts_refusal`, `nothing_is_left_behind_on_a_cloud_host`,
`a_cloud_host_with_no_partitions_key`, the four `a_cloud_hosts_token…`,
`…refusal…` and `the_control_planes_word…` tests, and the four of
`clones.rs`.

moon gave two kinds of warning, and neither failed a task. It skipped the
Rust toolchain's dependency install for want of a network, and it could
not package task outputs into its cache archive under `.moon/cache/outputs`.
sluice's sandbox refused no task this run.

## OpenSpec

- `openspec instructions apply --change a-cloud-host-gets-its-github-tokens-from-the-cloud --json`:
  `state` `all_done`, every one of the 24 tasks done, 1.1 to 5.1.
- `openspec validate a-cloud-host-gets-its-github-tokens-from-the-cloud --strict`:
  `Change 'a-cloud-host-gets-its-github-tokens-from-the-cloud' is valid`.

## The work against the specs

Every test the tasks name is in the tree under the name the task gives
it, and each passed in the gate. The two `forge-token` tests task 4.3
removes, `a_get_on_a_cloud_host_is_refused` and
`an_erase_on_a_cloud_host_is_refused`, are gone, and so is
`NO_FORGE_TOKEN`.

- **runner-protocol.** `protocol/v4/` holds version 3's eleven schema
  files, `forge-token.schema.json`, version 3's examples and the four
  forge-token examples. `files.rs`, `forms.rs`, `refused.rs`, `names.rs`,
  `examples.rs` and `signatures.rs` prove each scenario. That includes
  version 4 held to version 3, version 3 held to `v0.18.0`, no forge
  token in versions 1 to 3, the one credential name and the ask's worked
  context. No file under `protocol/v1/`, `v2/` or `v3/` differs from
  `main` (core.42).
- **hosted-control-plane.** The hosted crate's tests prove each modified
  scenario under version 4. They also prove the forge token's answer on a
  socket (`connection.rs`) and every added scenario of `tests/forge_tokens.rs`.
  `src/forge_token.rs` matches design Decision 7:
  - partitions are asked in the table's order, each statement signed
    afresh for its repository;
  - the asks stop together at `ASK_FOR`, twenty seconds;
  - every answer is checked against version 4 before it is read, and is
    taken only where it names its ask and repository;
  - the App's absence is given only where every partition asked said so;
  - a host to fix is one where no partition could be asked.
- **forge-tokens.** `crates/boomdrive/tests/forge_tokens.rs` proves every
  modified and added scenario. On a cloud host, `get` and `erase`, the
  refusals, exit 2 with no partition's key, and nothing left behind. A
  kept token is renewed five minutes before its end, a refusal is kept
  not at all, and the control plane's word is kept for fifty-five minutes.
- **clones.** `crates/boomdrive/tests/clones.rs` proves a cloud host's
  fetch given the control plane's token, a repository the installation
  does not cover, a failed fetch dropping its token, and `run --once`
  naming `boomdrive enroll home` after git's words.

Every path the branch changes since `main` is one the proposal's
*Touches* lists. Record 0053, record 0007's amendment of 2026-11-18,
core.33's rule and details and core.42's details are in the tree.

## The archive

`openspec archive a-cloud-host-gets-its-github-tokens-from-the-cloud --yes`
applied the change's deltas and archived it as
`openspec/changes/archive/2026-11-18-a-cloud-host-gets-its-github-tokens-from-the-cloud/`:

| spec | added | modified | renamed |
|---|---|---|---|
| `clones` | 1 | 0 | 0 |
| `forge-tokens` | 1 | 6 | 2 |
| `hosted-control-plane` | 5 | 13 | 1 |
| `runner-protocol` | 4 | 5 | 1 |

It warned, without blocking, that a change of more than ten deltas might
be split. `openspec validate --specs --strict` then passed all 27 specs.
