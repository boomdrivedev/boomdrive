# The runner's protocol

The messages between a Boomdrive runner and a hosted control plane, as
JSON Schema of draft 2020-12, one directory per version. The runner
speaks it over one outgoing WebSocket, which it opens and keeps; no
connection ever comes in to a host. Both ends check every message they
are sent against the version before reading it, and refuse one that
fails whole.

This repository's records and rules are the protocol's source: rule
core.42 and record 0015, under `docs/`, and the change that wrote
version 1, `the-runner-keeps-its-protocol-as-one-json-schema-per-version`.
They cite boomdrive-cloud's records and rules by id.

## Versions

Version 1 is `v1/`. A version may change until a release of the runner
speaks it to a control plane. From that release on it never changes,
and its directory is never removed: every change after it is a new
version, in a directory of its own beside it. A control plane accepts
the current version and the one before.

A connection asks for the WebSocket subprotocol `boomdrive.v1` at
its upgrade, so a control plane that does not accept the version refuses
the connection before any message.

## The files

| file | what it holds |
|---|---|
| `from-runner.schema.json` | the envelope of every message a runner sends |
| `to-runner.schema.json` | the envelope of every message a control plane sends |
| `shared.schema.json` | what both directions use: ids, a time, integers, a lease, a repository's address, a commit, a digest, bytes, and `message_refused` |
| `hello.schema.json` | the hello, a tool's version, and the copy of Flume's host part |
| `wants.schema.json` | a want, an offer with its launch, an act, a command, an inbox, and `pulled` |
| `report.schema.json` | the runner's report and `reported` |
| `place-status.schema.json` | the copy of a Flume place's status |
| `submission-entry.schema.json` | the copy of a sluice submission's entry |
| `terminal.schema.json` | a key share and a terminal's messages |

`examples/from-runner/` and `examples/to-runner/` hold whole messages
of every kind, each of which passes its direction's envelope.

Each file's `$id` is its address among this repository's raw files at
`main`. No check fetches a `$id`: a validator is handed the version's
files, each under its `$id`, and nothing else, and every `$ref` is
relative, to a file of the same version. A reference outside them fails
to resolve. A pattern uses only what ECMA-262 and Rust's `regex` read
alike, so a validator in either language refuses the same strings.

## The forms

- **Every message** is `{"protocol_version": 1, "id": …, "body": {"<kind>": {…}}}`.
  The `id` is the sender's own for the message, 1 to 128 letters, digits
  and `._:/-`, unique among what that sender sends on its enrollment, and
  kept when the message is sent again. The body holds exactly one kind,
  and each direction's envelope takes only that direction's kinds.
- **Every object is closed**: it takes only the fields it names, at
  every depth. A map, keyed by ids, paths or tools' names, names the
  form of its keys and of its values. Two values are carried whole, of
  any form, as their tool printed them: a sluice row's `flaky`, and a
  thing a stage's end made.
- **Every field the runner's interface always writes is required**,
  `null` where it holds none.
- **A time** of the runner's or the control plane's is RFC 3339 in UTC
  to the second, `YYYY-MM-DDTHH:MM:SSZ`. A time inside a copy is the
  tool's, as it printed it: Flume's are Unix seconds.
- **An integer** is one JavaScript reads exactly, within 2^53 − 1 of
  zero. A lease's number is 1 or more.
- **Bytes**, a key, a signature, a certificate or a sealed frame, are
  base64url without padding.

## The messages

| kind | sent by | holds |
|---|---|---|
| `hello` | the runner | the host's interface and runner versions, its tools' versions, the repositories it keeps for this control plane, its capacity, and the leases it holds |
| `report` | the runner | the runner's whole report: each launch's answer with its place's status and its submission's entry as their copies, each act and command it carried out or refused, and each inbox it carried |
| `pulled` | the control plane | the whole of what it wants, in its order, and the runner's report as it holds it, or `null` |
| `reported` | the control plane | the `report` message it answers, by id; the ids it lets go; the ids it refused, with its words |
| `message_refused` | either | a message that failed its check, by id, with words |
| `terminal_open` | the control plane | a terminal on a launch's agent, under the launch's lease number, with the browser's share |
| `terminal_opened` | the runner | the terminal, with the runner's share |
| `terminal_output` | the runner | a sealed frame of the pane, at its offset, and whether it is a redraw |
| `terminal_input` | the control plane | a sealed frame of the person's keys, at its offset |
| `terminal_resume` | the control plane | the offset a reconnected browser has every byte up to |
| `terminal_close` | the control plane | the terminal, closed from the browser's end |
| `terminal_closed` | the runner | the terminal, closed by the runner, with its words |

Their order:

1. At every connection the runner's first message is its whole `hello`.
   The control plane answers it with `pulled`, holding the runner's
   report as the control plane holds it, so a runner that lost its own
   takes it back.
2. Whenever its wants change, the control plane sends `pulled` again,
   its `report` `null`.
3. The runner sends `hello` again when it changes. At each report it
   makes, it sends `report`, which renews every lease it names.
4. The control plane answers each `report` with `reported`, naming it by
   id. After a reconnect the runner sends again, under its own id, the
   last `report` no `reported` answered, and no earlier one, since a
   report is whole.
5. A message that fails its check is answered with `message_refused`,
   and nothing else is done with it. Its sender does not send it again.
6. A terminal opens, carries its frames and closes as below.

A `pulled` is the whole of what the control plane wants, and a `report`
the whole report, so a second delivery of either changes nothing. The
socket is kept alive by the WebSocket protocol's own pings: no message
is a heartbeat.

Version 1 has no message for enrollment, for the proof at each
connection that the host's key is held, for a forge token, for health,
or for the runner's own commands. The changes that build them add them.

## The copies

A tool's answer goes up as a copy: an object holding only the fields
the cloud keeps, each as the tool printed it. The schema has no field
for the rest, so the cloud's check refuses it.

- **A place's status** (`place-status.schema.json`) is `flume status
  --json`'s one place, as Flume 0.4.1 prints it, cut to its ids, states,
  times and counts, what it made, its live agent's pane, and the words a
  person reads to follow or answer it: an end's, a question's and a
  hold's. It holds no agent's last words, screen, printed command,
  notes, settings or mode; no task's text; no readiness; no task group's
  title; no teardown's words; no end's report; no verdict's words; and
  no path but the place's own. A field Flume adds after 0.4.1 is left
  out.
- **The host's capacity** (`host_part` in `hello.schema.json`) is
  Flume's host part as values: the host's and each stream's room, a
  stream without its queue, which names places of every control plane
  the host serves. Flume's pools, addresses and bases are left out.
- **sluice's entry** (`submission-entry.schema.json`) is `sluice status
  <n> --json` cut to its `id`, `branch`, `head`, `state` and
  `landed_as`, and its failed rows' `task`, `hash`, `flaky` and
  `system`, never a row's `log`.

## A terminal

A terminal's bytes leave the host only encrypted end to end, to the
browser that opened it. The control plane relays them and cannot read
them, and neither end keeps them.

- **The shares.** `terminal_open` carries the browser's share, and
  `terminal_opened` the runner's. A share holds its `suite`, the end's
  `key` for this terminal alone, its `signature` by the end's signing
  key, and the `certificate` the control plane issued on that signing
  key: a member's for the browser, the host's for the runner. The
  runner takes the browser's share only where its signature checks
  against a key certified to a member, and never falls back to the
  clear.
- **What a signature is made over**: the UTF-8 bytes of
  `boomdrive.v1 terminal share`, then, each after a NUL byte, the
  end (`browser` or `runner`), the terminal's id, the launch's id, the
  suite, and the key's bytes. A share is therefore never taken for
  another terminal or reflected back to its own end.
- **The suite** is a name the two ends agree; the control plane relays
  it. The change that builds both halves of a terminal chooses it and
  names it here, without a new version. Two ends naming different
  suites open no terminal.
- **Frames.** `terminal_output` and `terminal_input` carry their bytes
  only as `sealed`, the suite's ciphertext. Each carries `offset`, the
  place of its first byte in its direction's stream, counted from 0 at
  the terminal's opening.
- **A resume.** A browser whose socket dropped, its keys still held,
  sends `terminal_resume` with `seen`, the offset it has every byte up
  to. Where that is behind what the runner sent, the runner answers
  with a frame whose `redraw` is `true`: the pane's present screen, at
  its next offset, which the browser draws after clearing its terminal.
  A browser that lost its keys opens a new terminal.
- **Closing.** `terminal_close` closes a terminal from the browser's
  end, and `terminal_closed` says the runner closed it, with its words.
  A terminal is closed with its socket.
