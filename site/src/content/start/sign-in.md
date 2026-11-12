---
title: Sign in
at: { mac: 2, nixos: 4 }
files:
  - path: both/sign-in.sh
test: { project: boomdrive, name: the_sign_ins_are_the_ones_setup_names }
---

Sign in to GitHub, Claude Code and Pending You, one command each, in the order setup checks them. Each is the command setup names when it finds you signed out.

It leaves you signed in to all three, so the drive can reach your repositories and its agents can run and ask you questions.
