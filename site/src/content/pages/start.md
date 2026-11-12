---
title: Quick start
description: From nothing of the family installed to a drive whose agents build and land a change, on an Apple silicon Mac or a NixOS machine.
---

These steps take you from nothing of the Boomdrive family installed to a working drive. Its agents build a change you ask for and land it on your repository's main branch. Each step's text is the files below it, and a test of the repository's gate runs each file as it stands.

Bring these:

- an Apple silicon Mac with [Nix](https://nixos.org/download/) and its [flakes](https://nix.dev/concepts/flakes) on, or a [NixOS](https://nixos.org/manual/nixos/stable/) machine whose configuration is a flake;
- a [GitHub account](https://github.com/signup);
- [Claude Code](https://docs.claude.com/en/docs/claude-code/setup).

The steps name what is yours with placeholders. `your-github-name` is your GitHub account, and `your-repository` the repository the drive works on. `your-github-name/drive-state` holds the drive's state. `your-host` and `you` are a NixOS machine and its user, and `me` and `home` the account and its drive. Put your own names in their place wherever they appear.
