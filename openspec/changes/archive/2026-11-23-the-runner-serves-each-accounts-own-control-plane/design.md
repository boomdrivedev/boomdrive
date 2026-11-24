# Design

## Context

Record 0054 (accepted 2026-11-19, point 3 as amended with record 0056) and the rules it amended, core.10, core.19, core.26, core.33 and core.42, say what this change builds. Record 0048 is withdrawn, and the user ruled on 2026-11-24 that no observing stays in the code, and that no transition or compatibility code is written, since no public client runs the runner.

## Decisions

### Decision 1: An account names its plane by its implementation's word

`[accounts.<account>]` holds `control_plane`, `"crew"` or `"hosted"`, beside `drives`. `[control_plane]` holds `poll_interval` and the table of each implementation an account names, `[control_plane.crew]` and `[control_plane.hosted]`, each handed whole to its implementation as before. The configuration is refused where an account names an implementation this binary does not carry, or one whose table `[control_plane]` does not hold; where `[control_plane]` holds a table no account names; where it holds `type`, the words naming `accounts.<account>.control_plane`; and where it holds no table at all. An account naming none is served by no control plane: its repositories are kept and no work comes for them. The local layer sets no account's `control_plane` (core.17). `poll_interval` is every part's.

A host serves at most one plane of each implementation. A second hosted plane, such as production beside a local copy, is a later change of `[control_plane]`'s shape, which core.19 already allows. A repository stays in one drive on a host (record 0028), so it has one account and one plane; record 0054's repository in drives of two accounts, reported to both, is a later change of that rule.

### Decision 2: The runner opens each plane its accounts name, in `PLANES`' order

`planes::open` takes the configuration's control-plane table and opens, in `PLANES`' order, crew then hosted, each implementation an account names, giving one list of parts. Two parts of one label are refused at the start, naming both implementations, since the report file keeps each part's share by its label. Every part is carried on its own thread as before (record 0026; core.49).

### Decision 3: The runner gives each part the places it did not launch; the implementation cuts them

At each control pass, after the statuses are carried, the runner reads Flume's listing, the pass's own, and gives the part's share `unleased`: each place whose `repository` is the clone of a repository the runner keeps or drains, whose mark is not the runner's mark of a launch the share holds, by its `place` as Flume printed it, with the repository's address and the place's entry, as Flume printed it, carried with when the runner first read it as it stands and `listed` true. The runner keeps what it carried in memory alone, so the report file never holds it (core.35), and a place carried again as it stood keeps its time.

The runner gives each part every such place, as it gives every part the hello's repositories. Each implementation cuts them as it cuts the hello (core.26): the hosted implementation to the repositories its partition names, crew's to those its drive's hello names. A place launched by another part of the host goes to this part with no lease, as record 0054 says, and a place a person started by hand alike. `ControlPlane` is unchanged: `unleased` rides in the `Report` the part is handed, and no answer names it, so a plane lets go of nothing of it and confirms no end of it (core.10, core.54).

### Decision 4: Version 5 is version 4 but for the observer and a report's unleased places

`protocol/v5/` is version 4's files, each `$id` under `…/protocol/v5/` and `protocol_version` 5, but that `hello.schema.json` holds no `observing_hello`, `report.schema.json` no `observed_report` or `observed_launch`, each kind taking its one form, and the report requires `unleased`: a map from a place's path to `{repository, status}`, its `status` a carried status or null. Its examples are version 4's but the observing connection's three, and one more, `report-a-place-started-elsewhere.json`. The runner's types move from `protocol/v4.rs` to `protocol/v5.rs`, and `v4.rs` holds version 4's files and no type, as `v3.rs` does. A connection's upgrade asks for `boomdrive.v5` alone, and every request, enrollment and forge token's ask, is of version 5.

Version 4's files stay as `v0.21.0` holds them (core.42). The runner stops speaking version 4 before Boomdrive Cloud's main speaks version 5, against record 0056's order of one version at a time; the user's ruling that no compatibility is kept, with no public client, is the reason, and the cloud moves to version 5 in its own change.

### Decision 5: crew's implementation writes `unleased.rec`

crew's runner branch, `<label>/runner/<host>`, gains `unleased.rec`, under the descriptor this repository proposes for crew to hold:

```
%rec: Unleased
%key: Place
%type: At date
%mandatory: Place Repository At Status
%allowed: Place Repository At Status
```

one record for each unleased place in the repositories the partition's drive keeps, written at each write of the branch where any is held, and removed where none is. crew acts on none of them.

### Decision 6: A repository's forge tokens follow its accounts' plane

`Forge::of` keeps its minting and, where an account names a plane that holds the App, that plane's tokens, with the repositories whose every account names that plane. A repository among them is asked of the plane; any other is minted from `[forge]`, or fetched with the host's own helpers where the file names none. `[forge]` is refused only where every account names a plane that holds the App (core.33).

### Decision 7: The observer goes whole

`Observer`, `Observed` and `fake::Observing` leave `boomdrive-control`. `observer.rs`, `table::Role`, `Table::read_observer`, `enroll_observer`, `Dial::observing`, `check::observing_hello`, `check::observed_report`, `check::prepare_observing` and the observer's refusals in `connection.rs` leave `boomdrive-hosted`. `[observer]`, `planes::observers`, `read_observer`, `enroll_observer`, `boomdrive enroll --observer`, `Runner::observe_with` and `Carrier::observed_pass` leave the runner, with every test of them. Record 0048's status reads superseded by record 0054. core.42's details name version 5.

### Decision 8: The flake's module writes each account's plane

`services.boomdrive.accounts.<account>.control_plane`, `null`, `"crew"` or `"hosted"`, is written into the account. `settings.control_plane.type` fails evaluation, naming the accounts' option. crew's table is written where an account names crew and crew's module is on, as it was written where `type` was crew. NixOS's check of systemd's version holds where an account names hosted.
