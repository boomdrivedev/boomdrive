# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings, for reading.

- Every test runs on the scratch host of its crate's tests, with neutral names and paths (core.41), and no real Flume or sluice.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). No file under `crates/boomdrive-hosted/protocol/` changes (core.42). `flume::LEAST` and the set's Flume do not move (design Decision 8). Nothing outside the paths proposal.md's Touches lists is changed. Record 0002's amendment of 2026-11-09 and core.56 are already written with the change.

## 1. The interface names a confirm

- [ ] 1.1 `boomdrive-control`'s `ActSort` gains `Confirm { recipient: String, deliveries: Vec<String> }`, whose serde form is `{"confirm": {"recipient": …, "deliveries": […]}}`. Its documentation says it asks that each end of the launch's place it names by its delivery's key be taken as delivered to the one recipient it names, in the control plane's order, and that the runner carries it out as Flume's confirm of exactly those keys, each held to the launch's place (core.56). It also says version 1 of the hosted protocol names no confirm (design Decisions 1 and 7; Task notes 1.1). The binary's crate builds again at 2.1. True when `devenv shell -- cargo test -p boomdrive-control --test json an_act_of_each_sort` passes, proving the control-plane-interface spec's "An act of each sort".

## 2. The runner carries out a confirm

- [ ] 2.1 `crates/boomdrive/src/flume.rs`'s `Act` gains `Confirm { recipient, keys }`, whose arguments are `confirm --recipient=<recipient> -- <key>…`, naming no place. In `crates/boomdrive/src/carry.rs`:
  - `flume_act` maps `ActSort::Confirm` to it, with the act's keys in its order;
  - `sort` names it `confirm as <recipient>`;
  - `act_unlaunched` refuses it, naming where the launch stands, as it refuses a retry, a land or a done.

  `tests/flume.rs` gains `a_confirms_argv`, and its module documentation names a confirm (design Decisions 2 and 5; Task notes 2.1). True when `devenv shell -- cargo test -p boomdrive --test flume a_confirms_argv` passes, proving the work-into-flume spec's "Neither the recipient nor a key read as a flag", and "A confirm" in Flume's arguments.
- [ ] 2.2 `tests/fakes/mod.rs`'s `FakeFlume` carries out a confirm as any act, changing no place, and `fakes::ends` gives a place's ends by their delivery keys. A new `tests/confirms.rs` holds the confirm's tests. A confirm runs Flume's confirm of exactly the keys it names, as the recipient it names, on its launch's place, says its line, and carries Flume's refusal as its answer (design Decision 2; Task notes 2.2). True when `devenv shell -- cargo test -p boomdrive --test confirms -- a_confirm_names_only_the_keys_it_was_given flumes_refusal_is_the_confirms_answer` passes, proving the work-into-flume spec's "A confirm" and "A confirm Flume refuses".
- [ ] 2.3 Before it runs a confirm, `act_on` reads the launch's place's entry: the pass's listing's, where it holds the place, else `facts::entry` of the status Flume gives for it, which `act_on` now keeps. `facts::deliveries` gives each end's `delivery` of an entry. A confirm naming a key that is none of them is answered `refused` with the runner's words, naming that key and the place, ahead of the check of whether the next pass is due, and no `flume confirm` is run (design Decision 3; Task notes 2.3). True when `devenv shell -- cargo test -p boomdrive --test confirms a_key_of_no_end_of_the_launchs_place_is_refused` passes, proving the work-into-flume spec's "A key of another place".
- [ ] 2.4 A confirm of a tidied place's record Flume keeps is carried out, and a confirm on a place the listing does not hold is held to the status Flume gives for it (design Decision 3; Task notes 2.4). True when `devenv shell -- cargo test -p boomdrive --test confirms -- a_tidied_places_record_is_confirmed a_place_the_listing_does_not_hold_is_held_to_its_status` passes, proving the work-into-flume spec's "A tidied place's record" and "A place the listing does not hold".
- [ ] 2.5 `Work::act`, once the act's launch is the runner's and its lease is live, refuses a confirm whose recipient is `flume::RECIPIENT` or that names no key, each with design Decision 4's words, running no command (design Decision 4; Task notes 2.5). True when `devenv shell -- cargo test -p boomdrive --test confirms -- a_confirm_as_the_runner_is_refused a_confirm_of_no_key_is_refused` passes, proving the work-into-flume spec's "A confirm as the runner" and "A confirm of no key".
- [ ] 2.6 A confirm of a launch held on its After is refused with the words a retry of it is given, and runs no `flume confirm` (design Decision 5; Task notes 2.6). True when `devenv shell -- cargo test -p boomdrive --test after a_confirm_of_a_held_launch` passes, proving the work-into-flume spec's "A confirm of a held launch".
- [ ] 2.7 The binary's sources and tests this group touched pass clippy with no warning. True when `sluice exec main -- moon run boomdrive:lint` passes.

## 3. crew's implementation reads a confirm

- [ ] 3.1 `read::want` gives a request whose `Act` is `confirm` as `ActSort::Confirm` on the launch its `Of` names, its `recipient` the one `Recipient` and its `deliveries` each `Delivery` in the order written, reading no other field. `Request::of`'s documentation names a confirm. `tests/common/mod.rs`'s copy of crew's `requests.rec` descriptor gains `confirm` in its `Act` enum and `Delivery` in its `%allowed`, and its `FIELDS` gains `Delivery` after `Words`, as crew's main at fa8501c1 holds them (design Decision 6; Task notes 3.1). True when `devenv shell -- cargo test -p boomdrive-crew --test read a_confirm_is_an_act_on_the_launch_it_names` passes, proving the crew-control-plane spec's "A confirm".
- [ ] 3.2 A confirm naming no `Recipient`, two, or no `Delivery` cannot be read: `it names no Recipient`, `it names Recipient twice` or `it names no Delivery` (design Decision 6; Task notes 3.2). True when `devenv shell -- cargo test -p boomdrive-crew --test read a_confirm_naming_no_recipient_two_or_no_delivery_is_refused` passes, proving the crew-control-plane spec's "A confirm naming no recipient, two, or no delivery".
- [ ] 3.3 A `confirm` naming no `Host` goes to the runner that holds its launch, and to no other, as a `retry`, `land`, `drop` or `priority` does (`crates/boomdrive-crew/src/lib.rs`; design Decision 6; Task notes 3.3). True when `devenv shell -- cargo test -p boomdrive-crew --test claims a_confirm_naming_no_host_goes_to_the_runner_holding_its_launch` passes, proving the crew-control-plane spec's "A confirm naming no host, of a launch this runner holds".
- [ ] 3.4 crew's implementation's sources and tests this group touched pass clippy with no warning. True when `sluice exec main -- moon run boomdrive-crew:lint` passes.

## 4. Version 1 carries no confirm

- [ ] 4.1 A want whose act is a confirm, as `boomdrive-control`'s types write it, fails version 1's `want`. Every act sort version 1 names still passes, and no protocol file changes (design Decision 7; Task notes 4.1). True when `devenv shell -- cargo test -p boomdrive-hosted --test interface -- a_confirm_is_no_act_of_version_1 the_interfaces_values_pass` passes, proving the runner-protocol spec's "A confirm is no act of version 1" and "The interface's values pass".

## 5. The words

- [ ] 5.1 The README's control pass says the runner carries out each retry, land, confirm, drop, priority or done as Flume's own command on the launch's place. It says a confirm is `flume confirm --recipient=<recipient> -- <key>…`, of exactly the keys its control plane names, each the delivery of an end of the launch's place, as the reader it names and never as `boomdrive`. It says the runner confirms no end for another reader of its own accord (design Decision 10). True when `sluice exec main -- moon run root:format` passes.
- [ ] 5.2 The spec deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive the-runner-carries-a-control-planes-confirm --yes`, `sluice exec main -- moon run root:specs` passes.
