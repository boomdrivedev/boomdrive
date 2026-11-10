# Proposal

## Why

The brief, 2026-11-09: "Version 2 of the runner's protocol with a hosted control plane carries, before it is first released, everything crew's control plane can ask of the runner: launches, acts, a work's land, a preview with its done, and a confirm, beside every end with its delivery key, so a hosted control plane asks what crew's can and no later version is needed for them." Its sources are record 0015, rule core.42 and the control-plane-interface spec.

The interface names five wants: an offer, an act, a command, an inbox and a preview. An act is a retry, a land, a confirm, a drop, a priority or a done. crew's implementation gives every one of them. Version 2 of the hosted protocol, as record 0037 wrote it, names every one but two: a land among the acts, and a preview among the wants, with what the runner answered of each preview in its report. A hosted control plane can therefore land a work only by a retry naming the submission, and cannot bring up a preview at all.

Version 2 is on the bolt `the-hosted-protocol-carries-every-end`, and no release has spoken it: the newest tag, `v0.7.3`, holds `protocol/v1/` alone. So version 2 may still change (core.42). Once the bolt lands on `main`, a release follows (record 0024) and version 2 is fixed. A land and a preview would then wait for a version 3. Written now, they go up with version 2's first release.

## What Changes

- **Record 0037 is amended**: version 2 carries a land and a preview, with what the runner answered of each preview. core.27's, core.42's, core.53's and core.55's details read with it. Both are written with this change.
- **Version 2, at `crates/boomdrive-hosted/protocol/v2/`, gains:**
  - a land among the acts, `"land"`, as the interface writes `ActSort::Land`;
  - a preview among the wants, `{"preview":{"id","lease","repository","branch","revision","done"}}`, as the interface writes `Want::Preview`, its branch and revision any string;
  - `previews` in every report, each preview the runner answered by its id: its lease, repository, revision, whether it was done, its outcome, the copy of sluice's answer, its words, the copy of sluice's listing of it as its status, and its time;
  - a new file, `preview.schema.json`, holding the copy of a preview, its `name`, `branch`, `revision`, `state`, `made` and `addresses`, and the copy of a release, its `preview` and `released`.
- **The hosted implementation carries them.** A land and a preview reach the runner as any want, under the partition's prefix. A preview naming a repository the partition is not served is withheld, as an offer is. What the runner answered of each preview goes up with sluice's answer and listing cut to their copies and checked, and a copy that fails goes up `null`. A report the control plane hands back holds its previews.
- **Every sort the interface names passes version 2**, which a test holds by a match with no wildcard, so a sort added to the interface fails to build that test until a version names it.
- **Version 1 stays as a release spoke it.** It carries no land, no preview and no confirm.
- **The protocol's README, the root README, and `boomdrive-control`'s documentation** say version 2 carries a land and a preview.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `runner-protocol`: version 2 carries a land, a preview and what the runner answered of each preview, every sort the interface names among its wants; the copy of a preview and of a release; the README.
- `hosted-control-plane`: a control plane's land and preview reach the runner as any want; what the runner answered of each preview goes up as its copies; a report handed back holds its previews.

## Impact

- `docs/adr/0037-…md` gains an amendment. `docs/architecture/core.md` amends the details of core.27, core.42, core.53 and core.55. Both are written with this change.
- `crates/boomdrive-hosted`:
  - `protocol/v2/`: `wants.schema.json`, `report.schema.json` and `hello.schema.json` changed, `preview.schema.json` new, every example holding a report given its `previews`, and new examples;
  - `protocol/README.md`: version 2's land, preview and previews, and how version 1 differs;
  - `src/protocol/v2.rs`, `src/copy.rs` and `src/plane.rs`;
  - `tests/`: `interface.rs`, `copies.rs`, `cut.rs`, `refused.rs`, `plane.rs` and `signatures.rs`.
- `crates/boomdrive-control/src/want.rs` and `src/report.rs`: documentation alone.
- `README.md` and `crates/boomdrive/tests/readme.rs`: the hosted section.
- No crate, tool, configuration key or field of the report file is added. The runner's own carrying of a land and of a preview does not change (core.53, core.55).
- Outside this repository: boomdrive-cloud takes version 2, with its land and preview, by a change of its own, for the orchestrator to ask of it.

## Touches

```
docs/adr/0037-the-runner-speaks-version-2-of-its-protocol-which-carries-every-end-of-a-place-with-its-delivery-and-a-control-planes-confirm.md
docs/architecture/core.md
README.md
crates/boomdrive/tests/readme.rs
crates/boomdrive-control/src/want.rs
crates/boomdrive-control/src/report.rs
crates/boomdrive-hosted/protocol/README.md
crates/boomdrive-hosted/protocol/v2/
crates/boomdrive-hosted/src/
crates/boomdrive-hosted/tests/
openspec/changes/the-hosted-protocol-carries-land-and-preview/
```
