# Terminal relay

## Purpose

How the runner relays a terminal the control plane opens on a launch, between the control plane's link and the herdr pane where the launch's agent runs. It reaches the pane through herdr's command line, and over no connection but the runner's own outgoing one. Nothing of the terminal is kept (core.3, core.11; flywheel-cloud record 0016 and core.18; ledger item N1.b).

## ADDED Requirements

### Requirement: A terminal opens only on a launch held whose agent's pane Flume names

The runner SHALL open a terminal only where its report holds the terminal's launch under the terminal's lease number, and that launch's carried status names its agent's pane as `clues.shown_at.agent`, in the form `herdr:<pane>@<socket>`. Otherwise it SHALL close the terminal's link at once, with words saying why.

#### Scenario: A launch not held

- **WHEN** a terminal names a launch the runner's report does not hold
- **THEN** its link is closed with words naming the launch, and no `herdr` command is run

#### Scenario: No agent running

- **WHEN** a terminal names a launch whose carried status gives no `clues.shown_at.agent`
- **THEN** its link is closed with words saying no agent is running in its place

### Requirement: A terminal relays the pane's screen and its input through herdr's command line

An open terminal SHALL run on a thread of its own, with `HERDR_SOCKET_PATH` set to the pane address's socket for every `herdr` command:

- **out**: it SHALL run `herdr pane read <pane> --source visible --format ansi` every quarter of a second, and send a screen on the link only when it differs from the last one sent;
- **in**: it SHALL pass each text from the link as `herdr pane send-text <pane> <text>`, and each set of keys as `herdr pane send-keys <pane> <keys>…`.

#### Scenario: A screen sent once while it stands

- **WHEN** a terminal is open on pane `w1:p2` at socket `S`, and the stub `herdr` prints the same screen at three reads, then another
- **THEN** each `herdr` command runs with `HERDR_SOCKET_PATH=S` and names `w1:p2`, and the link receives two screens

#### Scenario: Input passed in

- **WHEN** the link sends the text `yes` and then the keys `enter`
- **THEN** the stub `herdr` is run as `herdr pane send-text w1:p2 yes`, then as `herdr pane send-keys w1:p2 enter`

### Requirement: A terminal closes with its link, its pane or its launch

An open terminal SHALL close when its link closes or stops listening, when `herdr` refuses a read, or when the runner lets its launch go.

#### Scenario: The pane gone

- **WHEN** the stub `herdr` refuses a read of the pane
- **THEN** the terminal's link is closed and its thread ends

#### Scenario: The launch let go

- **WHEN** a report refuses the terminal's launch as no longer this runner's
- **THEN** the terminal's link is closed

### Requirement: Nothing of a terminal is kept

The runner SHALL write no screen or input of a terminal to a file, a log line or a report, and SHALL hold only the last screen sent, to compare against. Its lines SHALL name the terminal's id, its launch and the pane, and nothing that passed (core.11).

#### Scenario: Nothing written

- **WHEN** a terminal relays a screen holding the words `secret-screen-text`, then closes
- **THEN** no file under the data directory holds `secret-screen-text`, no line the runner wrote holds it, and no report carries it
