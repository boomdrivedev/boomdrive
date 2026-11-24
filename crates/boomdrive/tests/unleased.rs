//! Every place in a kept repository that a part did not launch goes to
//! that part with no lease, as Flume's listing gives it, and is kept in
//! no file (record 0054; the work-into-flume spec's "Every place in a kept
//! repository that a part did not launch is reported to it, with no
//! lease").

mod fakes;

use boomdrive_control::fake::Fake;
use boomdrive_control::{Id, Want};
use fakes::{offer, place_of, Bench, KEPT, PART};
use serde_json::{json, Value};

/// A place Flume holds in the clone `container`, named `name`, under the
/// mark `mark` where it has one.
fn place(name: &str, container: &str, mark: Option<&str>) -> Value {
    json!({
        "place": place_of(name),
        "repository": container,
        "kind": "note",
        "name": name,
        "mark": mark,
        "phase": "live",
        "ends": [],
    })
}

/// A bench keeping [`KEPT`] in its clone, and that clone's container.
fn bench() -> (Bench, String) {
    let bench = Bench::new();
    let container = bench.container(KEPT);
    std::fs::create_dir_all(&container).expect("the clone's container");
    bench.clones.ready(KEPT, &container);
    (bench, container.to_string_lossy().into_owned())
}

/// The work-into-flume spec's "A place started by hand": the place with
/// no mark in [`KEPT`]'s clone goes in the report with [`KEPT`] and its
/// entry as Flume printed it, beside the part's own launch, and the
/// report file holds no unleased place (core.35).
#[test]
fn a_place_started_by_hand() {
    let (bench, container) = bench();
    bench.flume.stand(place("by-hand", &container, None));
    let mut plane = Fake::holding(vec![Want::Offer(offer(&format!("{PART}/1"), "a"))]);
    bench.pass(&mut plane);

    let sent = plane.reports().last().expect("a report").clone();
    assert!(sent.launches.contains_key(&Id::new(format!("{PART}/1"))));
    let keys: Vec<&str> = sent.unleased.keys().map(String::as_str).collect();
    assert_eq!(keys, [place_of("by-hand").as_str()]);
    let unleased = &sent.unleased[&place_of("by-hand")];
    assert_eq!(unleased.repository, KEPT);
    assert!(unleased.status.listed);
    let entry: Value = serde_json::from_str(&unleased.status.text).expect("Flume's entry");
    assert_eq!(entry["name"], "by-hand");
    assert!(bench.report().unleased.is_empty(), "the file holds none");
}

/// The work-into-flume spec's "Another part's launch": the place under
/// the mark of `home/1`, which the part `work` did not launch, goes to
/// `work` with no lease, and not to `home`, whose share holds `home/1`.
#[test]
fn another_parts_launch() {
    let (bench, _) = bench();
    let mut home = Fake::owning("home/");
    home.set_wants(vec![Want::Offer(offer("home/1", "a"))]);
    bench.pass_of("home", &bench.flume, &mut home);
    let launched = place_of("a");
    let home_sent = home.reports().last().expect("home's report");
    assert!(home_sent.launches.contains_key(&Id::new("home/1")));
    assert!(!home_sent.unleased.contains_key(&launched), "{home_sent:?}");

    let mut work = Fake::owning("work/");
    bench.pass_of("work", &bench.flume, &mut work);
    let work_sent = work.reports().last().expect("work's report");
    assert!(work_sent.launches.is_empty());
    assert_eq!(work_sent.unleased[&launched].repository, KEPT);
}

/// The work-into-flume spec's "A place in no kept repository": a place
/// whose clone the runner neither keeps nor drains goes to no part.
#[test]
fn a_place_in_no_kept_repository() {
    let (bench, _) = bench();
    bench
        .flume
        .stand(place("elsewhere", "/somewhere/else", None));
    let mut plane = Fake::new();
    bench.pass(&mut plane);
    let sent = plane.reports().last().expect("a report");
    assert!(sent.unleased.is_empty(), "{sent:?}");
}

/// The work-into-flume spec's "Carried again as it stood": a place whose
/// entry is the same at two passes keeps the first pass's time.
#[test]
fn carried_again_as_it_stood() {
    let (bench, container) = bench();
    bench.flume.stand(place("by-hand", &container, None));
    let mut plane = Fake::new();
    bench.pass(&mut plane);
    let first = plane.reports().last().expect("a report").unleased[&place_of("by-hand")]
        .status
        .at;
    std::thread::sleep(std::time::Duration::from_millis(1100));
    bench.pass(&mut plane);
    let again = plane.reports().last().expect("a report").unleased[&place_of("by-hand")]
        .status
        .at;
    assert_eq!(first, again);
}
