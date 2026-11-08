//! Room, read from the pass's one `flume --json status` and counted down
//! as the pass launches: a launch is made only while the stream it runs
//! in has room, or the host where it names none (design Decision 8;
//! core.9; the work-into-flume spec's "Room is read from Flume's status
//! and counted down within a pass" and "A launch is made only while the
//! stream it runs in has room"; Task notes 4.2).

mod fakes;

use boomdrive::flume::Reply;
use boomdrive_control::fake::Fake;
use boomdrive_control::{Id, Offer, Outcome, Want};
use fakes::{offer, Bench};
use serde_json::{json, Value};

/// A host part whose own room is `room`, holding `streams`.
fn host(room: Value, streams: Value) -> Value {
    json!({ "running": 1, "bound": null, "room": room, "queue": [], "streams": streams })
}

/// A stream `name` whose room is `room`, its queue holding `queued`
/// places.
fn stream(name: &str, room: Value, queued: usize) -> Value {
    let queue: Vec<Value> = (0..queued)
        .map(|n| json!({ "place": format!("/places/queued-{n}"), "stage": "code", "stream": name }))
        .collect();
    json!({ "name": name, "priority": 0, "running": 1, "bound": 1, "room": room, "queue": queue })
}

/// An offer `id` named `name` that runs in `stream`.
fn in_stream(id: &str, name: &str, stream: Option<&str>) -> Want {
    let mut offer: Offer = offer(id, name);
    offer.launch.stream = stream.map(str::to_string);
    Want::Offer(offer)
}

/// The outcome the report file holds for `launch`, where it answers it.
fn outcome(bench: &Bench, launch: &str) -> Option<Outcome> {
    let report = boomdrive::report::read(&bench.report_file()).unwrap()?;
    Some(report.launches.get(&Id::new(launch))?.answer.outcome)
}

/// The work-into-flume spec's "Room for one".
#[test]
fn room_for_one() {
    let bench = Bench::new();
    bench
        .flume
        .host(host(json!(null), json!([stream("work", json!(1), 0)])));
    let mut plane = Fake::holding(vec![
        in_stream("home/1", "a", Some("work")),
        in_stream("home/2", "b", Some("work")),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), Some(Outcome::Launched));
    assert_eq!(outcome(&bench, "home/2"), None);
    assert_eq!(bench.flume.launches().len(), 1);
    assert_eq!(lines, ["home/1: launched at /places/a"]);
}

/// The work-into-flume spec's "A stream with a queue".
#[test]
fn a_stream_with_a_queue() {
    let bench = Bench::new();
    bench
        .flume
        .host(host(json!(null), json!([stream("work", json!(2), 1)])));
    let mut plane = Fake::holding(vec![in_stream("home/1", "a", Some("work"))]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), None);
    assert_eq!(bench.flume.commands(), []);
    assert_eq!(lines, [] as [String; 0]);
}

/// The work-into-flume spec's "A host with no room".
#[test]
fn a_host_with_no_room() {
    let bench = Bench::new();
    bench
        .flume
        .host(host(json!(0), json!([stream("work", json!(null), 0)])));
    let mut plane = Fake::holding(vec![in_stream("home/1", "a", Some("work"))]);
    bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), None);
    assert_eq!(bench.flume.commands(), []);
}

/// The work-into-flume spec's "Unbounded".
#[test]
fn unbounded() {
    let bench = Bench::new();
    bench.flume.host(host(
        json!(null),
        json!([stream("default", json!(null), 0)]),
    ));
    let mut plane = Fake::holding(vec![
        in_stream("home/1", "a", Some("default")),
        in_stream("home/2", "b", Some("default")),
    ]);
    bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), Some(Outcome::Launched));
    assert_eq!(outcome(&bench, "home/2"), Some(Outcome::Launched));
}

/// The work-into-flume spec's "A stream the host does not declare".
#[test]
fn a_stream_the_host_does_not_declare() {
    let bench = Bench::new();
    bench
        .flume
        .host(host(json!(null), json!([stream("work", json!(0), 0)])));
    let reason =
        "the stream nosuch the launch names is not declared by the host's configuration (143)";
    bench
        .flume
        .launch_answers("a", Ok(Reply::Refused(reason.to_string())));
    let mut plane = Fake::holding(vec![in_stream("home/1", "a", Some("nosuch"))]);
    bench.pass(&mut plane);
    let answer = &bench.report().launches[&Id::new("home/1")].answer;
    assert_eq!(
        (answer.outcome, answer.words.as_deref()),
        (Outcome::Refused, Some(reason))
    );
    assert_eq!(bench.flume.launches()[0].stream.as_deref(), Some("nosuch"));
}

/// The work-into-flume spec's "No stream named": judged by the host's
/// room alone, and launched with no `--stream`.
#[test]
fn no_stream_named() {
    let bench = Bench::new();
    bench
        .flume
        .host(host(json!(0), json!([stream("default", json!(null), 0)])));
    let mut plane = Fake::holding(vec![in_stream("home/1", "a", None)]);
    bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), None);
    assert_eq!(bench.flume.commands(), []);
    bench
        .flume
        .host(host(json!(1), json!([stream("work", json!(0), 3)])));
    bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), Some(Outcome::Launched));
    let launches = bench.flume.launches();
    assert_eq!(launches.len(), 1);
    assert_eq!(launches[0].stream, None);
}

/// Each launch counts against the host's room as well as its stream's.
#[test]
fn the_hosts_room_is_counted_down() {
    let bench = Bench::new();
    bench.flume.host(host(
        json!(1),
        json!([
            stream("work", json!(null), 0),
            stream("wide", json!(null), 0)
        ]),
    ));
    let mut plane = Fake::holding(vec![
        in_stream("home/1", "a", Some("work")),
        in_stream("home/2", "b", Some("wide")),
        in_stream("home/3", "c", None),
    ]);
    bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), Some(Outcome::Launched));
    assert_eq!(outcome(&bench, "home/2"), None);
    assert_eq!(outcome(&bench, "home/3"), None);
}

/// A room Flume's status gives as neither `null` nor a whole number reads
/// as none, and is no room.
#[test]
fn a_room_that_is_no_number_is_no_room() {
    let bench = Bench::new();
    bench
        .flume
        .host(host(json!(null), json!([stream("work", json!("1"), 0)])));
    let mut plane = Fake::holding(vec![in_stream("home/1", "a", Some("work"))]);
    bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), None);
    bench.flume.host(host(json!("unbounded"), json!([])));
    bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), None);
    assert_eq!(bench.flume.commands(), []);
}

/// The fixture `status.json`'s own streams: `wide` with room 1, `work`
/// with none and a queue.
#[test]
fn flumes_own_streams() {
    let bench = Bench::new();
    bench
        .flume
        .listing(Ok(Reply::Given(fakes::fixture("status.json"))));
    let mut plane = Fake::holding(vec![
        in_stream("home/1", "a", Some("work")),
        in_stream("home/2", "b", Some("wide")),
        in_stream("home/3", "c", Some("wide")),
        in_stream("home/4", "d", Some("default")),
    ]);
    bench.pass(&mut plane);
    let launched: Vec<_> = bench
        .flume
        .launches()
        .into_iter()
        .map(|args| args.name)
        .collect();
    assert_eq!(launched, ["b", "d"]);
}
