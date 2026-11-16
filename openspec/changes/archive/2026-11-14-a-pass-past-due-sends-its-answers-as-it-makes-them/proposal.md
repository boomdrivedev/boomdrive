# Proposal

## Why

The brief, 2026-11-14: deneb's runner wrote no answers and no places to `boomdrive/runner/deneb` of the drive's state repository from 14:35:07Z, only claims, while its log showed each offer launched, so crew read every launch since as held by the runner, raised no landings card and refused `crew answer`; "Make a request Flume refuses answered as refused, in Flume's words, and passed over, so the runner's answers and places for every other request are written each round as before."

The code stage found the brief's lead did not hold: each refused priority request was answered once, in Flume's words, and passed over. What held the answers was the length of one control pass, which sends its report only at its end. The pull of 14:35:29Z claimed offers 70 to 147 and launched them one after another, each `flume launch` taking from seconds to 5.6 minutes, and the pass was still launching when a restart at about 14:55Z ended it unsent. The pass after the restart launched the rest until 15:11:36Z and wrote its answers at 15:21:30Z.

boomdrive-design ruled on the stop-short: once the next control pass is due, a pass sends its report after each launch, act, command or preview it carries out (record 0002 as amended on 2026-11-14; core.50 as amended).

## What Changes

- **A pass past due sends each answer as it makes it.** Once the next control pass is due, by its timer or by the control plane's wake, a pass sends its report after each want whose answer changed it, then goes on in the order core.50 gives. A pass that ends before the next is due sends once, at its end, as before. The pass's end sends as it always did.
- **The control plane's answer to such a send is taken at once**, as at the pass's end: what it let go or refused leaves the report, and no want of it, offer, act, command, inbox or preview, is taken more that pass, so a launch whose lease was refused is not launched again from the pull in hand (core.10).
- **A send before the pass's end that fails is said once**, and leaves the pass's answers to the send at its end, so a control plane that cannot be reached is tried once more that pass, not after each answer (core.22).
- Nothing changes in what a pass takes or in what order: launches go first, and no launch is left to a later pass for the time the pass has run. A request Flume refuses is answered as before, refused in Flume's words, and passed over.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: a pass past due sends each answer as it makes it.

## Impact

- `crates/boomdrive/src/carry.rs`: the send after each answer once the pass is due, and the wants the control plane let go or refused at it.
- `crates/boomdrive/tests/sends.rs`: the spec's scenarios.
- `README.md`: when a control pass sends its report.
- `docs/architecture/core.md`: core.50's details name the failed send and the wants let go at a send as not ruled out and ruled out. `docs/adr/0002-*.md`: how the amendment of 2026-11-14 was built.
- Under crew's control plane each send is a commit to the runner's branch of the drive's state repository, so a long pass past due writes one commit for each answer it makes after the due time.
- Nothing else changes: no crate, no tool, no tracked file type, no message of the protocol, no control plane's implementation.

## Touches

```
crates/boomdrive/src/carry.rs
crates/boomdrive/tests/sends.rs
README.md
docs/architecture/core.md
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
```
