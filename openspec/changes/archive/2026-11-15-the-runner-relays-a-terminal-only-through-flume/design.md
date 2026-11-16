# Design

## Context

See proposal.md for why. There are no area pages for the crates, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.1: the runner decides nothing Flume decides, and carries each tool's answer as the tool gave it;
- core.2: the control plane is reached through one interface, and only an implementation names the hosted protocol;
- core.10: a launch let go is acted on no more;
- core.11: no terminal byte is written to the runner's data or its log, or carried in a report;
- core.13: the runner starts, keeps and stops no tool's long-running process, and runs a tool's own command its control plane asked for once, awaited;
- core.29: the runner reaches an agent's terminal only through Flume's command line, and no code of the runner's names a multiplexer or runs one's command line, but setup's link of the set's herdr plugins;
- core.30: a terminal leaves the host only encrypted end to end to the browser that opened it, and never in the clear;
- core.35: the data directory holds nothing of a terminal;
- core.38: below a tool's floor the runner takes no work that needs the tool;
- core.42: a protocol version a release has spoken is never changed.

It amends record 0004 with an amendment of 2026-11-15, and the details of core.11, core.13 and core.29. All are written with this change. It departs from no rule. It brings the runner's code to core.29, which the herdr relay and the hello broke since record 0004 was accepted.

The control plane is reached through `boomdrive-control`'s `ControlPlane` interface alone (core.2). A pull gives the terminals to open as `Terminal { id, launch, lease, link }`, each `link` a `Link` of the implementation's own. crew's implementation, `crates/boomdrive-crew`, opens none. The hosted implementation, `crates/boomdrive-hosted`, closes each at once (Decision 8). So today the relay runs only over the interface's fake control plane, as the herdr relay did.

### What stands

- **Flume's relay**, Flume's statement 221 and 222 and C.5's herdr relay row, read at boomdrivedev/blueprints b54a9b4 on main, and Flume's change `flume-relays-a-places-terminal`, landed as #70 and released in `v0.24.0`:
  - `flume relay <place>` names its place as status does, its location or `<kind>--<name>` in the repository. It relays the agent of the place's current stage whose start is live, through the terminal that start recorded;
  - it writes one JSON object a line to standard output, whatever `--json` says: `{"screen": "<what the terminal shows>"}`, the first at once and then each time it differs from the last given; `{"unread": "<why>"}`, a read the terminal did not answer, given once until a read answers again; `{"untyped": "<why>"}`, an input not typed. The last line is its answer: `{"relayed": {"place": …, "agent": …, "ended": "caller" | "gone"}}`, exit 0, or its refusal, `{"refused": {"refusal": {"reason": …}}}`, exit 1, which is then the only line;
  - it reads standard input one line at a time on a thread of its own: `{"text": "<text>"}` typed as it is, `{"keys": ["<key>", …]}` each by Flume's key name, an empty line passed over. A line that is neither, or names a key Flume does not name, is written back as `untyped`, naming it;
  - Flume's key names: `enter`, `tab`, `esc`, `backspace`, `delete`, `insert`, `home`, `end`, `pageup`, `pagedown`, `up`, `down`, `left`, `right`, `space`, `f1` to `f12`, or one printable character, each led by any of `ctrl+`, `alt+` and `shift+` in that order. herdr 0.9.5 types none of `delete`, `insert`, `home`, `end`, `pageup` and `pagedown`, and Flume gives each back as untyped, typing the keys after it (222 as amended by b54a9b4);
  - it ends as the caller's when its input closes or its output is no longer read, and as gone when the agent's terminal goes. It refuses a place with no live agent and a terminal that offers no relay, in their own words. The printed command offers none;
  - every herdr call of the relay is bounded at one second, and herdr's screen is read at most every 250 milliseconds (C.5's relay row);
  - it settles, writes and keeps nothing, and is no process of Flume's: it lives inside its command (Flume's 120).
- **The set's Flume** is `v0.24.0` since `db6ffb7e`. On a scratch host it answers `flume --json relay /no/such/place` with `{"refused":{"refusal":{"reason":"no place /no/such/place stands on this host"}}}` and exit 1, as this stage read it.
- **The runner's relay**, `crates/boomdrive/src/terminal.rs`:
  - `Relays::open` opens a terminal where the report holds its launch under its lease number and the launch's carried status names its agent's pane, `clues.shown_at.agent` read by `facts::pane` as `herdr:<pane>@<socket>`. Otherwise it closes the link at once;
  - each open terminal runs on its own thread. It runs `herdr pane read <pane> --source visible --format ansi` every `READ_INTERVAL`, a quarter of a second, and sends a screen only where it differs from the last. It passes text by `herdr pane send-text` and keys by `herdr pane send-keys`, each with `HERDR_SOCKET_PATH` set;
  - it closes when the link closes or stops listening, herdr refuses a read, the runner lets the launch go (`Relays::let_go`, called from `carry.rs` where a launch is dropped from the report) or `Relays` is dropped;
  - `run.rs`'s `Programs` holds `flume`, `sluice` and `herdr`, and `Programs::relays` builds the relays on herdr.
- **The hello**, `crates/boomdrive/src/hello.rs`: `TOOLS` is `["flume", "sluice", "herdr", "git"]`, each run as `<tool> --version` at every pull. The protocol's `tool_versions` is a map by tool's name (versions 1 and 2, `hello.schema.json`), so a hello naming three tools passes either version. Its examples naming herdr are of versions a release has spoken and stay (core.42). boomdrive-cloud reads no `herdr` from a hello: on its main, `herdr` appears only in its design pages and research.
- **The hosted implementation**, `crates/boomdrive-hosted/src/connection.rs`, answers every `terminal_open` with `terminal_closed`, whose words are `NO_TERMINAL`: "this host's Flume relays no terminal, so the runner closes it at once". It makes no key, and a pull gives no terminal. The runner's half of the end-to-end exchange, record 0004's consequence, is not built. The protocol's README leaves its suite to "the change that builds both halves of a terminal".

## Goals / Non-Goals

**Goals:**

- Every terminal the runner relays goes through `flume relay`, and no code of the runner's names or runs a multiplexer, but setup's link of the set's herdr plugins (core.29).
- The hello runs no multiplexer's `--version`.
- Nothing of a terminal is held, written or said, and its input and its end are Flume's.
- The hosted implementation's words say truly why a terminal is closed.

**Non-Goals:**

- The runner's half of a terminal's end-to-end exchange. It is built with the cloud's half, which chooses the suite (record 0004's consequences; the protocol's README). Until then the hosted implementation relays nothing (Decision 8).
- Any change to the interface's shapes, the hosted protocol or `report.json`.
- herdr in the flake, its modules and setup: they install it, link its plugins and name it Flume's terminal, which core.29 allows.
- Moving `flume::LEAST` or the set's Flume (Decision 7).

## Decisions

### 1. One `flume relay` for each open terminal, naming the place by its location

An open terminal runs `flume --json relay <place>` once, `<place>` the location Flume named at the launch (`answer.place_dir`). It runs on the terminal's own thread, as every call of `flume::Cli` runs: `--json` first, `FLUME_RECIPIENT` unset. Its standard input, output and error are piped to the runner. The process is held for as long as the terminal is open and awaited once it ends, so it is a tool's own command its control plane asked for, run once and awaited (core.13, core.16).

The command's line belongs with Flume's other calls. `flume.rs` gains `relay_args(place)`, `["relay", <place>]`, and `Cli::relay(place)`, which spawns the program with those arguments after `--json` and gives back the child. `terminal.rs` holds the relay's loop. `Programs::relays` builds the relays on `Programs::flume`. So `flume.rs` stays the one place that runs Flume (record 0001), and the relay reaches the far end over no connection of its own (core.3).

Not taken:

- **`<kind>--<name> --repository <container>`.** The runner names a place by its location in every act and status it asks, and Flume's relay takes it the same way (core.51).
- **A trait of the relay's for tests, beside a program.** The relay is a child process read and written line by line, and the stub `herdr` already proves its kind through a program. A stub `flume` that relays proves the same pipes the host runs.

### 2. The runner opens a terminal on a launch held at a place; Flume says whether there is an agent

`Relays::open` keeps its own check: the report holds the terminal's launch under the terminal's lease number. Otherwise the link is closed at once with `this runner holds no launch <launch> under lease <lease>`, and nothing runs (core.10). Where the launch holds no place Flume named, held unlaunched on its After or refused, the link is closed with `<launch> has no place Flume named`, and nothing runs.

The runner no longer reads a pane from Flume's status. Whether the place has a live agent, and whether its terminal offers a relay, are Flume's to answer, and Flume refuses each in its own words (221). Where Flume refuses the relay, the link is closed with Flume's refusal's `reason` as given (core.1, core.11).

Not taken:

- **Keep reading `clues.shown_at.agent` before running Flume.** It names a multiplexer (core.29), and it judges from a status read at the last pass what Flume answers now.

### 3. Out: each line as Flume gave it

The thread reads Flume's standard output one line at a time, on a reader thread of the terminal's own, which ends when Flume's output closes:

- `{"screen": s}`: `s` is sent on the link with `Link::screen`, as Flume gave it and in Flume's order. None is compared, held or dropped, since Flume gives a screen only when it changes, within its terminal's bound (222; C.5's 250 milliseconds for herdr). `READ_INTERVAL` goes. Where `Link::screen` answers that the far end no longer listens, the terminal closes (Decision 5).
- `{"unread": w}`: said in one line, `<terminal>: flume relay of <place> for <launch> read nothing: <w>`. `w` is Flume's words of a read the terminal did not answer, which hold nothing of the screen (221).
- `{"untyped": w}`: said in one line, `<terminal>: flume relay of <place> for <launch> left an input untyped`. Flume's words are left out, since they may name the key the person typed (core.11). The relay goes on.
- `{"relayed": …}` or `{"refused": …}`: Flume's answer, read as Decision 5 reads it.
- Any other line is passed over, unsaid: Flume's lines only gain forms (Flume's core.14), and a line the runner cannot read may hold the screen.

The far end is told nothing of an unread read or an untyped input. Not taken:

- **A third call of `Link`, carrying Flume's words to the far end.** It changes the interface, and the hosted protocol, whose terminal frames are sealed bytes, has no message for it. The change that builds the runner's half of the exchange can carry Flume's words sealed, as frames the browser shows.

### 4. In: each input as one line, by Flume's names

- `Input::Text(t)` is written to Flume's standard input as the line `{"text": t}`.
- `Input::Keys(ks)` is written as the line `{"keys": ks}`, each key as the far end gave it, by Flume's name. The runner maps, checks and drops none: a name Flume does not take, or one its terminal cannot type, comes back as `untyped` (Decision 3).
- `Input::Close` closes the terminal (Decision 5).
- An empty text or no key is written as given, and Flume passes it over or answers it.

Each line is written whole and flushed. `boomdrive-control`'s documentation of `Input::Keys` says its keys are Flume's key names, and of `Link::screen` that it sends what Flume's relay shows of the agent. Neither shape changes.

### 5. A terminal closes by closing Flume's input, and awaits Flume's end

A terminal closes for one of these, and its link is closed with the words given:

| what ends it | words |
|---|---|
| the far end sends `Input::Close` | `its link closed` |
| `Link::screen` answers the far end no longer listens | `its far end stopped listening` |
| the runner lets its launch go (`Relays::let_go`) | `its launch <launch> is let go` |
| `Relays` is dropped | `the runner stopped` |
| Flume answers `relayed`, ended `gone` | `Flume's relay ended: gone` |
| Flume answers `relayed`, ended `caller`, with no close of the runner's | `Flume's relay ended: caller` |
| Flume refuses, exit 1 with its refusal | the refusal's `reason`, as given |
| Flume exits otherwise, or answers no JSON | Flume's standard error, as given, its one final newline left off |
| Flume cannot be spawned | `flume could not run: <error>` |
| a signal ends Flume | `flume was ended: <status>` |

The last three read Flume as `flume.rs`'s `read` and `Cli::run` do, so a Flume before `0.24.0`, which has no `relay`, closes the terminal with its usage error (Decision 7).

For the first four, the runner closes Flume's standard input, Flume's caller's word to end (221), and stops sending screens. It then reads Flume's output to its end and waits for Flume to exit, which Flume does once its input has closed: each of its terminal's calls is bounded at a second (C.5). The runner sends Flume no signal and kills nothing (core.13). A write to Flume's input that fails, Flume having ended, ends the input the same way, and the terminal closes with Flume's answer. The link is closed once Flume has exited, so no screen follows the close. The thread then says `<terminal>: the terminal on <launch> at <place> is closed: <words>` and ends.

Flume's standard error is read whole, on a thread of its own, so a Flume that writes to it never waits on the runner. It is read for nothing but the close's words where Flume answers no JSON.

Not taken:

- **Kill Flume's relay at the close.** It would stop a tool's process (core.13), and Flume ends its relay at its caller's close within its own bound.
- **Close the link before Flume has ended.** The far end would have no word of what Flume answered, and a screen Flume gave before its end could follow the close.

### 6. The hello names Flume, sluice and git

`hello::TOOLS` is `["flume", "sluice", "git"]`, so the runner runs `flume --version`, `sluice --version` and `git --version` at every pull, and no multiplexer's (core.29). `run.rs`'s `Programs` loses `herdr`. Its `tools` names Flume and sluice, and its `relays` builds on its Flume.

A control plane that reads a hello's tools reads them by name: version 1 and 2 of the protocol take any map of names, crew's implementation reads no tool's version from a hello, and boomdrive-cloud reads no `herdr`. So a hello without herdr is no change of form (core.39's only-gains-fields binds a command's `--json`, not a map's keys).

### 7. No floor moves

`flume::LEAST` stays `0.21.0`. A host below `0.24.0` keeps launching and acting. A terminal on it is answered by Flume's own usage error, which closes the link (Decision 5), as a confirm on a Flume with no `confirm` is answered by Flume's refusal of the command (core.56's details). The set pins `0.24.0`, so a host of the set relays.

Not taken:

- **`LEAST` raised to `0.24.0`.** Below it the runner takes no work that needs Flume (core.38), so a host would stop launching for a terminal no implementation opens yet (Decision 8).
- **A relay floor of its own, refusing a terminal below `0.24.0` before running Flume.** It reads Flume's version to judge what Flume's own answer says, and is a second floor beside `LEAST`.

### 8. The hosted implementation closes each terminal at once, saying why

The hosted implementation answers every `terminal_open` with `terminal_closed` and makes no key, as before. A terminal leaves the host only encrypted end to end, and never in the clear (core.30), and the runner's half of that exchange is not built. `NO_TERMINAL`'s words become: `this runner has no end-to-end exchange for a terminal yet and relays none in the clear, so it closes it at once`. The words it replaces, that the host's Flume relays no terminal, are false on a host of the set's Flume.

Not taken:

- **Build the runner's half here.** Its suite is chosen with the browser's half in the cloud, by "the change that builds both halves of a terminal" (the protocol's README), and the brief keeps the encryption as it stands.

### 9. Nothing of a terminal is held

The runner holds no screen: it passes each on as Flume gives it (Decision 3). It writes no screen or input to a file, a line or a report, and its lines name the terminal, its launch and its place alone, with Flume's words of an unread read and of a refusal (core.11, core.35). Flume itself keeps nothing it carried (221).

### 10. No crate's sources name a multiplexer

`facts::Pane` and `facts::pane` go: nothing reads a pane once the herdr relay is gone. A test of `tests/rules.rs`, `no_crate_names_a_multiplexer`, reads every `.rs` file under each crate's `src`, as `no_crate_asks_sluice_for_a_branch` does, but `crates/boomdrive/src/setup.rs`, whose link of the set's herdr plugins core.29 allows. It fails where one holds `HERDR_`, or a string literal that begins `"herdr`, `"tmux`, `"zellij` or `"rex`, naming each file. A comment that cites another repository's rule, as `view.rs`'s `herdr.14` does, names no multiplexer's command and passes.

### 11. The test's Flume relays

`tests/fakes/mod.rs` gains a stub `flume` that relays, `RelayStub`, in place of `HerdrStub`. It is a shell program in a temporary directory of its own. Each run writes its arguments, NUL-terminated, to `call-<n>`, and whether `FLUME_RECIPIENT` was set to `recipient-<n>`. Then:

- where the test wrote `refuse`, it prints `{"refused":{"refusal":{"reason":<that file's text as JSON>}}}` and exits 1;
- where the test wrote `usage`, it prints that file's text on standard error and exits 2;
- otherwise it copies each line of its standard input to `input-<n>` as it comes, on a background reader, and marks `closed-<n>` once its input has closed. It prints each file `out-<k>` the test writes, in order of `k`, once, each a whole JSON line the test made. Once `closed-<n>` is there it prints `{"relayed":{"place":<its place>,"agent":"code-1","ended":"caller"}}` and exits 0; once the test writes `gone`, it prints the same with `"ended":"gone"` and exits 0.

`Bench` builds its `Relays` on it. `tests/control.rs`, `tests/pass.rs`, `tests/loop.rs` and `tests/local_layer.rs` drop their herdr. The binary's run proves no herdr is run through `tests/hello.rs` and `tests/rules.rs` (Decisions 6 and 10).

`tests/family.rs` gains `the_sets_flume_answers_a_relay`: a terminal relayed through the set's own Flume, on the scratch host, of a launch held at a place that stands nowhere, is closed with the set's Flume's refusal, `no place <place> stands on this host` (core.45).

### 12. The rule and the record

- **Record 0004's amendment of 2026-11-15**, *the runner relays through Flume's relay*: the stage's own choices on the written rules, Decisions 2, 3, 5, 7 and 8, each with the path not taken. It is cheap to change and is not asked.
- **core.29's details** gain, not ruled out, `flume --json relay <place>` for a terminal its control plane opened, its lines read and written as Flume's relay speaks them; and, ruled out, a multiplexer named among the hello's tools. A pane's address parsed by the runner is ruled out already.
- **core.13's details** gain, not ruled out, a terminal's `flume relay`, held while its terminal is open and ended by closing its input, never by a signal.
- **core.11's details** gain, ruled out, Flume's words of an input its relay left untyped, which may name what the person typed, in a line or a report; and, not ruled out, Flume's words of a read its relay did not answer, in a line naming the terminal, its launch and its place.

### 13. The terminal-relay spec's Purpose

openspec's deltas carry no Purpose for a spec that stands (openspec 1.14.3's `specs-apply`: "A delta Purpose only seeds a spec that does not exist yet"). So the last task rewrites the terminal-relay spec's Purpose once the change is archived, as the change `a-boomdrive-release-reports-its-own-version` rewrote the releases spec's: the runner relays a terminal the control plane opens on a launch through Flume's relay of the launch's place, `flume relay`, and reaches it over no connection but its own outgoing one, keeping nothing of it (core.3, core.11, core.29; Flume's 221 and 222).

## Who owns what

| piece | owner | cited by |
|---|---|---|
| which terminal opens, on which launch | the control plane (core.1) | the runner |
| the terminal's link, its shapes | the interface, `boomdrive-control` (core.2) | the runner, each implementation |
| whether a place has a live agent, and its relay | Flume (221, 222) | the runner |
| the terminal's screen, its key names, which keys it types | Flume and its terminal profile (C.5) | the runner, the far end |
| opening only on a launch held, closing at a let-go or a stop | the runner (core.10, core.13) | the control plane |
| the end-to-end exchange of a hosted terminal | the runner's hosted implementation and the cloud, together (core.30) | the protocol's README |
| herdr on a host: its package, its plugins, Flume's terminal setting | the flake's modules and setup (core.29's allowance) | Flume |

## Named follow-ups

- **boomdrivedev/boomdrive and boomdrive-cloud: the runner's half of a terminal's end-to-end exchange.** It chooses the suite with the browser's half, and in the hosted implementation opens a `terminal_open` on the browser's share. It relays through this change's `Relays`, and replaces `NO_TERMINAL`. It can carry Flume's `unread` and `untyped` words to the browser, sealed (Decision 3).

Nothing else is needed outside this repository. Flume `v0.24.0` gives `flume relay`, and the set pins it.

## Risks / Trade-offs

- **A Flume that does not end its relay.** The runner awaits Flume's exit after closing its input, and a Flume that never exits holds the terminal's thread, and `Relays`'s drop at the runner's stop. Flume ends at its input's close, each terminal call bounded at a second (C.5), and the host's service manager ends the runner's process group at a stop that outlasts it.
- **An untyped key the person never learns of.** A key herdr 0.9.5 cannot type, such as `delete`, does nothing at the far end, and only the runner's line says so (Decision 3). No implementation opens a terminal today, and the follow-up carries Flume's words to the browser.
- **A relay on a host below Flume `0.24.0`** closes with Flume's usage error, which names no relay (Decision 7).

## Migration Plan

Nothing held changes shape. No field of `report.json`, no protocol file and no configuration key changes (core.36, core.42). A host that runs this release runs no herdr for the runner. Its hello names three tools.

A runner rolled back runs its herdr relay again, which no implementation reaches (Decision 8).

## Open Questions

None.

## Task notes

- **1.1.**
  - `src/flume.rs` gains `pub fn relay_args(place: &str) -> Vec<OsString>`, `["relay", place]`, and `impl Cli { pub fn relay(&self, place: &str) -> std::io::Result<std::process::Child> }`, which spawns the program with `--json`, then `relay_args`, `FLUME_RECIPIENT` removed, and standard input, output and error piped.
  - `tests/flume.rs` gains `a_relays_argv`: `relay_args("/places/P")` is `["relay", "/places/P"]`, and a place `--json` is passed as one argument after `relay`.
- **1.2.** `tests/fakes/mod.rs`:
  - `RelayStub`, as Decision 11 has it, with `program()`, `dir`, `calls()`, `inputs(n) -> Vec<Value>` reading `input-<n>` line by line as JSON, `out(line: &Value)` writing the next `out-<k>`, `refuse(words)`, `usage(words)` and `gone()`. Its program's place is its last argument;
  - `Bench` holds `pub flume_relay: RelayStub` in place of `herdr`, and builds `Relays::new(flume::Cli::new(flume_relay.program()), …)`;
  - `HerdrStub` and its program go.
- **1.2 to 1.6** are `tests/terminals.rs`, rewritten over the stub. Its module comment names the terminal-relay spec, core.11, core.13 and core.29. Its helper `held(bench)` makes `o1` with one pass over `Fake::holding(vec![Want::Offer(offer("o1", "a"))])`, so the report holds `o1` under lease 1 at `place_of("a")`, and gives the plane. `shown`, `PANE`, `SOCKET` and `argv` of herdr go.
  - **1.2 `a_launch_not_held`:** terminals on `o9` under lease 1 and on `o1` under lease 2 are each closed with `this runner holds no launch … under lease …`, the stub has no call, and the line `t1: the terminal on o9 is not opened: this runner holds no launch o9 under lease 1` is said.
  - **1.2 `a_launch_at_no_place`:** the plane holds the offer `o1` named `a` and the offer `o2` named `b` whose `launch.after` names `o1`, as `tests/after.rs`'s `after` makes one. After one pass `o1` is launched and `o2` held. A terminal on `o2` under lease 1 is closed with `o2 has no place Flume named`, and the stub has no call.
  - **1.2 `flumes_refusal_closes_the_terminal`:** `refuse("fix--a has no live agent: waiting for room")`. The link is closed with exactly those words, and the stub's one call is `--json relay <place_of("a")>`.
  - **1.2 `a_flume_with_no_relay`:** `usage("error: unrecognized subcommand 'relay'\n")`. The link is closed with `error: unrecognized subcommand 'relay'`.
  - **1.3 `screens_as_flume_gives_them`:** the stub gives `{"screen": "\u{1b}[1mone\u{1b}[0m\r\n$ "}`, `{"screen": "one"}` and `{"screen": "one"}` again. The link receives the three, in order, each as given, and the call ran with `FLUME_RECIPIENT` unset.
  - **1.4 `input_passed_in`:** the far end sends `Input::Text("yes")` and then `Input::Keys(["ctrl+c", "enter"])`. `inputs(1)` is `[{"text": "yes"}, {"keys": ["ctrl+c", "enter"]}]`.
  - **1.3 `an_unread_read_and_an_untyped_input`:** the stub gives `{"unread": "herdr did not answer within 1s"}`, then `{"untyped": "secret-key-name is no key flume names"}`, then a screen. The relay lines hold `t1: flume relay of <place> for o1 read nothing: herdr did not answer within 1s` and one line for the untyped input that holds no `secret-key-name`. The screen still reaches the link.
  - **1.5 `the_agents_terminal_gone`:** `gone()`. The link is closed with `Flume's relay ended: gone`, and the thread ends.
  - **1.5 `the_link_closed`:** the far end sends `Input::Close`. `closed-1` is there, the link is closed with `its link closed`, and the thread ends.
  - **1.5 `the_far_end_stops_listening`:** after a screen, the far end's receiver is dropped and the stub gives a second screen. The stub's input is closed and the line names `its far end stopped listening`.
  - **1.5 `the_launch_let_go`** and **`a_launch_the_control_plane_lets_go`:** as today, over the stub. The stub's input is closed, and the link is closed with `its launch o1 is let go`.
  - **1.5 `the_runner_stopped`:** a `Relays` built over the stub, a terminal opened, then the `Relays` dropped. The drop returns, the stub's input was closed, and the link is closed with `the runner stopped`.
  - **1.6 `nothing_written`:** as today, the stub giving a screen holding `secret-screen-text` and the far end typing `secret-typed-text`, which `inputs(1)` holds. No file under the data directory, no line and no report holds either. The relay lines name `t1`, `o1` and the place alone.
- **1.7.** `tests/family.rs`'s `the_sets_flume_answers_a_relay` builds `Relays::new(flume::Cli::new(…), …)` on the set's Flume as its other tests run it, on the scratch host's environment. Its terminal is the one the fake control plane's pull gives after `open_terminal(t1, l-1, 1)`, opened on a `Report` holding launch `l-1` under lease 1, launched at `<scratch>/places/nowhere`. The link is closed with `no place <scratch>/places/nowhere stands on this host`.
- **2.1.** `src/hello.rs`: `TOOLS` is `["flume", "sluice", "git"]`. `src/run.rs`: `Programs { flume, sluice }`, its `tools` and `relays` as Decision 6 has them, and its documentation names no herdr. `tests/control.rs`, `tests/pass.rs`, `tests/loop.rs` and `tests/local_layer.rs` drop `herdr` from their programs and stubs, and `tests/control.rs`'s two assertions that the stub `herdr` was not called go with its stub.
- **2.1 and 2.2.**
  - `tests/hello.rs`'s `each_tools_first_line_and_one_not_there` reads `git` as the one not there, in place of herdr: `git` given as a program that does not exist, its words beginning `git could not run: `, and the hello's `tool_versions["git"]` missing, `flume` and `sluice` printed. `a_tool_that_fails_or_prints_nothing_is_missing` gives the failing program as `flume` in place of herdr.
  - `the_hello_runs_no_multiplexer`: `hello::Cli::on_path()` given programs for `flume`, `sluice` and `git` that print their versions, and for `herdr` a program that writes a file when it runs. The hello's `tool_versions` keys are `["flume", "git", "sluice"]`, and the file is never written.
  - `tests/offers.rs`'s `a_tool_missing` makes `sluice` missing. Its keys are `["flume", "git", "sluice"]`.
- **2.3.** `crates/boomdrive-control/tests/json.rs`'s hello names `sluice`, missing, in place of herdr. `src/want.rs`'s `Link::screen` sends "what Flume's relay shows of the agent", and `Input::Keys` holds "keys by Flume's key names, as the far end gave them".
- **3.1.** `src/facts.rs` loses `Pane`, `pane` and their unit tests. `tests/facts.rs`'s `a_live_place_shows_its_agents_pane` loses its assertion of `facts::pane`, keeping those of `live_or_making` and `mark`, and is named `a_live_place`.
- **3.2.** `tests/rules.rs`'s `no_crate_names_a_multiplexer`, as Decision 10 has it.
- **4.1.** `crates/boomdrive-hosted/src/connection.rs`'s `NO_TERMINAL` is Decision 8's words, and its documentation cites core.30 and record 0004 as amended on 2026-11-15. `tests/plane.rs`'s `a_terminal_opened` still compares with `NO_TERMINAL`, and also asserts that the words name `end-to-end` and do not name Flume.
