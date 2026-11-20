# Tasks

## 1. The threads and the channel

- [x] 1.1 `hands.rs` gives the jobs, scoped threads that send each `Done` back over one channel and wake the part, and `Underway`, what a part has in flight across its passes (Decisions 1 and 6). True when `sluice exec main -- moon run boomdrive:lint` passes.
- [x] 1.2 The pass starts each want's command and waits on none; room is taken in the control plane's order, a launch in flight holding its room; an act waits on its launch and the acts before it on that launch; previews start after the launches; `collect` and `finish` take answers between passes and at the stop (Decisions 2 to 7). True when `sluice exec main -- moon run boomdrive:test` passes, proving "A slow launch holds no other launch", "A slow launch holds no land", "A slow launch keeps its room", "An act on a launch being made", "A unit waits for its bolt's launch", "An answer goes up while another launch runs", "A later pass while a launch runs", "A land whose launch's lease is refused while it runs", "A launch in flight at the stop", "A preview runs while a launch runs" and every case before.
- [x] 1.3 A part's thread keeps its commands under way across its passes, and its bell rings when an answer comes back (Decision 2). True when `sluice exec main -- moon run boomdrive:test` passes.
- [x] 1.4 The turn-taking comes out: `Carrier::due`, `TURN` and the past-due sends, with `tests/turns.rs` (Decision 7). True when `sluice exec main -- moon run boomdrive:lint` passes.

## 2. The specs

- [x] 2.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
