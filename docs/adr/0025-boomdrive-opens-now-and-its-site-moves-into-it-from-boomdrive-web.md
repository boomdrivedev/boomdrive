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

Status: accepted by the user on 2026-11-07, on their card. Proposed the same day from their own words in the design agent's pane. Amended on 2026-11-09, below, twice: the site moves in before the opening; and Boomdrive opens in the GitHub org boomdrivedev, where the family's public tools move. Amended on 2026-11-12, below, twice: a repository moves to boomdrivedev once its work is finished; and every repository that moves does so before deneb's switch, through madswan-agentplot, its folders moving with it, timber first.

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

Status: accepted on 2026-11-09 by the design agent, on the user's words that day in its pane: "If the Boomdrive web thing was already decided, how come it wasn't getting worked on?" The order is the design agent's own choice, cheap to change. The scrub landed on 2026-11-07, as the change `boomdrives-tree-holds-nothing-private`, and nothing put the opening to the user, so the site's move, held behind it, had no start.

- **The site moves into this repository now.** The change `boomdrive-keeps-its-site` brings boomdrive-web's tree and history under `site/` here, while this repository is still private. boomdrive-web takes no new work once that change takes its main, and keeps publishing the site as it stands until the opening.
- **The opening carries the site across by hand.** The public repository's fresh root is made from the scrubbed tree without `site/`, and `site/`'s history, filtered to that path, is merged onto it. An agent does it, in a step the opening names, with no code for the move (principle 12). The site's history is boomdrive-web's, already public, and the commits made here that touch `site/` alone.
- **The site's deploy waits for the opening.** Its Pages workflow lands with the change and fails to deploy while this repository is private on GitHub's free plan. Turning Pages on is part of the opening; boomdrive-web is archived once the site publishes from here.
- **The opening waits only on the user's word** to make the repository public, the scrub having landed.

The *Bad* consequence that the site waits on the scrub and the opening no longer holds: only its deploy waits, on the opening.

## Amendment, 2026-11-09: Boomdrive opens in the boomdrivedev org, and the family's public tools move there

Status: accepted by the user on 2026-11-09, on the design agent's card: the org's name, "boomdrivedev"; which repositories move, "Every public tool: boomdrive, flume, sluice, timber, weir, crew, blueprints"; and the go to publish Boomdrive there once ops' last check and secrets scan pass. Their plan came first, relayed that day by another drive's design agent, once agentplot's free Actions minutes for November were spent on this repository's Mac builds: a new GitHub org for Boomdrive, "the org will be public anyway". How it is carried out is the design agent's own choice, cheap to change.

- **The org.** The family's public repositories live in the GitHub org `boomdrivedev`, public, on the free plan. agentplot keeps every other repository.
- **Boomdrive opens there with a fresh history.** `boomdrivedev/boomdrive` starts from this repository's scrubbed tree. Before that, ops reads the tree against core.41 again and scans for secrets what goes public: the tree, and `site/`'s history carried across from boomdrive-web, which is public already, so a secret found there is rotated. This repository stays private as the archive, under its own name, `agentplot/boomdrive`, and is not renamed. The opening waits only on the scrub. Each open bolt is replayed onto the new root with its merges, since that root's tree is this repository's main, and the site's bolt goes with them.
- **The other tools move whole.** flume, sluice, timber, weir, crew and blueprints are public, with nothing to scrub, so each is transferred to `boomdrivedev` with its history, branches and refs. GitHub redirects a transferred repository's old address for git and the web, but not its Pages site.
- **Every address moves once.** After its move, each repository's own change rewrites its `agentplot/` addresses to `boomdrivedev/`: flake inputs, its site's address, READMEs, policies and skills. The shared site theme's family list moves with a release of its own. Each host's clones, the runner's configuration and crew's kits are moved by hand (principle 12). swancloud's are swancloud's own, and it is told.
- **What the user does.** They create the org, and create and install on it the one Boomdrive App (record 0007 as amended on 2026-11-09), whose key each host's runner holds. They give `boomdrivedev/boomdrive` the secret its cache workflow reads, `OP_SERVICE_ACCOUNT_TOKEN`.
- **The order.** The org comes first, then the transfers, which nothing waits on. Boomdrive opens once the scrub lands, and each repository's addresses change after its own move.

Consequences:

- Good: every public tool's Actions run free, and Boomdrive's releases resume once it is public.
- Bad: each site's old address under `agentplot.github.io` stops serving once its repository moves.
- Bad: until each host's data is moved by hand, its clones and records name the old addresses, which GitHub's redirect still serves for git.

## Amendment, 2026-11-12: a repository moves once its work is finished, and once the family's App reaches it

Status: accepted on 2026-11-12 by agentplot-design. It is the design agent's own choice, under the 2026-11-09 amendment's word that how the move is carried out is the design agent's, and cheap to change. It answers agentplot-planner's question that day and rests on agentplot-ops' reading of deneb.

Sources: agentplot-ops' reading of deneb, 2026-11-12. Deneb's sluice mints its tokens from App 5324750, installed on agentplot alone (installation 171576254), so it can't fetch, push or land in a repository once that repository leaves agentplot. boomdrivedev holds only the Boomdrive App, 5719968 (installation 181060998), and no host's sluice holds its key. Each repository's main and `bolt/*` are kept by repository rulesets, which move with it, and their one bypass actor is App 5324750. GitHub's page on transferring a repository says git clone, fetch and push follow the old address and says nothing of an App's installation. Records 0007 (as amended on 2026-11-09) and 0021; crew records 0060 and its decision 3; Flume's statement, C.4 (a repository named by its clone's git directory).

- **The family's App reaches every repository first.** No repository moves until each host that lands work through sluice takes its tokens from the runner, which holds the Boomdrive App's key (0007 as amended; the bolt `the-runner-gives-every-tool-its-forge-token`). The App is installed on agentplot as well as on boomdrivedev, for the repositories that stay and for each one until it moves. Giving the runner the key and installing the App are the user's steps.
- **A repository moves once its work is finished.** On a host, a move is a removal and an addition (0021, crew 0060). The new address is a new clone in a new folder, which Flume reads as a new repository and sluice as a new queue. So a repository moves only once crew 0060's decision 3 finds its work finished: no live place, no bolt or fix unlanded, no open card naming it, no unanswered batch carrying its work and no open proposal naming it. New starts in it are held while it drains.
- **The transfer and each host's move are one step.** Once the repository is drained, it is transferred and its rulesets name the Boomdrive App as their bypass in place of 5324750. It is then removed from each host's runner, Flume and sluice and added back at its `boomdrivedev/` address, and its own change rewrites its addresses. No host works a moved repository through GitHub's redirect.
- **The kit keeps its name.** crew names a kit by the last part of its address. A move changes the address the drive names and no record of crew's, and `crew kit rename` is for a change of name alone (crew 0060).
- **One at a time.** Repositories move one after another, each once drained, so no other repository's work is held.

The 2026-11-09 amendment's order, the transfers waiting on nothing, no longer holds, and nor does its last *Bad* consequence: no host's data names an old address while the repository is worked.

## Amendment, 2026-11-12: every repository moves before deneb's switch, through madswan-agentplot, its folders moving with it

Status: accepted by the user on 2026-11-12, on their card ("Approved"). Its order, its folders, its bridge and its pilot are the user's words below; how each move is carried out is the design agent's.

Sources: the user's words that day, relayed by agentplot-operator, on moving the repositories (signal `2026-11-12-agentplot-planner-c292bbdc/01`) and on their folders; afterthought-design's reading that no Boomdrive release carries the runner's forge token (#165), and that sluice finds an App's installation repository by repository (sluice `profiles/github/app.rs:483`); agentplot-ops' readings of deneb, 2026-11-12. The user installed madswan-agentplot (App 5324750) on boomdrivedev, every repository, and said "app is ready", which ops verified. On deneb every path names the owner: the runner's clone is `~/.local/share/boomdrive/repositories/agentplot/<repo>/<repo>.git`, with sluice's records inside it at `<repo>.git/sluice`. Each session folder, `~/Code/boomdrive/agentplot/<repo>`, is a worktree of that bare repository. Flume's places, crew's kits, herdr's sessions and Claude Code's project folders are keyed by those paths. afterthought's Nix writes the runner's list of repositories as `owner/repo`.

- **Every repository moves before deneb's switch.** In the user's words: "We also need to move all of the repos before the switch because I want to make sure that when we switch we're pointing to a solid foundation." The repositories are the ones the 2026-11-09 amendment moves. flume, sluice, timber, weir, crew and blueprints are transferred whole. Boomdrive opens at `boomdrivedev/boomdrive` from its scrubbed tree, and `agentplot/boomdrive` stays as its archive, so for a host Boomdrive is a new repository and not a moved folder. boomdrive-web is archived once the site moves into Boomdrive, and agentplot and boomdrive-cloud stay in agentplot. The user's words set when the moves happen, not which repositories move.
- **madswan-agentplot lands them until the switch.** It is installed on boomdrivedev beside the Boomdrive App, so deneb's sluice lands each moved repository as it does today, and each repository's rulesets keep their one bypass, 5324750. This is a stopgap. Deneb taking the Boomdrive App from its runner at the switch retires it, and madswan-agentplot leaves boomdrivedev once no host's sluice lands there with it. It replaces this record's earlier amendment of 2026-11-12, under which no repository moved until each host's runner held the Boomdrive App's key.
- **A repository's folders move with it.** In the user's words: "This also changes the folder storage locally so we need to be moving local folders to keep the folder structure correct." Once a repository has moved, every path on a host that names its owner names `boomdrivedev`. Nothing reads an old path, and no code maps an old path to a new one (principle 12). The folders are moved, not cloned afresh, so sluice's records travel inside the bare repository and main's numbers carry on.
- **A move is a window on the host.** A repository is ready once it is drained: no live place, nothing queued or under test in sluice, and no bolt or fix unlanded. Flume keys a repository by a hash of its git directory's path and carries no place across a changed path, so every place of the repository has ended and been tidied first. The window opens with the drive's main level, its runner and sluice stopped, so no host works a moved repository through GitHub's redirect. The main level's sessions work inside these folders, so the window's steps run from outside them: from the user's own terminal, from a script ops hands over, or by afterthought-ops. Each repository is then transferred on GitHub, and:
  - both folders move to `boomdrivedev/<repo>`, and git's worktree links are repaired;
  - origin names the new address;
  - afterthought's list names the new address, deployed to deneb, since a list deployed before the folders move clones an empty repository, and a folder moved first is cloned again at the old path;
  - each agent's Claude Code project folder keyed by an old path is renamed to its new path;
  - crew's kit paths and Flume's repository paths name the new folders.

  Then the runner starts, `sluice forge check` passes in each repository, and the main level comes back up in the new folders. Every repository that is ready moves in the same window, under one deploy. The first window holds timber alone, as a test of the steps, in the user's words: "we should pick one easy-to-move repo that is low risk. … If no bolts are running on Timber, I say we move Timber or something like that first." The rest move together once its forge check passes, each in the first window it is ready for.
- **The kit keeps its name.** crew names a kit by the last part of its address, so no record of crew's changes (crew 0060).

This amendment's bridge replaces the earlier one's precondition. That amendment's other points stand where this one doesn't restate them: a repository moves once drained, and the kit keeps its name.
