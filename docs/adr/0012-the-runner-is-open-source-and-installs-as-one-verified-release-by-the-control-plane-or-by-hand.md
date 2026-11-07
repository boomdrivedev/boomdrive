---
number: 12
title: The runner is open source, and installs as one verified release, by the control plane or by hand
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The runner is open source, and installs as one verified release, by the control plane or by hand

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-01, from the user's words in the constitution interview. It supersedes record 0001's "It is private for now".

Amended in part by record 0023, accepted on 2026-11-06: herdr, with the family's herdr plugins, and Ghostty are installed with the set, each replaceable or dropped through the modules' options.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (`design/research/the-constitution-interview.md`), its topic on installing and upgrading; record 0001; agentplot/boomdrive-cloud `design/research/awesome-lists-disposition.md`, rows S8, F9 and F51; self_update, sigstore-rust, GitHub's artifact attestations and dist (cargo-dist), each public; agentplot/boomdrive-cloud's rule core.96, cited by id; the running spec.

## Context and problem statement

Record 0001 made this repository private for now, its binary `boomdrive`. The awesome-lists research recorded the user's ruling that the runner is to be public (S8). It proposed that an upgrade run only a release whose provenance is verified (F9), and that the install be built by dist with service units of our own (F51).

A host needs the runner, Flume and sluice. The user asked that installing be thought of for the many people who will use it, not for the operator's hosts: "We could try to be in package managers [...] It sounds like it's going to be tough to start managing [...] multiple operating systems and installing all the different packages. Would it be easier to just do `curl` installs? I'm really not sure how happy I am that you pushed me to have all of the tools as individual installs."

## Considered options

For releases: signed and served from the cloud's own domain; private GitHub releases; or public binaries only.

For installing: one `curl` install for everything; one install for each tool; or package managers with `curl` as the fallback.

For tool versions: named, with a floor; exact pins; or named only.

For provenance (F9): verified against the workflow that built the release; that and a key of ours as well; or a key of ours alone.

For the build (F51): dist with our own unit templates; dist with generated units; or a hand-written installer.

## Decision outcome

- **Open source.** In the user's words: "the runner needed to be open source [...] if people were going to use it, and that's what other systems like GitHub Actions do: they have open-source runners". S8 is kept, and record 0014 settles how the repository opens.
- **Releases verified by their provenance.** "I do think we need signed releases, though." F9 is kept. Every release of the runner, and of each tool its install brings, carries an attestation of provenance from the release workflow that built it. An install or an `upgrade` runs a release only once it has verified that provenance against the workflow's identity, and a failed check keeps the running binary (core.37).
- **One install for the host.** One `curl` install puts the host's whole set in place: the runner, and the Flume and sluice releases it was checked against, the host's tools and never a repository's pins (record 0009, as amended on 2026-11-05). Each is verified and installed together under the user's home. The service unit is written, a launchd agent on a Mac and a systemd user unit on Linux, and `boomdrive enroll` is run. An upgrade does the same again: fetched, verified, installed beside the old one, switched, and restarted by the service manager, with places running on in Flume throughout. git and the multiplexer are the host's own, found on it and never installed: "We have to assume people might already have Git and a multiplexer installed."
- **Built by dist**, F51 kept. The release workflow builds each platform's binaries and publishes the shell installer beside them, attestations included. The two service units are templates of our own, which the install fills in.
- **Or by hand.** As the user added, "in addition to a control-plane-driven installation, manual installations that I could do with my nix. Maybe we even have nix flakes that people can use to manage their configs." This repository publishes a nix flake, its packages and a module, for installing the runner and its tools, the host's and never a repository's pins, and for managing a host's own level of configuration (record 0009 and its amendment of 2026-11-05). Both routes install the same attested releases.
- **A floor for each tool.** Each release names the least version of each tool it runs with, git and the multiplexer among them, and the versions it was checked against. Below a floor, the runner takes no work that needs the tool, says so, and keeps its repositories. Above what it was checked against, the version is named in the hello and by `check`, and work goes on (core.38).

### Consequences

- Fleets of hosts managed for an enterprise are a third way in, still to be decided: "we'll need to decide how to do that."
- The runner protocol's schema has to be somewhere a public runner can build against. The cloud's record 0046 keeps it in a private package, so the cloud is asked to re-read that.
- The hosts' service units are written by the install or by the flake's module, and no longer by swancloud alone.
