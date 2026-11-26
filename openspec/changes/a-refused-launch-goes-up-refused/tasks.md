# Tasks

## 1. The test

- [x] 1.1 The runner's tests take the hosted crate's fake control plane by its path, with its crates as development dependencies. True when `devenv shell -- cargo test -p boomdrive --test hosted_offers` compiles.
- [x] 1.2 One control pass over the stub `flume`, which refuses the launch, sends a report that holds the launch refused, with Flume's words. True when `devenv shell -- cargo test -p boomdrive --test hosted_offers` passes, proving "A launch Flume refuses goes up refused". The test fails where the runner does not keep Flume's refusal as the launch's answer.

## 2. The gate

- [ ] 2.1 The projects' tasks pass. True when `sluice exec main` passes.
