---
number: 28
title: A host's configuration names its accounts, each account its drives, and each drive its repositories
status: accepted
date: 2026-11-07
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# A host's configuration names its accounts, each account its drives, and each drive its repositories

Status: accepted by the user on 2026-11-07, on their card.

Amends record 0023 in part, where each repository is named under its owner with the owner's stream; records 0006, 0018 and 0026 in their word "partition"; and rule core.5, as drafted below. Record 0017, revised the same day, follows this one.

Deciders: the user, in agentplot-design's pane on 2026-11-07: "Since crew is only being used by one person on a computer, we still might want to have the concept of different accounts. In my setup with Willdan, I could have that separate from my personal account. For example, I could have a personal account and a Willdan account. In my personal account, I would have an Afterthought, an Agentplot, and a MadSwan Drive, and those would be different crew states." Later the same day, on the flake's configuration: "there was other context where we talked about Boomdrive, the runner working with the cloud, and how you could enroll multiple accounts from the cloud. Each one would be a partition with the runner. That word being used in different places could be confusing", "Yes the branches in crew correlate to the drives. That's correct" and "Willdan work on DeNEB is just for reading and the work stays on the box". And on record 0017's card: "Per recent discussion with designer agent, this becomes one workspace per boomdrive drive". Loosening core.5 is the user's alone, and the reason for this card. agentplot-design for the rest: the two words, the stream's defaults, the keys and the rule for a drive on another machine.

Sources: record 0023 and its talk's `design/the-flakes-configuration/design.md` and `deneb.nix`; records 0006, 0009, 0017, 0018 and 0026; rules core.5, core.17, core.26 and core.43; deneb's `~/.config/crew/teams.json` as read 2026-11-07 (drives agentplot, madswan and swancloud in one state repository, `afterthought/crew-state`, and wldn on `chuck-herdr-alpha` in `WilldanGroup/crew-state`); the place `fix--a-kit-rename-renames-every-record-naming-it`, which ran in the stream swancloud because its owner `afterthought` is mapped there, though crew's work is Agent Plot's; signal 2026-11-07-agentplot-design-6cdf4ed4/01 on afterthought/crew-state agentplot/main.

## Context and problem statement

Record 0023 names each repository once, under its owner, and gives the owner a stream. The runner's file names its repositories in a flat list, and core.5 rules out a drive named in it. "Partition" means two things in this repository: what a runner enrolls with a control plane, one enrollment each (records 0006 and 0018), and a body of work that runs on its own (record 0026, from crew's partitions). The user keeps two accounts on one computer, each holding its own drives, and each drive is a branch of its account's crew state.

An owner is not a drive. `afterthought` holds swancloud's repositories and crew, which is Agent Plot's work, so a stream by owner runs crew's places in swancloud's stream. A workspace by owner gathers what the user does not work on together.

## Considered options

1. **Accounts hold drives, and drives hold repositories.** Two words, each with one meaning. The configuration names each account, each account's drives, and each drive's repositories by owner.
2. **Owners with a stream each**, as record 0023 stands, with one workspace per owner (record 0017 as first drafted). core.5 stands as written.
3. **Keep "partition"** for one of its two meanings, and add a word for the other.

## Decision outcome

Option 1.

- **An account** is what a runner enrolls with a control plane: one enrollment for each account on each machine, with its own key and connection (records 0006 and 0018, which read "account" where they say "partition"). Under crew, an account names its crew state repository.
- **A drive** is a body of work of one account: its repositories, one Flume stream and one branch of the account's crew state. Each drive runs on its own (record 0026, which reads "drive" where it says "partition"). crew's own word "partition" becomes "drive" with crew's rename (crew record 0054).
- **Each repository is named once, under its drive and its owner.** `services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner> = [ <names> ]`. A repository belongs to one drive. The module writes the runner's file and gives each repository its drive's stream in Flume's module. The person writes no folder.
- **Each drive has its stream.** The stream is named after the drive unless the drive names another. Its room, priority and each label's reserve are the drive's `stream` keys, and the module gives a default for each, so a drive writes only what it changes. The defaults are deneb's: four agents, a reserve of one for operation and two for planning.
- **A drive on another machine.** A drive whose `machine` names another computer gets no stream and no agents on this one. This host keeps its repositories to read, and crew's settings still list the drive with its machine.
- **The runner's file names its accounts and their drives** (core.5 as drafted below). It groups the repositories it keeps by drive, so the runner knows each repository's drive, as each drive's workspace needs (record 0017). It still names no stream and no kind. Under a hosted control plane, the managed layer writes the drives as it writes the repositories (record 0009).

A laptop with one account and one drive writes one line:

```nix
services.boomdrive.accounts.personal.drives.agentplot.repositories.agentplot = [ "flume" "sluice" ];
```

Its agents run under the login's own `claude`, in a stream named `agentplot` with the default room and labels. A host with several, as deneb, writes only what differs:

```nix
services.boomdrive.accounts = {
  personal = {
    state = "afterthought/crew-state";
    drives = {
      agentplot = {
        stream.priority = 11;
        repositories.agentplot = [ "blueprints" "boomdrive" "crew" "flume" "sluice" "timber" ];
      };
      swancloud.repositories.afterthought = [ "blueprints" "swancloud" ];
      madswan.repositories.madswan-dev = [ "paymaster" ];
    };
  };
  willdan = {
    state = "WilldanGroup/crew-state";
    drives.wldn = {
      machine = "chuck-herdr-alpha";
      repositories.WilldanGroup = [ "willdan-blueprints" ];
    };
  };
};
```

A drive's name is its stream's and its crew branch's. So deneb's drives keep the names they run under today, and none is renamed.

Option 2 runs a repository's work in its owner's stream, which is the wrong one wherever an owner's repositories belong to different drives. It keeps the user's accounts out of the configuration. Option 3 leaves a word the user found confusing.

### Rule core.5, as drafted

- **[core.5]** The runner keeps a clone or a view only of a repository its configuration names, and keeps none of one it does not; the configuration names its accounts, each account's drives, each drive's repositories and where their views go, and no stream or kind. (Record 0028, accepted 2026-11-07.)

Its details no longer rule out a drive named in the configuration. They still rule out a stream or a kind there.

### Consequences

- On the user's yes, records 0006, 0018 and 0026 and the rules that cite them read "account" or "drive" for "partition", each where it means one. A chore does that, citing this record.
- The configuration, service-modules and views specs change through one OpenSpec change, queued on the user's yes, citing this record and record 0017.
- The Boomdrive flake's bolt `the-boomdrive-flake-configures-the-whole-host` takes the accounts' shape in its unit `each-repository-is-named-once-by-its-owner`, which waits on this record.
- afterthought/swancloud passes deneb's accounts and drives as option values, once the modules land.

## Amendment, 2026-11-07: the keys of the runner's file

Status: decided on 2026-11-07 by the construct stage of the change `a-hosts-configuration-names-its-accounts-and-their-drives`. These are choices this record leaves to the runner's file, made on the written rules, each cheap to change. The change's design gives the path not taken for each.

- **The form.** The runner's file names `[accounts.<account>.drives.<drive>]`. Each drive's `repositories` is a list of addresses `owner/name`, in the order kept. The module turns its owners' names into addresses. Not taken: a table of owners in each drive. TOML does not order a table's keys, and a drive's workspace lists its folders in its list's order (record 0017).
- **Every level is required.** `accounts`, each account's `drives` and each drive's `repositories` are required, and each may be empty. A key left out is a mistake, never a host that keeps nothing.
- **Names.** An account's and a drive's name is the hosted label's form: 1 to 32 lower-case letters, digits and `-`, starting with a letter. A drive's name is given once on the host, across its accounts, and a repository is named in one drive.
- **Nothing else in an account or a drive.** The drive's stream and machine, and the account's state repository, are the modules' options and never keys of the runner's file (core.2, core.5). The `[control_plane]` tables keep their `partitions`, each handed whole to its implementation.
- **The flat list.** A top-level `repositories` is refused, naming `accounts`, as `views` and `fetch_every` are. The configuration is no file the runner keeps (core.35), so core.36 does not hold its keys for a release.
