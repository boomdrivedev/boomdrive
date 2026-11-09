# Verify

The verify stage's report on `change/land-is-the-message-for-a-works-landing` at `1c5a119`, cut from `bolt/the-runner-lands-on-a-land-message`. The code stage archived the change in `36cf346` and ticked task 5.2 in `1c5a119`, as task 5.2 asks, so this report sits in the archived change's directory.

## The whole checks

`devenv shell -- sluice exec main`, run once on `1c5a119`, passed: exit 0, "passed: 22 tasks", and sluice "recorded 22 results under their keys for 1c5a119". sluice ran the checks `1c5a119` affects against main (`14a9139`), every one a `check-fast` task:

- `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test`, each passed;
- `nix`: `typecheck` and `test`, each passed;
- `root`: `format`, `adrs`, `specs` and `changes`, each passed.

The tasks `1c5a119` does not affect, `nix:format`, `nix:lint`, `root:toolchain`, `root:commits`, `root:releases` and `root:workflow`, were not run. Every test result in the run reads `0 failed`. Among them, `boomdrive-hosted:test`'s `the_hello_first` passed, which is task 4.2's check.

moon warned that it could not write its output archives and shared cache blobs, and the OTLP exporter did not shut down cleanly. None of these is a check, and none failed a task.

sluice also said: "would conflict: docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md with main". Main has landed `bolt/a-dropped-launch-is-never-offered` (`14a9139`), which also amends record 0002, so its amendment and this change's amendment of 2026-11-08 meet in the file. The change lands onto `bolt/the-runner-lands-on-a-land-message`, which stands at `c6eb79b`, this branch's base, so it does not conflict there. The bolt resolves the conflict when it takes main. It is not a fault in this change's work.

## Tasks

Every task of `tasks.md` is ticked, 1.1 to 5.2. Each check a task names ran in the gate above:

- the crates' tests ran under `boomdrive*:test`;
- `the_hello_first` ran under `boomdrive-hosted:test`;
- the README ran under `root:format`;
- the archived specs ran under `root:specs`.

## Requirements and scenarios

| spec | requirement | scenario | where it is proved (each `ok` in the gate) |
|---|---|---|---|
| control-plane-interface | An act asks one thing of its launch's place | An act of each sort | `crates/boomdrive-control/tests/json.rs` `an_act_of_each_sort`: `ActSort::Land` round-trips, and a land's `Act` writes `"what": "land"` and nothing more |
| work-into-flume | A retry, a land, a drop, a priority and a done are Flume's own commands on the launch's place | A land | `crates/boomdrive/tests/acts.rs` `a_land`: one call, `--json retry <P> --submission`; no drop, priority or done; `Done` with no words; a line beginning `home/13: land of home/12` |
| work-into-flume | A retry, a land or a done of a launch not launched, and an act on another's launch, are refused | A land of a held launch | `crates/boomdrive/tests/after.rs` `a_land_of_a_held_launch`: refused with the retry's words, no act run, and `b` launched at the next pass with its offer's arguments, `hold_submission` kept |
| work-into-flume | A bolt is landed on a land | A bolt's send; A send Flume refuses | `crates/boomdrive/tests/bolts.rs` `a_bolts_send` and `a_send_flume_refuses`, whose `send()` is `ActSort::Land` of `home/bolt/3`; lines read `land of`, Flume's reason is carried whole, and no `sluice` call is made |
| crew-control-plane | A retry, a land, a drop and a priority are acts on the launch they name | A land | `crates/boomdrive-crew/tests/read.rs` `a_land_is_an_act_on_the_launch_it_names`: request 17 gives `home/17` landing `home/12` under lease 1 |
| crew-control-plane | An act naming no host is given to the runner that holds its launch | A land naming no host, of a launch this runner holds | `crates/boomdrive-crew/tests/claims.rs` `a_land_naming_no_host_goes_to_the_runner_holding_its_launch`: `host-a`'s pull is `home/21`, `home/24`, with `home/24` landing `home/21`, and nothing for 25 |
| runner-protocol | Version 1 carries no land | A land is no act of version 1 | `crates/boomdrive-hosted/tests/interface.rs` `a_land_is_no_act_of_version_1`: refused by version 1's `want` |
| runner-protocol | The messages carry the interface's shapes | The interface's values pass | `the_interfaces_values_pass`, with every act sort version 1 names and no land |

The scenarios the deltas carry over unchanged keep their tests, which run in the gate:

- "A retry with words";
- "A done";
- "A retry of a held launch";
- "An act on another's launch";
- "A retry of the submission";
- "An act on another host's launch";
- "An act's outcome".

## The code against the design

- `ActSort::Land` is a unit variant, serde `"land"`, documented as Decision 1 and Task notes 1.1 say, citing core.53 and core.42 (`crates/boomdrive-control/src/want.rs`).
- In `crates/boomdrive/src/carry.rs` (Decisions 2 to 4):
  - `flume_act` maps a land to `flume::Act::Retry { stage: None, words: None, submission: true }`;
  - `sort` names it `land`;
  - `act_unlaunched` refuses it together with a retry and a done.

  Every match on `ActSort` in the crates' sources is exhaustive or reads a priority alone, so no sort falls to a wildcard. The runner reads no kind for a land.
- crew's `read::want` gives `Act: land` as `act(ActSort::Land)` and reads `Of` alone. `lib.rs` sends a `land` naming no host with the retry, drop and priority (Decision 6). The test descriptor's `Act` enum gains `land`.
- `the_hello_first` reads home before it drops home's first connection, so the `pulled` answering the first hello is taken before the drop (task 4.2). The test changes no code under `src/`.
- No file under `crates/boomdrive-hosted/protocol/` changed since `c6eb79b` (core.42; Decision 7). `ActSort::Retry`'s `submission` stays (Decision 5).
- The README's control pass names the land, `flume retry <place> --submission` whatever the launch's kind, and says a bolt is landed on a land. It no longer calls a bolt's send a retry (Decision 9).
- core.53, core.52's amended text and details, and record 0002's amendment of 2026-11-08, proposed, are in the tree (Decision 8).
- Every path changed since `c6eb79b` is listed in proposal.md's Touches, or is part of the change's own archive.
- The archive merged each delta into `openspec/specs/`:
  - control-plane-interface's "An act asks one thing of its launch's place";
  - work-into-flume's three renamed requirements;
  - crew-control-plane's renamed and amended requirements;
  - runner-protocol's "Version 1 carries no land" and its amended shapes.

## Outcome

The whole checks pass, and every task is done. Every requirement and scenario of the change's four delta specs is met, and each is proved by a test the gate ran. The change stands archived since `36cf346`, and this report is filed with it.
