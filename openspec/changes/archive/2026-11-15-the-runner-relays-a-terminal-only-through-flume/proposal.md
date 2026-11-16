# Proposal

## Why

The brief: "the runner relays a place's terminal to the control plane only through flume relay, encrypted end to end as before, and no code of the runner's names or runs a multiplexer, its hello no longer running herdr --version."

Record 0004, accepted by the user on 2026-11-07, decided that a terminal goes through Flume's seam. The runner reaches an agent's terminal only through Flume's command line, and no code of the runner's names a multiplexer or runs one's command line (core.29). Its consequences asked Flume for a command that relays a place's terminal, and left two things to "a change of their own": removing the runner's herdr relay and the terminal-relay spec's herdr requirements, and the hello no longer running `herdr --version`.

Flume now has that command. `flume relay <place>` relays the place's live agent through the terminal its start recorded, in JSON lines both ways: `{"screen": …}`, `{"unread": …}` and `{"untyped": …}` out, then its answer; `{"text": …}` and `{"keys": […]}` in. It ends when its input closes or the agent's terminal goes (Flume's statement 221 and 222, and C.5's herdr relay row, at boomdrivedev/blueprints b54a9b4; Flume's change `flume-relays-a-places-terminal`, landed as #70, in Flume `v0.24.0`, which the set pins).

The runner still relays through herdr. `crates/boomdrive/src/terminal.rs` reads the pane's address from Flume's status, `clues.shown_at.agent`. It runs `herdr pane read`, `send-text` and `send-keys` with `HERDR_SOCKET_PATH` set, which core.29's details rule out by name. The hello runs `herdr --version` for every pull, which core.29 also rules out.

## What Changes

- **A terminal relays through `flume relay`.** An open terminal runs `flume --json relay <place>`, the place's location as Flume named it at the launch, on the terminal's own thread. Each screen Flume gives goes on the link as Flume gave it. Each text and each set of keys from the link goes to Flume's input as one line, each key by Flume's name, as the far end gave it.
- **Flume says whether there is an agent to relay.** The runner still opens a terminal only on a launch its report holds under the terminal's lease, at a place Flume named. It no longer reads a pane from Flume's status. Where Flume refuses the relay, the link is closed with Flume's words as given.
- **A terminal closes by closing Flume's input.** When the link closes or stops listening, the launch is let go or the runner stops, the runner closes the relay's input. That is Flume's caller's word to end. It then awaits Flume's answer and exit, and stops no process. A relay Flume ends, its agent's terminal gone, closes the link.
- **Nothing of a terminal is held.** Flume gives a screen only when it changes, so the runner compares none and holds none. Its lines name the terminal, its launch and its place. A line about an input Flume did not type leaves Flume's words out, since they may name the key the person typed (core.11).
- **The hello names Flume, sluice and git.** The runner runs no `herdr --version`, and `Programs` holds no herdr.
- **No code of the runner's names a multiplexer**, but setup's link of the set's herdr plugins, which core.29 allows. `facts::Pane` and `facts::pane` go, with the herdr relay and its stub. A test of `tests/rules.rs` holds every crate's sources to core.29.
- **The hosted implementation still closes every terminal at once**, encrypted end to end as before (core.30). Its words now say why: the runner's half of the terminal's end-to-end exchange is not built, and it relays nothing in the clear. They no longer say the host's Flume relays none.
- **No floor moves.** `flume::LEAST` stays `0.21.0`. A Flume before `0.24.0` answers `relay` with its usage error, which closes the terminal in Flume's words.
- **Record 0004** gains an amendment of 2026-11-15, the stage's own choices on the written rules. **core.11, core.13 and core.29** gain details naming the relay. All are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `terminal-relay`: a terminal opens on a launch held at a place Flume named, and relays through `flume relay`: Flume's screens out, the person's input in, Flume's refusal or end closing it, its close closing Flume's input; nothing is held, and no crate's sources name a multiplexer.
- `work-into-flume`: the hello names the versions of Flume, sluice and git, and runs no multiplexer's.
- `hosted-control-plane`: a terminal the control plane opens is closed at once, with words saying the runner's half of its end-to-end exchange is not built.

## Impact

- `docs/adr/0004-…`: the amendment of 2026-11-15, *the runner relays through Flume's relay*. `docs/architecture/core.md`: the details of core.11, core.13 and core.29. All are written with this change.
- `crates/boomdrive`:
  - `src/terminal.rs`: the relay through `flume relay`, in place of herdr's command line;
  - `src/facts.rs`: `Pane` and `pane` removed, with their unit tests;
  - `src/hello.rs`: `TOOLS` without herdr;
  - `src/run.rs`: `Programs` without herdr, the relays run through its Flume;
  - `tests/fakes/mod.rs`: a stub `flume` that relays, in place of the stub `herdr`;
  - `tests/terminals.rs`, `tests/hello.rs`, `tests/offers.rs`, `tests/facts.rs`, `tests/pass.rs`, `tests/control.rs`, `tests/loop.rs`, `tests/local_layer.rs`, `tests/family.rs` and `tests/rules.rs`.
- `crates/boomdrive-control/src/want.rs`: the documentation of `Link::screen` and `Input::Keys`. `crates/boomdrive-control/tests/json.rs`: its hello's tools without herdr.
- `crates/boomdrive-hosted`: `src/connection.rs`, `NO_TERMINAL`'s words; `tests/plane.rs`.
- Nothing else changes. No crate, tool or tracked file type is added. No protocol file changes (core.42): `tool_versions` is a map by tool's name, and the examples of versions 1 and 2 that name herdr stay. No field of `report.json` changes. `flume::LEAST` and the set's Flume stay where they are. The flake, its modules and setup keep herdr: they install it, link its plugins and name it Flume's terminal, none of which is the runner's relay (core.29).
- Outside this repository: nothing is needed. Flume `v0.24.0` gives `flume relay`, and the set pins it. boomdrive-cloud reads no `herdr` from a hello. Building the runner's half of a terminal's end-to-end exchange stays the change that builds both halves with the cloud (record 0004's consequences). Each is named in design.md.

## Touches

```
docs/adr/0004-a-terminal-goes-through-flumes-seam-and-leaves-the-host-only-encrypted-end-to-end.md
docs/architecture/core.md
crates/boomdrive/src/terminal.rs
crates/boomdrive/src/facts.rs
crates/boomdrive/src/hello.rs
crates/boomdrive/src/run.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/terminals.rs
crates/boomdrive/tests/hello.rs
crates/boomdrive/tests/offers.rs
crates/boomdrive/tests/facts.rs
crates/boomdrive/tests/pass.rs
crates/boomdrive/tests/control.rs
crates/boomdrive/tests/loop.rs
crates/boomdrive/tests/local_layer.rs
crates/boomdrive/tests/family.rs
crates/boomdrive/tests/rules.rs
crates/boomdrive-control/src/want.rs
crates/boomdrive-control/tests/json.rs
crates/boomdrive-hosted/src/connection.rs
crates/boomdrive-hosted/tests/plane.rs
openspec/specs/terminal-relay/spec.md
openspec/specs/work-into-flume/spec.md
openspec/specs/hosted-control-plane/spec.md
```
