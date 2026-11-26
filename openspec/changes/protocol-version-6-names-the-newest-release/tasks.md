# Tasks

## 1. Version 6

- [x] 1.1 `protocol/v6/` is version 5 and `newest_release`, with its example (Decision 1). True when `devenv shell -- cargo test -p boomdrive-hosted --test files --test examples` passes, proving "Version 6 is version 5 but for the newest release", "Version 6's examples are version 5's" and "Version 6's examples read".

## 2. The code

- [x] 2.1 The types move to `protocol::v6`. `protocol::Version` and `protocol::spoken` choose the version by `spoken.json`'s mark. `spoken.json` names version 6, not spoken (Decisions 2 and 3). True when `devenv shell -- cargo test -p boomdrive-hosted --test spoken` passes. This proves "The file names each version directory" and "The runner speaks what the file marks spoken". It also proves "Each version the code speaks is its own files".
- [x] 2.2 A connection of version 6 says the newest release, and one of version 5 refuses it (Decision 4). True when `devenv shell -- cargo test -p boomdrive-hosted --test connection` passes, proving "A newest release is said once on each connection" and "Version 5 carries no newest release".
- [x] 2.3 The suite passes with either mark (Decision 3). True when `devenv shell -- cargo test -p boomdrive-hosted` passes with `spoken.json` marking version 5 spoken, and again with it marking version 6 spoken.

## 3. The gate

- [ ] 3.1 The crate's checks and the cloud's lead pass. True when `sluice exec main` passes, `boomdrive-hosted:cloud-protocol` among its tasks.

## 4. The specs

- [ ] 4.1 The change passes OpenSpec's strict check and archives into specs that pass it (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
