# Tasks

## 1. The gate

- [x] 1.1 `.config/sluice.toml`'s executor allows the network (Decision 4). True when `sluice exec main -- moon run root:format` passes on the policy file.

## 2. The file

- [x] 2.1 `protocol/spoken.json` names versions 1 to 5, version 5 alone spoken, and the crate's test holds it to the directories and to the version the code speaks (Decisions 1 and 2). True when `devenv shell -- cargo test -p boomdrive-hosted --test spoken` passes, proving "The file names each version directory" and "The runner speaks what the file marks spoken".

## 3. The check

- [x] 3.1 `boomdrive-hosted:cloud-protocol` reads the cloud's main's pin and refuses a runner that offers a version it does not speak, and `boomdrive-hosted:cloud-protocol-checks` runs it on scratch repositories (Decision 3). True when `sluice exec main -- moon run boomdrive-hosted:cloud-protocol-checks boomdrive-hosted:cloud-protocol` passes, proving "A runner whose versions the cloud speaks passes", "A runner that offers a version the cloud does not speak is refused" and "A pin that cannot be read is refused".

## 4. The specs

- [x] 4.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
