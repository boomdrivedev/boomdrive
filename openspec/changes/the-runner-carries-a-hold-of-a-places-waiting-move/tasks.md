# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings, for reading.

- Every test runs on the scratch host of its crate's tests, with neutral names and paths (core.41), and no real Flume or sluice.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). No file under `crates/boomdrive-hosted/protocol/v1/` or `protocol/v2/` changes (core.42). `flume::LEAST` and the set's Flume do not move (design Decision 8). Nothing outside the paths proposal.md's Touches lists is changed. Record 0002's amendment of 2026-11-14 and core.59 are already written with the change.

## 1. The interface names a hold

- [x] 1.1 `boomdrive-control`'s `ActSort` gains `Hold { words: String, bound_interval: u64 }`, whose serde form is `{"hold": {"words": …, "bound_interval": …}}`. Its documentation says it asks that the move the launch's place waits on its caller for be held under its words for its bound, in whole seconds from when Flume takes it, and that the runner carries it out as Flume's hold of that waiting move (core.59; Flume's 130). It also says no version of the hosted protocol names a hold (design Decisions 1 and 7; Task notes 1.1). The binary's crate builds again at 2.1, its tests that hold `mod fakes;` at 2.2, and the hosted crate's `tests/interface.rs` at 4.1. True when `devenv shell -- cargo test -p boomdrive-control --test json an_act_of_each_sort` passes, proving the control-plane-interface spec's "An act of each sort" and "A hold's form".

## 2. The runner carries out a hold

- [x] 2.1 `crates/boomdrive/src/flume.rs`'s `Act` gains `Hold { words, bound_interval }`, whose arguments are `hold <place> --waiting --for=<n>s -- <words>`. In `crates/boomdrive/src/carry.rs`:
  - `flume_act` maps `ActSort::Hold` to it, with the act's words and bound as given;
  - `sort` names it `hold for <n>s`;
  - `act_unlaunched` refuses it, naming where the launch stands, as it refuses a retry, a land, a confirm or a done.

  `tests/flume.rs` gains `a_holds_argv`, and its module documentation names a hold (design Decisions 2 and 5; Task notes 2.1). True when `devenv shell -- cargo test -p boomdrive --test flume a_holds_argv` passes, proving the work-into-flume spec's "The words not read as a flag".
- [ ] 2.2 `tests/fakes/mod.rs`'s `FakeFlume` answers a hold as any act, changing no place. A new `tests/holds.rs` holds the hold's tests. A hold runs Flume's hold of its launch's place's waiting move with exactly the act's words and bound, says its line with no words, and carries Flume's refusal as its answer (design Decisions 2 and 3; Task notes 2.2). True when `devenv shell -- cargo test -p boomdrive --test holds -- a_hold_is_flumes_hold_of_the_waiting_move flumes_refusal_is_the_holds_answer` passes, proving the work-into-flume spec's "A hold" and "A hold Flume refuses".
- [ ] 2.3 `Work::act`, once the act's launch is the runner's and its lease is live, refuses a hold whose words are empty, with design Decision 4's words, running no command, beside its refusals of a confirm (design Decision 4; Task notes 2.2). True when `devenv shell -- cargo test -p boomdrive --test holds a_hold_with_no_words_is_refused` passes, proving the work-into-flume spec's "A hold with no words".
- [ ] 2.4 A hold of a launch held on its After is refused with the words a retry of it is given, and runs no `flume hold` (design Decision 5; Task notes 2.4). True when `devenv shell -- cargo test -p boomdrive --test after a_hold_of_a_held_launch` passes, proving the work-into-flume spec's "A hold of a held launch".

## 3. crew's implementation reads a hold

- [ ] 3.1 `read::want` gives a request whose `Act` is `hold` as `ActSort::Hold` on the launch its `Of` names, its `words` its `Words`, read as a retry's are, and its `bound_interval` its `Bound`, reading no other field. `Request::of`'s documentation names a hold. `tests/common/mod.rs`'s copy of crew's `requests.rec` descriptor gains `hold` in its `Act` enum, `%type: Bound int`, and `Bound` in its `%allowed` after `Delivery`, and its `FIELDS` gains `Bound` after `Delivery`, as crew's unit `a-bolt-waiting-on-the-user-waits-under-words` writes them (design Decision 6; Task notes 3.1). True when `devenv shell -- cargo test -p boomdrive-crew --test read a_hold_is_an_act_on_the_launch_it_names` passes, proving the crew-control-plane spec's "A hold".
- [ ] 3.2 A hold naming no `Words`, no `Bound`, or a `Bound` that is not decimal digits fitting a `u64` cannot be read: `it names no Words`, `it names no Bound` or `its Bound <text>, which is no whole number of seconds` (design Decision 6; Task notes 3.2). True when `devenv shell -- cargo test -p boomdrive-crew --test read a_hold_naming_no_words_no_bound_or_a_bound_of_no_whole_seconds_is_refused` passes, proving the crew-control-plane spec's "A hold naming no words, no bound, or a bound of no whole seconds".
- [ ] 3.3 A `hold` naming no `Host` goes to the runner that holds its launch, and to no other, as a `retry`, `land`, `confirm`, `drop` or `priority` does (`crates/boomdrive-crew/src/lib.rs`; design Decision 6; Task notes 3.3). True when `devenv shell -- cargo test -p boomdrive-crew --test claims a_hold_naming_no_host_goes_to_the_runner_holding_its_launch` passes, proving the crew-control-plane spec's "A hold naming no host, of a launch this runner holds".

## 4. No version carries a hold

- [ ] 4.1 `every_sort_the_interface_names` names the hold among the act sorts, gives a value of it, and finds it refused by version 2's `want` while every other sort's value passes and the sorts named are still every sort the interface has. Its match with no `_` arm names the hold, so the hosted crate's `tests/interface.rs`, which 1.1 left unbuilt, builds again (design Decision 7; Task notes 4.1). True when `devenv shell -- cargo test -p boomdrive-hosted --test interface every_sort_the_interface_names` passes, proving the runner-protocol spec's "Every sort the interface names".
- [ ] 4.2 A want whose act is a hold, as `boomdrive-control`'s types write it, fails version 1's `want` and version 2's, and no file of either version changes (design Decision 7; Task notes 4.2). True when `devenv shell -- cargo test -p boomdrive-hosted --test interface a_hold_is_no_act_of_version_1_or_2` passes, proving the runner-protocol spec's "A hold is no act of version 1 or 2".

## 5. The words

- [ ] 5.1 The README's control pass says the runner carries out each retry, land, confirm, hold, drop, priority or done as Flume's own command on the launch's place. It says a hold is `flume hold <place> --waiting --for=<n>s -- <words>`, with exactly the words and bound its control plane names, and that the runner judges nothing of the wait and renews and ends no hold of its own accord. Its hosted section says no version carries a hold yet. `crates/boomdrive-hosted/protocol/README.md`'s list of what version 2 has no message for gains a hold of a place's waiting move (design Decisions 7 and 10). True when `sluice exec main -- moon run root:format` passes.
- [ ] 5.2 The spec deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive the-runner-carries-a-hold-of-a-places-waiting-move --yes`, `sluice exec main -- moon run root:specs` passes.
