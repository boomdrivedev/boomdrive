---
number: 25
title: Boomdrive opens now, and its site moves into it from boomdrive-web
status: accepted
date: 2026-11-07
decision-makers:
- Chuck Swanberg
- the design agent
---

# Boomdrive opens now, and its site moves into it from boomdrive-web

Status: accepted by the user on 2026-11-07, on their card. Proposed the same day from their own words in the design agent's pane. Amended on 2026-11-09, below, twice: the site moves in before the opening; and Boomdrive opens in the GitHub org boomdrivedev, where the family's public tools move.

Deciders: the user, 2026-11-07, in the design agent's pane: "I was thinking it might make sense that BoomDrive Web just becomes the website under BoomDrive, and that they don't need to be two separate repos", and, told that the site can't publish from a private repository on GitHub's free plan: "We can make Boomdrive public and merge the website." The order, the names and how the site's rules come over are the design agent's, each cheap to change before the move.

Sources: records 0012 (the runner is open source), 0014 (nothing private enters the repository, and it opens with a fresh history) and 0024 (releases and their notes from the commits); agentplot/boomdrive-web `README.md`, `CLAUDE.md`, `.config/flume.toml` and `docs/adr/0002-flywheel-is-renamed-boomdrive.md`; agentplot/sluice, agentplot/flume, agentplot/timber and agentplot/boom, each keeping its site in its own `site/`; GitHub Pages, which on the Free plan publishes only from a public repository (the agentplot organization's plan is Free).

## Context and problem statement

boomdrive-web is Boomdrive's site, the Method, its field notes and the fleet's measurements, in a repository of its own, public, published to GitHub Pages at each landing on its main. Every other tool of the family keeps its site in its own repository's `site/`. This repository is private until it opens (0014), and the organization's plan publishes Pages only from a public repository, so the site can't move here while this repository stays private without going dark.

## Considered options

1. **Open now, then move the site in.** 0014's scrub lands here, the public repository starts from the scrubbed tree, and the site moves into its `site/`.
2. **Move the site in now and open later.** The site goes dark until this repository opens, or the organization pays for Pages from a private repository.
3. **Keep two repositories.** boomdrive-web stays the site, apart from the family's pattern.

## Decision outcome

Option 1, the user's decision.

- **The order.** The change that scrubs the tree to 0014's rule lands here first. The public repository starts from the scrubbed tree with a fresh history; this one is kept private as its archive. The site moves into the public repository once it exists, and boomdrive-web is archived once the site publishes from there.
- **The names.** The public repository takes `agentplot/boomdrive`, which every flake and install already names, and this one is renamed `agentplot/boomdrive-archive`. The other path, a new name for the public repository, would change every reference to keep a name for the archive.
- **The site's home and address.** The site is the repository's `site/`, its own project of the task runner with its own checks, as in the family's other repositories. It is served under the repository's name, `/boomdrive/`, and boomdrive-web's address points readers there.
- **Its history.** The site's files come with boomdrive-web's history, which is already public, so a field note's date and its first words stay readable from the repository.
- **Its rules.** boomdrive-web's working rules come with it, scoped to `site/`: nothing commercial and no client named, a field note never rewritten after it is published, the glossary as the one vocabulary, and the site's content work run as its `content` kind. Its records and specs come into this repository's.
- **The releases.** The site's commits stay out of the runner's release notes and move no version (0024).
- **Work already queued.** The repositories' MCP chores run as they stand, boomdrive-web's among them; its devenv's servers merge into this repository's when the site moves.

### Consequences

- Good: one repository for Boomdrive, its runner and its site, as every tool of the family keeps its own.
- Good: the site's address names the product, `/boomdrive/`, as the other tools' sites do.
- Bad: the site waits on the scrub and the opening. Until then it stays in boomdrive-web, published as it is.
- Bad: the runner's gate gains the site's project. Its checks run only where the site changed, as in sluice and flume.

## Amendment, 2026-11-09: the site moves in before the opening

Status: accepted on 2026-11-09 by agentplot-design, on the user's words that day in its pane: "If the Boomdrive web thing was already decided, how come it wasn't getting worked on?" The order is the design agent's own choice, cheap to change. The scrub landed on 2026-11-07, as the change `boomdrives-tree-holds-nothing-private`, and nothing put the opening to the user, so the site's move, held behind it, had no start.

- **The site moves into this repository now.** The change `boomdrive-keeps-its-site` brings boomdrive-web's tree and history under `site/` here, while this repository is still private. boomdrive-web takes no new work once that change takes its main, and keeps publishing the site as it stands until the opening.
- **The opening carries the site across by hand.** The public repository's fresh root is made from the scrubbed tree without `site/`, and `site/`'s history, filtered to that path, is merged onto it. An agent does it, in a step the opening names, with no code for the move (principle 12). The site's history is boomdrive-web's, already public, and the commits made here that touch `site/` alone.
- **The site's deploy waits for the opening.** Its Pages workflow lands with the change and fails to deploy while this repository is private on GitHub's free plan. Turning Pages on is part of the opening; boomdrive-web is archived once the site publishes from here.
- **The opening waits only on the user's word** to make the repository public, the scrub having landed.

The *Bad* consequence that the site waits on the scrub and the opening no longer holds: only its deploy waits, on the opening.

## Amendment, 2026-11-09: Boomdrive opens in the boomdrivedev org, and the family's public tools move there

Status: accepted by the user on 2026-11-09, on agentplot-design's card: the org's name, "boomdrivedev"; which repositories move, "Every public tool: boomdrive, flume, sluice, timber, weir, crew, blueprints"; and the go to publish Boomdrive there once ops' last check and secrets scan pass. Their plan came first, relayed that day by afterthought-design, once agentplot's free Actions minutes for November were spent on this repository's Mac builds: a new GitHub org for Boomdrive, "the org will be public anyway". How it is carried out is agentplot-design's own choice, cheap to change.

- **The org.** The family's public repositories live in the GitHub org `boomdrivedev`, public, on the free plan. agentplot keeps every other repository.
- **Boomdrive opens there with a fresh history.** `boomdrivedev/boomdrive` starts from this repository's scrubbed tree. Before that, ops reads the tree against core.41 again and scans for secrets what goes public: the tree, and `site/`'s history carried across from boomdrive-web, which is public already, so a secret found there is rotated. This repository stays private as the archive, under its own name, `agentplot/boomdrive`, and is not renamed. The opening waits only on the scrub. Each open bolt is replayed onto the new root with its merges, since that root's tree is this repository's main, and the site's bolt goes with them.
- **The other tools move whole.** flume, sluice, timber, weir, crew and blueprints are public, with nothing to scrub, so each is transferred to `boomdrivedev` with its history, branches and refs. GitHub redirects a transferred repository's old address for git and the web, but not its Pages site.
- **Every address moves once.** After its move, each repository's own change rewrites its `agentplot/` addresses to `boomdrivedev/`: flake inputs, its site's address, READMEs, policies and skills. The shared site theme's family list moves with a release of its own. Each host's clones, the runner's configuration and crew's kits are moved by hand (principle 12). swancloud's are swancloud's own, and it is told.
- **What the user does.** They create the org, and install on it the GitHub Apps the tools use: `madswan-agentplot`, sluice's, and `agentplot-flywheel`. They give `boomdrivedev/boomdrive` the secret its cache workflow reads, `OP_SERVICE_ACCOUNT_TOKEN`.
- **The order.** The org comes first, then the transfers, which nothing waits on. Boomdrive opens once the scrub lands, and each repository's addresses change after its own move.

Consequences:

- Good: every public tool's Actions run free, and Boomdrive's releases resume once it is public.
- Bad: each site's old address under `agentplot.github.io` stops serving once its repository moves.
- Bad: until each host's data is moved by hand, its clones and records name the old addresses, which GitHub's redirect still serves for git.

