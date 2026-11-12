---
title: Switch to it
at: { nixos: 3 }
files:
  - path: nixos/switch.sh
test: { project: nix, name: "quick start: NixOS's switch" }
---

Build the configuration your flake gives and switch to it.

It leaves the system switched, the set installed and the runner running as your user's service.
