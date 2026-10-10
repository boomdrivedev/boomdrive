# Site design: outline and content model

A handbook with a lab notebook attached: a versioned Method kept current,
dated field notes that age honestly, and live fleet numbers, all as typed
content in git and built with Astro.

## Decisions

Plain Astro with content as Markdown and YAML in git, hosted on Cloudflare.

- **The site is Flywheel.** The name of the Method's home and of the fleet it
  measures. MadSwan is the company and stays on the commercial side, and a
  personal name would make this a blog (`adr/0001-the-site-is-flywheel.md`).
  The domain is still open.
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

## The arc: scaling agents

The site's spine is the journey from one agent to a thousand: 1, 5, 20, 50,
100, 500, 1,000. The Method is the mental model for working at each step,
before it is any technology, and "every day the same" is what lets a fleet
take the next step without the day changing shape. The journey appears in
two places: Home shows it with where the fleet stands today, read from the
latest fleet-utilization point, and the statement opens the Method with it.
Nowhere does the site promise when a step comes.

## Site map

Eight sections; version 1 launches with Home, the Method, Notes, Tools and
About, and the first page of the Fleet.

| Section | What it holds | How it ages | In version 1 |
|---|---|---|---|
| Home | The animated flywheel beside the principles, one line each, under the line "Every day the same, for developers and agents"; the journey to a thousand agents with where the fleet stands today; the latest note; signup | Changes with each note and each fleet count | Yes |
| The Method | The statement and one page per principle, each showing how we do it today and where it is going (`method-outline.md`); read one principle at a time as the reader scrolls, each beside an animated diagram of its idea | Living: versioned, last-reviewed date, changelog | Yes |
| Notes | Dated field notes, filterable by principle and series | Snapshots: never rewritten, superseded with a banner | Yes |
| The Fleet | A page per measurement: its definition and version, its chart across fleet contexts, and the notes that cite it; the current fleet context | Always current, rebuilt from data | Fleet utilization's page; the rest as measurements arrive |
| Uses | The stack we run, by category: harnesses, multiplexer and plugins, worktrees, secrets, toolchain, our own tools | Current; each context records past stacks | Month 2 |
| Roadmap | Everything we know is coming, by status from idea to released | Current | Month 2 |
| Tools | Crew, and the shell, moon and hook configurations we publish | Updated per release | Yes |
| About | Who we are, learning in public, how we treat client work | Rarely | Yes |

## How Home and the Method read

- **Home opens on the flywheel.** The hero is the animated four-ring
  flywheel from agentplot/flywheel's site (`site/index.html`: human review
  on the outside, then intent, then bolts, construction at the core, each
  ring's ticks turning at its own rate), with the line "Every day the same,
  for developers and agents". Beside it, the principles, one line each in
  the Method's order, each a link to its page: two columns on a wide
  screen, the flywheel above the list on a phone.
- **The journey is on Home.** A strip of marks, 1 · 5 · 20 · 50 · 100 ·
  500 · 1,000 agents, with the mark the fleet has reached lit from the
  latest fleet-utilization point and the peak so far named. It says where
  we are on the way, never when the next mark comes.
- **The Method reads one principle at a time.** Scrolling the Method moves
  through the principles in order, each taking the screen in turn: the
  write-up (the claim, what it rules out, today, where it is going, what
  measures it) beside an animated diagram of the idea, in the manner of
  sluice's merge-queue diagram (`agentplot/sluice docs/diagrams/sluice.svg`),
  which is the diagram for "Order at landing, never at start". A principle
  without a diagram yet shows its write-up alone; diagrams arrive one per
  principle.
- **Motion never hides content.** Every diagram honours reduced motion. With
  scripts off, Home is the flywheel, the line, the list and the journey, and
  the Method is the principles in order; nothing is reachable only by
  scrolling or clicking.

## Content model

Principles are the living truth, notes are dated evidence, and measurements
connect them; every arrow is a typed reference the build checks.

```
Note ──cites──▶ Measurement ──has──▶ its page in The Fleet
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
| Measurement | id, name, definition, definitionVersion, principle, collector, series (name, unit, points (date, value, context)) | One definition, versioned once for all its series, cited by notes and charted on its own page (`measurement-roadmap.md`) |
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
   (`herdr agent list`), and the host's footprint beside them: CPU and
   memory in use and the number of processes, since agents run tests, dev
   servers and containers. Raw samples stay private on the host.
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

## A measurement page

Each measurement has one page in The Fleet, at its id. It shows:

- **The latest reading as the headline**, with when it was taken, one tile
  per series.
- **The chart:** every series over time, with the fleet context each point
  was true under marked along the time axis, so a change of stack is
  visible where it happened. A reader can pick a point and read its value,
  date and context, and switch between a series' own unit and its share
  where a share means something (busy agents of running agents; a stage's
  hours of a unit's).
- **The definition beside the numbers:** the name, the definition in words,
  its version and when each version began, the principle it serves and the
  collector that produces it.
- **The notes that cite it**, newest first.
- **What it cannot show**, in a sentence, as the audience plan's honest
  numbers ask.
- A page with no points yet shows the definition and says the first points
  are coming; a number is never invented.

Fleet utilization is the first page, and the one Home reads: agents running
and how many of them are busy, like cores on a machine, with the host's CPU,
memory and process count beside them (`measurement-roadmap.md`).

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
- The domain for Flywheel?
- Is a log a separate type from a field note, or a kind of note?
