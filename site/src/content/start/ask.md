---
title: Ask for a change and watch it land
at: { mac: 7, nixos: 8 }
files:
  - path: both/watch.sh
test: { project: boomdrive, name: a_change_the_drive_is_given_lands }
---

Tell the drive's agents the change you want, and approve its plan on your rail. Then open the drive's stream and watch its agents build the change and land it.

It leaves the change on your repository's `main`.
