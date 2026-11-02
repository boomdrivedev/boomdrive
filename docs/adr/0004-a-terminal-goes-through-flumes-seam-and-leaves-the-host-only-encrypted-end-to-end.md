---
number: 4
title: A terminal goes through Flume's seam, and leaves the host only encrypted end to end
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# A terminal goes through Flume's seam, and leaves the host only encrypted end to end

Status: accepted by the user on 2026-11-01, in the constitution interview.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01, its topics on what the runner owns and on the state directory; flume's statement, clause 107 and its actors table, and flume's terminal seam, `crates/flume-seams/src/terminal.rs`; agentplot/flywheel-cloud's record 0016 and rule core.18, cited by id; agentplot/flywheel-cloud `design/research/awesome-lists-disposition.md`, rows S6 and F15; the terminal-relay spec; rule core.11.

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
