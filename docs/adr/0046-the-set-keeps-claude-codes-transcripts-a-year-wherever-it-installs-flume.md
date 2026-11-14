---
number: 46
title: The set keeps Claude Code's transcripts a year wherever it installs Flume
status: accepted
date: 2026-11-14
decision-makers:
- afterthought-design
- the construct stage of change/the-set-keeps-claude-codes-transcripts-a-year
---

# The set keeps Claude Code's transcripts a year wherever it installs Flume

Status: accepted. Point 1 is afterthought-design's ruling of 2026-11-14, relayed by boomdrive-operator in the brief of the change `the-set-keeps-claude-codes-transcripts-a-year`. Points 2 to 5 are that change's construct stage's own choices, each cheap to change.

Deciders: afterthought-design, whose ruling the brief carries: "wherever the set installs Flume, Claude Code keeps every session's transcript 365 days unless the host names another span, by a document of the set's beside 10-boomdrive-telemetry.json in Claude Code's managed-settings.d holding cleanupPeriodDays alone, so a hibernated stage's session can always be resumed". The construct stage of `change/the-set-keeps-claude-codes-transcripts-a-year`, 2026-11-14, for points 2 to 5.

Sources: boomdrivedev/flume record 0069, accepted by the user on 2026-11-14, and its consequence that the host keeps transcripts; boomdrivedev/blueprints `design/flume/requirements.md` clauses 212 and 213, C.4 and S127; record 0013 as amended on 2026-11-12, the set's sessions in Claude Code's managed settings; rules core.12, core.18 and core.46; Claude Code 2.1.322's binary, read on 2026-11-14, for `cleanupPeriodDays` and its sweep; deneb writes about 2.7 GB of transcripts every 30 days, as boomdrive-operator relayed.

## Context and problem statement

Under flume's record 0069, a stage whose kind hibernates it ends its agent once its prompt cache no longer pays. A message, the answer or a caller's `flume resume` later resumes that agent's own session, carried from its transcript (flume 212, 213). flume keeps no copy: the host keeps transcripts. At each session's start, Claude Code deletes every transcript older than its home's `cleanupPeriodDays`, thirty days where the home sets none. So a stage hibernated longer starts afresh, and its conversation is lost (C.4, S127). Record 0069 names this repository for it: "the host's setup sets `cleanupPeriodDays` in every home". No home on the user's hosts sets it.

Claude Code 2.1.322 takes `cleanupPeriodDays` as a whole number of one or more, and refuses a smaller one as a settings error. Its sweep's cutoff is the merged settings' value, else thirty days. Its managed settings are merged over every user and project setting, whatever the session's home. The set already writes one document there, `10-boomdrive-telemetry.json` (record 0013 as amended on 2026-11-12).

## Considered options

1. **A document of the set's in Claude Code's managed settings, holding `cleanupPeriodDays` alone** (chosen).
2. **The key in each Claude Code home's `settings.json`,** as record 0069's words read.
3. **The key in the telemetry document.**
4. **Flume's module writing it.**

## Decision outcome

1. **The set keeps every session's transcript a year, wherever it installs Flume (afterthought-design's).** Claude Code keeps every session's transcript 365 days unless the host names another span. The set does this by a document of its own beside `10-boomdrive-telemetry.json` in Claude Code's `managed-settings.d`, holding `cleanupPeriodDays` alone, so a hibernated stage's session can always be resumed.
2. **The document.** It is `10-boomdrive-transcripts.json`, in `/Library/Application Support/ClaudeCode/managed-settings.d` on macOS and `/etc/claude-code/managed-settings.d` on Linux. It holds `{ "cleanupPeriodDays": <days> }`, a JSON number, and nothing else. The Boomdrive module writes it, as root, as it writes the telemetry document: NixOS by `/etc`, nix-darwin by the activation's root step, which removes it where it is not written. The Boomdrive module stays the one writer of the set's documents in Claude Code's managed settings.
   - Not taken: each home's `settings.json`, a Claude Code home's file, which no module of the set writes (core.46), and which a home made later misses. Managed settings reach every home at once, as record 0069's words ask.
   - Not taken: the key in the telemetry document, which holds telemetry keys alone and is written only with a collector.
   - Not taken: Flume's module writing it. Claude Code's managed settings are no file of Flume's (core.18), and two writers of one folder would be two owners of one thing.
3. **Wherever Flume is on.** The document is written where `services.flume.enable` is on in a system that imports the set's modules, with the runner or without, whatever the collector. With the runner, Flume is on by default. Hibernation is Flume's, so a host where Flume runs is one whose sessions may hibernate.
   - Not taken: with the runner, as the telemetry document is. A host that runs Flume alone through the set's modules would lose its hibernated sessions at thirty days.
   - Not taken: every system that imports the modules, which writes Claude Code's settings where nothing of the set resumes a session.
4. **The option, named as Claude Code names its key.** It is `services.boomdrive.claude-code.cleanupPeriodDays`, a whole number of one or more, `365` by default. Its name is Claude Code's key (core.46; core.12 allows another tool's field spelled as that tool spells it).
   - Not taken: a null that writes nothing, which loses a host's hibernated stages at thirty days. A host that wants Claude Code's own span names `30`.
   - Not taken: `0`, which Claude Code refuses.
   - Not taken: a duration string, where Claude Code's key is a number of days.
5. **A host's own span.** A host names its span in the option. A host's own document named after the set's in Claude Code's order, such as `20-transcripts.json`, sets the key over it. A host's own `managed-settings.json` is read first and does not.

## Consequences

- Good: a session hibernated for anything less than a year resumes with its conversation, on every host of the set that runs Flume, whatever account and home it ran under.
- Bad: a host keeps about twelve times the transcripts it keeps at thirty days: on deneb, about 33 GB at today's rate. A host short of room names a shorter span.
- Bad: Claude Code sweeps at each session's start. A shorter span, Flume turned off or a pin before this record deletes every older transcript at the next session's start.
- Neutral: Claude Code 2.1.322 ignores `desktopSessionCleanupPeriodDays` where `cleanupPeriodDays` is managed. No session of the family's runs in Claude Desktop.
- core.18's and core.46's details name the transcripts document beside the telemetry document, in the commit that writes this record. Flume's statement does not change: C.4 leaves the span to the host, and the set is how a host of the set sets it.
