# Verify

The change `the-runner-carries-a-hold-of-a-places-waiting-move`, verified on
`change/the-runner-carries-a-hold-of-a-places-waiting-move` at `30ea15b9`,
cut from `bolt/the-runner-needs-no-hand-on-a-host` at `6808fbf3`.

**Verdict: the work holds.** The whole checks pass, every task is ticked,
and every requirement and scenario of the four spec deltas is met by code
and a test that passed in the gate.

The code stage archived the change at `30ea15b9`, as its task 5.2 asks. So
this report sits in the archived change's directory, and archiving needs
nothing more.

## The whole checks

`sluice exec main`, run once at `30ea15b9`, exited 0: `passed: 22 tasks`,
in 4m 59s, its results recorded under their keys for `30ea15b`. No test
failed.

| project | tasks |
|---|---|
| `boomdrive` | format, lint, typecheck, test |
| `boomdrive-control` | format, lint, typecheck, test |
| `boomdrive-crew` | format, lint, typecheck, test |
| `boomdrive-hosted` | format, lint, typecheck, test |
| `nix` | typecheck, test |
| `root` | format, adrs, changes, specs |

moon warned once that it could not archive `boomdrive:test`'s outputs into
its cache. That is a cache write, not a check, and the task passed.

## Tasks

All eleven tasks, 1.1 to 5.2, are ticked. Each one's named test passed in
the gate run:

| task | test | result |
|---|---|---|
| 1.1 | `boomdrive-control` `json::an_act_of_each_sort` | ok |
| 2.1 | `boomdrive` `flume::a_holds_argv` | ok |
| 2.2 | `boomdrive` `holds::a_hold_is_flumes_hold_of_the_waiting_move`, `holds::flumes_refusal_is_the_holds_answer` | ok |
| 2.3 | `boomdrive` `holds::a_hold_with_no_words_is_refused` | ok |
| 2.4 | `boomdrive` `after::a_hold_of_a_held_launch` | ok |
| 3.1 | `boomdrive-crew` `read::a_hold_is_an_act_on_the_launch_it_names` | ok |
| 3.2 | `boomdrive-crew` `read::a_hold_naming_no_words_no_bound_or_a_bound_of_no_whole_seconds_is_refused` | ok |
| 3.3 | `boomdrive-crew` `claims::a_hold_naming_no_host_goes_to_the_runner_holding_its_launch` | ok |
| 4.1 | `boomdrive-hosted` `interface::every_sort_the_interface_names` | ok |
| 4.2 | `boomdrive-hosted` `interface::a_hold_is_no_act_of_version_1_or_2` | ok |
| 5.1 | `root:format` | passed |
| 5.2 | `root:specs`, after the archive | passed |

## Requirements and scenarios

### control-plane-interface

- **An act asks one thing of its launch's place** (modified), with the
  scenario "An act of each sort". `ActSort::Hold { words, bound_interval }`
  is in `crates/boomdrive-control/src/want.rs`. `an_act_of_each_sort`
  round-trips every sort, the hold among them.
- **A land is written as the word land** (added at the archive, the land's
  sentence moved out of the modified requirement, no SHALL changed), with
  the scenario "A land's form". `an_act_of_each_sort` asserts the land's
  `what` is `"land"`.
- **A hold holds its words and its bound**, with the scenario "A hold's
  form". The same test asserts the hold's JSON is exactly
  `{"hold": {"words": "the user sends it on the landings card, row 3",
  "bound_interval": 28800}}`.

### work-into-flume

- **A retry, a land, a confirm or a done of a launch not launched … are
  refused** (modified), with the scenario "A hold of a held launch".
  `act_unlaunched` in `crates/boomdrive/src/carry.rs` refuses
  `ActSort::Hold` beside a retry, a land, a confirm and a done.
  `a_hold_of_a_held_launch` finds the hold's answer equal to the retry's
  and no `flume` act run. The other scenarios of the requirement are
  unchanged and their tests still pass.
- **A hold is Flume's hold of the move its launch's place waits on**:
  - "A hold" is met. `flume_act` maps the hold to `flume::Act::Hold` with
    the act's words and bound, and `sort` names it `hold for <n>s`.
    `a_hold_is_flumes_hold_of_the_waiting_move` finds exactly one Flume
    act, the hold. It also finds the act `done`, the line `a/hold: hold for
    28800s of a at P: done`, and no line holding the words.
  - "The words not read as a flag" is met. `act_args` gives `hold <place>
    --waiting --for=<n>s -- <words>`. `a_holds_argv` finds `--json hold
    /places/P --waiting --for=0s -- --stage code` over the stub, the words
    one argument after `--`.
- **Flume's answer is the hold's**, with the scenario "A hold Flume
  refuses". `flumes_refusal_is_the_holds_answer` finds the act `refused`
  with Flume's words as given. The runner keeps no timer and no renewal of
  a hold: nothing in the diff reads a place's wait or schedules a hold.
- **A hold with no words is refused**, with the scenario "A hold with no
  words". `refused_hold` refuses empty words in `Work::act`, once the
  launch is the runner's and its lease is live, beside `refused_confirm`.
  `a_hold_with_no_words_is_refused` finds the act `refused` with the words
  `a hold with no words says nothing of who makes the move`, and no Flume
  act.

### crew-control-plane

- **An act naming no host is given to the runner that holds its launch**
  (modified), with the scenario "A hold naming no host, of a launch this
  runner holds". `crates/boomdrive-crew/src/lib.rs` sends a `hold` naming
  no host with the other acts. `a_hold_naming_no_host_goes_to_the_runner_holding_its_launch`
  finds `host-a`'s pull giving `home/21` and `home/26` alone, 26 holding
  21 under the words for 28800 seconds. The other scenarios of the
  requirement still pass.
- **A hold request is an act holding the move its launch's place waits
  on**, with the scenario "A hold". `read::want` reads `Of`, `Words` and
  `Bound` alone. `a_hold_is_an_act_on_the_launch_it_names` finds the act
  `home/17` on `home/12` under lease 1 with the words and 28800.
- **A hold naming no words, no bound, or a bound of no whole seconds
  cannot be read**, with its scenario. The test finds requests 18, 19 and
  20 refused, with `it names no Words`, `it names no Bound` and `its Bound
  8h, which is no whole number of seconds`. A hold naming no host that
  cannot be read goes the way of any act naming no host: given nothing,
  and named in the log.

### runner-protocol

- **The messages carry the interface's shapes** (modified), with the
  scenario "Every sort the interface names". `every_sort_the_interface_names`
  names `"hold"` in `act_sort` and `ACT_SORTS`, and gives a hold among
  `every_sort`. It finds every value passing version 2's `want` but the
  hold's, which is refused. The sets of sorts named are still every sort
  the interface has. "The interface's values pass" and "An act's outcome"
  are unchanged and pass.
- **No version carries a hold**, with the scenario "A hold is no act of
  version 1 or 2". `a_hold_is_no_act_of_version_1_or_2` finds a hold
  refused by version 1's `want` and by version 2's. No file under
  `crates/boomdrive-hosted/protocol/v1/` or `protocol/v2/` changed since
  `6808fbf3`.

## The change's bounds

- Every path the branch changed since `6808fbf3` is one proposal.md's
  Touches lists, or the change's own directory.
- No crate, tool or tracked file type is added.
- `flume::LEAST` stays `(0, 21, 0)`.
- No field of `report.json` changes.
- Record 0002's amendment of 2026-11-14 and core.59 are written, at
  `0af552df`, before the code.
- The README's control pass and hosted section and
  `crates/boomdrive-hosted/protocol/README.md` say what design Decisions 7
  and 10 ask.

## Not checked here

The named follow-ups in design.md are outside this repository's tree:

- crew's unit `a-bolt-waiting-on-the-user-waits-under-words`;
- a protocol version that names `hold`;
- Flume's statement's row of callers;
- a digest check of version 2's files.

The design says each is told to the orchestrator, and this stage checks
none of them.
