# Design

## Decision 1: millpond's package with its git crates by hash

millpond's flake builds with `cargoLock.allowBuiltinFetchGit`, which fetches its weir crates at evaluation; the gate evaluates the set against no store and no network, so evaluation fails there. The set takes millpond's flake's package and replaces its `cargoDeps` with `importCargoLock` over the same `Cargo.lock`, its git crates fetched by hashes held here, as the runner's own are (git-crates.nix). The build is otherwise millpond's. A test holds each hash's revision to millpond's `Cargo.lock`, so a move of millpond's tag that moves weir fails until the hashes move with it. The lasting fix is millpond's flake naming its hashes, which is millpond's to take.

## Decision 2: One place names the inboxes

`services.boomdrive.millpond.settings.inbox` names each inbox, and the module gives the runner's `inboxes` their names, so the runner's file and millpond's never disagree. A host that sets the runner's `inboxes` itself wins.

## Decision 3: Linked, as the runner's own file is

millpond's host configuration is rendered at evaluation by the runner's own TOML renderer, with a header of its own, and linked at millpond's own path at each activation by `activate.sh millpond`, which moves a file standing there aside once, as `link` does the runner's.
