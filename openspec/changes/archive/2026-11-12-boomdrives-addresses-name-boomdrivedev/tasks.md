# Tasks

A task is true once the check it names passes, run as it names it.

- Every address this change writes is `boomdrivedev/<repo>` at the same tag or revision as before, and no source's hash moves.
- What stays is the proposal's "What stays", and no task changes it.

## 1. The set's inputs

- [x] 1.1 The flake's sluice, Flume and crew inputs and the lock's owners name `boomdrivedev`, and the weir crates' git source in `Cargo.toml` and `Cargo.lock` does too. `nix/tests/packages.nix`'s cases that move a tag or weir's revision name the new addresses. True when `sluice exec main -- moon run nix:test` passes.
- [x] 1.2 `site/package.json` takes `github:boomdrivedev/site-theme#v0.1.3`, and `site/bun.lock` is bun 1.4.2's own install of it. True when `sluice exec main -- moon run site:build` passes.

## 2. Boomdrive's own addresses

- [x] 2.1 setup names `github:boomdrivedev/boomdrive#herdr-plugins` for missing plugins and `github:boomdrivedev/boomdrive#herdr` for a missing herdr. True when `devenv shell -- cargo test -p boomdrive --test setup` passes, proving the setup delta's "A plugin missing or refused is named, and setup goes on".
- [x] 2.2 crew's setup names `github:boomdrivedev/boomdrive#crew` for a missing crew. True when `devenv shell -- cargo test -p boomdrive-crew --test setup` passes, proving the setup delta's "Each program setup runs is found before it runs one".
- [x] 2.3 README names `github:boomdrivedev/boomdrive` and the quick start at `https://boomdrivedev.github.io/boomdrive/start/`, and `nix/tests/cache.nix` reads it there. True when `devenv shell -- cargo test -p boomdrive --test readme` and `sluice exec main -- moon run nix:test` pass, proving the service-modules delta's "The README's words".
- [x] 2.4 The site is built for `https://boomdrivedev.github.io`, its repository link is `https://github.com/boomdrivedev/boomdrive`, and the quick start's files name `github:boomdrivedev/boomdrive`. True when `sluice exec main -- moon run site:test nix:test` passes, proving the site delta's "boomdrive-web's name is gone" and the quick-start spec's cases as they stand.
- [x] 2.5 `cliff.toml`'s commit and issue links and the packages' homepages name `boomdrivedev/boomdrive`. True when `sluice exec main -- moon run root:releases` passes.

## 3. The specs

- [x] 3.1 The change's delta archives into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive boomdrives-addresses-name-boomdrivedev --yes`, `sluice exec main -- moon run root:specs` passes.
