# Proposal

## Why

The public repository starts from this tree with a fresh history (records 0014 and 0025), and its opening waits only on the user's word (record 0025's amendment of 2026-11-09). core.41 says what the tree may hold: no text of a private repository, which it cites by its name and an id at most, and no host, drive, state repository, person's path or account of anyone's in its code, tests, fixtures or docs.

The change `boomdrives-tree-holds-nothing-private` brought the tree to that rule on 2026-11-07, and `what-main-adds-takes-the-neutral-names` took the same names into what `main` added the next day. Since then `main` has landed records 0029 to 0036, amendments to records 0009 and 0025, rules core.54 and core.57, and the changes `each-pass-ends-with-a-settle`, `the-runner-brings-up-a-preview-its-control-plane-asks-for` and `the-runner-carries-each-places-end`. Some of that text names the agents of the user's drives after their drives, names the user's own host, quotes a planner naming a drive, quotes a line of a host's configuration kept in a private repository, and describes a private repository's own packages, actions and commits. The code, the tests, the fixtures, the specs, the README and the flake's design hold nothing private.

## What Changes

- **Agents are named by their role.** Each agent named after a drive takes its role, as the first scrub's vocabulary has it ("the design agent", "the ops agent", "the operator agent", "another drive's design agent"), in the records' text and in their `decision-makers`, in core.54's details, and in the archived changes that name one.
- **No host of the user's is named.** Records 0032, 0034, 0035 and 0036 and core.57's details speak of "a host of the user's" or "a host's draft configuration" where they named one.
- **A private repository is cited, not described.** Record 0034 cites afterthought/swancloud by its name, its record and its files' paths, and says what the questions asked in this repository's own words. Its commit, the text it relayed, and its own packages and actions leave. Records 0035 and 0036 describe a host's configuration line rather than quote it, and name none of that repository's own settings. A consequence still names afterthought/swancloud with what this repository's decision asks of it, in one line, as records 0003, 0012, 0023 and 0028 do.
- **A drive's name inside a quote is cut.** The planner's words that record 0029 and the archived proposal of `each-pass-ends-with-a-settle` quote keep their sense with the drive's name cut.
- **What stays.** A brief's, a planner's or the design agent's words quoted as a record's source, but a private name inside them. Record 0002's settled points, which are the brief's own words. A request's, a launch's or a submission's number and the times a host ran it. The site's pages, which the change `boomdrive-keeps-its-site` brings with its own history and its own amendment of core.41, under which the site's measurement and field-note pages keep naming our own drives and machines as ours, as published.
- Nothing is added to the gate: no check, test or task. core.41 stays the rule a reviewer holds every diff to.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. No requirement or scenario of `openspec/specs/` names anything private. They cite private repositories by name and id alone, and their scenarios already take the first scrub's neutral names. The change declares `skip_specs` for that reason.

## Impact

- **Docs**: records 0009, 0025, 0029, 0030, 0031, 0032, 0034, 0035 and 0036 under `docs/adr/`; the details of core.54 and core.57 in `docs/architecture/core.md`, whose rules keep their text; the archived changes `2026-11-07-each-pass-ends-with-a-settle`, `2026-11-08-the-runner-brings-up-a-preview-its-control-plane-asks-for` and `2026-11-08-the-runner-carries-each-places-end`.
- No code, test, fixture, spec, crate, tool, tracked file type, task or rule changes, and no behaviour of the runner changes.
- Out of scope: the opening itself, the fresh history, the rename to `agentplot/boomdrive-archive`, turning Pages on, and the site's move (record 0025's amendment); everything under `site/`; the commit messages of this repository's history, which the fresh history leaves behind.
