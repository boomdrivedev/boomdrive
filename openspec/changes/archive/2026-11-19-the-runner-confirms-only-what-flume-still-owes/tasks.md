# Tasks

## 1. The confirm

- [x] 1.1 `Work::confirm` gathers each launch's keys owed in the status its control plane took, and, where any are, reads Flume's listing once and confirms only the keys it still gives pending for `boomdrive` (Decision 1). True when `sluice exec main -- moon run boomdrive:test` passes, proving "A record Flume forgot before the confirm" and "Flume's listing unread at the confirm", and every confirm case before.

## 2. The specs

- [x] 2.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
