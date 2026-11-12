---
title: Start the drive's agents
at: { mac: 6, nixos: 7 }
files:
  - path: both/start.sh
test: { project: boomdrive, name: the_drives_agents_start_as_setup_says }
---

Start the drive's agents, with the command setup gives for the drive `home`.

It leaves the drive's agents running, ready for what you ask.
