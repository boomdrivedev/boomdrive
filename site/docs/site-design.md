# Site design: outline and content model

A handbook with a lab notebook attached: a versioned Method kept current,
dated field notes that age honestly, and live fleet numbers, all as typed
content in git and built with Astro.

## Decisions

Plain Astro with content as Markdown and YAML in git, hosted on Cloudflare.

- **Content in git** because the writers are us and our agents: a note is a
  pull request, reviewed at the gate, versioned with the Method.
- **Typed content collections** give every page type a schema; a broken
  reference or a missing field fails the build.
- **EmDash considered and deferred.** It is agent-ready (a built-in MCP server
  with about 60 tools, a CLI, agent skills), but its content lives in a
  database, agents publish around the gate, and it reached 1.0 on 2026-09-28.
  Its themes are Astro projects, so moving later is realistic if an editor who
  does not use git joins. Sources:
  [GitHub](https://github.com/emdash-cms/emdash),
  [InfoQ](https://www.infoq.com/news/2026/04/cloudflare-emdash-wordpress/),
  [CMSWire on 1.0](https://www.cmswire.com/digital-experience/cloudflare-releases-emdash-1-0-cms-with-decentralized-plugin-registry/),
  [Bitdoze review](https://www.bitdoze.com/emdash-cms-review/).
- **Vale checks the prose; the schemas check the structure.** Agents' drafts
  get Vale's findings and fix them before review, so the operator's attention
  goes to the ideas. See "The house style" below.
- **Video stays on YouTube**, embedded with a click-to-load player; images are
  validated and optimized by Astro at build time.

## Site map

Eight sections; version 1 launches with Home, the Method, Notes, Tools and
About.

| Section | What it holds | How it ages | In version 1 |
|---|---|---|---|
| Home | The line "Every day the same, for developers and agents", the latest note, signup, later a strip of live numbers | Changes with each note | Yes |
| The Method | The statement and one page per principle, each showing how we do it today and where it is going (`method-outline.md`) | Living: versioned, last-reviewed date, changelog | Yes |
| Notes | Dated field notes, filterable by principle and series | Snapshots: never rewritten, superseded with a banner | Yes |
| The Fleet | Live charts of the measurements, and the current fleet context | Always current, rebuilt from data | Month 2 to 3 |
| Uses | The stack we run, by category: harnesses, multiplexer and plugins, worktrees, secrets, toolchain, our own tools | Current; each context records past stacks | Month 2 |
| Roadmap | Everything we know is coming, by status from idea to released | Current | Month 2 |
| Tools | Crew, and the shell, moon and hook configurations we publish | Updated per release | Yes |
| About | Who we are, learning in public, how we treat client work | Rarely | Yes |

## Content model

Principles are the living truth, notes are dated evidence, and measurements
connect them; every arrow is a typed reference the build checks.

```
Note ──cites──▶ Measurement ──charted on──▶ The Fleet page
  │  └─written under──▶ FleetContext ──lists──▶ Component (+ version)
  ├─informs──▶ Principle ◀──released as── MethodVersion ◀── ChangeEntry ◀─prompted by── Note
  ├─uses──▶ Term (glossary)
  ├─has──▶ Video ──part of──▶ Series ◀──part of── Note
  └─ships──▶ Release ──of──▶ Component (ours) ──implements──▶ Principle

Principle ──has──▶ Practice (today vs target) ──delivered by──▶ RoadmapItem ──▶ Component (ours) | Product
Product ──builds on──▶ Component (ours)
```

The loop over time: a note reports a finding under a fleet context and cites a
measurement; a finding that changes our view prompts a change entry; the
principle is revised in the next Method version; older notes keep the version
they were written against.

## The types

Fifteen types; the first five are needed for version 1.

| Type | Key fields | Why it exists |
|---|---|---|
| Principle | order, title, summary, status (draft, stable, retired), introducedIn, lastReviewed, related | The Method pages, kept current in place |
| Note | title (the claim), date, kind (finding, how-to, essay, incident), context, methodVersion, principles, measurements, artifacts, openQuestions, video, series, supersededBy | Dated evidence, never rewritten |
| Component | name, url, category, role, status (trialing, in use, retired), since, serves, alternatives, partOf, ours, repo, becomes | Every tool in the fleet, ours or others' (Worktrunk, secretspec, password managers, Claude Code, Herdr and its plugins, Crew) |
| Term | term, definition, principle | One vocabulary across pages and agent drafts |
| Artifact | kind (repo, gist, release, config), url, runs | The thing a reader can run, in every note |
| MethodVersion | version, date, changes | Makes "the Method as of v0.3" citable |
| ChangeEntry | principle, kind (added, revised, retired), summary, promptedBy | The changelog, and the loop from evidence to method |
| Practice | principle, asOf, today (how, components, shortcuts), target (how, delivers, status, design link) | Shows honestly how we do it now and where it is going |
| RoadmapItem | title, status (idea, designing, building, preview, released), principle, delivers, design link, lastUpdated | Everything we know is coming, without dates |
| FleetContext | id (hash of the stack), date range, agents (peak, typical), hosts, components with versions, repos | The conditions a note was true under |
| Measurement | id, name, definition, definitionVersion, unit, principle, collector, points (date, value, context) | One definition, cited by notes and charted live (`measurement-roadmap.md`) |
| Video | youtubeId, title, recorded, duration, chapters, note, series | Videos as records, embedded and indexed |
| Series | title, summary, items (ordered notes and videos) | An arc a reader can follow |
| Release | component, version, date, launchNote, highlights | Every release is a launch |
| Product | name, problem, status (idea, waitlist, beta, available), builtOn, principles | The paid side, shown only as a waitlist until it ships |

A component, for example:

```yaml
# content/components/worktrunk.yaml
name: Worktrunk
category: worktree-manager   # harness | multiplexer | multiplexer-plugin | worktree-manager
                             # | secrets | password-manager | toolchain | task-graph | model | ours
role: One worktree per agent; merge hooks run the gates
status: in-use
since: 2026-05
serves: [one-agent-one-worktree]
alternatives: [git worktree by hand]
```

A Herdr plugin is a component with `partOf: herdr`; Crew is `ours: true` with
`becomes: flywheel-next`. A secrets component never records what it holds.

## Today and where it's going

Every principle page shows two panels, how we do it today with its shortcuts
and where it is going, so ideas that are not built yet are shared honestly.

```yaml
# content/practices/never-prove-twice-2026-10.yaml
principle: never-prove-twice
asOf: 2026-10-05
today:
  how: moon's cache, shared by the worktrees of one repository; CI runs from cold
  components: [moon, worktrunk, crew]
  shortcuts:
    - A pass on one machine is invisible to every other machine, and to CI
    - Flaky checks are retried by hand, not recorded
target:
  how: A remote cache shared by every machine; agents read it, only CI writes to it
  delivers: [moon]
  status: designing
```

- **The Roadmap page** lists every target by status, never by date.
- **When a tool ships**, its practice moves from shortcut to the real thing,
  and that change is a field note.
- **What stays private:** pricing and commercial plans. Products appear only as
  a waitlist.

## How measurements are collected

A measurement is a definition plus a collector on a schedule; fleet contexts
are captured, never written by hand.

1. **Sample, every 5 minutes:** live agents, their repos and states
   (`herdr agent list`). Raw samples stay private on the host.
2. **Collect, nightly:** each collector reads its sources (Crew's plan and
   stage changes, git timestamps, moon's run logs). A repository allowlist
   means only our own repositories are read.
3. **Snapshot the context, nightly:** component versions, hosts, peak and
   typical agents. A new fleet context is made only when the stack changes,
   keyed by a hash of it.
4. **Publish, weekly before Tuesday's note:** collectors roll up data points
   and an agent opens a pull request appending them; we approve it at the
   gate, the merge rebuilds the Fleet page, and the note cites those points.
5. **Version definitions:** changing how a measurement is computed bumps its
   definition version; old points keep theirs, so charts never mix
   definitions silently.

The collectors start as a launchd job on the fleet host and become a Flywheel
job later.

```yaml
# content/contexts/fleet-2026-10.yaml
id: fleet-2026-10
from: 2026-10-12
agents: { peak: "[__]", typical: "[__]" }
hosts: 1
components: { claude-code: "[version]", herdr: "[version]", worktrunk: "[version]", crew: "[sha]", moon: "[version]" }
repos: [flywheel-next, crew, flywheel-web]
```

## The house style

[Vale](https://docs.vale.sh) lints every page in `src/content/` against
`.vale.ini` and the rules in `styles/Flywheel/`. The same Vale, pinned in
`devenv.nix`, runs in the editor, in an agent's worktree and in `site-check`.

| Rule | Level | Binds | What it flags |
|---|---|---|---|
| `Vale.Terms` (from the glossary) | error | everything | A glossary term in the wrong form ("Worktree" mid-sentence is fine; "work tree" is not) |
| `Terms` (generated) | warning | everything | A word the glossary replaces ("work tree" → worktree, "swarm" → fleet) |
| `Hype` | warning | everything | revolutionary, seamless, effortless, cutting-edge, supercharge, 10x |
| `NotXButY` | warning | everything | "It's not X, it's Y" |
| `SentenceLength` | warning | everything | Sentences over 30 words |
| `Headings` | warning | everything | Headings not in sentence case |
| `Exclamation` | warning | everything | Exclamation marks |
| `Filler` | suggestion | everything | simply, just, really, very, basically, actually, obviously |
| `Acronyms` | suggestion | everything | An acronym not spelled out on first use |
| `Absolutes` | suggestion | field notes | always, never, proves, guarantees, impossible, without a cited measurement |
| `UnsourcedNumber` | suggestion | field notes | A percentage or "3x faster" with no named measurement |

- **The glossary is the one source.** `scripts/vocab.ts` turns
  `src/content/terms/terms.yaml` into Vale's vocabulary and the `Terms`
  substitution rule; a term marked `lowercase` stays lowercase even at
  the start of a sentence.
- **Rules ratchet.** A rule starts as a suggestion or warning and is promoted
  to error once it has been quiet for a while; only errors fail `site-check`.
- **Not yet on:** Vale's spelling check, until the vocabulary has grown past
  the glossary; packaged styles (Google, Microsoft, write-good, proselint,
  alex), which need `vale sync` and are tried one at a time.
- **Structure is not Vale's job.** A required section in a note belongs in its
  schema or a remark check.

## Open questions

- Is the Method's list of principles right (`method-outline.md`)?
- Brand and domain: MadSwan, a new name, or Chuck's own name?
- Is a log a separate type from a field note, or a kind of note?
- Which measurement do we collect first? Cycle time needs only Crew's data.
