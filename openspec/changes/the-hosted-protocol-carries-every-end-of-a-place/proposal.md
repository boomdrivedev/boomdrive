# Proposal

## Why

The brief, 2026-11-09: "A runner under the hosted control plane reports every end Flume gives a place, a drop and a done among them, each with its delivery's key, and carries the hosted control plane's confirm of a reader's ends as it carries crew's, in one protocol version that names them both, and a place whose end a version does not name never empties the rest of the host's status." Its sources are record 0015 and weir's record 0006, accepted by the user on 2026-11-08.

Version 1 of the runner's protocol copies a place's status as Flume 0.4.1 printed it, and a release has spoken it, so it never changes (core.42). Under Flume `v0.21.0`, the least the runner runs with, a place's status gives a drop's or a done's end, `{"of":"over",…}`, a hand-over at a session's start, and each end's delivery: its key, its delivery's key, its expiry, and the readers it is owed to and confirmed by. Version 1 names none of these. Its objects are closed, so the delivery is cut away, and an end of the first two forms fails the copy, so the place's whole status goes up `null`. The control plane loses the whole place, and the runner never confirms that place's ends, since it confirms only what its control plane took (record 0030 as amended on 2026-11-09).

Version 1 names no confirm either. crew's control plane confirms a reader's ends through the runner (core.56), but a hosted one cannot, and has no delivery key to name.

## What Changes

- **Record 0037 decides version 2**, and core.42's and core.27's details read with it. Both are written with this change.
- **Version 2 of the protocol, at `crates/boomdrive-hosted/protocol/v2/`, is version 1 whole but for these:**
  - every end of a place's status names its `key`, its `delivery`, its `expires_at`, the readers it is `owed` to and those that `confirmed` it;
  - an end of the place's work by a drop or a done, `{"of":"over",…}`, is a form of an end;
  - a hand-over may be `by` `session_start`;
  - an end's delivery alone is a form of an end;
  - an act may be a confirm, as the interface writes it.
- **Each end of a place is copied on its own.** An end version 2 does not name goes up as its delivery alone, or is left out where even that fails, and the rest of the place's status goes up. A status taken by the control plane is one that went up whole.
- **The runner speaks version 2 alone**, at its upgrade, `boomdrive.v2`, and at its enrollment. Its contexts start `boomdrive.v2`. A control plane accepts the current version and the one before.
- **A hosted control plane's confirm reaches the runner** as any act, and is carried out as crew's is (core.56).
- **Version 1 stays as a release spoke it.** Its files and examples stay and are still checked, and a test holds each file to its digest.
- **The protocol's README describes version 2**, with its worked bytes and how version 1 differs.
- **The branch takes `bolt/the-runner-confirms-the-ends-it-carries`**, whose confirm act, statuses taken and Flume `v0.21.0` this change builds on, before its own work.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `runner-protocol`: version 2 beside version 1, naming every end Flume 0.21.0 gives a place with its delivery, an end's delivery alone, and a confirm; version 1 held as spoken; the README of version 2.
- `hosted-control-plane`: the runner speaks version 2; each end is copied on its own; a status is taken only where it went up whole; a control plane's confirm is given to the runner.

## Impact

- `docs/adr/0037-…md` is new. `docs/architecture/core.md` amends core.42's and core.27's details. Both are written with this change.
- `crates/boomdrive-hosted`:
  - `protocol/v2/`, new: version 1's ten schema files and their examples, with version 2's changes, and examples of its own;
  - `protocol/README.md`: version 2, its subprotocol, contexts and worked bytes, and how version 1 differs;
  - `src/protocol.rs`, `src/protocol/v1.rs` and `src/protocol/v2.rs`: version 2's types, and version 1's files alone;
  - `src/check.rs`, `src/copy.rs`, `src/connection.rs`, `src/enrollment.rs`, `src/key.rs`, `src/plane.rs`, `src/lib.rs`: version 2;
  - `tests/`: every test file, and `tests/common/mod.rs` and `tests/common/fake.rs`, on version 2, with version 1's files still checked.
- `crates/boomdrive/tests/fixtures/flume-0.21.0/`, new: Flume 0.21.0's answers, captured on a scratch host.
- `crates/boomdrive-control/src/want.rs`: the confirm's and land's documentation name version 2.
- `README.md`: the hosted section names version 2, each end's delivery, and a control plane's confirm.
- The merge of `bolt/the-runner-confirms-the-ends-it-carries` brings that bolt's own files. This change writes none of them but where a conflict is resolved.
- No crate, tool, configuration key or field of the report file is added. Version 2 is a new directory of the protocol, which core.42 provides for.
- Outside this repository: boomdrive-cloud pins version 1 (`workers/control-plane/runner-protocol/v1/`) and takes version 2 by a change of its own, for the orchestrator to ask of it.

## Touches

```
docs/adr/0037-the-runner-speaks-version-2-of-its-protocol-which-carries-every-end-of-a-place-with-its-delivery-and-a-control-planes-confirm.md
docs/architecture/core.md
README.md
crates/boomdrive-control/src/want.rs
crates/boomdrive-hosted/Cargo.toml
crates/boomdrive-hosted/protocol/README.md
crates/boomdrive-hosted/protocol/v2/
crates/boomdrive-hosted/src/
crates/boomdrive-hosted/tests/
crates/boomdrive/tests/fixtures/flume-0.21.0/
openspec/changes/the-hosted-protocol-carries-every-end-of-a-place/
```
