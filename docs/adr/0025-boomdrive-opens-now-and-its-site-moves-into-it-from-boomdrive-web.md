---
number: 25
title: Boomdrive opens, and its site moves into it from boomdrive-web
status: accepted
date: 2026-11-07
decision-makers:
- Chuck Swanberg
- the design agent
---

# Boomdrive opens, and its site moves into it from boomdrive-web

Status: accepted by the user on 2026-11-07, on their card. Proposed the same day from their own words in the design agent's pane. Amended on 2026-11-09, below: Boomdrive opens in the GitHub org boomdrivedev, where the family's public tools move. Amended on 2026-11-12, below, twice: a repository moves to boomdrivedev once its work is finished; and every repository that moves does so through madswan-agentplot, its folders moving with it.

Deciders: the user, 2026-11-07, in the design agent's pane: "I was thinking it might make sense that BoomDrive Web just becomes the website under BoomDrive, and that they don't need to be two separate repos", and, told that the site can't publish from a private repository on GitHub's free plan: "We can make Boomdrive public and merge the website." The names and how the site's rules come over are the design agent's, each cheap to change before the move.

Sources: records 0012 (the runner is open source), 0014 (nothing private enters the repository) and 0024 (releases and their notes from the commits); agentplot/boomdrive-web `README.md`, `CLAUDE.md`, `.config/flume.toml` and `docs/adr/0002-flywheel-is-renamed-boomdrive.md`; agentplot/sluice, agentplot/flume, agentplot/timber and agentplot/boom, each keeping its site in its own `site/`; GitHub Pages, which on the Free plan publishes only from a public repository (the agentplot organization's plan is Free).

## Context and problem statement

boomdrive-web is Boomdrive's site, the Method, its field notes and the fleet's measurements, in a repository of its own, public, published to GitHub Pages at each landing on its main. Every other tool of the family keeps its site in its own repository's `site/`. This repository is private until it opens (0014), and the organization's plan publishes Pages only from a public repository, so the site can't move here while this repository stays private without going dark.

## Considered options

1. **Open now, then move the site in.** 0014's scrub lands here, the public repository starts from the scrubbed tree, and the site moves into its `site/`.
2. **Move the site in now and open later.** The site goes dark until this repository opens, or the organization pays for Pages from a private repository.
3. **Keep two repositories.** boomdrive-web stays the site, apart from the family's pattern.

## Decision outcome

Option 1, the user's decision.

- **The names.** The public repository takes `agentplot/boomdrive`, which every flake and install already names, and this one is renamed `agentplot/boomdrive-archive`. The other path, a new name for the public repository, would change every reference to keep a name for the archive.
- **The site's home and address.** The site is the repository's `site/`, its own project of the task runner with its own checks, as in the family's other repositories. It is served under the repository's name, `/boomdrive/`, and boomdrive-web's address points readers there.
- **Its history.** The site's files come with boomdrive-web's history, which is already public, so a field note's date and its first words stay readable from the repository.
- **Its rules.** boomdrive-web's working rules come with it, scoped to `site/`: nothing commercial and no client named, a field note never rewritten after it is published, the glossary as the one vocabulary, and the site's content work run as its `content` kind. Its records and specs come into this repository's.
- **The releases.** The site's commits stay out of the runner's release notes and move no version (0024).
- **Its devenv.** boomdrive-web's devenv's servers merge into this repository's.

### Consequences

- Good: one repository for Boomdrive, its runner and its site, as every tool of the family keeps its own.
- Good: the site's address names the product, `/boomdrive/`, as the other tools' sites do.
- Bad: the runner's gate gains the site's project. Its checks run only where the site changed, as in sluice and flume.

## Amendment, 2026-11-09: Boomdrive opens in the boomdrivedev org, and the family's public tools move there

Status: accepted by the user on 2026-11-09, on the design agent's card: the org's name, "boomdrivedev"; which repositories move, "Every public tool: boomdrive, flume, sluice, timber, weir, crew, blueprints"; and the go to publish Boomdrive there. Their plan came first, relayed that day by another drive's design agent, once agentplot's free Actions minutes for November were spent on this repository's Mac builds: a new GitHub org for Boomdrive, "the org will be public anyway". How it is carried out is the design agent's own choice, cheap to change.

- **The org.** The family's public repositories live in the GitHub org `boomdrivedev`, public, on the free plan. agentplot keeps every other repository.
- **Boomdrive opens there with a fresh history.** `boomdrivedev/boomdrive` starts from this repository's scrubbed tree. This repository stays private as the archive, under its own name, `agentplot/boomdrive`, and is not renamed.
- **The other tools move whole.** flume, sluice, timber, weir, crew and blueprints are public, with nothing to scrub, so each is transferred to `boomdrivedev` with its history, branches and refs. GitHub redirects a transferred repository's old address for git and the web, but not its Pages site.
- **Every address moves once.** Each repository's `agentplot/` addresses become `boomdrivedev/`: flake inputs, its site's address, READMEs, policies and skills.
- **The App and the cache's secret.** The one Boomdrive App is installed on the org (record 0007 as amended on 2026-11-09), and each host's runner holds its key. `boomdrivedev/boomdrive` holds the secret its cache workflow reads, `OP_SERVICE_ACCOUNT_TOKEN`.

Consequences:

- Good: every public tool's Actions run free, and Boomdrive's releases resume once it is public.
- Bad: each site's old address under `agentplot.github.io` stops serving once its repository moves.

## Amendment, 2026-11-12: a repository moves once its work is finished, and the family's App reaches every repository

Status: accepted on 2026-11-12 by agentplot-design. It is the design agent's own choice, under the 2026-11-09 amendment's word that how the move is carried out is the design agent's, and cheap to change. It answers agentplot-planner's question that day and rests on agentplot-ops' reading of deneb.

Sources: agentplot-ops' reading of deneb, 2026-11-12. Deneb's sluice mints its tokens from App 5324750, installed on agentplot alone (installation 171576254), so it can't fetch, push or land in a repository once that repository leaves agentplot. boomdrivedev holds only the Boomdrive App, 5719968 (installation 181060998), and no host's sluice holds its key. Each repository's main and `bolt/*` are kept by repository rulesets, which move with it, and their one bypass actor is App 5324750. GitHub's page on transferring a repository says git clone, fetch and push follow the old address and says nothing of an App's installation. Records 0007 (as amended on 2026-11-09) and 0021; crew records 0060 and its decision 3; Flume's statement, C.4 (a repository named by its clone's git directory).

- **The family's App reaches every repository.** The Boomdrive App is installed on agentplot as well as on boomdrivedev, for the repositories that stay.
- **A repository moves once its work is finished.** On a host, a move is a removal and an addition (0021, crew 0060). The new address is a new clone in a new folder, which Flume reads as a new repository and sluice as a new queue. So a repository moves only once crew 0060's decision 3 finds its work finished: no live place, no bolt or fix unlanded, no open card naming it, no unanswered batch carrying its work and no open proposal naming it. New starts in it are held while it drains.
- **The kit keeps its name.** crew names a kit by the last part of its address. A move changes the address the drive names and no record of crew's, and `crew kit rename` is for a change of name alone (crew 0060).

## Amendment, 2026-11-12: every repository moves, through madswan-agentplot, its folders moving with it

Status: accepted by the user on 2026-11-12, on their card ("Approved"). Its folders and its bridge are the user's words below; how each move is carried out is the design agent's.

Sources: the user's words that day, relayed by agentplot-operator, on moving the repositories (signal `2026-11-12-agentplot-planner-c292bbdc/01`) and on their folders; afterthought-design's reading that no Boomdrive release carries the runner's forge token (#165), and that sluice finds an App's installation repository by repository (sluice `profiles/github/app.rs:483`); agentplot-ops' readings of deneb, 2026-11-12. The user installed madswan-agentplot (App 5324750) on boomdrivedev, every repository, and said "app is ready", which ops verified. On deneb every path names the owner: the runner's clone is `~/.local/share/boomdrive/repositories/agentplot/<repo>/<repo>.git`, with sluice's records inside it at `<repo>.git/sluice`. Each session folder, `~/Code/boomdrive/agentplot/<repo>`, is a worktree of that bare repository. Flume's places, crew's kits, herdr's sessions and Claude Code's project folders are keyed by those paths. afterthought's Nix writes the runner's list of repositories as `owner/repo`.

- **Every repository moves.** The repositories are the ones the 2026-11-09 amendment moves. flume, sluice, timber, weir, crew and blueprints are transferred whole. Boomdrive opens at `boomdrivedev/boomdrive` from its scrubbed tree, and `agentplot/boomdrive` stays as its archive, so for a host Boomdrive is a new repository and not a moved folder. boomdrive-web is archived, and agentplot and boomdrive-cloud stay in agentplot.
- **madswan-agentplot lands them.** It is installed on boomdrivedev beside the Boomdrive App, so deneb's sluice lands each moved repository as it does today, and each repository's rulesets name 5324750 as their one bypass.
- **A repository's folders move with it.** In the user's words: "This also changes the folder storage locally so we need to be moving local folders to keep the folder structure correct." Once a repository has moved, every path on a host that names its owner names `boomdrivedev`. Nothing reads an old path, and no code maps an old path to a new one (principle 12). The folders are moved, not cloned afresh, so sluice's records travel inside the bare repository and main's numbers carry on.
- **Every repository the user named moved, Boomdrive whole.** In the user's words to agentplot-ops on 2026-11-12: "No there's more to move... Boomdrive Cloud, strategy, Boom, Site theme, Sounding, Mill Pond, headwater, eddy. All of those move", then "Boomdrive itself too". boomdrive-cloud, strategy, boom, site-theme, sounding, millpond, headwater and eddy are on boomdrivedev beside timber, weir, flume, crew, blueprints and sluice. `agentplot/boomdrive` moved whole, history and all, still private, as `boomdrivedev/boomdrive`, Boomdrive's kit, where its work goes on landing. agentplot/agentplot stays in agentplot, with the drive's other repositories the user did not name. Where the 2026-11-09 amendment's list differs, this replaces it: boomdrive-cloud moves, and Boomdrive opens from no scrubbed root at its move.
- **Boomdrive opens to the public on the user's word, from a new root.** The opening makes a new public repository from `boomdrivedev/boomdrive`'s tree as it stands then, once that tree names nothing private (core.41; agentplot-ops counted 122 files naming private things on 2026-11-12). The moved repository never turns public, since its history names what core.41 keeps out. Which name the public repository takes is the user's at the opening: a new `boomdrivedev/boomdrive` would end GitHub's redirect, through which deneb's switch pin, `agentplot/boomdrive` at `db6f9270`, resolves.
- **The kit keeps its name.** crew names a kit by the last part of its address, so no record of crew's changes (crew 0060).

The earlier amendment of 2026-11-12 stands where this one doesn't restate it: a repository moves once drained, and the kit keeps its name.
