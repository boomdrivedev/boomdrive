---
title: Make the state repository
at: { nixos: 5 }
files:
  - path: nixos/state.sh
test: { project: boomdrive, name: the_state_repository_is_made_as_setup_makes_it }
---

Make the private repository that holds the drive's state, as setup makes it on a Mac.

It leaves `your-github-name/drive-state` on GitHub, where crew and the runner keep the drive's records.
