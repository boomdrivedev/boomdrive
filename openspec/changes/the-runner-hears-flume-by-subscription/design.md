# Design

## Decision 1: The runner hears flume host by one subscription

flume's decision `flume-host-serves-status-on-its-socket` makes flume host answer Flume's status on its socket, and `flume status` a client of it. The runner is a resident, so it asks flume host by its socket (`residents-ask-each-other-by-socket`), and holds one subscription for all its parts (`one-subscription-per-plugin`). The subscription is held only while the runner carries a control plane's parts: a runner that only keeps its repositories reads no status.

## Decision 2: The socket and the lines

The socket is where flume host keeps it: `host-<key>.sock` in the user's own directory, `/tmp` resolved, then `flume-<uid>`, or `$FLUME_USER_SCRATCH` where it is set; the key is the first 10 hex digits of the SHA-1 of flume's state folder, `$XDG_STATE_HOME/flume` or `~/.local/state/flume`, resolved. This is flume's rule, as flume's subscription to sluice host computes sluice's; the runner reads it at its start from its own environment.

Each line is one JSON object (`residents-speak-json-lines-over-a-unix-socket`). The runner says `{"hello":"boomdrive/<version>"}`, reads flume host's answer within ten seconds, and takes it only where `protocol` is 1. It then asks `{"ask":"subscribe"}`, or `{"ask":"status"}` for one answer. Each `{"document":…}` line holds what `flume --json status` prints. The runner keeps the document's bytes as flume host sent them, read with `serde_json`'s raw value, so the hello's capacity and the statuses carry each field as Flume printed it (core.11, core.27). A document that is a refusal, `{"refused":{"refusal":{"reason":…}}}`, is Flume's refusal, as the command's exit 1 was. An `{"error":…}` line, a line the runner cannot read, or a closed connection ends the subscription, with its words.

## Decision 3: The latest document is the pass's listing

The subscription's thread keeps the latest document. A pass reads it where it ran `flume --json status`: before its pull, once its wants are answered where they ran a Flume command, and before it confirms a place's ends. Until the first document comes, a read waits for it up to ten seconds. While the subscription is down, a read is unanswered, with words naming the socket and why, and the pass does what it does today when Flume's status is unread: it launches nothing, and carries nothing against a listing.

A document lags a Flume command by as long as flume host takes to read its state again, about two seconds. A pass whose launches are not yet in the document carries them at the next pass, which the change of the document wakes (Decision 4). Room is read from the document, and a launch in flight still holds its room until Flume lists it (core.9, core.50).

## Decision 4: A change heard wakes every part

Each document whose bytes differ from the one held wakes every part, through the same bell its control plane's wake rings. A pass under way takes the wake, so changes heard together, or during a pass, make one pass after it (`one-read-in-flight`). So a control plane hears a place's change at once, and `poll_interval` is a backstop (`residents-learn-of-change-from-events-and-poll-once-a-minute-as-a-backstop`).

## Decision 5: Reach and stop

Where flume host does not answer, or the subscription ends, the runner tries again after one second, doubling up to a minute, with jitter (core.22). It says `boomdrive: flume host does not answer: <why>` once, when it stops answering or at the first try, and `boomdrive: flume host answers again` once it does (core.23). The thread reads with a short timeout so the runner's stop ends it within a fraction of a second, and the runner waits for it before it says it stopped.

## Decision 6: What stays a command

flume host serves status and the subscription alone. A place's own status, `flume --json status <place>`, `flume launch`, the acts, `flume confirm` and `flume relay` stay Flume's commands, which the runner runs once each at a control plane's asking. `flume status <place>` is itself flume host's client.

## Decision 7: The tests' flume host

The crate's tests that run the runner with a control plane serve a stub flume host on a scratch socket: it answers the hello, a status ask and a subscription from the stub `flume`'s status answer file, sends a new document when that file's answer changes, and records each ask. Tests that hold the pass's Flume as a fake, and the tests of `flume::Cli`, are unchanged.

## Decision 8: No transition

The runner asks no `flume --json status` where flume host does not answer. A host whose Flume serves no socket reads no status until it takes a Flume that does.
