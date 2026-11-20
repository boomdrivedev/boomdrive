# Design

The construct's own choices within record 0002 as amended on 2026-11-19, core.1, core.10, core.42, core.50 and core.55, each cheap to change.

## Decision 1: Scoped threads per part, one channel back

Each part keeps a `hands::Underway` for as long as its thread runs: the threads its commands run on belong to a `std::thread::scope` the part's thread opens around its passes, so they borrow the part's Flume, sluice and the clones' locks, and every one started has ended before the part's thread does. A thread runs one command, `hands::Job`, and sends its `Done`, the want and what the tool gave, back over the part's one channel, then rings the part's bell. A thread judges nothing: the part alone reads the report, says each line and answers the control plane (core.1).

Not taken: a fixed pool of workers (record 0002's option 3). Not taken: async. The commands are processes the runner waits on, and a thread each is what the record asks.

## Decision 2: A part's thread waits on its timer, its wake and its answers

The bell that wakes a part gains `answered`. Between passes the part's thread waits for its next pass, a wake, an answer or the stop. An answer is taken by `Carrier::collect`: the part reads its share, takes each answer that came back, starts each act or preview of the last pull that waited on it, and sends where the share changed. It pulls nothing and reads no listing, using the last pass's. A pass due is told before an answer, and the pass takes every answer that came back first.

## Decision 3: A launch takes its lock on its own thread

A launch's room is counted on the part's thread, at its turn, then its thread holds the clone's lock shared, waiting for its own repository's keep alone, finds the clone again and runs `flume launch`. So the pass no longer holds launches that wait on a keep for a second round. A launch in flight whose mark Flume's listing does not hold yet is counted against the room at each later pass.

## Decision 4: An act waits on its launch, and on the acts before it on that launch

An act whose launch is in flight waits, neither run nor answered. An act on a launch made waits while any act before it in the control plane's order, on the same launch, is unanswered: in flight, or left by a Flume that could not be run. Flume's status of a place the listing does not hold, and the confirm's check of its keys, run on the act's own thread before its command.

## Decision 4a: An offer waits on a launch it comes after or works from

An offer whose After names a launch in flight waits, so its held words read where that launch stands once made. An offer whose base is the branch of a place being made, `<kind>/<name>` with an identity's kind read as its name, as a bolt's unit on `bolt/<name>`, waits until that launch returns, since Flume asks sluice for the bolt's branch at its launch (Flume's 202) and a launch side by side with it could find no base. These are the only orders kept between launches. Not taken: holding every launch on its pull's earlier ones, which brings back the serial pass.

## Decision 5: A preview starts once every launch is under way

The pass starts the pull's previews after its other wants, so no preview's command starts ahead of a launch its pull offers (core.55). A done waits while a bring-up of its repository and revision is in flight.

## Decision 6: What was let go while it ran

The control plane's answer to a send that lets go or refuses a want in flight, or the launch an act in flight acts on, is kept by the part until those commands return; their answers are then said in one line and dropped. Ids let go are forgotten once nothing of them is in flight.

## Decision 7: `run --once` and the stop

`Carrier::pass`, which `run --once` and the tests' bench use, opens its own scope and waits for every command it started, taking their answers in the order they were started and starting what waited on them, before it carries the statuses and sends. A running part that is stopped begins no pass, and `Carrier::finish` waits for each command in flight, takes its answers and sends them, starting nothing more.

## Decision 8: The fakes

The fakes' Flume keeps its state under a mutex, and can hold a launch or an act until a condition on the calls made so far holds, so a test proves that one command ran while another did. The stub programs claim each call's number by making a folder, so two calls side by side never write the same file.
