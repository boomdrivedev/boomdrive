---
title: Take the set in your flake
at: { nixos: 1 }
files:
  - path: nixos/flake.nix
    writes: /etc/nixos/flake.nix
test: { project: nix, name: "quick start: NixOS's flake" }
---

Your machine's flake takes Boomdrive's flake as an input. It gives one configuration, `your-host`, from your own `configuration.nix`, the set's NixOS module and the drive's file of the next step.

It leaves your flake taking the set.
