# Tasks

## 1. Version 7

- [x] 1.1 `protocol/v7/` is version 6 and `repository_statuses`, with its examples (Decision 1). True when `devenv shell -- cargo test -p boomdrive-hosted --test files --test examples --test refused` passes. This proves "Version 7 is version 6 but for the repositories' statuses" and "Version 7's examples are version 6's". It also proves "Version 7's examples read" and "A status of another form".

## 2. The hosted implementation

- [x] 2.1 The types move to `protocol::v7`, and `protocol::V7` joins `speakable()`. `spoken.json` names version 7, not spoken (Decision 5). True when `devenv shell -- cargo test -p boomdrive-hosted --test spoken` passes. This proves "The file names each version directory" and "The runner speaks what the file marks spoken". It also proves "Each version the code speaks is its own files".
- [x] 2.2 The plane cuts each repository's status for version 7, and each partition's hello takes its own (Decision 4). True when `devenv shell -- cargo test -p boomdrive-hosted --test plane --test connection` passes with version 7 marked spoken. This proves "Each repository's status goes up in its own partition's hello" and "A repository's status whose copy fails goes up missing". It also proves "A hello of version 7 names its repositories' statuses".
- [x] 2.3 The suite passes with either mark (Decision 5). True when `devenv shell -- cargo test -p boomdrive-hosted` passes with `spoken.json` marking version 6 spoken, and again with it marking version 7 spoken.

## 3. The interface and the runner

- [x] 3.1 `Hello.repository_statuses` holds each kept repository's status (Decision 3). True when `devenv shell -- cargo test -p boomdrive-control --test json` passes, proving "A hello naming a repository's status".
- [x] 3.2 The subscription and the status ask name the kept clones' folders (Decision 2). True when `devenv shell -- cargo test -p boomdrive --test flume_host --test control` passes, proving "The asks name the folders read" and "Passes read the subscription".
- [x] 3.3 The hello takes Flume's item for each kept repository (Decision 3). True when `devenv shell -- cargo test -p boomdrive --test offers` passes, proving "Each kept repository's status, as Flume gave it" and "A status Flume refused gives no repository's status".

## 4. The set

- [x] 4.1 The set takes Flume v0.37.0 (Decision 6). True when `flake.lock`'s flume node names `132253d27286a73ac27e7839a8235b676cf6dcb7`, the commit of `v0.37.0`, and `sluice exec main -- moon run nix:test` passes.

## 5. The gate

- [x] 5.1 The checks of each project changed pass. True when `sluice exec main -- moon run` passes the format, lint, typecheck and test tasks of `boomdrive`, `boomdrive-control`, `boomdrive-crew`, `boomdrive-hosted` and `nix`, and `boomdrive-hosted:cloud-protocol`.
- [x] 5.2 The change passes OpenSpec's strict check and archives into specs that pass it (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
