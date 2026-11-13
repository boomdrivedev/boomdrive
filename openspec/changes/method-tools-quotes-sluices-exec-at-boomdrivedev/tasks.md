# Tasks

A task is true once the check it names passes, run as it names it.

## 1. The spec

- [x] 1.1 The change's delta passes openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [ ] 1.2 The change's delta archives into specs that pass openspec's strict check, `openspec/specs/method-tools/spec.md` quoting `https://boomdrivedev.github.io/sluice/reference/commands/#sluice-exec` and no `agentplot.github.io` address. True when, after `devenv shell -- openspec archive method-tools-quotes-sluices-exec-at-boomdrivedev --yes`, `sluice exec main -- moon run root:specs` passes.
