# terminal-relay Specification

## Purpose
How the runner relays a terminal the control plane opens on a launch, between the control plane's link and Flume's relay of the launch's place, `flume relay`. It reaches the far end over no connection but the runner's own outgoing one, and keeps nothing of the terminal (core.3, core.11, core.29; Flume's 221 and 222; record 0004 as amended on 2026-11-15).

## Requirements

### Requirement: Nothing of a terminal is kept

The runner SHALL write no screen or input of a terminal to a file, a log line or a report, and SHALL hold no screen once it is sent on. Its lines SHALL name the terminal's id, its launch and its place, Flume's words of an unread read and Flume's refusal, and nothing that passed (core.11, core.35).

#### Scenario: Nothing written

- **WHEN** a terminal relays a screen holding the words `secret-screen-text`, the link sends the text `secret-typed-text`, and the terminal closes
- **THEN** the stub's standard input held `secret-typed-text`, and no file under the data directory, no line the runner wrote and no report holds either

### Requirement: A terminal opens only on a launch held at a place Flume named

The runner SHALL open a terminal only where its report holds the terminal's launch under the terminal's lease number, at a place Flume named, and SHALL then run `flume --json relay <place>`, the place's location, with `FLUME_RECIPIENT` unset. Otherwise it SHALL close the terminal's link at once, with words saying why, running nothing. It SHALL read no pane from Flume's status (core.10, core.29).

#### Scenario: A launch not held

- **WHEN** a terminal names a launch the runner's report does not hold, or holds under another lease number
- **THEN** its link is closed with words naming the launch and the lease, and no `flume relay` is run

#### Scenario: A launch at no place

- **WHEN** a terminal names a launch the runner's report holds unlaunched, waiting on its After
- **THEN** its link is closed with words saying the launch has no place Flume named, and no `flume relay` is run

### Requirement: Flume's refusal of a relay closes the terminal in Flume's words

Where Flume refuses a terminal's relay, the runner SHALL close the terminal's link with Flume's words as given: its refusal's `reason`, or, where Flume answers with no refusal, its standard error. Whether the place has a live agent, and whether its terminal offers a relay, SHALL be Flume's to answer (core.1, core.11; Flume's 221).

#### Scenario: Flume refuses the relay

- **WHEN** a terminal names a launch held at a place, and the stub `flume` refuses the relay with `fix--a has no live agent: waiting for room`
- **THEN** the stub was run once as `flume --json relay <place>`, and the link is closed with `fix--a has no live agent: waiting for room`

#### Scenario: A Flume with no relay

- **WHEN** the stub `flume` answers `relay` with the usage error `error: unrecognized subcommand 'relay'` on standard error, exiting 2
- **THEN** the link is closed with `error: unrecognized subcommand 'relay'`

#### Scenario: The set's Flume refuses a relay of no place

- **WHEN** a terminal names a launch held at a place that stands nowhere on the scratch host, and the runner relays it through the set's own Flume
- **THEN** the link is closed with the set's Flume's words, `no place <place> stands on this host`

### Requirement: A terminal sends Flume's screens as Flume gives them

An open terminal SHALL run on a thread of its own, its one process Flume's relay, held while the terminal is open. It SHALL send each `{"screen": …}` line Flume writes on the link as Flume gave it, in Flume's order, none compared, held or dropped. It SHALL run no multiplexer's command (core.1, core.29; Flume's 221 and 222).

#### Scenario: Screens as Flume gives them

- **WHEN** a terminal is open, and the stub `flume` writes three screens, the second and third the same
- **THEN** the link receives the three screens, in that order, each as the stub wrote it, and the stub ran with `FLUME_RECIPIENT` unset

### Requirement: Flume's unread and untyped lines are said, the untyped without Flume's words

Each `{"unread": …}` line Flume's relay writes SHALL be said in a line naming the terminal, its launch and its place, with Flume's words. Each `{"untyped": …}` line SHALL be said in a line naming them that leaves Flume's words out, since they may name what the person typed. The relay SHALL go on after either (core.11; Flume's 221 and 222).

#### Scenario: An unread read and an untyped input

- **WHEN** the stub writes `{"unread": "herdr did not answer within 1s"}`, then `{"untyped": "secret-key-name is no key flume names"}`, then a screen
- **THEN** the runner says a line naming the terminal, its launch, its place and `herdr did not answer within 1s`, and a line naming the terminal, its launch and its place that holds no `secret-key-name`, and the link receives the screen

### Requirement: The person's input goes to Flume as typed

Each text from a terminal's link SHALL be written to Flume's standard input as one line, `{"text": …}`, and each set of keys as one line, `{"keys": [ … ]}`, each key as the link gave it, by Flume's name. The runner SHALL map, check and drop none (core.1; Flume's 221 and 222).

#### Scenario: Input passed in

- **WHEN** the link sends the text `yes`, and then the keys `ctrl+c` and `enter`
- **THEN** the stub's standard input holds the line `{"text":"yes"}`, then the line `{"keys":["ctrl+c","enter"]}`

### Requirement: A terminal the runner closes closes Flume's input and awaits Flume's end

A terminal SHALL close when its link closes or stops listening, or when the runner lets its launch go or stops. The runner SHALL then close Flume's standard input, send no further screen, and await Flume's answer and its exit, sending Flume no signal. It SHALL then close the link with `its link closed`, `its far end stopped listening`, `its launch <launch> is let go` or `the runner stopped` (core.10, core.13; Flume's 221).

#### Scenario: The link closed

- **WHEN** the link sends its close
- **THEN** the stub's standard input reaches its end, the stub answers that its relay ended by its caller and exits, and the link is closed with `its link closed`

#### Scenario: The launch let go

- **WHEN** a report refuses the terminal's launch as no longer this runner's
- **THEN** the stub's standard input reaches its end, and the link is closed with `its launch <launch> is let go`

#### Scenario: The runner stopped

- **WHEN** the runner's terminals are dropped while one is open
- **THEN** the drop returns once the stub has exited, the stub's standard input having reached its end, and the link is closed with `the runner stopped`

### Requirement: A relay Flume ends closes the terminal

Where Flume's relay ends of its own, its answer `relayed` and the runner having closed nothing, the link SHALL be closed with `Flume's relay ended: <ended>`, `<ended>` as Flume gave it, and the terminal's thread SHALL end (Flume's 221).

#### Scenario: The agent's terminal gone

- **WHEN** the stub `flume` answers that its relay ended `gone`, and exits
- **THEN** the link is closed with `Flume's relay ended: gone`, and the terminal's thread ends

### Requirement: No code of the runner's names a multiplexer

No `.rs` file under any crate's `src` but `crates/boomdrive/src/setup.rs`, whose link of the set's herdr plugins core.29 allows, SHALL hold `HERDR_` or a string literal that begins with `herdr`, `tmux`, `zellij` or `rex` (core.29; record 0004).

#### Scenario: The crates' sources read

- **WHEN** every `.rs` file under each crate's `src` but `crates/boomdrive/src/setup.rs` is read
- **THEN** none holds `HERDR_`, or a string literal that begins with `herdr`, `tmux`, `zellij` or `rex`
