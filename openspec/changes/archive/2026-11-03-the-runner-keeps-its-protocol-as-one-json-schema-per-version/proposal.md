# Proposal

## Why

The runner reaches its control plane through one interface, `ControlPlane`, built by `the-runner-carries-crews-work-into-flume` (landed as #2): a pull that hands the control plane the host's hello and takes back its wants, and a report that carries the runner's report. crew's implementation is built. The hosted control plane's is named and unbuilt (that change's Decision 20), and the messages it would send were left to the change that builds it.

agentplot/flywheel-cloud has since settled where those messages are written down. Its record 0056 asks this repository for the runner protocol's JSON Schema. The cloud pins it and checks a runner's messages against it (cloud core.36, core.106). This repository is to be public (record 0012), so the schema is the runner's public contract, which both sides test against.

Two changes wait on it. The cloud's 1.6 pins the schema. This repository's 1.15 builds the hosted implementation and speaks it. Neither can start while the messages are unwritten. Some of them must be right from the first version, or a later version has to fix them:

- the terminal's key exchange (cloud record 0016; core.30);
- a place's status cut to the fields the cloud keeps (cloud record 0049, core.108; core.27);
- an id of its own on every message (cloud core.38; core.21).

This change is the cloud roadmap's item 1.16, with its research rows S6, F54 and F50.

## What Changes

- **Rule core.42 and record 0015**, in this change's commit beside its design:
  - the runner's protocol is JSON Schema in the hosted implementation's crate, one directory per version, its messages carrying the interface's shapes;
  - every object it names is closed;
  - each end checks every message it is sent against its version, and refuses a failing one whole;
  - a version a release has spoken never changes.
- **A new crate, `crates/flywheel-hosted`**, the hosted implementation's (core.2), made now to hold the protocol. It implements no `ControlPlane` yet, and `type = "hosted"` stays refused.
- **Version 1 as JSON Schema**, in `crates/flywheel-hosted/protocol/v1/`, draft 2020-12, with a README for the public:
  - **An envelope for each direction.** Every message names `protocol_version` 1, carries an `id` of its own, and holds one body.
  - **From the runner:** `hello`, `report`, `terminal_opened`, `terminal_output`, `terminal_closed` and `message_refused`.
  - **To the runner:** `pulled`, `reported`, `terminal_open`, `terminal_resume`, `terminal_input`, `terminal_close` and `message_refused`.
  - **The interface's shapes**, as its serde types write them: the hello, the wants, the leases, the launch, and the report's acts, commands and inboxes.
  - **Three places where the interface's types are not carried as written**, each forced by a rule:
    - a place's status is carried as the copy of it (core.27; cloud core.108);
    - sluice's entry is carried as an object, not a text;
    - the host's capacity is Flume's host part as values, its streams without their queues (core.26).
  - **A terminal's frames** (S6, F54). Each end's key share is signed. Each frame is sealed, and carries its stream's byte offset. A browser that reconnects says what it has seen, and a redraw from the pane answers it.
- **The runner's types for every message**, serde types in `flywheel_hosted::protocol`, reusing the interface's wherever the shape is the same. They are tested against the schema with the `jsonschema` crate (F50): every file against its draft's metaschema, every example both ways, the interface's own values against their definitions, the copies' field lists, and a refusal of each shape the version rules out.
- **treefmt** claims JSON outside the tools' fixtures with a formatter of its own.
- **The rules test** holds the new crate to core.2 and core.7: it depends on the interface alone, no crate depends on it, and it is a moon project of its own.

## Capabilities

### New Capabilities

- `runner-protocol`: the runner's protocol with a hosted control plane, version 1, as JSON Schema. It covers:
  - where each version lives, and when a version stops changing;
  - the envelope, its ids and its two directions;
  - the closed objects;
  - the interface's shapes it carries;
  - the copies of Flume's and sluice's answers;
  - the terminal's key exchange and frames;
  - what version 1 leaves out;
  - the runner's types, tested against it.

### Modified Capabilities

- `control-plane-interface`: a requirement of its own holds the protocol in the hosted implementation's crate alone, which depends on the interface alone and which no other crate depends on.

## Impact

- `docs/architecture/core.md` gains core.42, and `docs/adr/` gains record 0015, in this change's commit.
- New crate `crates/flywheel-hosted`, a member of the cargo workspace and a moon project of its own in `.moon/workspace.yml`. It holds:
  - `protocol/v1/` and its README;
  - `src/`;
  - its tests.
- `Cargo.toml` and `Cargo.lock` take the `jsonschema` crate from crates.io as a dev-dependency, with no default features, so that nothing is fetched.
- `treefmt.nix` claims `*.json` with a JSON formatter, pinned by `devenv.lock`.
- `crates/flywheel/tests/rules.rs` and `crates/flywheel/moon.yml`: the new crate in the dependency test, and its manifest among the test's inputs.
- `README.md` and `CLAUDE.md` name the new crate and the protocol's directory.
- Nothing the runner does changes. Its configuration, its report file and crew's implementation stay as they are.
- flywheel-cloud's 1.6 pins `crates/flywheel-hosted/protocol/v1/` by revision. This repository's 1.15 speaks it, and may change it until a release does.

## Touches

```
docs/architecture/core.md
docs/adr/0015-the-runners-protocol-is-one-closed-json-schema-per-version-kept-with-the-hosted-implementation-and-checked-at-both-ends.md
Cargo.toml
Cargo.lock
.moon/workspace.yml
treefmt.nix
crates/flywheel-hosted/**
crates/flywheel/tests/rules.rs
crates/flywheel/moon.yml
README.md
CLAUDE.md
```
