# Proposal

## Why

The brief, 2026-11-16: "once the runner's next control pass is due, its launches and its acts on places take turns, so a land, an answer or a hold waits behind at most one launch and a launch behind at most one act, and the next pull behind at most one of each". Sources: record 0002, its amendment of 2026-11-14 on taking turns, and rule core.50 as amended (7958f236, submitted as #215); boomdrive-ops' reading of request 267 on 2026-11-14.

`flume launch` answers once its place is made and readied and its first stage started, and readying takes from seconds to several minutes. A pass makes every launch its pull offers before it carries out any act on a place, whether or not its next pass is due. On 2026-11-14 a pass's launches ran from 15:39Z to 16:04Z and the next pass's from 16:10Z, while the user's land asked at 15:41Z waited behind both as an act on a place.

## What Changes

- **Once due, a pass makes no launch after its first.** Once the next control pass is due, by its timer or by the control plane's wake, a pass runs no `flume launch` after the first it ran, whether that first ran before it fell due or after, as it already runs no act's command on a place after its first. Before due, it makes its launches first, as now.
- **A launch the pass took and left is held.** Each offer it would launch after its first is answered `held`, with the words `waits for the next control pass, which is due`, its room counted at its turn, so every offer after it gets the answer the control plane's order gives. The next pass takes it as any held offer. Its acts are answered as a held launch's are.
- **So launches and acts take turns.** A pass due from its start runs one launch, then one act. One that falls due among its launches runs its first act next. No act waits behind more than one launch, no launch behind more than one act, and the next pull behind at most one of each. `boomdrive run --once` still makes every launch and carries out every act.
- **Once due, a send goes before each command.** A pass past due sends its report before each tool's command it runs for a want, and once it has answered its last want, wherever an answer changed it since the last send, in place of after each want whose answer changed it. So the many launches a pass holds for their turn go up together before its act runs, not one send each, and no answer waits on a command.
- **What a send lets go is taken no more, its acts with it.** Where the answer to a send lets go or refuses the want about to run, or its launch, nothing runs for it, and no want the control plane let go or refused, nor an act on a launch it let go or refused, is taken more that pass.
- **A preview waits on no launch held for its turn.** Its command runs once every offer of its pull is answered, a launch held for its turn among them.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: once due, a pass makes no launch after its first, holds the rest for their turn, and so takes turns between its launches and its acts; a pass past due sends before each command; what a send lets go is taken no more, its acts with it.
- `previews`: a preview's command runs once every offer of its pull is answered, a launch held for its turn among them.

## Impact

- `crates/boomdrive/src/carry.rs`: the pass's count of its launches, the hold of a launch for its turn, the send before each command, and the wants a send let go.
- `crates/boomdrive/tests/turns.rs`, a new test file of the crate's: the spec's scenarios of taking turns.
- `crates/boomdrive/tests/sends.rs`: the sends' scenarios, as the send before each command has them.
- `crates/boomdrive/tests/offers.rs`: the scenario of a launch held for its turn that waited on its clone's keep, beside that file's keeper.
- `crates/boomdrive/tests/previews.rs`: the preview's scenario of a launch held for its turn.
- `README.md`: what a control pass takes once its next pass is due, and when it sends.
- `docs/adr/0002-*.md` and `docs/architecture/core.md`: the construct stage's choices are recorded in the amendment of 2026-11-14 on taking turns, and core.50's and core.55's details carry them, in this change's first commit.
- Where a launch takes longer than `poll_interval`, a pass makes one launch and carries out one act, so a long pull's launches are made over as many passes.
- Nothing else changes: no crate, no tool, no tracked file type, no message of the protocol, no control plane's implementation.

## Touches

```
crates/boomdrive/src/carry.rs
crates/boomdrive/tests/turns.rs
crates/boomdrive/tests/sends.rs
crates/boomdrive/tests/offers.rs
crates/boomdrive/tests/previews.rs
README.md
docs/architecture/core.md
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
```
