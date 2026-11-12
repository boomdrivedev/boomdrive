---
title: Declare the drive
at: { nixos: 2 }
files:
  - path: nixos/boomdrive.nix
    writes: /etc/nixos/boomdrive.nix
test: { project: nix, name: "quick start: NixOS's drive" }
---

The drive's file trusts the family's cache and installs git, gh and Node.js. It declares the account `me`'s drive `home` keeping your repository, with crew's drive `home`, its host and its state repository, as setup gives them on a Mac. The set's module gives flume its host's keys, the drive's stream and crew's roles from it.

It leaves the drive in the runner's, crew's and flume's files.
