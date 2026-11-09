# Verify

The verify stage's report on `change/the-runner-carries-a-control-planes-confirm` at `e752345`, cut from `bolt/the-runner-confirms-the-ends-it-carries`, which stands at `0859e14`, main's tip. The code stage archived the change in `e752345` and ticked task 5.2 there, as task 5.2 asks, so this report sits in the archived change's directory.

## The whole checks

`sluice exec main`, run once on `e752345`, passed: exit 0, "passed: 22 tasks", and sluice "recorded 22 results under their keys for e752345". sluice ran the checks `e752345` affects against main (`0859e14`), every one a `check-fast` task:

- `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test`, each passed;
- `nix`: `typecheck` and `test`, each passed;
- `root`: `format`, `adrs`, `specs` and `changes`, each passed.

sluice did not run the tasks `e752345` does not affect: `nix:format`, `nix:lint`, `root:toolchain`, `root:commits`, `root:releases` and `root:workflow`. The test results across the run total 807 passed and 0 failed. sluice named no conflict with main.

moon warned that it could not write its output archives and shared cache blobs, and that it skipped installing dependencies with no internet connection. The OTLP exporter also did not shut down cleanly. None of these is a check, and none failed a task.

## Tasks

Every task of `tasks.md` is ticked, 1.1 to 5.2. Each check a task names ran in the gate above:

- the crates' named tests ran under `boomdrive*:test`, each `ok`;
- 2.7's and 3.4's clippy ran under `boomdrive:lint` and `boomdrive-crew:lint`;
- 5.1's README ran under `root:format`;
- 5.2's archived specs ran under `root:specs`.

## Requirements and scenarios

| spec | requirement | scenario | where it is proved (each `ok` in the gate) |
|---|---|---|---|
| control-plane-interface | An act asks one thing of its launch's place | An act of each sort | `crates/boomdrive-control/tests/json.rs` `an_act_of_each_sort`: every sort round-trips, a confirm among them, and a land writes `"what": "land"` |
| control-plane-interface | A confirm holds the one recipient it confirms as and each delivery key | A confirm's form | the same test: a confirm of two keys as `scratch/planner` writes `{"confirm": {"recipient": …, "deliveries": [K1, K2]}}`, its keys in order |
| work-into-flume | A confirm is Flume's confirm of the keys it names, as the recipient it names | A confirm | `crates/boomdrive/tests/confirms.rs` `a_confirm_names_only_the_keys_it_was_given`: the one act is Flume's confirm of `K1` as `scratch/planner`, `Done`, and the line `a/confirm: confirm as scratch/planner of a at <P>: done` |
| work-into-flume | the same | A confirm Flume refuses | `flumes_refusal_is_the_confirms_answer`: `Refused` with Flume's words, as given |
| work-into-flume | the same | A tidied place's record | `a_tidied_places_record_is_confirmed`: the place tidied to `<P>@<at>` with its end pending, the confirm of `K1` is run and `Done` |
| work-into-flume | the same | Neither the recipient nor a key read as a flag | `crates/boomdrive/tests/flume.rs` `a_confirms_argv`: `--json confirm --recipient=-x -- --recipient=boomdrive K1` |
| work-into-flume | Each key a confirm names is the delivery of an end of its launch's place | A key of another place | `confirms.rs` `a_key_of_no_end_of_the_launchs_place_is_refused`: `Refused`, its words naming `K3` and `<P>` and not `K1`, and no act run |
| work-into-flume | the same | A place the listing does not hold | `a_place_the_listing_does_not_hold_is_held_to_its_status`: `status <P>` comes ahead of the confirm, which is `Done`, and a key of no end in that status is refused, running nothing |
| work-into-flume | A confirm as the runner, or of no key, is refused | A confirm as the runner | `a_confirm_as_the_runner_is_refused`: `Refused`, naming `boomdrive`, and no act run |
| work-into-flume | the same | A confirm of no key | `a_confirm_of_no_key_is_refused`: `Refused` with `a confirm naming no delivery key confirms nothing`, and no act run |
| work-into-flume | A retry, a land, a confirm or a done of a launch not launched, and an act on another's launch, are refused | A confirm of a held launch | `crates/boomdrive/tests/after.rs` `a_confirm_of_a_held_launch`: refused with the very words of `b`'s retry, and no act run |
| crew-control-plane | A confirm request is an act confirming the deliveries it names, as the recipient it names | A confirm | `crates/boomdrive-crew/tests/read.rs` `a_confirm_is_an_act_on_the_launch_it_names`: request 17 gives `home/17` on `home/12` under lease 1, confirming `K1`, `K2` in order as `scratch/planner` |
| crew-control-plane | A confirm naming no recipient, two, or no delivery cannot be read | A confirm naming no recipient, two, or no delivery | `a_confirm_naming_no_recipient_two_or_no_delivery_is_refused`: 18 `it names no Recipient`, 19 `it names no Delivery` and 20 `it names Recipient twice`, each given as one the runner refuses |
| crew-control-plane | An act naming no host is given to the runner that holds its launch | A confirm naming no host, of a launch this runner holds | `crates/boomdrive-crew/tests/claims.rs` `a_confirm_naming_no_host_goes_to_the_runner_holding_its_launch`: `host-a`'s pull is `home/21`, `home/26`, with 26 confirming ends of `home/21`, and nothing for 27 |
| runner-protocol | Version 1 carries no confirm | A confirm is no act of version 1 | `crates/boomdrive-hosted/tests/interface.rs` `a_confirm_is_no_act_of_version_1`: refused by version 1's `want` |
| runner-protocol | The messages carry the interface's shapes | The interface's values pass | `the_interfaces_values_pass`, with every act sort version 1 names, neither a land nor a confirm |

The scenarios the deltas carry over unchanged keep their tests, which ran in the gate:

- "A retry of a held launch";
- "A land of a held launch";
- "An act on another's launch";
- "An act on another host's launch";
- "A land naming no host, of a launch this runner holds";
- "An act's outcome".

## The code against the design

- `ActSort::Confirm { recipient, deliveries }` is serde `{"confirm": {…}}`. Its documentation is the one Decision 1 and task 1.1 ask for, citing core.56 and core.42 (`crates/boomdrive-control/src/want.rs`).
- `flume::Act::Confirm { recipient, keys }` gives `confirm --recipient=<recipient> -- <key>…` and names no place (Decision 2; `crates/boomdrive/src/flume.rs`).
- In `crates/boomdrive/src/carry.rs`:
  - `flume_act` maps a confirm to `flume::Act::Confirm` with its keys in order;
  - `sort` names it `confirm as <recipient>`;
  - `act_unlaunched` refuses it together with a retry, a land and a done (Decision 5).
- `Work::act` refuses a confirm as `boomdrive` or of no key once the launch is the runner's and its lease is live, running nothing, in Decision 4's words (`refused_confirm`).
- `act_on` keeps the status Flume gives for a place the listing does not hold. `unheld` reads the place's entry from that status or the listing through `facts::entry`, and reads its `ends[].delivery` through `facts::deliveries`. A key that is none of them is refused with `<key> is the delivery of no end of <place> as Flume gives it`, ahead of the check of whether the next pass is due (Decision 3).
- Every match on `ActSort` in the crates' sources is exhaustive, with no wildcard arm, so no sort falls through.
- crew's `read::want` reads a confirm's `Of`, its one `Recipient` and each `Delivery` in order, refusing none, two or no `Delivery` with Decision 6's words. `lib.rs` sends a `confirm` naming no host with the `retry`, `land`, `drop` and `priority`. The test descriptor gains `confirm` and `Delivery`, and `FIELDS` gains `Delivery` after `Words`.
- No file under `crates/boomdrive-hosted/protocol/`, `nix/` or `flake.lock` changed since `0859e14`, and `flume::LEAST` did not move (core.42; Decisions 7 and 8).
- The README's control pass names the confirm, `flume confirm --recipient=<recipient> -- <key>…`, of exactly the keys named, held to the launch's place, never as `boomdrive`, and none of the runner's own accord (Decision 10).
- core.56 and its details, and record 0002's amendment of 2026-11-09, proposed, are in the tree (Decision 9).
- Every path changed since `0859e14` is in proposal.md's Touches or the change's own archive, but `crates/boomdrive/tests/flume.rs`. Task 2.1 names that file for `a_confirms_argv`, so the plan asks for it, and Touches leaves it out.
- The archive merged each delta into `openspec/specs/`:
  - control-plane-interface's amended act and its new confirm requirement;
  - work-into-flume's renamed requirement and its three new ones;
  - crew-control-plane's two new requirements and its amended act naming no host;
  - runner-protocol's "Version 1 carries no confirm" and its amended shapes.
- The code stage moved the confirm's form out of the act's requirement into one of its own (`516c35c`), since openspec's strict check bounds a requirement's text. Task 1.1's test proves both.

## Outcome

The whole checks pass, and every task is done. Every requirement and scenario of the change's four delta specs is met, and each is proved by a test the gate ran. The change has stood archived since `e752345`, and this report is filed with it.
