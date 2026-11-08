# Tasks

Build the groups in order. A task is true once the check it names passes, run as it names it.

- No task adds a crate, a tool, a check or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. The spec and record 0002 name nothing private

- [ ] 1.1 Every scenario of the sixteen requirements takes the scrub fix's lines, and each requirement keeps this bolt's text (design Decision 1). True when `sluice exec main -- moon run boomdrive-crew:test` passes, crew's tests proving each scenario with the names it gives.
- [ ] 1.2 Record 0002's amendment for a-launch-waits-on-no-other-act names no host (design Decision 2). True when `sluice exec main -- moon run root:format root:adrs` passes.
- [ ] 1.3 The delta archives into specs that pass openspec's strict check. True when, after `openspec archive the-crew-control-plane-scenarios-take-the-neutral-names --yes`, `sluice exec main -- moon run root:specs root:changes` passes.
