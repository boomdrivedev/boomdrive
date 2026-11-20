# Tasks

## 1. The gate

- [x] 1.1 `.config/sluice.toml`'s executor allows the network (Decision 4). True when `sluice exec main -- moon run root:format` passes on the policy file.

## 2. The file

- [ ] 2.1 `protocol/spoken.json` names versions 1 to 4, version 4 alone spoken, and `boomdrive_hosted::protocol` reads it, the upgrade offering what it marks spoken (Decisions 1 and 2). True when `devenv shell -- cargo test -p boomdrive-hosted --test spoken` passes, proving "The file names each version directory" and "The runner offers what the file marks spoken".

## 3. The check

- [ ] 3.1 `boomdrive-hosted:cloud-protocol` reads the cloud's main's pin and refuses a tree sharing no spoken version with it, and `boomdrive-hosted:cloud-protocol-checks` runs it on scratch repositories (Decision 3). True when `sluice exec main -- moon run boomdrive-hosted:cloud-protocol-checks boomdrive-hosted:cloud-protocol` passes, proving "A version shared passes", "No version shared is refused" and "A pin that cannot be read is refused".

## 4. The specs

- [ ] 4.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
