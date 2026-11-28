//! Each part of the control plane carried on its own by the runner's loop,
//! on its edge's paused clock: its control passes on a timer of their own,
//! one at once when its implementation wakes it and the timer kept from
//! the woken pass, a quiet pass saying nothing, a part's reach said once
//! each way, a slow or a failing part delaying no other, a stop with a
//! part's pass under way, the ticks the first passes share, the
//! configuration broken or its poll interval changed while running, a
//! keeping pass under way, and the leases that hasten a pass or set none
//! sooner (the running spec's "On the control timer", "Woken by the
//! control plane", "One part woken", "Three passes unread, then read",
//! "Each part's reach its own", "A slow part", "A failing part", "Stopped
//! with a part's pass under way", "Passes back to back share their ticks",
//! "The configuration broken while running", "A keeping pass under way",
//! "A lapsed lease", "A pass that did not report" and "Each part on its
//! own timer", and the configuration spec's "A poll interval changed
//! while running"; core.13, core.22, core.23, core.49, core.50). The
//! runner starts over a keeper and tools held in memory, so no test starts
//! a process, git or a socket, and its clock moves only when a test moves
//! it (logic-is-tested-as-units-and-few-tests-run-the-binary;
//! the-runtime-flavour-is-chosen-in-main-and-tests-pause-tokios-clock).

mod fakes;
mod held;

use boomdrive::report;
use boomdrive_control::fake::Fake;
use boomdrive_control::{At, Id, Outcome};
use fakes::{FakeFlume, KEPT};
use held::{
    carrying, holding, launch, part_holding, patiently, slow, told, Clock, Running, HOUR, QUIET,
    SECOND,
};
use std::thread;
use std::time::Duration;

/// The running spec's "On the control timer", over a part: `home`'s first
/// control pass follows the first keeping pass, and the next come each
/// `poll_interval`, a second, while the keeping pass keeps its own
/// `fetch_interval`, an hour.
#[test]
fn control_passes_on_their_own_timer() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::owning("home/"), 0);
    let running = Running::carrying(clock, "1s", vec![("home", home)], FakeFlume::new());

    let first = heard.pull();
    running.advance(SECOND);
    let second = heard.pull();
    running.advance(SECOND);
    let third = heard.pull();

    let kept = running.kept();
    assert_eq!(kept.len(), 1, "no keeping pass but the first");
    assert!(first >= kept[0], "the first control pass follows it");
    assert_eq!(second - first, SECOND);
    assert_eq!(third - first, 2 * SECOND);
    running.stop();
}

/// The running spec's "Woken by the control plane": woken through the
/// wake its implementation was handed for `home` at its opening, a
/// control pass of `home` runs at once, and none follows it within the
/// hour's interval.
#[test]
fn woken_by_the_control_plane() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::owning("home/"), 0);
    let running = Running::carrying(clock, "1h", vec![("home", home)], FakeFlume::new());
    let first = heard.pull();
    heard.report();

    running.wake("home");
    let woken = heard.pull();
    heard.report();
    running.advance(HOUR - SECOND);
    let quiet = heard.quiet();
    running.advance(SECOND);
    let next = heard.pull();

    assert_eq!(woken, first, "the woken pass runs at once");
    assert!(quiet, "no control pass within the hour");
    assert_eq!(next - woken, HOUR);
    running.stop();
}

/// The running spec's "One part woken": the wake handed for `home` makes
/// `home`'s control pass at once, and `work`'s next pass begins when its
/// own timer says, an hour on.
#[test]
fn one_part_woken() {
    let clock = Clock::paused();
    let (home, home_heard) = told(&clock, Fake::owning("home/"), 0);
    let (work, work_heard) = told(&clock, Fake::owning("work/"), 0);
    let running = Running::carrying(
        clock,
        "1h",
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
    );
    let first = home_heard.pull();
    let work_first = work_heard.pull();
    home_heard.report();

    running.wake("home");
    let woken = home_heard.pull();
    let quiet = work_heard.quiet();
    running.advance(HOUR);
    let work_next = work_heard.pull();

    assert_eq!(woken, first, "home's woken pass runs at once");
    assert!(quiet, "work is not woken");
    assert_eq!(work_next - work_first, HOUR, "work's own timer");
    running.stop();
}

/// The running spec's "Woken by the control plane", its timer: the pass
/// after the woken one comes `poll_interval`, four seconds, after the
/// woken pass's start, not after the start of the pass before it.
#[test]
fn the_timer_kept_from_the_woken_pass() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::owning("home/"), 0);
    let running = Running::carrying(clock, "4s", vec![("home", home)], FakeFlume::new());
    let first = heard.pull();
    heard.report();
    running.advance(SECOND);

    running.wake("home");
    let woken = heard.pull();
    heard.report();
    running.advance(3 * SECOND);
    let quiet = heard.quiet();
    running.advance(SECOND);
    let next = heard.pull();

    assert_eq!(woken - first, SECOND);
    assert!(quiet, "no pass four seconds after the one before the wake");
    assert_eq!(next - woken, 4 * SECOND);
    running.stop();
}

/// A control pass that changes nothing says nothing (design Decision 5):
/// over a control plane wanting nothing, the runner says nothing until it
/// stops.
#[test]
fn a_quiet_control_pass_says_nothing() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::owning("home/"), 0);
    let running = Running::carrying(clock, "1s", vec![("home", home)], FakeFlume::new());
    heard.pull();
    running.advance(SECOND);
    heard.pull();

    let said: Vec<String> = running.stop().into_iter().map(|(_, line)| line).collect();

    assert_eq!(said, ["boomdrive: stopped"]);
}

/// The running spec's "Three passes unread, then read": `home` refuses
/// three pulls in a row with `no network`, then gives the fourth. One
/// line names `home` unread, and one, at the fourth pass, says it is read
/// again; the passes after say nothing of it.
#[test]
fn three_passes_unread_then_read() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::new(), 3);
    let running = Running::carrying(clock, "1s", vec![("home", home)], FakeFlume::new());
    let first = heard.pull();
    let mut pulls = vec![first];
    for _ in 0..3 {
        running.advance(SECOND);
        pulls.push(heard.pull());
    }

    let (read_again, said) = running.until("boomdrive: home: the control plane is read again");
    running.advance(SECOND);
    heard.pull();
    running.advance(SECOND);
    heard.pull();
    let after: Vec<String> = running.stop().into_iter().map(|(_, line)| line).collect();

    let reach: Vec<&String> = said
        .iter()
        .filter(|line| line.starts_with("boomdrive: home: the control plane"))
        .collect();
    assert_eq!(
        reach,
        ["boomdrive: home: the control plane could not be read: no network"]
    );
    assert_eq!(read_again, pulls[3], "said at the fourth pass");
    assert_eq!(after, ["boomdrive: stopped"]);
}

/// The running spec's "Each part's reach its own": `home` refuses every
/// pull, and `work` gives every pull. One line names `home` as unread,
/// and no line names `work`'s reach.
#[test]
fn each_parts_reach_its_own() {
    let clock = Clock::paused();
    let mut refusing = Fake::owning("home/");
    refusing.refuse_pulls("no network");
    let (home, home_heard) = told(&clock, refusing, 0);
    let (work, work_heard) = told(&clock, Fake::owning("work/"), 0);
    let running = Running::carrying(
        clock,
        "1s",
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
    );
    for _ in 0..3 {
        home_heard.pull();
        work_heard.pull();
        running.advance(SECOND);
    }

    let said: Vec<String> = running.stop().into_iter().map(|(_, line)| line).collect();

    let reach: Vec<&String> = said
        .iter()
        .filter(|line| line.contains(": the control plane "))
        .collect();
    assert_eq!(
        reach,
        ["boomdrive: home: the control plane could not be read: no network"],
        "{said:#?}"
    );
}

/// The running spec's "A failing part": `home` refuses every pull and
/// report with `no network`, and `work` offers a launch whose stream has
/// room. `work`'s launch is made and reported, and one line says `home`
/// cannot be read.
#[test]
fn a_failing_part() {
    let clock = Clock::paused();
    let mut failing = Fake::owning("home/");
    failing.refuse_pulls("no network");
    failing.refuse_reports("no network");
    let (home, home_heard) = told(&clock, failing, 0);
    let (work, work_heard) = told(&clock, part_holding("work", vec![launch("work/1")]), 0);
    let running = Running::carrying(
        clock,
        "1s",
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
    );

    let (_, launched, mut said) = running.until_starting("work/1: launched at ");
    work_heard.report();
    home_heard.pull();
    running.advance(SECOND);
    home_heard.pull();

    said.push(launched);
    said.extend(running.stop().into_iter().map(|(_, line)| line));
    let unread = "boomdrive: home: the control plane could not be read: no network";
    let named: Vec<&String> = said
        .iter()
        .filter(|line| line.starts_with("boomdrive: home") || line.starts_with("home/"))
        .collect();
    assert_eq!(named, [unread], "{said:#?}");
}

/// The running spec's "A slow part": `home`'s pull answers only once the
/// test lets it go, and `work` offers a launch whose stream has room.
/// `work`'s launch is made and reported while `home`'s pull waits, and
/// `work`'s next control pass begins `poll_interval` after the start of
/// its last.
#[test]
fn a_slow_part() {
    let clock = Clock::paused();
    let (home, home_heard, home_go) = slow(&clock, Fake::owning("home/"));
    let (work, work_heard) = told(&clock, part_holding("work", vec![launch("work/1")]), 0);
    let running = Running::carrying(
        clock,
        "2s",
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
    );
    let began = home_heard.pull();

    let (launched, _, _) = running.until_starting("work/1: launched at ");
    let reported = work_heard.report();
    let first = work_heard.pull();
    running.advance(2 * SECOND);
    let second = work_heard.pull();

    assert_eq!(launched, began, "launched while home's pull waits");
    assert_eq!(reported, launched);
    assert_eq!(second - first, 2 * SECOND, "work's own timer");
    assert!(
        home_heard.reports.try_recv().is_err(),
        "home's pass still waits"
    );
    home_go.send(()).expect("home's pull let go");
    drop(home_go);
    running.stop();
}

/// The running spec's "Stopped with a part's pass under way": stopped
/// while `home`'s control pass waits on its pull and `work` waits for its
/// next pass, `work` begins no pass, `home`'s pass ends with its report
/// saved, the runner says it stopped, and no pass begins after that line.
#[test]
fn stopped_with_a_parts_pass_under_way() {
    let clock = Clock::paused();
    let (home, home_heard, home_go) = slow(&clock, part_holding("home", vec![launch("home/1")]));
    let (work, work_heard) = told(&clock, Fake::owning("work/"), 0);
    let running = Running::carrying(
        clock,
        "1h",
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
    );
    home_heard.pull();
    work_heard.pull();
    work_heard.report();

    running.stopper.stop();
    home_go.send(()).expect("home's pull let go");
    let reported = home_heard.report();
    let data = running.data.clone();
    let (said, _home) = running.stopped();

    let (stopped, last) = said.last().cloned().expect("a line");
    assert_eq!(last, "boomdrive: stopped", "{said:#?}");
    assert!(reported <= stopped, "home's pass ended before the stop");
    assert!(
        work_heard.pulls.try_recv().is_err(),
        "work began no pass once stopped"
    );
    assert!(
        home_heard.pulls.try_recv().is_err(),
        "home began no pass once stopped"
    );
    let file = report::file(&report::control_dir(&data));
    let saved = report::read(&file).unwrap().expect("the report file");
    assert_eq!(
        saved.launches[&Id::new("home/1")].answer.outcome,
        Outcome::Launched
    );
}

/// The running spec's "Passes back to back share their ticks": the first
/// keeping pass and the first control pass run back to back, and one tick
/// of the inbox `home` follows them; no other runs before the next turn,
/// an hour on. Flume is asked through the runner's tools, which hold no
/// settle of Flume's, so none can be asked (core.13).
#[test]
fn passes_back_to_back_share_their_ticks() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::owning("home/"), 0);
    let text = format!("inboxes = [\"home\"]\n{}", carrying("1h"));
    let running = Running::over(clock, &text, vec![("home", home)], FakeFlume::new(), None);
    heard.pull();

    let ticked = patiently(|| !running.ticks().is_empty());
    thread::sleep(QUIET);

    assert!(ticked, "no tick followed the passes");
    assert_eq!(running.ticks(), ["home"]);
    running.stop();
}

/// The running spec's "The configuration broken while running": `home`'s
/// next control pass names the file and acts on nothing, and once the
/// file is valid again, the pass after acts as before.
#[test]
fn the_configuration_broken_while_running() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::new(), 0);
    let running = Running::carrying(clock, "1s", vec![("home", home)], FakeFlume::new());
    heard.pull();
    heard.report();
    let valid = carrying("1s");
    running.rewrite(&valid.replace(&format!("{KEPT:?}"), "\"flume\""));
    let broken = format!(
        "boomdrive: home: {}:2: `accounts.own.drives.home.repositories` entry \"flume\": an \
         address is `owner/name`: nothing carried this pass",
        running.config_file().display()
    );

    running.advance(SECOND);
    running.until(&broken);
    running.advance(SECOND);
    running.until(&broken);
    let unread = heard.pulls.try_iter().count();
    running.rewrite(&valid);
    running.advance(SECOND);
    heard.pull();

    assert_eq!(unread, 0, "no pull while the file does not read");
    running.stop();
}

/// The configuration spec's "A poll interval changed while running": the
/// control passes keep the interval the runner started with, a second.
#[test]
fn a_poll_interval_changed_while_running() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::new(), 0);
    let running = Running::carrying(clock, "1s", vec![("home", home)], FakeFlume::new());
    let first = heard.pull();
    heard.report();

    running.rewrite(&carrying("5m"));
    running.advance(SECOND);
    let second = heard.pull();
    running.advance(SECOND);
    let third = heard.pull();

    assert_eq!(second - first, SECOND);
    assert_eq!(third - second, SECOND);
    running.stop();
}

/// The running spec's "A keeping pass under way": a keeping pass is held
/// under the clone's lock of [`KEPT`], and `work`'s control passes fall
/// due meanwhile: they pull, answer and report while it is held.
#[test]
fn a_keeping_pass_under_way() {
    let clock = Clock::paused();
    let (work, heard) = told(&clock, part_holding("work", vec![launch("work/1")]), 0);
    let text = carrying("1s").replace("fetch_interval = \"1h\"", "fetch_interval = \"3s\"");
    let running = Running::over(clock, &text, vec![("work", work)], FakeFlume::new(), None);
    // `work/1` is launched before the keeping pass is held: a launch into
    // the clone waits on its keep (core.49; record 0026).
    running.until_starting("work/1: launched at ");
    running.hold_keeping();
    running.advance(3 * SECOND);
    running.keeping_held();
    // Every pull and report heard so far may have come before the hold;
    // those that come now come while it holds.
    heard.pulls.try_iter().for_each(drop);
    heard.reports.try_iter().for_each(drop);

    running.advance(SECOND);
    heard.pull();
    heard.report();
    running.advance(SECOND);
    heard.pull();
    heard.report();

    assert_eq!(running.kept().len(), 2, "the second keeping pass is held");
    running.let_keeping_go();
    running.stop();
}

/// The running spec's "A lapsed lease", over the runner: the report file
/// holds a launch under a lease that lapsed, and the control plane cannot
/// be reached, so the next control pass comes `poll_interval`, two
/// seconds, after the one before, never back to back.
#[test]
fn a_lapsed_lease_sets_no_pass_sooner() {
    let clock = Clock::paused();
    let mut fake = Fake::new();
    fake.refuse_pulls("no network");
    let (home, heard) = told(&clock, fake, 0);
    let lapsed = holding("held", At::from(At::now().time() - Duration::from_secs(60)));
    let running = Running::over(
        clock,
        &carrying("2s"),
        vec![("home", home)],
        FakeFlume::new(),
        Some(lapsed),
    );
    let first = heard.pull();

    let quiet = heard.quiet();
    running.advance(2 * SECOND);
    let second = heard.pull();

    assert!(quiet, "no control pass back to back");
    assert_eq!(second - first, 2 * SECOND);
    running.stop();
}

/// The running spec's "A pass that did not report", over the runner: the
/// report file holds a launch under a lease that lapses in seconds, and
/// the control plane takes no report, so no pass comes sooner than
/// `poll_interval`, an hour; hastened by the lease, the next would come
/// within half the time it has left, two and a half seconds.
#[test]
fn a_pass_that_did_not_report_sets_no_pass_sooner() {
    let clock = Clock::paused();
    let mut fake = Fake::new();
    fake.refuse_reports("no network");
    let (home, heard) = told(&clock, fake, 0);
    let lapsing = holding("held", At::from(At::now().time() + Duration::from_secs(5)));
    let running = Running::over(
        clock,
        &carrying("1h"),
        vec![("home", home)],
        FakeFlume::new(),
        Some(lapsing),
    );
    heard.pull();

    for _ in 0..4 {
        running.advance(SECOND);
    }

    assert!(heard.quiet(), "a pass that did not report hastens none");
    running.stop();
}

/// The running spec's "Each part on its own timer": `work` holds a lease
/// expiring eight seconds after a pass of `work` that reported, so
/// `work`'s next pass comes by the midpoint of the time left, and
/// `home`'s keeps its own `poll_interval`, eight seconds.
#[test]
fn each_part_on_its_own_timer() {
    let clock = Clock::paused();
    let (home, home_heard) = told(&clock, Fake::owning("home/"), 0);
    let (work, work_heard) = told(&clock, Fake::owning("work/"), 0);
    let lease = holding(
        "work/held",
        At::from(At::now().time() + Duration::from_secs(8)),
    );
    let running = Running::over(
        clock,
        &carrying("8s"),
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
        Some(lease),
    );
    let home_first = home_heard.pull();
    let work_first = work_heard.pull();
    work_heard.report();

    // A step at a time, so a pass that ends as the clock moves is due
    // within a step of when it would be had the clock stood.
    for _ in 0..5 {
        running.advance(SECOND);
    }
    let work_second = work_heard.pull();
    let home_quiet = home_heard.quiet();
    for _ in 0..3 {
        running.advance(SECOND);
    }
    let home_second = home_heard.pull();

    let work_apart = work_second - work_first;
    assert!(
        work_apart < Duration::from_secs(6),
        "{work_apart:?} between work's passes, hastened by its lease"
    );
    assert!(home_quiet, "home's pass is not hastened by work's lease");
    assert_eq!(home_second - home_first, 8 * SECOND, "home's own timer");
    running.stop();
}
