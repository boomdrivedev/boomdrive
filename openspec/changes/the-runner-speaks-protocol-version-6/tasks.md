# Tasks

## 1. The mark

- [x] 1.1 `spoken.json` marks version 6 spoken, and version 5 not. True when `sluice exec main -- moon run boomdrive-hosted:test` passes, proving "The runner speaks what the file marks spoken", "The upgrade's subprotocol" and "A newest release is said once on each connection".

## 2. The gate

- [x] 2.1 The crate's checks and the cloud's lead pass. True when `sluice exec main -- moon run boomdrive-hosted:format boomdrive-hosted:lint boomdrive-hosted:typecheck boomdrive-hosted:test boomdrive-hosted:cloud-protocol boomdrive-hosted:cloud-protocol-checks` passes.

## 3. The specs

- [x] 3.1 The change passes OpenSpec's strict check and archives into specs that pass it (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
