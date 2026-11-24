# Tasks

## 1. The socket

- [x] 1.1 `flume_host` finds flume host's socket by flume's rule, says the hello, takes protocol 1 alone, asks `status` and `subscribe`, and reads each document as Flume's status, a refusal as Flume's refusal (Decision 2). True when `sluice exec main -- moon run boomdrive:test` passes, proving "The socket of a scratch home", "Another protocol" and "A refusal in the document".

## 2. The subscription

- [x] 2.1 While the runner carries parts it holds one subscription, each pass reads its latest document, and a changed document wakes every part; `--once` asks once for each read (Decisions 1, 3 and 4). True when `sluice exec main -- moon run boomdrive:test` passes, proving "Passes read the subscription", "A change heard wakes the parts" and "A runner that carries no part".
- [x] 2.2 Where no flume host answers, status is unread, the subscription is tried again on its backoff, and the runner says each way once (Decision 5). True when `sluice exec main -- moon run boomdrive:test` passes, proving "flume host stops and comes back".
- [x] 2.3 The crate's runner-level tests run over a stub flume host (Decision 7), and no source of the runner's names a listener (core.3). True when `sluice exec main -- moon run boomdrive:lint boomdrive:test` passes.

## 3. The words

- [x] 3.1 `README.md` says how the runner reads Flume's status, and core.13's details name the subscription. True when `sluice exec main -- moon run root:format` passes.
- [x] 3.2 The decision `the-runner-hears-flume-host-by-one-subscription` is proposed to Boom, linted with 0 hard findings.

## 4. The specs

- [x] 4.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
