# Design

## Context

See proposal.md for why. `main`, taken into the bolt `the-boomdrive-flake-configures-the-whole-host` on 2026-11-08 at `e8f0b33`, holds what core.41 and core.6 allow. What the bolt adds, read after that merge:

- **Where the private names are.** The unit tests of `crates/boomdrive/src/config.rs` the accounts and the local layer added; crew's table in `crates/boomdrive/tests/planes.rs`; the comment on a drive's stream defaults in `nix/modules/options.nix`; the specs `configuration` and `views`; and the archived changes `a-hosts-configuration-names-its-accounts-and-their-drives`, `the-flake-configures-each-tool-by-its-own-module`, `crews-partitions-give-the-runner-its-crew-table`, `each-repository-is-named-once-by-its-owner` and `herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents`.
- **What the merge already scrubbed.** Where a conflict put `main`'s neutral name beside the bolt's text, the merge took the neutral name: the README's example, `tests/readme.rs`, `tests/code_workspaces.rs`, and the tests of `carry.rs` and `code_workspace.rs`.
- **What holds nothing private.** The records the bolt amends, 0009, 0017, 0023 and 0028, whose consequences `main` had already made neutral, and every rule in `docs/architecture/core.md`. The nix tests' user `alice`, at `/Users/alice`, is the scrub's own neutral user.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches. It relies on core.41 and core.6, which say what the tree may hold.

## Goals / Non-Goals

**Goals:**

- Nothing the bolt adds lands on `main` with a host, partition, state repository, person's path or account of anyone's, or an employer's name.
- The specs reconciled through this change's deltas, and every scenario still proven by the test that proved it.

**Non-Goals:**

- **A check that the tree holds nothing private**, for the reason the scrub's design gives: it would hold the names it looks for.
- **Record 0032**, which `main` holds as accepted; this change touches no record `main` holds as it is.

## Decisions

### 1. The scrub's vocabulary, unchanged

What the bolt adds takes the names of the scrub's design Decision 1: a repository no public owner holds is under `example`, crew's old address becoming `example/crew`; a host is `host-a`. Where an archived change names the facts, accounts, drives or herdr of the operator's host, it says a host's, as `main`'s records 0023 and 0028 say "its hosts' facts".

Alternatives:

- **Names fresh to the bolt.** Not taken: two vocabularies for one thing.

### 2. The module's defaults are record 0028's

The comment on a drive's stream defaults named whose room and reserves they are. They are record 0028's defaults, which the module gives, so the comment cites the record.

### 3. The user's words keep what they decide

The proposal of `each-repository-is-named-once-by-its-owner` quotes the user on a drive of another machine, naming the operator's host and employer. The scrub keeps the user's words each decision rests on (its design, Decision 4), and core.41 and core.6 rule out the host and the employer. The quote keeps the words that decide, that such work "is just for reading and the work stays on the box", and drops the two names.

Alternatives:

- **The quote whole.** Not taken: it names a real host and the operator's employer.
- **A paraphrase alone.** Not taken: the decision rests on the user's own words, and the part that decides names neither.

### 4. The specs take the names through this change's deltas

Each requirement the bolt added whose scenarios name a private value is modified with Decision 1's names, and nothing else of it changes: three of `configuration`'s and one of `views`'. Each scenario is proven by the test that proved it, which takes the same names.

Alternatives:

- **The spec files edited in the merge.** Not taken: a spec changes only through an OpenSpec change (`openspec/config.yaml`; CLAUDE.md).

## Risks / Trade-offs

- **A missed name.** Nothing in the gate finds one. The verify stage reads the tree.
