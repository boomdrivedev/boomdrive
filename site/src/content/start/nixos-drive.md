---
title: Declare the drive
at: { nixos: 2 }
files:
  - path: nixos/boomdrive.nix
    writes: /etc/nixos/boomdrive.nix
test: { project: nix, name: "quick start: NixOS's drive" }
---

The drive's file trusts the family's cache and installs git, gh and Node.js. It declares the account `me`'s drive `home` keeping your repository, with crew's partition `home` and its state repository. The set's module gives flume its host's keys, the drive's stream and crew's roles from it.

It leaves the drive in the runner's, crew's and flume's files.
