# Verify

The verify stage of a-cloud-host-gets-its-github-tokens-from-the-cloud,
2026-11-18, on `change/a-cloud-host-gets-its-github-tokens-from-the-cloud`
at `7f866b8e`.

**Verdict: not archived.** The whole checks fail. Two tasks of the gate
fail, `boomdrive-hosted:lint` and `boomdrive:test`. The work goes back to
its code stage.

## The whole checks

`sluice exec main`, run once against `main` at `f954b77`: 20 tasks
completed and 2 failed, in 4m 19s.

| task | outcome |
|---|---|
| `boomdrive-hosted:lint` | **failed**: clippy, `-D warnings` |
| `boomdrive:test` | **failed**: `tests/control.rs`, `a_keeping_pass_under_way` |
| `boomdrive-hosted:format`, `:typecheck`, `:test` | passed |
| `boomdrive:format`, `:lint`, `:typecheck` | passed |
| `boomdrive-control:format`, `:lint`, `:typecheck`, `:test` | passed |
| `boomdrive-crew:format`, `:lint`, `:typecheck`, `:test` | passed |
| `nix:typecheck`, `nix:test` | passed |
| `root:format`, `root:changes`, `root:releases`, `root:versions` | passed |

sluice also says each failed task "could not run" because its sandbox
refused the run, and records nothing for those two:

```
could not run: boomdrive-hosted:lint: sluice's sandbox refused the run network-outbound remote:*:53 (by moon), network-outbound /private/var/run/mDNSResponder (a name lookup, by moon), file-write-data /Users/chuck/.cargo/.package-cache (by cargo) and 25 more
could not run: boomdrive:test: sluice's sandbox refused the run network-outbound remote:*:53 (by moon), network-outbound /private/var/run/mDNSResponder (a name lookup, by moon), file-write-data /Users/chuck/.cargo/.package-cache (by cargo) and 25 more
nothing was recorded for a test that could not run: once the host can run it, run again
```

Both tasks also failed on their own terms, as below.

### `boomdrive-hosted:lint`

```
error: very complex type used. Consider factoring parts into `type` definitions
   --> crates/boomdrive-hosted/tests/common/fake.rs:933:33
    |
933 |     (key, time, nonce, signed): (Option<[u8; 32]>, Option<u64>, Option<[u8; 16]>, bool),
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D clippy::type-complexity` implied by `-D warnings`
error: could not compile `boomdrive-hosted` (test "ends") due to 1 previous error
error: could not compile `boomdrive-hosted` (test "forms") due to 1 previous error
error: could not compile `boomdrive-hosted` (test "forge_tokens") due to 1 previous error
error: could not compile `boomdrive-hosted` (test "files") due to 1 previous error
```

This change wrote that line: `forge_answer`, the fake control plane's
answer to `runner/forge-token`, came in `90472a95` (task 3.3).

### `boomdrive:test`

```
---- a_keeping_pass_under_way stdout ----
thread 'a_keeping_pass_under_way' panicked at crates/boomdrive/tests/control.rs:693:14:
a pull made in time: Timeout

test result: FAILED. 30 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 48.04s
error: test failed, to rerun pass `-p boomdrive --test control`
```

This change does not touch `tests/control.rs` or `src/pass.rs`. The test
runs under crew's control plane, where `Forge::for_runner` makes
`Forge::new()`, as `run.rs` did before. sluice had the host at its bound
when the run began. Following this stage's brief, the check was not run a
second time, so whether this is a timing failure under load is still
open. Cargo stopped at this failure, so the gate did not run the test
binaries after `control`. Those include `tests/forge_tokens.rs`, which
holds every forge-tokens scenario of a cloud host's `get` and keeping.
The gate passed `tests/clones.rs` and its four new tests, and every test
of `boomdrive-hosted` and `boomdrive-control`.

## OpenSpec

- `openspec instructions apply --change a-cloud-host-gets-its-github-tokens-from-the-cloud --json`:
  `state` `all_done`, 22 of 22 tasks done.
- `openspec validate a-cloud-host-gets-its-github-tokens-from-the-cloud --strict`:
  `Change 'a-cloud-host-gets-its-github-tokens-from-the-cloud' is valid`.
- `openspec archive` was not run (the checks fail).

## The work against the specs

Each scenario of the four delta specs (clones, forge-tokens,
hosted-control-plane, runner-protocol) has the test its task names, in the
file the task names:

- **runner-protocol**: `forms.rs`, `refused.rs`, `names.rs`, `files.rs`
  (`version_4_is_version_3_but_for_a_forge_token`,
  `version_3_is_as_released`), `examples.rs` and `signatures.rs`
  (`the_asks_worked_context`).
- **hosted-control-plane**: `tests/forge_tokens.rs`, all twelve ask
  scenarios, and `connection.rs` (`a_message_of_version_3`,
  `a_forge_tokens_answer_on_the_socket`).
- **forge-tokens**: `crates/boomdrive/tests/forge_tokens.rs`, eleven cloud
  host tests and `no_forge_must_be_fixed`. `a_get_on_a_cloud_host_is_refused`
  and `an_erase_on_a_cloud_host_is_refused` are gone.
- **clones**: `crates/boomdrive/tests/clones.rs`, all three scenarios.
- **The interface**: `boomdrive-control`'s unit tests
  `a_forge_tokens_debug_names_its_end_alone` and
  `the_fake_tokens_count_each_ask`.

The code follows design Decisions 1 to 10:

- `protocol/v4/` holds twelve schema files, with `forge-token.schema.json`'s
  three closed kinds and four new examples.
- No file under `protocol/v1/`, `v2/` or `v3/` differs from the branch's
  base, and `v3/` equals `v0.18.0`'s.
- `NO_FORGE_TOKEN` is gone from the tree.
- The hosted crate's `forge_token::Tokens` asks each served partition in
  the table's order, checks each answer against version 4 whatever its
  status, and takes only an answer that names its ask and its repository.
- `Forge::asking` and `Forge::fresh` keep what the control plane gives as
  a minted token is kept.
- `forge-token` and `run` each make their `Forge` through `planes::forge_tokens`.
- Record 0053, record 0007's amendment of 2026-11-18 and core.33's and
  core.42's amendments are in the tree.
- Nothing outside proposal.md's Touches changed.

No scenario was found unmet in the code that was read. What holds the
change back is the two failed tasks above.
