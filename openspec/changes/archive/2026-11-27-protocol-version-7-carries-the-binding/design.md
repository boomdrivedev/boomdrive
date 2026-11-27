# Design

## Context

Flume decides at a launch whether a kind can land in a repository. It refuses a kind that lands where no merge queue lands the default branch. Flume v0.37.0 gives that fact before a launch. Flume answers a status ask that names repositories by their folders with `status.repositories`, one item for each folder, in the order of the ask. Flume's change `flume-reports-a-repositorys-binding` gives this form in its design, decision 5. Each item holds four fields:

- `repository`, the folder as the ask named it.
- `binding`, `{"read": …}` or `{"could_not_read": {"reason": …}}`.
- `merge_queue`, with `profile` and `ready`, and `words` where it is not ready.
- `kinds`, each with `kind` and `lands`, and `refused` where the binding refuses it.

Flume's status reads the clone as the last fetch left it. A flume host before v0.37.0 reads the ask and leaves the field out.

The runner reads Flume's status only from flume host's one subscription (Boom: `the-runner-hears-flume-host-by-one-subscription`). The cloud leads the protocol (Boom: `the-cloud-leads-the-runners-protocol`). The cloud's change `the-first-run-checks-the-merge-queue` proposes a shape of version 7 and takes the names that this change writes.

The design relies on these rules: core.1, the runner decides nothing. core.11 and core.27, a tool's answer as it gave it, and only the fields that the copy names. core.21, the hello is sent whole at each connection and again when it changes. core.26, one account's connection names only its own repositories. core.42, the protocol. core.45, the lock moves with the tag.

## Decisions

### Decision 1: version 7 is version 6 and one field of the hello

`protocol/v7/` is version 6 whole, with three changes. Each `$id` is under `…/protocol/v7/`. Each message names `protocol_version` 7. The hello requires one more field, `repository_statuses`, with six new definitions in `hello.schema.json`:

```json
"repository_statuses": {
  "example/app": {
    "printed": {
      "binding": { "read": {
        "branch": "main",
        "revision": "4f1c2a9e0b7d4c3a2f1e0d9c8b7a6f5e4d3c2b1a",
        "from": "origin",
        "declaration": ".config/flume.toml",
        "binding": { "place": null, "change": "openspec", "merge_queue": "sluice" }
      } },
      "merge_queue": { "profile": "sluice", "ready": false,
                       "words": "sluice does not land main: no .config/sluice.toml at main or a remote's main" },
      "kinds": [
        { "kind": "flume/fix@1", "lands": true,
          "refused": "the kind flume/fix@1 lands, and no merge queue lands it: sluice does not land main: no .config/sluice.toml at main or a remote's main" }
      ]
    }
  },
  "example/tools": { "missing": "Flume's status names no status of the repository's clone" }
}
```

- The map's keys are the repositories' addresses, `owner/name`, and it names each address of `repositories` once. A map keyed by addresses is a form that the protocol already uses. A list would carry the address again in each item.
- `printed` is the copy of Flume's item. It leaves out `repository`, the folder of the runner's clone, which is a path on the host (core.27). Each other field is Flume's, closed, as Flume's design names it. `binding` is `read` or `could_not_read`, and `from` is `origin` or `local`. `declaration`, each profile and `lands` can be `null`. `words` and `refused` are absent where Flume has nothing to say.
- `missing` holds the words that say why there is no copy. These are Flume's refusal, or the words of a status that the runner could not read. They can also say that Flume's status names no item for the clone, or name the path where the copy fails its check.

The field is in the hello, and not in a message of its own. The hello carries what the host keeps, and the runner sends it whole at each connection and again when it changes (core.21). A binding changes when a fetch moves the default branch. The next document of the subscription then differs, the parts make a pass, and the hello goes again. A message of its own would need its own resend and its own acknowledgement.

The cloud's draft put each status inside `repositories`, as objects. This change keeps `repositories` a list of addresses, as in version 6, and adds the map beside it. So version 7 is version 6 and one field, and the runner's types read all three versions. The cloud's draft took the name `report`, which is the name of the runner's `report` message (core.12). This change names the field `repository_statuses`, after Flume's `status.repositories`.

Each example of version 6 is an example of version 7, with `protocol_version` 7, and each hello names its statuses. `from-runner/hello-with-no-merge-queue-ready.json` is new. Its first repository has no declaration at its default branch, and a kind that Flume could not read. Flume could not read the binding of its second repository. The test `files` holds version 7 to version 6 but for this.

### Decision 2: the runner asks for the folders of the clones it keeps

`Flume::status` takes the folders of the clones to read. Each control pass names the folder of each clone of the hello's drives, sorted, once each, at each status read. So every read of a pass, and every pass, names the same folders.

The subscription asks first at the first read, and names the folders of that read. A read that names other folders makes the subscription end its connection. The subscription then asks again at once on a new connection, with no backoff and no line. The read waits up to ten seconds for a document of its own ask, as it waits for the first document. An ask that names no folder names no `repositories`, so it is the ask of before.

A document that differs from the one held of the same ask wakes every part. The first document, and the first document of a new ask, wake no part, because the read that made the ask waits for it. A document after flume host stopped answering wakes every part, as before.

`boomdrive run --once` names the same folders in each `{"ask":"status"}`. The command line, `flume status`, names none, so its answer holds no item.

### Decision 3: the hello takes Flume's item as Flume printed it

`hello::repository_statuses` gives each kept repository, by its address, the item of `status.repositories` whose `repository` is its clone's folder, as Flume printed it. Where Flume's status names no such item, the repository is `missing`, with words that say so. Where Flume refused its status, or the runner could not read it, each repository is `missing`, with those words. The interface's `Hello.repository_statuses` holds the result. A hello's JSON that names none reads as naming none.

crew's implementation reads nothing of it. A crew role reads Flume on the host.

### Decision 4: the hosted implementation cuts the copy only for version 7

`Hosted::pull` builds the statuses once for each pull, where the version spoken is 7 or later. It cuts each printed item by `hello.schema.json#/$defs/repository_status_copy` and checks it (`copy::repository_status`). Where the check fails, the status goes up `missing`, with words that name the path and no value. The plane says one line while it fails. Each partition's hello takes the statuses of its own repositories alone (core.26). Before version 7, the hello names no `repository_statuses`, and a hello that holds them fails version 6's check.

### Decision 5: the types move to `protocol::v7`

The runner's types move to `protocol::v7`. `v6.rs` keeps its files and constants alone, as `v5.rs` does. `v7::Hello.repository_statuses` is optional: the plane writes it under version 7, and leaves it out under versions 6 and 5. Each message is checked against the version spoken before it is read, so the types never see a field that the version does not name. `protocol::V7` joins `speakable()`. A test that sets a `Dial`'s version to `V7` runs the code that a host runs once the mark moves. The crate's tests pass with version 6 or version 7 marked spoken.

### Decision 6: the set takes Flume v0.37.0

`flake.nix` names Flume at `v0.37.0`, and `nix flake update flume` moves the flume node of the lock alone, to the tag's commit `132253d` (core.45).

## Risks

- Flume reads the binding as `origin` was last fetched. After a person sets a repository up, its status stays as before until Flume's next fetch. The next document then changes, and the hello goes again.
- Flume's words can name a folder of the host, for example where a binding could not be read. The copy carries Flume's words as Flume gave them (core.11).
- A host whose flume host is older than v0.37.0 gives no item. Each repository then goes up `missing`, and the control plane reads it as not read.
- The hello grows with the kinds of each repository. It goes at each connection and at each change, not on a timer.
