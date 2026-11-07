# Verify

The verify stage of `the-runner-reads-a-local-layer-over-its-managed-one`, on the branch `change/the-runner-reads-a-local-layer-over-its-managed-one` at `90d90e8`, 2026-11-07.

**Verdict: not passed.** The gate fails on one task, `boomdrive:lint`. The work otherwise meets the change: every requirement and scenario of its two specs is built and tested, and every task is ticked. The change goes back to its code stage.

## Where the change stands

Task 8.1 had the code stage archive the change, and `90d90e8` holds it archived, at `openspec/changes/archive/2026-11-07-the-runner-reads-a-local-layer-over-its-managed-one/`. The directory the brief names, `openspec/changes/the-runner-reads-a-local-layer-over-its-managed-one/`, is gone, so this report is written in the archived one. The verify stage archived nothing and changed no file but this one.

## The whole checks

`devenv shell -- sluice exec main`, run once at `90d90e8`: exit 1. 19 tasks passed, 1 failed.

| task | outcome |
|---|---|
| `boomdrive:format` | passed |
| `boomdrive:lint` | **failed** |
| `boomdrive:typecheck` | passed |
| `boomdrive:test` | passed |
| `boomdrive-crew:format`, `:lint`, `:typecheck`, `:test` | passed |
| `boomdrive-hosted:format`, `:lint`, `:typecheck`, `:test` | passed |
| `nix:format`, `:lint`, `:typecheck`, `:test` | passed |
| `root:format`, `root:adrs`, `root:specs`, `root:changes` | passed |

Every test binary the run built passed: 58 test results, none failed.

The failure, clippy under `-D warnings`, which fails the test binary `local_layer`'s build in the lint:

```
error: unnecessary use of `clone` to create a slice from a reference
   --> crates/boomdrive/tests/local_layer.rs:292:5
    |
292 |     assert_eq!(running.until(&named), [named.clone()]);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#cloned_ref_to_slice_refs
    = note: `-D clippy::cloned-ref-to-slice-refs` implied by `-D warnings`
error: could not compile `boomdrive` (test "local_layer") due to 1 previous error
```

clippy's own fix is `std::slice::from_ref(&named)` in place of `[named.clone()]`. The code stage makes it, and runs `sluice exec main -- moon run boomdrive:lint`.

## Tasks

All 13 tasks are ticked `[x]`, and each is held by what it names:

| task | held by |
|---|---|
| 1.1 record 0009's amendment of 2026-11-07; core.5, core.17, core.46 | the amendment holds Decisions 1 to 7, each with the path not taken; core.17's details gain Decision 11's words; core.5's and core.46's details read "local layer"; `root:format` and `root:adrs` pass |
| 2.1 `LOCAL_KEYS`, `Entry::local_keys` and `Entry::read` | both crates' `LOCAL_KEYS = ["partitions"]`; `tests/planes.rs` `each_implementation_lists_partitions_for_the_local_layer` |
| 3.1 the two layers | `config::local_path`, `read_layers`, `parse_layers`, `LocalLayer`, `PinnedKey`; unit tests `no_local_file`, `the_local_layers_repositories`, `a_repository_both_layers_name`, `the_local_layers_partitions`, `the_local_layers_file` |
| 3.2 a local layer at fault | `a_root_in_the_local_layer`, `a_control_plane_named_in_the_local_layer`, `a_token_in_the_local_layer`, `a_local_file_that_is_not_toml`, `a_local_address_out_of_form`, `a_partition_the_implementation_refuses`, `partitions_of_a_control_plane_the_managed_layer_does_not_name`, `a_local_layer_naming_one_repository_twice` |
| 3.3 `pin` | `repositories_pinned`, `a_table_pinned`, `a_pin_of_nothing`, `a_pin_of_another_kind`, `a_pin_in_the_local_layer` |
| 4.1 every reading reads both layers | `Runner` holds `local`, and its start, keeping pass, control pass and landing read through `read_layers`; `tests/running.rs` `the_default_file`, `the_file_named`, `no_local_file` |
| 4.2 `enroll` reads both layers | `enroll.rs` reads through `read_layers`; `tests/enroll.rs` `a_partition_of_the_local_layer` |
| 5.1 the line, once each way | `Runner::say_local`, `local_line`; `tests/local_layer.rs` `taken_at_the_start`, `a_fault_said_once`, `pinned_repositories_passed_over_said_once` |
| 5.2 nothing let go while not taken | `pass::holds_back` skips the let-go loop and `wanted_workspaces`; `a_local_file_broken_while_running`, `a_repository_the_local_layer_drops`, `a_local_file_removed` |
| 6.1 the modules leave the local layer | `nix/tests/activate.sh` "A local layer beside it"; `nix:test` passes |
| 7.1 README | the two layers, what the local layer sets, `pin`, a local layer at fault, and a second ```toml block; `tests/readme.rs` `the_readme_local_layer_reads` |
| 8.1 archived into the specs | `90d90e8`; `root:specs` passes |

Every test is held by `boomdrive:test`, which passes. The test binary `local_layer` holds the lint's one failure, but it builds and passes under test.

## Requirements and scenarios

### configuration

| requirement | scenario | held by |
|---|---|---|
| A managed file and a local one over it | The default file | `running.rs` `the_default_file` |
| | The file named | `running.rs` `the_file_named` |
| | No file | the existing `running.rs` test, unchanged in meaning |
| | No local file | `running.rs` `no_local_file`; unit `no_local_file` |
| The local layer sets the repositories and a control plane's partitions alone | The local layer's repositories | unit `the_local_layers_repositories` |
| | A partition enrolled from the local layer | `enroll.rs` `a_partition_of_the_local_layer` |
| Repositories add, partitions replace | A repository both layers name | unit `a_repository_both_layers_name` |
| | A repository the local layer drops | `local_layer.rs` `a_repository_the_local_layer_drops` |
| | The local layer's partitions | unit `the_local_layers_partitions` |
| Any other key is not taken | A root in the local layer | unit `a_root_in_the_local_layer` |
| | A control plane named in the local layer | unit `a_control_plane_named_in_the_local_layer` |
| | A token in the local layer | unit `a_token_in_the_local_layer`, the value in no output |
| A local layer the runner cannot read is not taken | A local file that is not TOML | unit `a_local_file_that_is_not_toml`, its file and line named, its text never |
| | A local address out of form | unit `a_local_address_out_of_form` |
| | A partition the implementation refuses | unit `a_partition_the_implementation_refuses`, the managed table handed on |
| While not taken, nothing is let go | A local file broken while running | `local_layer.rs` `a_local_file_broken_while_running`, the workspace's `folders` unchanged |
| | A local file removed | `local_layer.rs` `a_local_file_removed`, the gone line once |
| The managed layer may pin | Repositories pinned | unit `repositories_pinned` |
| | A table pinned | unit `a_table_pinned` |
| A pin of nothing is the managed layer's fault | A pin of nothing | unit `a_pin_of_nothing`, by `read` and `read_layers` alike |
| Said once each way | Taken at the start | `local_layer.rs` `taken_at_the_start` |
| | A fault said once | `local_layer.rs` `a_fault_said_once` |

### service-modules

| requirement | scenario | held by |
|---|---|---|
| The file is at the runner's own path for commands run by hand | A local layer beside it | `nix/tests/activate.sh`, under `nix:test` |

The code follows design Decisions 1 to 10 as written: the local path is the default file's directory whatever `--config` names; `config.rs` takes the local keys and each implementation's reading from `planes::PLANES` and names no implementation's key (core.2); a fault names the local file and a key's dotted path, never a value (core.4, core.11); the line's words are Decision 7's.

## Notes, none blocking

- The archive tightened two requirement texts to openspec's strict limit of 500 characters, in the delta and in `openspec/specs`. Each still requires what it did; the service-modules text drops only its reason, "since the operator or a control plane writes it".
- The configuration spec's Purpose still opens "The runner's configuration on a host is one TOML file". `openspec archive` writes no Purpose; a later change may say "two layers".
