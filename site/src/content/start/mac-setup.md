---
title: Set up the drive
at: { mac: 4 }
files:
  - path: mac/setup.sh
test: { project: boomdrive, name: setup_sets_up_the_drive_as_written }
---

One `boomdrive setup` sets up the account `me`'s drive `home`, keeping your repository, with crew as its control plane and `your-github-name/drive-state` as its state repository, which setup makes where it is missing. Every other answer is setup's own.

It leaves the drive set up and the runner running as your login's service. After a later `nix profile upgrade`, run `launchctl kickstart -k gui/$(id -u)/io.github.agentplot.boomdrive` so the runner starts again on the new set.
