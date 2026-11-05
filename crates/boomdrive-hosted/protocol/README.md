# The runner's protocol

The messages between a Boomdrive runner and a hosted control plane, as
JSON Schema of draft 2020-12, one directory per version. The runner
enrolls each partition by one outgoing request, and speaks it over one
outgoing WebSocket for each, which it opens and keeps; no connection
ever comes in to a host. Both ends check every message they
are sent against the version before reading it, and refuse one that
fails whole.

This repository's records and rules are the protocol's source: rule
core.42 and record 0015, under `docs/`, and the change that wrote
version 1, `the-runner-keeps-its-protocol-as-one-json-schema-per-version`.
They cite boomdrive-cloud's records and rules by id.

## Versions

Version 1 is `v1/`. A version may change until a release of the runner
speaks it to a control plane. From that release on it never changes,
and its directory is never removed: every change after it is a new
version, in a directory of its own beside it. A control plane accepts
the current version and the one before.

A connection asks for the WebSocket subprotocol `boomdrive.v1` at
its upgrade, so a control plane that does not accept the version refuses
the connection before any message.

## The files

| file | what it holds |
|---|---|
| `from-runner.schema.json` | the envelope of every message a runner sends |
| `to-runner.schema.json` | the envelope of every message a control plane sends |
| `shared.schema.json` | what both directions use: ids, a time, integers, a lease, a repository's address, a commit, a digest, bytes, and `message_refused` |
| `enrollment.schema.json` | `enroll`, and its answers `enrolled` and `enroll_refused` |
| `hello.schema.json` | the hello, a tool's version, and the copy of Flume's host part |
| `wants.schema.json` | a want, an offer with its launch, an act, a command, an inbox, and `pulled` |
| `report.schema.json` | the runner's report and `reported` |
| `place-status.schema.json` | the copy of a Flume place's status |
| `submission-entry.schema.json` | the copy of a sluice submission's entry |
| `terminal.schema.json` | a key share and a terminal's messages |

`examples/from-runner/` and `examples/to-runner/` hold whole messages
of every kind, each of which passes its direction's envelope.

Each file's `$id` is its address among this repository's raw files at
`main`. No check fetches a `$id`: a validator is handed the version's
files, each under its `$id`, and nothing else, and every `$ref` is
relative, to a file of the same version. A reference outside them fails
to resolve. A pattern uses only what ECMA-262 and Rust's `regex` read
alike, so a validator in either language refuses the same strings.

## The forms

- **Every message** is `{"protocol_version": 1, "id": …, "body": {"<kind>": {…}}}`.
  The `id` is the sender's own for the message, 1 to 128 letters, digits
  and `._:/-`, unique among what that sender sends on its enrollment, and
  kept when the message is sent again. The body holds exactly one kind,
  and each direction's envelope takes only that direction's kinds.
- **Every object is closed**: it takes only the fields it names, at
  every depth. A map, keyed by ids, paths or tools' names, names the
  form of its keys and of its values. Two values are carried whole, of
  any form, as their tool printed them: a sluice row's `flaky`, and a
  thing a stage's end made.
- **Every field the runner's interface always writes is required**,
  `null` where it holds none.
- **A time** of the runner's or the control plane's is RFC 3339 in UTC
  to the second, `YYYY-MM-DDTHH:MM:SSZ`. A time inside a copy is the
  tool's, as it printed it: Flume's are Unix seconds.
- **An integer** is one JavaScript reads exactly, within 2^53 − 1 of
  zero. A lease's number is 1 or more.
- **Bytes**, a key, a signature, a certificate or a sealed frame, are
  base64url without padding.

## The messages

| kind | sent by | holds |
|---|---|---|
| `enroll` | the runner, as the body of `POST runner/enroll` | a partition's public key, its signature over the enrollment's context, and the machine's name |
| `enrolled` | the control plane, answering `enroll` | its id for the host |
| `enroll_refused` | the control plane, answering `enroll` | the `enroll` message, by id, with words |
| `hello` | the runner | the host's interface and runner versions, its tools' versions, the repositories it keeps for this control plane, its capacity, and the leases it holds |
| `report` | the runner | the runner's whole report: each launch's answer with its place's status and its submission's entry as their copies, each act and command it carried out or refused, and each inbox it carried |
| `pulled` | the control plane | the whole of what it wants, in its order, and the runner's report as it holds it, or `null` |
| `reported` | the control plane | the `report` message it answers, by id; the ids it lets go; the ids it refused, with its words |
| `message_refused` | either | a message that failed its check, by id, with words |
| `terminal_open` | the control plane | a terminal on a launch's agent, under the launch's lease number, with the browser's share |
| `terminal_opened` | the runner | the terminal, with the runner's share |
| `terminal_output` | the runner | a sealed frame of the pane, at its offset, and whether it is a redraw |
| `terminal_input` | the control plane | a sealed frame of the person's keys, at its offset |
| `terminal_resume` | the control plane | the offset a reconnected browser has every byte up to |
| `terminal_close` | the control plane | the terminal, closed from the browser's end |
| `terminal_closed` | the runner | the terminal, closed by the runner, with its words |

On the socket, their order:

1. At every connection the runner's first message is its whole `hello`.
   The control plane answers it with `pulled`, holding the runner's
   report as the control plane holds it, so a runner that lost its own
   takes it back.
2. Whenever its wants change, the control plane sends `pulled` again,
   its `report` `null`.
3. The runner sends `hello` again when it changes. At each report it
   makes, it sends `report`, which renews every lease it names.
4. The control plane answers each `report` with `reported`, naming it by
   id. After a reconnect the runner sends again, under its own id, the
   last `report` no `reported` answered, and no earlier one, since a
   report is whole.
5. A message that fails its check is answered with `message_refused`,
   and nothing else is done with it. Its sender does not send it again.
6. A terminal opens, carries its frames and closes as below.

A `pulled` is the whole of what the control plane wants, and a `report`
the whole report, so a second delivery of either changes nothing. The
socket is kept alive by the WebSocket protocol's own pings: no message
is a heartbeat.

Version 1 has no message for a forge token, for health, for the
runner's own commands, or for a terminal's certificate. The changes
that build them add them.

## The paths

Two paths go under the control plane's address, the `url` of the
runner's `[control_plane.hosted]` table, which gains a `/` where it
ends without one:

| path | how | the kinds it takes |
|---|---|---|
| `runner/enroll` | one HTTPS `POST` | the runner's `enroll`, answered by the control plane's `enrolled`, `enroll_refused` or `message_refused` |
| `runner/connect` | a WebSocket upgrade of `GET`, over TLS, on the subprotocol `boomdrive.v1` | every other kind, each direction's its own |

Under the `url` `https://control.example/boomdrive`, they are
`https://control.example/boomdrive/runner/enroll` and
`wss://control.example/boomdrive/runner/connect`. A `url` is `https`,
or `http` only to a loopback address, as a test's control plane is,
whose socket is then `ws`.

The schema checks a message's form, not the path it came by: a control
plane refuses an `enroll` on the socket, and any other kind on
`runner/enroll`, by its path, and a runner refuses an `enrolled` or
`enroll_refused` on the socket with `message_refused`.

## The signatures

Each partition's key is an Ed25519 key pair the runner makes on the
host, whose private key never leaves it. It signs a context: the UTF-8
bytes of `boomdrive.v1`, a space and the context's name, then
each of the context's fields after a NUL byte, with nothing after the
last. A signature is Ed25519's as RFC 8032 gives it, 64 bytes, over the
context's bytes themselves, never a digest of them.

Every field names what the signature binds, so a signature made for
one context, one control plane or one token is never taken for
another. The contexts are an enrollment's (`enroll`), a connection's
(`connect`), and a terminal's share (`terminal share`, under *A
terminal*).

A field that names the control plane is its `url`'s origin: the scheme,
`://` and the host, then `:` and the port where the port is not the
scheme's own, all lowercased, with no path and no `/` after it. The
origin of `https://control.example/boomdrive` is
`https://control.example`.

## Enrollment

A partition is enrolled once, by `boomdrive enroll <label>`, with a
one-time token the control plane issued. The runner makes the
partition's key in memory, and sends one request:

```text
POST <url>/runner/enroll
Authorization: Bearer <token>
Content-Type: application/json

{"protocol_version":1,"id":"<the message's id>","body":{"enroll":{"key":"…","signature":"…","machine":"…"}}}
```

- **`key`** is the partition's public key, 32 bytes, 43 characters in
  base64url.
- **`signature`** is the key's, 64 bytes, 86 characters in base64url.
- **`machine`** is the host's name, as its operating system gives it.

The token is the request's bearer alone: no message holds it.

**What its signature is made over**: the context `enroll`, whose fields
are the control plane's origin; `sha256:` and the lowercase hex of the
SHA-256 digest of the token's bytes, as the bearer carries them; and
the key's 32 bytes. It binds the key to this control plane and this
token, so no key is registered by one who does not hold it, and no
signature is taken with another token.

**The answer** is a message of version 1, which the runner checks
against `to-runner.schema.json` before it reads it:

- `200`, with `enrolled`, holding `host`, the control plane's id for
  the host;
- `401` or `403`, with `enroll_refused`, holding `enroll`, the request's
  message id, and `words`, such as that the token has lapsed or was
  spent;
- `400`, with `message_refused`, the request having failed the control
  plane's check.

The runner keeps the key in the host's credential store only once
`enrolled` has come. It keeps nothing else of enrollment, no host id and
no certificate: the control plane finds the enrollment by the key
itself. A request sent again with the same message id and the same key,
its answer lost, is answered `enrolled` again, not refused as a spent
token.

### The enrollment's worked bytes

From the secret key of RFC 8032's test 1, published there for tests and
so no host's, and a token that is this README's own:

```text
secret key   9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60
public key   d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a
token        example-enrollment-token
url          https://control.example/boomdrive
address      https://control.example/boomdrive/runner/enroll
origin       https://control.example
digest       sha256:2ff0ff8cabe082ca9b667090071b2f438a8c1665b25f655d7b305293a75fef7f
context      626f6f6d64726976652e763120656e726f6c6c0068747470733a2f2f636f6e74
             726f6c2e6578616d706c65007368613235363a32666630666638636162653038
             3263613962363637303930303731623266343338613863313636356232356636
             3535643762333035323933613735666566376600d75a980182b10ab7d54bfed3
             c964073a0ee172f3daa62325af021a68f707511a
signature    16502207e301fa460323eb35ccbcd795ee38582d54d59a994ca56d30b6c0fb31
             89e4406b382256d157aa48194c924a398734f8b342e2d6a451d07b7bc1869b0a
```

Read as text, the context's 148 bytes are `boomdrive.v1 enroll`, a NUL,
the origin, a NUL, the digest, a NUL and the public key's bytes. In
base64url, as `examples/from-runner/enroll.json` carries them, the key
is `11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo` and the signature
`FlAiB-MB-kYDI-s1zLzXle44WC1U1ZqZTKVtMLbA-zGJ5EBrOCJW0VeqSBlMkko5hzT4s0Li1qRR0Ht7wYabCg`.

## The connection's proof

Every upgrade of `runner/connect` proves the partition's key is held,
by a statement signed for that attempt alone, in its `Authorization`
header:

```text
GET <url>/runner/connect
Sec-WebSocket-Protocol: boomdrive.v1
Authorization: BoomdriveHost <key>.<time>.<nonce>.<signature>
```

- **`<key>`** is the partition's public key in base64url.
- **`<time>`** is the runner's clock in Unix seconds, as decimal
  digits.
- **`<nonce>`** is 16 random bytes in base64url.
- **`<signature>`** is the key's, in base64url.

**What its signature is made over**: the context `connect`, whose
fields are the control plane's origin; the key's 32 bytes; `<time>`'s
digits, as the header carries them; and the nonce's 16 bytes.

The control plane takes the upgrade only where the key is one it
enrolled and has not revoked, the signature checks, `<time>` is within
300 seconds of its own clock, and it has not seen `<nonce>` with that
key within that window. It finds the enrollment by the key itself. The
runner signs a new statement for every attempt, and keeps none.

It refuses the upgrade:

- with `401` or `403` where it does not take the key: the partition is
  enrolled again with a new token;
- without `boomdrive.v1` where it does not speak the version,
  which the runner takes as a refusal.

### The connection's worked bytes

From the same key, at 2026-11-04T00:00:00Z, with a nonce of the bytes
0 to 15:

```text
secret key   9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60
public key   d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a
url          https://control.example/boomdrive
address      wss://control.example/boomdrive/runner/connect
origin       https://control.example
time         1793750400
nonce        000102030405060708090a0b0c0d0e0f
context      626f6f6d64726976652e763120636f6e6e6563740068747470733a2f2f636f6e
             74726f6c2e6578616d706c6500d75a980182b10ab7d54bfed3c964073a0ee172
             f3daa62325af021a68f707511a00313739333735303430300000010203040506
             0708090a0b0c0d0e0f
signature    884279e06d1a3e54d54432d832bd8943f875a26148876d83989112145c1fc516
             3b9caf477c7f071d3dd9fea675739d8d6aa58abdfb89abfae8c65d77ab85430d
```

Read as text, the context's 105 bytes are `boomdrive.v1 connect`,
a NUL, the origin, a NUL, the public key's bytes, a NUL, the time's
digits, a NUL and the nonce's bytes. Its header is:

```text
Authorization: BoomdriveHost 11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo.1793750400.AAECAwQFBgcICQoLDA0ODw.iEJ54G0aPlTVRDLYMr2JQ_h1omFIh22DmJESFFwfxRY7nK9HfH8HHT3Z_qZ1c52NaqWKvfuJq_roxl13q4VDDQ
```

## The copies

A tool's answer goes up as a copy: an object holding only the fields
the cloud keeps, each as the tool printed it. The schema has no field
for the rest, so the cloud's check refuses it.

- **A place's status** (`place-status.schema.json`) is `flume status
  --json`'s one place, as Flume 0.4.1 prints it, cut to its ids, states,
  times and counts, what it made, its live agent's pane, and the words a
  person reads to follow or answer it: an end's, a question's and a
  hold's. It holds no agent's last words, screen, printed command,
  notes, settings or mode; no task's text; no readiness; no task group's
  title; no teardown's words; no end's report; no verdict's words; and
  no path but the place's own. A field Flume adds after 0.4.1 is left
  out.
- **The host's capacity** (`host_part` in `hello.schema.json`) is
  Flume's host part as values: the host's and each stream's room, a
  stream without its queue, which names places of every control plane
  the host serves. Flume's pools, addresses and bases are left out.
- **sluice's entry** (`submission-entry.schema.json`) is `sluice status
  <n> --json` cut to its `id`, `branch`, `head`, `state` and
  `landed_as`, and its failed rows' `task`, `hash`, `flaky` and
  `system`, never a row's `log`.

## A terminal

A terminal's bytes leave the host only encrypted end to end, to the
browser that opened it. The control plane relays them and cannot read
them, and neither end keeps them.

- **The shares.** `terminal_open` carries the browser's share, and
  `terminal_opened` the runner's. A share holds its `suite`, the end's
  `key` for this terminal alone, its `signature` by the end's signing
  key, and the `certificate` the control plane issued on that signing
  key: a member's for the browser, the host's for the runner. The
  runner takes the browser's share only where its signature checks
  against a key certified to a member, and never falls back to the
  clear.
- **What a signature is made over**: the context `terminal share`
  (*The signatures*), whose fields are the end (`browser` or
  `runner`), the terminal's id, the launch's id, the suite, and the
  key's bytes. A share is therefore never taken for
  another terminal or reflected back to its own end.
- **The suite** is a name the two ends agree; the control plane relays
  it. The change that builds both halves of a terminal chooses it and
  names it here, without a new version. Two ends naming different
  suites open no terminal.
- **Frames.** `terminal_output` and `terminal_input` carry their bytes
  only as `sealed`, the suite's ciphertext. Each carries `offset`, the
  place of its first byte in its direction's stream, counted from 0 at
  the terminal's opening.
- **A resume.** A browser whose socket dropped, its keys still held,
  sends `terminal_resume` with `seen`, the offset it has every byte up
  to. Where that is behind what the runner sent, the runner answers
  with a frame whose `redraw` is `true`: the pane's present screen, at
  its next offset, which the browser draws after clearing its terminal.
  A browser that lost its keys opens a new terminal.
- **Closing.** `terminal_close` closes a terminal from the browser's
  end, and `terminal_closed` says the runner closed it, with its words.
  A terminal is closed with its socket.
