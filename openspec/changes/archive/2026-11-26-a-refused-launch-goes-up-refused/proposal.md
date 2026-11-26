# Proposal

## Why

On 2026-11-26, a hosted control plane offered a `flume/fix@1` launch to a host that ran boomdrive 0.29.0. Flume refused the launch: the kind lands, and the repository binds no merge queue. The runner kept the launch refused, with Flume's words, and said so in its log. The control plane still showed the launch as offered, with nothing on its rail.

The runner's half works. The runner keeps a launch that Flume refuses as `refused`, with Flume's words, and its next report holds that answer. But the control plane answers each report and applies nothing of it. So a refusal never reaches the launch. The control plane's half is a change of its own.

No test proves the runner's whole path. The work-into-flume spec's "Flume refuses" proves the answer in the report file. The hosted-control-plane spec proves what goes up of a preview, but not of a launch's answer. A change to the hosted implementation could drop a launch's answer, and the gate would not see it.

## What Changes

- **A test proves the whole path.** One control pass runs over the stub `flume`, which refuses the launch, and over the hosted implementation, which reaches the hosted crate's fake control plane. The report that the pass sends must hold the launch `refused`, under its own id, with Flume's words as Flume gave them. The report must pass version 5.
- **The runner's tests use the hosted crate's fake.** The runner's crate takes the fake's module by its path, as its tests take crew's test module. It takes the fake's crates as development dependencies: `jsonschema`, `tokio`, `tokio-tungstenite` and `futures-util`.

The runner's code does not change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `hosted-control-plane`: a launch that Flume refuses goes up refused, with Flume's words.

## Impact

- `crates/boomdrive`: the test `hosted_offers`, and four development dependencies in `Cargo.toml`.
- No host changes a file.
