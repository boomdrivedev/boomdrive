---
number: 15
title: The runner's protocol is one closed JSON Schema per version, kept with the hosted implementation and checked at both ends
status: accepted
date: 2026-11-02
decision-makers:
- Chuck Swanberg
---

# The runner's protocol is one closed JSON Schema per version, kept with the hosted implementation and checked at both ends

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-02 by the construct stage of the change `the-runner-keeps-its-protocol-as-one-json-schema-per-version`, as the choices that change makes on rules already written.

Deciders: the construct stage, on the written rules; the user, at the change's review.
Sources: agentplot/boomdrive-cloud's records 0016, 0022, 0049 and 0056 and its rules core.18, core.36, core.38, core.106 and core.108, cited by id; its roadmap's item 1.16 and ruling D5; its research rows S6, F8, F50 and F54 (`design/research/awesome-lists-disposition.md`); records 0004, 0005, 0012 and 0014; rules core.2, core.11, core.26, core.27 and core.30; the change's design, Decisions 1 to 13.

## Context and problem statement

boomdrive-cloud's record 0056 puts the runner protocol's JSON Schema in this repository, one set of files per version, beside the `ControlPlane` interface whose shapes its messages carry, and has the cloud pin it by revision and check every runner message against it before any method sees it. Its rule core.36 accepts the current version and the one before, and makes a breaking change a new version. Its record 0049 gives the schema no field for the text the cloud does not keep. Its record 0016 puts a terminal's key exchange in the first version.

This repository's rules say what the schema must serve without saying what it is. core.2 keeps the hosted protocol out of every crate but an implementation's. core.27 carries to a hosted control plane only the fields the protocol's copy of a tool's status names. core.26 keeps each partition's work to its own connection. core.30 lets a terminal leave the host only encrypted to the browser. No rule says where the schema lives in the tree, how tight it is, who checks it, or when a version stops changing.

## Considered options

For where it lives:

1. In the hosted implementation's own crate, `crates/boomdrive-hosted`, made now to hold the protocol, the socket coming with the change that builds the implementation.
2. In the interface's crate, `crates/boomdrive-control`.
3. In a crate of the protocol alone, which the hosted implementation depends on.
4. At the tree's root, in a `protocol/` directory no crate owns.

For how tight it is: every object closed, so a message holds only what its version names; or objects open, so a newer sender's extra fields pass an older receiver.

For who checks it: the cloud's Worker alone, on what the runner sends; or both ends, each on what it is sent.

For the runner's types: serde types written by hand and tested against the schema with the `jsonschema` crate (F50); types generated from the schema by typify (F50); or the schema generated from the types by schemars.

## Decision outcome

Option 1, closed, both ends, and serde types tested against the schema. core.42 holds it.

- **In the hosted implementation's crate.** Version 1 is `crates/boomdrive-hosted/protocol/v1/`, beside the interface's crate, whose types the messages carry. The crate's tests are the schema's checks. Option 2 would put the hosted protocol in the interface's crate, and option 3 in a crate that is no implementation, each against core.2. Option 4 leaves the files to no crate's checks (core.7).
- **Closed.** Every object the schema names takes only the fields it names, at every depth, so the cloud's check refuses a field the version does not name. That is how core.27's allowed list and the cloud's core.108 hold at the check, not only in the runner's code. Open objects would let a leaked field pass. The price is that any added field is a new version.
- **Both ends.** The cloud's Worker checks what a runner sends (cloud core.36). The runner checks what its control plane sends against the same version before reading it into the interface's types, and refuses a failing message whole, as the Worker does.
- **Fixed once spoken.** A version may change until a release of the runner speaks it to a control plane. From that release it never changes, and its directory is never removed, so the cloud's pin can hold the current version and the one before. A change after that is a new version beside it.
- **Serde types tested against the schema.** The runner's message types are serde types in `boomdrive-hosted`, reusing the interface's own wherever the shape is the same. Tests check every value they write, and every example the version holds, against the schema with the `jsonschema` crate. typify would generate a second model of the interface's shapes under the schema's names (core.12). schemars would make every edit of a type an unreviewed edit of the public contract.

### Consequences

- `crates/boomdrive-hosted` exists before it implements `ControlPlane`, and `type = "hosted"` stays refused until the change that builds it (the control-plane-interface spec).
- Health, forge tokens, enrollment and the proof of the host's key, the runner's own commands and a stray place's drop are not in version 1 as this change writes it. Each change that builds one adds its messages to version 1 while no release speaks it, or else makes version 2.
- A Flume or sluice field the copies do not name never reaches the cloud, whatever the tool adds. A field the cloud then wants is a new version.
- The cloud's change 1.6 pins the version's directory and builds its validators from those files alone.
