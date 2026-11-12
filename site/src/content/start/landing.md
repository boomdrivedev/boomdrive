---
title: Land main through sluice
at: { mac: 5, nixos: 6 }
files:
  - path: both/sluice.toml
    writes: .config/sluice.toml
  - path: both/flume.toml
    writes: .config/flume.toml
  - path: both/land.sh
test: { project: boomdrive, name: sluices_file_lands_on_main }
---

In a clone of your repository, write sluice's policy and flume's binding of sluice, then commit them and push them to `main`. The policy lands `main` once one command passes: put your own tests' command in place of `true`. The binding tells flume to land the drive's work through sluice.

The quick start's first change is a fix, which this binding lands. A drive that takes crew's changes binds `change = "openspec"` in the same file too, in a repository that holds OpenSpec's `openspec/` folder.

It leaves `main` landing through sluice.
