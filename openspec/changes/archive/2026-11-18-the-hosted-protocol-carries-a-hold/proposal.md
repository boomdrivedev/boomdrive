# Proposal

## Why

The brief: "a new version of the runner's hosted protocol carries the hold act, its words and bound, so a hosted control plane can hold the move a place waits on it for as crew's can, versions 1 and 2 unchanged".

A control plane asks that the move a launch's place waits on it for be held with `hold`, an act naming its launch, its words and its bound in whole seconds (core.59; Flume's clause 130). The change `the-runner-carries-a-hold-of-a-places-waiting-move` built it for crew's control plane alone. Its design Decision 7 left a hold out of the hosted protocol, since version 1 was spoken by `v0.1.0` to `v0.7.3` and version 2 by every release from `v0.8.0`, and neither may change (core.42).

So a want whose act is a hold fails version 2's check today, and the whole `pulled` holding it is refused. A hosted control plane's proven work therefore stalls after five minutes, and its stall reaches the place's recipients as though something had failed. crew's control plane holds that move under words. Flume's statement now names the hold in the runner's row of callers, with its control plane's words and a bound (blueprints `7cdb007`).

## What Changes

- **Record 0052 decides version 3.** core.42's and core.59's details, and record 0002's amendment of 2026-11-14, read with it. All are written with this change.
- **Version 3 of the protocol, at `crates/boomdrive-hosted/protocol/v3/`, is version 2 whole but for these:**
  - each file's `$id` is under `boomdrivedev/boomdrive`'s address, `…/protocol/v3/`;
  - every message's `protocol_version` is 3;
  - `act_sort` takes a hold, `{"hold":{"words":…,"bound_interval":…}}`, as the interface writes `ActSort::Hold`. Its words are any string and its bound any integer from 0 that JavaScript reads exactly;
  - its descriptions name version 3, and `act_sort`'s names the hold.
- **Version 3's examples are version 2's**, each naming `protocol_version` 3, with `enroll.json` signed over version 3's context, and one more: a `pulled` holding a hold.
- **A test holds version 3 to version 2**: each schema file and each example is version 2's but for what record 0052 names.
- **The runner speaks version 3 alone.** Its upgrade asks for `boomdrive.v3`, its enrollment's `enroll` is a message of version 3, and its contexts start `boomdrive.v3`. The words it says of a check name version 3, made from the version's own number. A control plane accepts the current version and the one before.
- **A hosted control plane's hold reaches the runner** as any act, under the partition's prefix, and is carried out as crew's is (core.59). The hosted implementation reads and refuses nothing of it.
- **Version 2 stays as `v0.9.0` spoke it.** Its files and examples stay, held to their digests, and each example still passes version 2. Its module keeps its files and no type, as version 1's does.
- **What the runner says of the protocol names version 3**: the protocol's README, README's hosted section, the interface's documentation of each act and of a preview, and the words a cloud host's `boomdrive forge-token` refuses with.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `runner-protocol`: version 3 beside versions 1 and 2, version 2 whole but for a hold; the runner's types of version 3; version 2 held as `v0.9.0` spoke it; neither version 1 nor version 2 carrying a hold.
- `hosted-control-plane`: the runner speaks version 3; a hosted control plane's hold is given to the runner as any act.
- `forge-tokens`: a cloud host's `forge-token` says version 3 of the protocol carries no token yet.

## Impact

- `docs/adr/0052-…md` is new. `docs/architecture/core.md` amends core.42's and core.59's details, and record 0002's amendment of 2026-11-14 names version 3. Each is written with this change.
- `crates/boomdrive-hosted`:
  - `protocol/v3/`, new: version 2's eleven schema files and its examples, with version 3's changes and one example of its own;
  - `protocol/README.md`: version 3, its subprotocol, contexts and worked bytes, a hold among a `pulled`'s acts, and how versions 2 and 1 differ;
  - `src/protocol.rs`, `src/protocol/v2.rs` and a new `src/protocol/v3.rs`: version 3's types, and version 2's files alone;
  - `src/check.rs`, `src/copy.rs`, `src/connection.rs`, `src/enrollment.rs`, `src/key.rs`, `src/plane.rs`, `src/lib.rs` and `Cargo.toml`: version 3;
  - `tests/`: every test on version 3, with versions 1 and 2 still checked, and `tests/worked-bytes.json` with version 3's bytes.
- `crates/boomdrive-control/src/want.rs` and `src/report.rs`: the documentation of a land, a confirm, a hold and a preview names version 3.
- `crates/boomdrive/tests/forge_tokens.rs`: the words of a cloud host's refusal.
- `README.md`: the hosted section and the forge-token section name version 3 and a hosted control plane's hold.
- No crate, tool, configuration key or field of the report file is added, and no tracked file type. Version 3 is a new directory of the protocol, which core.42 provides for. No file under `protocol/v1/` or `protocol/v2/` changes.
- Outside this repository:
  - boomdrive-cloud pins version 2 (`workers/control-plane/runner-protocol/v2/`), and takes version 3 by a change of its own;
  - record 0048's observing connection is in no version yet, and either joins version 3 before it is released or comes in a version of its own.

  Each is named in design.md and told to the orchestrator.

## Touches

```
docs/adr/0052-the-runner-speaks-version-3-of-its-protocol-which-is-version-2-but-for-a-control-planes-hold.md
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
docs/architecture/core.md
README.md
crates/boomdrive-control/src/want.rs
crates/boomdrive-control/src/report.rs
crates/boomdrive/tests/forge_tokens.rs
crates/boomdrive-hosted/Cargo.toml
crates/boomdrive-hosted/protocol/README.md
crates/boomdrive-hosted/protocol/v3/
crates/boomdrive-hosted/src/
crates/boomdrive-hosted/tests/
openspec/changes/the-hosted-protocol-carries-a-hold/
```
