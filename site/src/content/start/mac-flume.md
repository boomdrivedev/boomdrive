---
title: Tell flume where agents run
at: { mac: 3 }
files:
  - path: mac/flume.toml
    writes: ~/.config/flume/config.toml
  - path: mac/crew-source.toml
    writes: ~/.config/flume/sources/crew.toml
test: { project: boomdrive, name: flumes_file_opens_agents_in_herdr }
---

Write flume's two files. The first opens each agent in herdr, in a Terminal window, and runs your repository's work in the drive's stream, `home`. The second lets flume read the drive's roles from the file crew keeps, so the drive's work reaches the roles it names.

A NixOS machine gets both from the set's module. On a Mac you write them yourself, until flume has a command that sets them and setup runs it; then this step goes.

It leaves flume opening agents in herdr, in the drive's stream, where crew's roles answer.
