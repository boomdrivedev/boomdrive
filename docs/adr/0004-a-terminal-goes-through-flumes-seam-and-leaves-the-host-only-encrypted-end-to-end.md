---
number: 4
title: A terminal goes through Flume's seam, and leaves the host only encrypted end to end
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# A terminal goes through Flume's seam, and leaves the host only encrypted end to end

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-01, from the user's words in the constitution interview. Amended on 2026-11-15, below: the runner relays through Flume's relay, the construct stage's own choices on the written rules.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (agentplot/boomdrive's `design/research/the-constitution-interview.md`), its topics on what the runner owns and on the state directory; flume's statement, clause 107 and its actors table, and flume's terminal seam, `crates/flume-seams/src/terminal.rs`; agentplot/boomdrive-cloud's record 0016 and rule core.18, cited by id; agentplot/boomdrive-cloud `design/research/awesome-lists-disposition.md`, rows S6 and F15; the terminal-relay spec; rule core.11.

## Context and problem statement

A person may open a launch's terminal from its card in the cloud. The terminal is relayed through the host's Host object and the runner's outgoing socket, and its bytes are never stored (cloud record 0016). As built, the runner's relay runs herdr's command line itself: `herdr pane read`, `send-text` and `send-keys`.

That stands against Flume's statement, whose clause 107 says Flume opens, sends to and closes every agent's terminal, and that no caller does. It also ties the runner to one multiplexer, where Flume already has a seam for multiplexers and TuiOS is to come. And the bytes crossed the cloud protected only by the socket's TLS.

## Considered options

1. A relayed hand counts as a person: the runner relays a person's screen and keys and sends nothing of its own.
2. Screen out, answers in: the relay shows the screen only, and input goes down as an answer.
3. The relay goes through a command of Flume's, which reaches the multiplexer through Flume's seam.

Then, for the relay as built: remove it and build on Flume's command once there is one, or keep it until then. And, from the awesome-lists research: the terminal encrypted end to end between the runner and the browser (S6), the runner's half by HPKE (F15), or relayed under the socket's TLS alone.

## Decision outcome

Options 1 and 3 together, the user's choice: "a combo of #1 and #3 might make the most sense. Remember that Herdr should be a seam for multiplexers. We will want to support TuiOS shortly."

- **Through Flume's seam.** The runner reaches an agent's terminal only through Flume's command line, which reaches the host's multiplexer through Flume's own terminal seam. No code of the runner's names a multiplexer or runs one's command line (core.29).
- **A relayed hand is a person's.** A terminal the runner relays carries only a person's screen out and that person's keys in. The runner sends nothing of its own into it and keeps nothing of it (core.11). Flume counts the person as a person at the keyboard, not as a caller.
- **The relay comes out until Flume's command exists**, the user's choice. A terminal the control plane opens is closed at once, with words saying the host's Flume relays none.
- **Encrypted end to end**, S6 and F15, kept by the user. A terminal leaves the host only encrypted to the browser that opened it. The runner makes a key pair for each terminal and signs its share with the host's key (record 0006). It accepts the browser's share only when a key certified to a member signed it, and it never sends a pane's bytes in the clear, whatever fails. The Host object passes only ciphertext (core.30).

### Consequences

- Flume is asked for a command that relays a place's terminal through its seam, screen out and keys in, and for clause 107 to count a relayed person as a person.
- The runner's herdr relay and the terminal-relay spec's herdr requirements are removed by a change of their own. The hello stops running `herdr --version`.
- The runner's half of the exchange is built with the hosted implementation, against the browser's half in the cloud.
- crew opens no terminal, so nothing it uses changes.

## Amendment, 2026-11-15: the runner relays through Flume's relay

Status: the construct stage's own choices, on the brief of the change `the-runner-relays-a-terminal-only-through-flume`, 2026-11-15. The direction is this record's, accepted by the user on 2026-11-07. What follows is how the runner meets it, each choice made on the written rules (core.1, core.11, core.13, core.29, core.30, core.38), and cheap to change, so it is not asked.

The brief: "the runner relays a place's terminal to the control plane only through flume relay, encrypted end to end as before, and no code of the runner's names or runs a multiplexer, its hello no longer running herdr --version."

### Context

Flume now relays a place's terminal. `flume relay <place>` relays the place's live agent through the terminal its start recorded, in JSON lines both ways, and ends when its input closes or the agent's terminal goes (Flume's 221 and 222, and C.5's herdr relay row, at boomdrivedev/blueprints b54a9b4; Flume's change `flume-relays-a-places-terminal`, in Flume `v0.24.0`, which the set pins). The runner still relayed through herdr's command line, reading the pane from Flume's status, and its hello ran `herdr --version`, each of which core.29 rules out. The runner's half of the end-to-end exchange is not built, so the hosted implementation closed every terminal at once, with words saying the host's Flume relays none.

### Decision outcome

- **One `flume relay` for each open terminal.** It names the place by the location Flume named at the launch, and runs as every Flume call of the runner's: `--json` first, `FLUME_RECIPIENT` unset. It is held while the terminal is open, a tool's own command its control plane asked for (core.13).
- **Flume says whether there is an agent to relay.** The runner opens a terminal only on a launch its report holds under the terminal's lease, at a place Flume named. It reads no pane from Flume's status, and Flume's refusal closes the link in Flume's words (core.1, core.29). Not taken: reading `clues.shown_at.agent` first, which names a multiplexer and judges from the last pass what Flume answers now.
- **Each line as Flume gave it.** Each screen goes on the link as Flume gave it, none compared or held, since Flume gives a screen only when it changes. An unread read is said in a line with Flume's words. An untyped input is said in a line without them, since they may name the key the person typed (core.11). The far end is told neither. Not taken: a call of the interface's link carrying Flume's words to the far end, which the hosted protocol, its frames sealed, has no message for. The change that builds the runner's half of the exchange can carry them sealed.
- **The person's input by Flume's names.** Each text and each set of keys goes to Flume's input as one line, each key as the far end gave it. The runner maps, checks and drops none: Flume answers a name it does not take.
- **A close closes Flume's input.** Where the link closes or stops listening, the launch is let go or the runner stops, the runner closes the relay's input, Flume's caller's word to end, and awaits Flume's answer and exit, sending no signal (core.13). Not taken: killing the relay at the close, which would stop a tool's process when Flume ends it at its caller's close within its own bound.
- **No floor moves.** `flume::LEAST` stays `0.21.0`. A Flume before `0.24.0` answers `relay` with its usage error, which closes the terminal in Flume's words, as a confirm on a Flume with no `confirm` is answered (core.56). Not taken: raising the least to `0.24.0`, under which a host would take no work for a terminal no implementation opens yet (core.38).
- **The hello names Flume, sluice and git**, and the runner runs no multiplexer's `--version`.
- **The hosted implementation still closes each terminal at once**, since a terminal leaves the host only encrypted end to end and never in the clear (core.30). Its words now say the runner has no end-to-end exchange for a terminal yet and relays none in the clear. The words it replaces, that the host's Flume relays no terminal, are false on a host of the set's Flume. Not taken: building the runner's half here. Its suite is chosen with the browser's half in the cloud, by the change that builds both halves.

### Consequences

- No code of the runner's names a multiplexer but setup's link of the set's herdr plugins, and a test of the crates' sources holds them to it (core.29).
- The terminal-relay spec's herdr requirements are removed, and its Flume requirements added, by that change.
- No implementation opens a terminal yet: crew's opens none, and the hosted implementation closes each until the runner's half of the exchange is built.
