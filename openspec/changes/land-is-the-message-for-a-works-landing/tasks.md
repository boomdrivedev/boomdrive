# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings, for reading.

- Every test runs on the scratch host of its crate's tests, with neutral names and paths (core.41), and no real Flume or sluice.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). No file under `crates/boomdrive-hosted/protocol/` changes (core.42). Nothing outside the paths proposal.md's Touches lists is changed. Record 0002's amendment of 2026-11-08, core.52 and core.53 are already written with the change.

## 1. The interface names a land

- [ ] 1.1 `boomdrive-control`'s `ActSort` gains `Land`, a unit variant whose serde form is `"land"`. Its documentation says it asks for the landing of its launch's work, holds nothing but its launch, and is carried out as Flume's retry of the submission (core.53). It also says version 1 of the hosted protocol names no land, a later version being the one to carry it (core.42) (design Decisions 1 and 7; Task notes 1.1). True when `devenv shell -- cargo test -p boomdrive-control --test json an_act_of_each_sort` passes, proving the control-plane-interface spec's "An act of each sort".

## 2. The runner carries out a land

- [ ] 2.1 `crates/boomdrive/src/carry.rs` carries out a land:
  - `flume_act` maps `ActSort::Land` to `flume::Act::Retry { stage: None, words: None, submission: true }`;
  - `sort` names it `land`;
  - `act_unlaunched` refuses it, naming where the launch stands, as it refuses a retry or a done.

  (design Decisions 2 to 4; Task notes 2.1.) True when `devenv shell -- cargo test -p boomdrive --test acts a_land` passes, proving the work-into-flume spec's "A land".
- [ ] 2.2 A land of a launch held on its After is refused with the words a retry of it is given, runs no `flume retry`, and leaves the launch to be made as its offer gives it (design Decision 4; Task notes 2.2). True when `devenv shell -- cargo test -p boomdrive --test after a_land_of_a_held_launch` passes, proving the work-into-flume spec's "A land of a held launch".
- [ ] 2.3 A bolt's send is a land, carried out as `flume --json retry <place> --submission`, and Flume's refusal of it is the act's answer. `tests/bolts.rs` sends it as `ActSort::Land`, and its module documentation says so (design Decision 2; Task notes 2.3). True when `devenv shell -- cargo test -p boomdrive --test bolts` passes with `a_bolts_send` and `a_send_flume_refuses`, proving the work-into-flume spec's "A bolt's send" and "A send Flume refuses".

## 3. crew's implementation reads a land

- [ ] 3.1 `read::want` gives a request whose `Act` is `land` as `ActSort::Land` on the launch its `Of` names, reading no other field of it. `tests/common/mod.rs`'s copy of crew's `requests.rec` descriptor gains `land` in its `Act` enum (design Decision 6; Task notes 3.1). True when `devenv shell -- cargo test -p boomdrive-crew --test read a_land_is_an_act_on_the_launch_it_names` passes, proving the crew-control-plane spec's "A land".
- [ ] 3.2 A `land` naming no `Host` goes to the runner that holds its launch, and to no other, as a `retry`, `drop` or `priority` does (`crates/boomdrive-crew/src/lib.rs`; design Decision 6; Task notes 3.2). True when `devenv shell -- cargo test -p boomdrive-crew --test claims a_land_naming_no_host_goes_to_the_runner_holding_its_launch` passes, proving the crew-control-plane spec's "A land naming no host, of a launch this runner holds".

## 4. Version 1 carries no land

- [ ] 4.1 A want whose act is a land, as `boomdrive-control`'s types write it, fails version 1's `want`. Every act sort version 1 names still passes, and no protocol file changes (design Decision 7; Task notes 4.1). True when `devenv shell -- cargo test -p boomdrive-hosted --test interface` passes with `a_land_is_no_act_of_version_1` and `the_interfaces_values_pass`, proving the runner-protocol spec's "A land is no act of version 1" and "The interface's values pass".

## 5. The words

- [ ] 5.1 The README's control pass says the runner carries out each retry, land, drop, priority or done as Flume's own command on the launch's place. It says a land is `flume retry <place> --submission`, whatever the launch's kind, and that a bolt is landed on a land, whose answer is Flume's. It no longer says a bolt's send is a retry naming its submission (design Decision 9). True when `sluice exec main -- moon run root:format` passes.
- [ ] 5.2 The spec deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive land-is-the-message-for-a-works-landing --yes`, `sluice exec main -- moon run root:specs` passes.
