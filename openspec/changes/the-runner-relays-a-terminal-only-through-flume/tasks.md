# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- **The tests.** Every test runs on the scratch host of its crate's tests, with core.41's neutral names, and no real Flume or sluice but the set's own in `tests/family.rs`. No test runs herdr.
- **What no task adds or changes.**
  - No task adds a crate, a tool or a tracked file type (core.7, core.8).
  - No file under `crates/boomdrive-hosted/protocol/` changes (core.42). No field of `report.json` and no configuration key changes.
  - `flume::LEAST` and the set's Flume do not move (design Decision 7).
  - The flake, its modules, setup and their tests keep herdr (core.29's allowance).
  - Nothing outside the paths proposal.md's Touches lists is changed.
- **What is already written.** This change's own commit holds record 0004's amendment of 2026-11-15 and the details of core.11, core.13 and core.29. No task changes them. A task that relies on a choice the design names cites its Decision, and its Task notes give the tests' shapes.

## 1. A terminal relays through Flume

- [x] 1.1 `crates/boomdrive/src/flume.rs` gives `relay_args(place)`, `["relay", <place>]`, and `Cli::relay(place)`, which spawns Flume with `--json` and those arguments, `FLUME_RECIPIENT` removed, and its standard input, output and error piped. `tests/flume.rs` gains `a_relays_argv` (design Decision 1; Task notes 1.1). True when `devenv shell -- cargo test -p boomdrive --test flume a_relays_argv` passes.
- [ ] 1.2 `crates/boomdrive/src/terminal.rs` opens a terminal through Flume. `Relays::new` takes a `flume::Cli` and the lines' sink. `Relays::open` opens only on a launch the report holds under the terminal's lease at a place Flume named, then runs `Cli::relay` of that place on the terminal's thread. Where Flume refuses, its words close the link. The `Herdr` trait, its `Cli`, `Refused`, `agent_pane` and `READ_INTERVAL` go. `src/run.rs`'s `Programs::relays` builds on its Flume. `tests/fakes/mod.rs` gains `RelayStub`, and `Bench` relays through it in place of `HerdrStub`, which goes. `tests/terminals.rs` is rewritten over the stub, opening with its first four tests (design Decisions 1, 2, 5 and 11; Task notes 1.2). True when `devenv shell -- cargo test -p boomdrive --test terminals -- a_launch_not_held a_launch_at_no_place flumes_refusal_closes_the_terminal a_flume_with_no_relay` passes, proving the terminal-relay spec's "A launch not held", "A launch at no place", "Flume refuses the relay" and "A Flume with no relay".
- [ ] 1.3 Each `screen` line Flume writes is sent on the link as Flume gave it, in order, none compared or held. An `unread` line is said with Flume's words, and an `untyped` line is said without them, the relay going on (design Decision 3; Task notes 1.3). True when `devenv shell -- cargo test -p boomdrive --test terminals -- screens_as_flume_gives_them an_unread_read_and_an_untyped_input` passes, proving the terminal-relay spec's "Screens as Flume gives them" and "An unread read and an untyped input".
- [ ] 1.4 Each text from the link is written to Flume's input as one `{"text": …}` line, and each set of keys as one `{"keys": […]}` line, as the link gave them (design Decision 4; Task notes 1.4). True when `devenv shell -- cargo test -p boomdrive --test terminals input_passed_in` passes, proving the terminal-relay spec's "Input passed in".
- [ ] 1.5 A terminal closes as design Decision 5's table has it. Where the runner closes it, it closes Flume's input, sends no further screen, reads Flume's output to its end and awaits its exit, signalling nothing, then closes the link and says its line. Flume's standard error is read on a thread of its own. `Relays`'s drop closes each open relay's input and joins its thread (design Decision 5; Task notes 1.5). True when `devenv shell -- cargo test -p boomdrive --test terminals -- the_agents_terminal_gone the_link_closed the_far_end_stops_listening the_launch_let_go a_launch_the_control_plane_lets_go the_runner_stopped` passes, proving the terminal-relay spec's "The agent's terminal gone", "The link closed", "The launch let go" and "The runner stopped".
- [ ] 1.6 Nothing of a terminal is held, written or said: the runner holds no screen, and its lines name the terminal, its launch and its place alone, with Flume's words of an unread read and of a refusal. The module's documentation says the relay goes through `flume relay` and names no multiplexer (design Decision 9; Task notes 1.6). True when `devenv shell -- cargo test -p boomdrive --test terminals nothing_written` passes, proving the terminal-relay spec's "Nothing written".
- [ ] 1.7 The set's own Flume refuses a relay of a place that stands nowhere, and the runner closes the terminal with its words (design Decision 11; Task notes 1.7; core.45). True when `devenv shell -- cargo test -p boomdrive --test family the_sets_flume_answers_a_relay` passes, proving the terminal-relay spec's "The set's Flume refuses a relay of no place".

## 2. The hello names Flume, sluice and git

- [ ] 2.1 `crates/boomdrive/src/hello.rs`'s `TOOLS` is `["flume", "sluice", "git"]`. `src/run.rs`'s `Programs` holds `flume` and `sluice` alone, and its documentation names no herdr. `tests/offers.rs`'s `a_tool_missing` makes `sluice` missing. `tests/control.rs`, `tests/pass.rs`, `tests/loop.rs` and `tests/local_layer.rs` drop their herdr (design Decision 6; Task notes 2.1). True when `sluice exec main -- moon run boomdrive:lint` passes, which builds and lints every target of the crate with the files this task touched, and `devenv shell -- cargo test -p boomdrive --test offers a_tool_missing` passes, proving the work-into-flume spec's "A tool missing".
- [ ] 2.2 `tests/hello.rs`'s two tests read sluice and Flume where they read herdr, and `the_hello_runs_no_multiplexer` is added (design Decision 6; Task notes 2.2). True when `devenv shell -- cargo test -p boomdrive --test hello` passes, proving the work-into-flume spec's "No multiplexer's version".
- [ ] 2.3 `crates/boomdrive-control/src/want.rs`'s documentation of `Link::screen` and `Input::Keys` names Flume's relay and Flume's key names, and `tests/json.rs`'s `a_hello` names `sluice` missing in place of herdr (design Decision 4; Task notes 2.3). True when `devenv shell -- cargo test -p boomdrive-control --test json a_hello` passes.

## 3. No code of the runner's names a multiplexer

- [ ] 3.1 `crates/boomdrive/src/facts.rs` loses `Pane`, `pane` and their unit tests. `tests/facts.rs`'s `a_live_place_shows_its_agents_pane` becomes `a_live_place_is_live_and_unmarked`, without its assertion of a pane (design Decision 10; Task notes 3.1). True when `devenv shell -- cargo test -p boomdrive --test facts a_live_place_is_live_and_unmarked` passes.
- [ ] 3.2 `tests/rules.rs` gains `no_crate_names_a_multiplexer` (design Decision 10; Task notes 3.2). True when `devenv shell -- cargo test -p boomdrive --test rules no_crate_names_a_multiplexer` passes, proving the terminal-relay spec's "The crates' sources read".

## 4. The hosted implementation says why it closes a terminal

- [ ] 4.1 `crates/boomdrive-hosted/src/connection.rs`'s `NO_TERMINAL` is design Decision 8's words, and `tests/plane.rs`'s `a_terminal_opened` holds them to naming the end-to-end exchange and not Flume (design Decision 8; Task notes 4.1). True when `devenv shell -- cargo test -p boomdrive-hosted --test plane a_terminal_opened` passes, proving the hosted-control-plane spec's "A terminal opened".

## 5. The specs

- [ ] 5.1 The change's deltas archive into specs that pass openspec's strict check. After the archive, `openspec/specs/terminal-relay/spec.md`'s Purpose is the one design Decision 13 gives, naming `flume relay` and no multiplexer, its only change outside a delta (design Decision 13). True when, after `devenv shell -- openspec archive the-runner-relays-a-terminal-only-through-flume --yes`, `sluice exec main -- moon run root:specs` passes.
