# running Specification

## Purpose
How `boomdrive` runs on a host. It covers the version it prints, and `boomdrive run`, the long-running process the host starts and keeps alive, one per data directory. It also covers the passes that process makes and when it stops, what it says, and the rule that no connection comes in to it (crew record 0052 part 2; core.3).

## Requirements

### Requirement: boomdrive prints its version

`boomdrive --version` and `boomdrive -V` SHALL print `boomdrive <version>` on standard output, the crate's version, and exit 0. `boomdrive` with no command, or with one it does not know, SHALL print its usage on standard error and exit 2.

#### Scenario: The version

- **WHEN** `boomdrive --version` runs
- **THEN** it prints `boomdrive <the crate's version>` and exits 0

#### Scenario: No command

- **WHEN** `boomdrive` runs with no arguments
- **THEN** it prints its usage on standard error and exits 2

### Requirement: boomdrive run keeps the repositories until it is stopped

`boomdrive run` SHALL make a pass over the configuration at once and then one every `fetch_interval`, timed from the start of the pass before. Between passes, and between two repositories of a pass, it SHALL act on each landing it is told of. It SHALL run until sent SIGTERM or SIGINT, then stop before the next repository it would keep, begin no part's pass, let each part end the pass in hand, and exit 0. A git call that fails SHALL be named and SHALL never stop it.

#### Scenario: Stopped while waiting

- **WHEN** `boomdrive run` has made its first pass and is sent SIGTERM
- **THEN** it exits 0 within a second

#### Scenario: Stopped with a part's pass under way

- **WHEN** the runner is stopped while `home`'s control pass waits on its pull, and `work` waits for its next pass
- **THEN** `work` begins no pass, `home`'s pass ends with its report saved, the runner says it stopped, and no pass begins after that line

#### Scenario: Passes on the timer

- **WHEN** `fetch_interval` is two minutes and a pass began at 10:00:00
- **THEN** the next pass begins at 10:02:00, or at once when the pass before ends later than that

#### Scenario: A failure does not stop it

- **WHEN** a pass of `boomdrive run` finds `agentplot/flume`'s fetch failing
- **THEN** the runner names it and goes on, keeping every other repository, and tries `agentplot/flume` again at the next pass

### Requirement: One runner per data directory

`boomdrive run`, with or without `--once`, SHALL hold an exclusive lock on its data directory for as long as it runs. Whichever starts while another holds the lock SHALL exit 2, naming the data directory as held, and touch nothing.

#### Scenario: A second runner

- **WHEN** `boomdrive run` is running and `boomdrive run --once` starts on the same data directory
- **THEN** the second exits 2 naming the data directory as held by a running runner, and makes, fetches, moves and removes nothing

### Requirement: boomdrive run refuses to start rather than run blind

`boomdrive run` SHALL exit 2 before it touches anything, naming the reason, when: its configuration cannot be read; `HOME` is unset; git cannot be run; or git is older than 2.48, the first release whose fetch keeps origin's HEAD current.

#### Scenario: An old git

- **WHEN** `git --version` on the host says `git version 2.47.1`
- **THEN** `boomdrive run` exits 2 naming the version found and 2.48 as the least it runs with

#### Scenario: A bad configuration at start

- **WHEN** `boomdrive run` starts with a configuration that names `flume` as a repository
- **THEN** it exits 2 naming the file and the entry, and clones nothing

### Requirement: The runner says what it does, one line each

The runner SHALL write one line on standard error for each thing it does to a clone or a view: a clone made, a branch moved, a view made, moved or removed, a dropped repository removed. It SHALL also write one line for each thing it names, prefixed with the repository's address. It SHALL say when it starts, with its version, configuration file, data directory and repository root, and when it stops. A pass in which nothing changed and nothing is named SHALL write nothing.

#### Scenario: A quiet pass

- **WHEN** a pass finds origin unchanged for every kept repository and every view where the runner last put it
- **THEN** the pass writes nothing

#### Scenario: A view moved

- **WHEN** a pass moves the view of `agentplot/flume` to a new `origin/main`
- **THEN** it writes one line beginning `agentplot/flume:` naming the view moved and the commit

### Requirement: No connection comes in to the runner

The runner SHALL open no listening socket of any kind. Every connection it makes SHALL be outgoing, git's fetch over https among them (core.3).

#### Scenario: No listener in the code

- **WHEN** the crate's sources are read
- **THEN** none names `TcpListener`, `UnixListener`, `UdpSocket` or a `bind` of a socket, and the crate depends on no HTTP or WebSocket server

### Requirement: boomdrive run makes control passes on a timer of their own

Where the configuration names a control plane, `boomdrive run` SHALL make each part's first control pass after its first keeping pass, and its next `poll_interval` after the start of the part's pass before. After a part's pass that reported, it SHALL make that part's next sooner, by the midpoint between that pass's end and the earliest expiry of a lease the part holds not yet lapsed. A lapsed lease, or a pass that did not report, SHALL hasten none (core.22).

#### Scenario: On the control timer

- **WHEN** `poll_interval` is one minute, `fetch_interval` two minutes, and a control pass of `home` began at 10:00:00
- **THEN** `home`'s next control pass begins at 10:01:00, and the keeping passes keep their own two minutes

#### Scenario: A lapsed lease

- **WHEN** `poll_interval` is one minute, the runner holds a lease that lapsed before a control pass that began at 10:00:00, and the control plane cannot be reached
- **THEN** the next control pass begins at 10:01:00, and none runs between

#### Scenario: A pass that did not report

- **WHEN** `poll_interval` is one minute, the runner holds a lease expiring 40 seconds after a control pass that began at 10:00:00, and that pass's report was not sent
- **THEN** the next control pass begins at 10:01:00

#### Scenario: Each part on its own timer

- **WHEN** `poll_interval` is one minute, the runner carries `home` and `work`, and `work` holds a lease expiring 40 seconds after a pass of `work` that reported
- **THEN** `work`'s next control pass begins 20 seconds after that pass ended, and `home`'s keeps its own minute

### Requirement: A control pass runs at once when its control plane wakes the runner

`boomdrive run` SHALL make a part's control pass at once when its implementation wakes that part, through the wake its opener was handed for it, and SHALL time that part's next one from that pass's start. No other part's timer SHALL move.

#### Scenario: Woken by the control plane

- **WHEN** the implementation calls the wake its opener was handed for `home`, between two control passes
- **THEN** a control pass of `home` runs at once, and its timer's next pass is `poll_interval` after its start

#### Scenario: One part woken

- **WHEN** the runner carries `home` and `work`, and the implementation calls the wake handed for `home` between two passes of each
- **THEN** `home`'s control pass runs at once, and `work`'s next pass begins when its own timer says

### Requirement: A control pass after a broken configuration acts on nothing

A control pass that follows a reading of the configuration that failed SHALL name the file and act on nothing, until the file reads again.

#### Scenario: The configuration broken while running

- **WHEN** the configuration of a running runner carrying crew's work is made invalid
- **THEN** the next control pass names the file and acts on nothing, and once the file is valid again, the pass after acts as before

### Requirement: A control plane that cannot be read is said once until it is read again

A part's control pass whose pull cannot be read SHALL say `boomdrive: <label>: the control plane could not be read`, with the pull's words, only where that part's pass before read it, or where it is the part's first. A part's pass that reads it after one that could not SHALL say `boomdrive: <label>: the control plane is read again`. That part's passes between SHALL say nothing of it, and no part's reach SHALL be said for another (core.23).

#### Scenario: Three passes unread, then read

- **WHEN** the fake control plane's part `home` refuses three pulls in a row with the words `no network`, then gives the fourth
- **THEN** one line names `home` as unread with `no network`, and one line, at the fourth pass, says `home` is read again

#### Scenario: Each part's reach its own

- **WHEN** the runner carries `home` and `work`, `home` refuses every pull, and `work` gives every pull
- **THEN** one line names `home` as unread, and no line names `work`'s reach

### Requirement: Each part of the control plane is carried on its own

Where the configuration names a control plane and it opens, `boomdrive run` SHALL carry each part its implementation serves on a thread of its own, from the end of its first keeping pass until it stops. Each part's control passes SHALL pull, answer its wants, carry its statuses and report for that part alone. No part's pull, act, command or report SHALL wait on another part's, and no control pass SHALL wait on a keeping pass (record 0026; core.49).

#### Scenario: A slow part

- **WHEN** the runner carries two parts, `home` and `work`, `home`'s pull answers only after two minutes, and `work` offers a launch whose stream has room
- **THEN** `work`'s launch is made and reported within seconds, and `work`'s next control pass begins `poll_interval` after the start of its last

#### Scenario: A failing part

- **WHEN** the runner carries `home` and `work`, `home` refuses every pull and report with the words `no network`, and `work` offers a launch whose stream has room
- **THEN** `work`'s launch is made and reported, and one line says `boomdrive: home: the control plane could not be read: no network`

#### Scenario: A keeping pass under way

- **WHEN** a keeping pass is under way, its fetch of one repository held for a minute, and `work`'s control pass falls due
- **THEN** `work`'s control pass pulls, answers and reports while the fetch is held

### Requirement: A stop of the runner ends its calls in flight

Sent SIGTERM or SIGINT, `boomdrive run` and `boomdrive run --once` SHALL end each try of a call to a remote in flight as its limit would, through the runner's halt, `weir-bounded`'s, begin no such try, keep no further repository, begin no part's pass and no tick, and let each part end the pass in hand. `boomdrive run` SHALL then exit 0, and `boomdrive run --once` SHALL say `boomdrive: stopped` and exit 1 (record 0042 as amended on 2026-11-12; core.39, core.58).

#### Scenario: Stopped with a fetch in flight

- **WHEN** `boomdrive run` keeps `agentplot/flume`, with no clone of it, from an origin that takes the connection and never answers, and is sent SIGTERM once the origin has taken it
- **THEN** it exits 0 within five seconds; the lines after its start are `agentplot/flume: not cloned: git fetch with origin was stopped as its caller asked` and `boomdrive: stopped`; no folder of `agentplot/flume`'s is left in the data directory; and the origin's connection is closed within a second of its exit

#### Scenario: Stopped with a part's read in flight

- **WHEN** `boomdrive run` carries crew's partition `home`, whose state repository takes the connection and never answers, and is sent SIGTERM once `home`'s read of `home/main` has taken it
- **THEN** it exits 0 within five seconds; one line reads `boomdrive: home: the control plane could not be read: ` followed by words ending `git fetch with <url> was stopped as its caller asked`, naming the state repository's URL; its last line is `boomdrive: stopped`; and the state repository's connection is closed within a second of its exit

#### Scenario: run --once stopped with a fetch in flight

- **WHEN** `boomdrive run --once` keeps `agentplot/flume`, with no clone of it, from an origin that takes the connection and never answers, and is sent SIGINT once the origin has taken it
- **THEN** it exits 1 within five seconds; the lines after its start are `agentplot/flume: not cloned: git fetch with origin was stopped as its caller asked` and `boomdrive: stopped`; it runs no `flume`; and the origin's connection is closed within a second of its exit

### Requirement: Each line of a part's pass names the part

Each line the runner says of a part's control pass SHALL name the part's label after `boomdrive:`, as `boomdrive: <label>: <words>`. A line about one want SHALL begin with the want's id, as it does, the id being the part's own.

#### Scenario: A part's line names its label

- **WHEN** `home`'s control pass cannot read Flume's status, which answers `flume was ended: signal: 9`
- **THEN** one line says `boomdrive: home: Flume's status could not be read: flume was ended: signal: 9`

### Requirement: Each pass ends with a tick of each inbox

`boomdrive run` SHALL end each keeping pass and each part's control pass with `millpond tick --inbox <name> --json`, once for each inbox the configuration's `inboxes` names, in its order, awaited on that pass's thread, and none where it names none; `boomdrive run --once` SHALL end its passes so. millpond SHALL decide what a tick runs, and the runner SHALL read nothing of its answer but whether millpond gave one (record 0033; core.1, core.13).

#### Scenario: Each pass ticks each inbox

- **WHEN** the configuration names the inboxes `home` and `work`, and the runner makes three keeping passes with a stub `millpond`
- **THEN** each pass ends with `millpond tick --inbox home --json`, then `millpond tick --inbox work --json`, and an answered tick says nothing

#### Scenario: No inbox, no tick

- **WHEN** the configuration names no `inboxes`
- **THEN** the runner runs no `millpond` command

### Requirement: A tick is said only when it changes

Where an inbox's tick is refused, or gives no answer, the runner SHALL say so in one line only where that inbox's tick before was answered or it is its first, and SHALL say so again at its first tick answered after one that was not. No tick SHALL stop the runner or fail a pass, and none SHALL be carried to a control plane or kept (record 0033; core.11, core.23).

#### Scenario: A refused tick said once until millpond ticks again

- **WHEN** millpond refuses `home`'s ticks with the words ``this host keeps no inbox named `home` `` at two passes, then answers the third
- **THEN** the first says ``boomdrive: millpond refused the tick of the inbox home: this host keeps no inbox named `home` ``, the second says nothing, and the third says `boomdrive: millpond ticks the inbox home again`

#### Scenario: A tick with no answer

- **WHEN** `millpond` cannot be run for two ticks of `home` in a row
- **THEN** the first says `boomdrive: millpond gave no answer to the tick of the inbox home: ` and why, and the second says nothing

### Requirement: The parts of every plane the accounts name are carried side by side

`boomdrive run` SHALL open, when it starts, each implementation an account names, crew's then the hosted one's, and carry every part of each on a thread of its own, as it carries one implementation's parts. No part of one plane SHALL wait on a part of another. Two parts of one label SHALL be refused at the start, naming the label and both implementations (record 0054; core.19, core.49).

#### Scenario: crew and a cloud side by side

- **WHEN** the account `own` names crew, whose partition `home` offers a launch in `agentplot/flume`, and the account `cloud` names the hosted control plane, whose partition `cloud` offers a launch in `example/app`, each stream with room
- **THEN** both launches are made, `home`'s reported to crew and `cloud`'s to the hosted control plane

#### Scenario: One label in two planes

- **WHEN** crew's table and the hosted table each name the partition `home`
- **THEN** `boomdrive run` refuses to start, exiting 2, naming `home`, `crew` and `hosted`

### Requirement: The runner runs no settle of Flume's

`boomdrive run` and `boomdrive run --once` SHALL run no `flume settle`, in no pass, keeping or control. Flume's watch over the host SHALL be `flume host`'s, which the host's service manager runs (core.13; `one-resident-per-tool`).

#### Scenario: No pass runs a settle

- **WHEN** the configuration names no control plane, `fetch_interval` is one second, and the runner makes three keeping passes with a stub `flume`
- **THEN** the stub `flume` is asked nothing

#### Scenario: A control pass runs no settle

- **WHEN** `fetch_interval` is an hour and `poll_interval` a second, and the runner makes three control passes of its part with a stub `flume`
- **THEN** the stub `flume` is asked for no settle, and each control pass reads Flume's status

### Requirement: boomdrive run --once makes one pass and exits

`boomdrive run --once` SHALL make one keeping pass, as `boomdrive run` does, then, where the configuration names a control plane, one control pass of each part, side by side, then the ticks that end them, and exit. It SHALL exit 0 when every git call of the keeping pass succeeded and some part both pulled and reported, and 1 otherwise. What the specs leave and name SHALL be no failure: an offer left for want of room, a tool not run or read, a part unread while another reported.

#### Scenario: Every repository kept

- **WHEN** `boomdrive run --once` clones and fetches every configured repository and makes their views
- **THEN** it exits 0

#### Scenario: A clone failed

- **WHEN** `boomdrive run --once` cannot clone one configured repository
- **THEN** it keeps the others and exits 1

#### Scenario: Left for the user

- **WHEN** `boomdrive run --once` leaves a dropped repository held by a branch the forge lacks, and every git call succeeds
- **THEN** it exits 0

#### Scenario: One control pass

- **WHEN** `boomdrive run --once` runs with crew's control plane, `fetch_interval` two minutes and `poll_interval` one minute, over a scratch state repository holding a launch for this host, and a stub `flume` that launches it
- **THEN** it keeps the repositories, launches once, writes the runner's branch, asks the stub `flume` for no settle, and exits 0

#### Scenario: Flume's status refused

- **WHEN** `boomdrive run --once` runs with crew's control plane over a scratch state repository holding a launch for this host, and a stub `flume` that refuses its status
- **THEN** it names Flume's status as unread, launches nothing, writes the runner's branch, and exits 0

#### Scenario: A control plane that cannot be read

- **WHEN** `boomdrive run --once` runs with crew's control plane and no partition's state repository can be read
- **THEN** it keeps the repositories, names each part as unread, asks a stub flume host for each hello's status, runs no `flume` command but each hello's `flume --version`, and exits 1

#### Scenario: One part unread

- **WHEN** `boomdrive run --once` runs with crew's control plane serving the partitions `home` and `work`, the state repository holds no `home/main`, and `work/main` holds a launch for this host
- **THEN** it names `home` as unread, launches `work`'s launch once, writes `work`'s runner branch, and exits 0

### Requirement: Passes share their ticks

The first keeping pass SHALL share the ticks of the parts' first control passes. A pass that ends while another's ticks are under way SHALL share them, run none and wait for none. A keeping pass whose configuration did not read SHALL tick all the same, the inboxes of the last reading, and a runner told to stop SHALL begin none (record 0033; core.13, core.49).

#### Scenario: Passes back to back share their ticks

- **WHEN** the configuration names the inbox `home`, `fetch_interval` and `poll_interval` are an hour, and the runner's first keeping pass and its part's first control pass run back to back with a stub `millpond`
- **THEN** one `millpond tick --inbox home --json` runs, and no other tick runs before the next pass

#### Scenario: A pass ends while a tick is under way

- **WHEN** one pass's ticks are under way, and another pass ends
- **THEN** the other pass runs no tick, and returns without waiting for the ones under way to end

#### Scenario: A pass on a broken configuration ticks

- **WHEN** the configuration of a running runner that names the inbox `home` and keeps a repository each second is made invalid
- **THEN** each pass names the file and keeps nothing, and is followed by `millpond tick --inbox home --json` all the same

### Requirement: The runner hears Flume's status from flume host

While `boomdrive run` carries a control plane's parts, it SHALL hold one subscription to flume host and keep the latest `document` as flume host sent it. Each pass SHALL read its status from that document, and run no `flume --json status`. A document that differs from the one held of the same ask SHALL wake every part. `boomdrive run --once` SHALL ask `{"ask":"status"}` for each read. A runner that carries no part SHALL hold none (core.13).

#### Scenario: Passes read the subscription

- **WHEN** the runner carries `home`, `poll_interval` is a second, and a stub flume host answers on the socket
- **THEN** three control passes run, the stub flume host is asked one subscription, which names the kept clone's folder, and the stub `flume` is asked no `status`

#### Scenario: A change heard wakes the parts

- **WHEN** the runner carries `home`, `poll_interval` is an hour, and its first control pass ended
- **AND** the stub flume host sends a document that differs from the one held
- **THEN** `home` makes a control pass within seconds

#### Scenario: A runner that carries no part

- **WHEN** the configuration names no control plane
- **THEN** the runner makes no connection to flume host's socket

### Requirement: The socket is flume host's own

The runner SHALL find flume host's socket as flume host makes it: `host-<key>.sock` in `$FLUME_USER_SCRATCH` where it is set, else in `flume-<uid>` under `/tmp` resolved, `<key>` the first 10 hex digits of the SHA-1 of flume's state folder, `$XDG_STATE_HOME/flume` or `~/.local/state/flume`, resolved. It SHALL take flume host's answer to its hello only where it names `protocol` 1, within ten seconds.

#### Scenario: The socket of a scratch home

- **WHEN** `XDG_STATE_HOME` names a folder holding `flume`, and `FLUME_USER_SCRATCH` names another folder
- **THEN** the socket is `host-<the first 10 hex digits of the SHA-1 of the resolved flume folder>.sock` in the second folder

#### Scenario: Another protocol

- **WHEN** flume host answers the hello with `protocol` 2
- **THEN** the runner asks nothing on the connection, and Flume's status is unread, with words naming protocol 2

### Requirement: No flume host, no status

Where no flume host answers, or the subscription ends, Flume's status SHALL be unread, with words naming the socket and why, and the pass SHALL act as it acts on any unread status. The runner SHALL try the subscription again after a second, doubling up to a minute, with jitter, and SHALL say `boomdrive: flume host does not answer: <why>` once when it stops answering, and `boomdrive: flume host answers again` once it does (core.22, core.23).

#### Scenario: flume host stops and comes back

- **WHEN** the stub flume host stops while the runner carries `home`, and starts again
- **THEN** one line says `boomdrive: flume host does not answer: ` and why, `home`'s passes meanwhile say its status could not be read and launch nothing, and one line says `boomdrive: flume host answers again`

#### Scenario: A refusal in the document

- **WHEN** flume host's document is `{"refused":{"refusal":{"reason":"no store"}}}`
- **THEN** the pass reads Flume's status as refused with `no store`, as it read the command's refusal

### Requirement: The asks name the folders of the clones read

The subscription SHALL ask `{"ask":"subscribe","repositories":[…]}`, naming the folder of each clone that a pass reads, and no `repositories` where it names none. A read that names other folders SHALL make it ask again on a new connection. A status ask of `boomdrive run --once` SHALL name the same folders. The first document of an ask SHALL wake no part.

#### Scenario: The asks name the folders read

- **WHEN** a status ask names `/r` and `/s`, and a subscription is read first naming `/r`, then naming `/r` and `/s`
- **THEN** the status ask is `{"ask":"status","repositories":["/r","/s"]}`, and the subscription asks `{"ask":"subscribe","repositories":["/r"]}`
- **AND** the subscription asks again on a new connection naming `/r` and `/s`, each read gives the document of its own ask, and no part wakes
