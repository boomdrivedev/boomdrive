# Tasks

## 1. The workflow

- [x] 1.1 `cache.yml`'s push names tags `v*` alone, beside `workflow_dispatch`, and its header says it is built once for each release (record 0020 as amended on 2026-11-19). True when `sluice exec main -- moon run nix:test nix:lint` passes, proving "A branch's push".
- [x] 1.2 Record 0020's amendment, the README and `CLAUDE.md` say the cache is built at each release's tag alone. True when `sluice exec main -- moon run root:format` passes.

## 2. The specs

- [x] 2.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
