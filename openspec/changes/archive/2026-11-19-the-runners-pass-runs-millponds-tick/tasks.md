# Tasks

## 1. The tick

- [x] 1.1 `millpond.rs` runs `millpond tick --inbox <name> --json` and says a refused or unanswered tick once for its inbox until millpond ticks it again (Decision 3). True when `sluice exec main -- moon run boomdrive:test` passes, proving "A refused tick said once until millpond ticks again".
- [x] 1.2 The watch ticks each inbox the configuration names after each settle, sharing them with a settle under way (Decision 2). True when `sluice exec main -- moon run boomdrive:test` passes, proving "Each pass ticks each inbox" and "No inbox, no tick".
- [x] 1.3 The configuration reads `inboxes` (Decision 1). True when `sluice exec main -- moon run boomdrive:test` passes, proving "The inboxes a host keeps" and "An inbox named twice or not named".
- [x] 1.4 core.13 names the tick beside the settle (record 0033). True when `sluice exec main -- moon run root:format` passes.

## 2. The specs

- [x] 2.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
