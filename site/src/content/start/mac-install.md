---
title: Install the set
at: { mac: 1 }
files:
  - path: mac/install.sh
test: { project: nix, name: "quick start: the Mac's install" }
---

One `nix profile add` installs the runner, flume, sluice, herdr and its plugins, and crew, all from Boomdrive's flake, with git, gh and Node.js from nixpkgs. The runner needs git 2.48 or later, which a Mac's own git may not be. gh signs you in to GitHub and makes the drive's state repository. Node.js runs Pending You's setup.

Nix asks whether to take the flake's settings, which name the family's cache: answer `y`. Where your Nix does not trust you to add a cache, it builds the same programs from source instead, which takes longer and gives the same result.

It leaves the set, git, gh and Node.js in your profile.
