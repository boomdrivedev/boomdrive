# Verify: the-runner-relays-a-terminal-only-through-flume

The work holds. The gate passes, all fourteen tasks are ticked, and a test proves each scenario of the three deltas.

The code stage archived the change under task 5.1, because `root:specs` can prove the delta and the terminal-relay spec's Purpose only after the archive. This report sits beside the archive, on `change/the-runner-relays-a-terminal-only-through-flume` at `3376ef1`.

## The whole checks

`sluice exec main` ran once on `3376ef1`. It ran the checks that commit affects against `main` (`23e4181`) and exited 0. All 29 passed, and sluice recorded their results under their keys for `3376ef1`:

- `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test` each;
- `nix`: `format`, `lint`, `typecheck` and `test`;
- `root`: `format`, `adrs`, `changes`, `releases` and `specs`;
- `site`: `check`, `build`, `prose` and `test`, with `install` and `vocab`, which they depend on and which carry no `check-fast` tag, run among moon's 31 tasks.

The crates' tests ran 71 test binaries, 1067 tests, with none failed. `root:specs` passed 27 items, with none failed. `root:adrs` read 45 records, each numbered apart. `root:changes` found no open change to validate. moon also warned 30 times that it could not write its output archives under `.moon/cache/outputs`, and once that it skipped the Rust dependency install for want of a network. Both warnings come from the sandbox, and no check failed because of them.

## Tasks

All fourteen tasks are ticked, 1.1 to 5.1, and none is open. Each names a check, and the gate's run holds each one passing:

| Task | Its check, as the gate ran it |
|---|---|
| 1.1 | `tests/flume.rs`'s `a_relays_argv` |
| 1.2 | `tests/terminals.rs`'s `a_launch_not_held`, `a_launch_at_no_place`, `flumes_refusal_closes_the_terminal` and `a_flume_with_no_relay` |
| 1.3 | `screens_as_flume_gives_them` and `an_unread_read_and_an_untyped_input` |
| 1.4 | `input_passed_in` |
| 1.5 | `the_agents_terminal_gone`, `the_link_closed`, `the_far_end_stops_listening`, `the_launch_let_go`, `a_launch_the_control_plane_lets_go` and `the_runner_stopped` |
| 1.6 | `nothing_written` |
| 1.7 | `tests/family.rs`'s `the_sets_flume_answers_a_relay`, on the set's own Flume |
| 2.1 | `boomdrive:lint`, and `tests/offers.rs`'s `a_tool_missing` |
| 2.2 | `tests/hello.rs`, with `the_hello_runs_no_multiplexer` |
| 2.3 | `boomdrive-control`'s `tests/json.rs`'s `a_hello` |
| 3.1 | `tests/facts.rs`'s `a_live_place_is_live_and_unmarked` |
| 3.2 | `tests/rules.rs`'s `no_crate_names_a_multiplexer` |
| 4.1 | `boomdrive-hosted`'s `tests/plane.rs`'s `a_terminal_opened` |
| 5.1 | `root:specs`, after the archive |

What no task changes stays unchanged. The branch adds no crate, tool or tracked file type, and changes no file under `crates/boomdrive-hosted/protocol/`, no field of `report.json` and no configuration key. `flume::LEAST` stays `0.21.0`. The flake, its modules and setup keep herdr. The tasks say record 0004's amendment of 2026-11-15 and the details of core.11, core.13 and core.29 were written in the planning commit, `6c8c7cb9`, and they were.

The diff from the branch's base, `9bba58b`, also changes `crates/boomdrive/src/flume.rs` and `crates/boomdrive/tests/flume.rs`, which proposal.md's Touches does not list. Task 1.1 and design Decision 1 name both, so the change is in the plan, and only the Touches list leaves them out. Task note 3.1 names the renamed test `a_live_place`, and task 3.1 names it `a_live_place_is_live_and_unmarked`. The code follows the task.

## The deltas, scenario by scenario

The tests are `crates/boomdrive/tests/terminals.rs`'s unless the table names another file.

### terminal-relay

| Scenario | Proven by |
|---|---|
| Nothing written | `nothing_written`: a screen holding `secret-screen-text` and the typed `secret-typed-text`, which Flume's input held, are in no file under the data directory, no line and no report; the relay's lines are the open and the close alone |
| A launch not held | `a_launch_not_held`: `o9` under lease 1 and `o1` under lease 2 are each closed with `this runner holds no launch … under lease …`, and the stub has no call |
| A launch at no place | `a_launch_at_no_place`: `o2`, held on its After, is closed with `o2 has no place Flume named`, and the stub has no call |
| Flume refuses the relay | `flumes_refusal_closes_the_terminal`: the stub's one call is `--json relay <place>`, and the link is closed with the refusal's words |
| A Flume with no relay | `a_flume_with_no_relay`: the usage error on standard error, exit 2, closes the link with its one final newline left off |
| The set's Flume refuses a relay of no place | `tests/family.rs`'s `the_sets_flume_answers_a_relay`: the set's Flume closes the link with `no place <scratch>/places/nowhere stands on this host`, and no host tool ran |
| Screens as Flume gives them | `screens_as_flume_gives_them`: three screens, the last two the same, reach the link in order as given, and the stub records `FLUME_RECIPIENT` unset though the test's environment set it |
| An unread read and an untyped input | `an_unread_read_and_an_untyped_input`: the unread line is said with Flume's words, the untyped line names `t1`, `o1` and the place and holds no `secret-key-name`, and the screen after both reaches the link |
| Input passed in | `input_passed_in`: Flume's input holds `{"text":"yes"}`, then `{"keys":["ctrl+c","enter"]}` |
| The link closed | `the_link_closed`: Flume's input reaches its end, and the link is closed with `its link closed` |
| The launch let go | `the_launch_let_go` (a report refusing the launch) and `a_launch_the_control_plane_lets_go`: Flume's input reaches its end, and the link is closed with `its launch o1 is let go` |
| The runner stopped | `the_runner_stopped`: the drop returns, Flume's input having reached its end, and the link is closed with `the runner stopped` |
| The agent's terminal gone | `the_agents_terminal_gone`: the link is closed with `Flume's relay ended: gone`, and the thread ends |
| The crates' sources read | `tests/rules.rs`'s `no_crate_names_a_multiplexer`: every `.rs` file under each crate's `src` but `setup.rs` holds no `HERDR_` and no literal beginning `"herdr`, `"tmux`, `"zellij` or `"rex` |

`the_far_end_stops_listening` also proves the requirement's fourth way to close, `its far end stopped listening`, which has no scenario of its own.

### work-into-flume

| Scenario | Proven by |
|---|---|
| Repositories kept | unchanged, and passing |
| A tool missing | `tests/offers.rs`'s `a_tool_missing`: `sluice` is named missing, and the pass goes on |
| No multiplexer's version | `tests/hello.rs`'s `the_hello_runs_no_multiplexer`: the tools are `flume`, `git` and `sluice`, and the program given for `herdr` never writes its file |

### hosted-control-plane

| Scenario | Proven by |
|---|---|
| A terminal opened | `crates/boomdrive-hosted/tests/plane.rs`'s `a_terminal_opened`: `terminal_closed` with `NO_TERMINAL`, whose words name `end-to-end` and not Flume, and nothing else after the hello |

## The code

- `crates/boomdrive/src/terminal.rs` relays through `flume::Cli::relay`, which spawns `flume --json relay <place>` with `FLUME_RECIPIENT` removed and its three streams piped. It opens only where the report holds the launch under the terminal's lease at a `place_dir`, and reads no pane. Flume's output and standard error are read on threads of their own. A screen is sent only while the runner has not closed the relay. An `untyped` line's words are parsed as `IgnoredAny`, so they are never read. A close of the runner's drops Flume's input, reads Flume's output to its end and awaits its exit, signalling nothing. Design Decision 5's table of words is met row by row in `ended`.
- `hello::TOOLS` is `["flume", "sluice", "git"]`, and `Programs` holds `flume` and `sluice` alone.
- `facts::Pane` and `facts::pane` are gone, with their unit tests.
- No `.rs` file under a crate's `src` names herdr but `setup.rs`, whose plugin link core.29 allows, and two comments: `main.rs`'s word on setup's plugin link and `view.rs`'s citation of `herdr.14`. Decision 10 lets both stand.
- `NO_TERMINAL` holds design Decision 8's words, citing core.30 and record 0004 as amended on 2026-11-15.

## Archive

The change is at `openspec/changes/archive/2026-11-15-the-runner-relays-a-terminal-only-through-flume`, and no change of that name is open. `openspec/specs/terminal-relay/spec.md`, `work-into-flume/spec.md` and `hosted-control-plane/spec.md` hold the deltas. The terminal-relay spec's Purpose is design Decision 13's, naming `flume relay` and no multiplexer, and `root:specs` passes all three under the strict check.
