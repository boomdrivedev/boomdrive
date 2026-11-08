//! Each launch, act and command is carried out once, across a restart
//! (design Decision 11; core.10; the work-into-flume spec's "Each launch,
//! act and command is carried out once"; Task notes 4.4). The report is
//! saved whole by rename after each act and before the next, read at the
//! start of every pass, and taken from the control plane where the file
//! is missing; a launch whose answer was lost is found by its mark. Each
//! pass here runs a carrier of its own, as a runner started again does:
//! all it keeps between passes is the report file. Each part keeps its
//! share of the one file, the entries its control plane owns, and keeps
//! every other part's share as it stands (record 0026; design Decision
//! 5).

mod fakes;

use boomdrive::facts;
use boomdrive::flume::{Act, Flume, LaunchArgs, Reply, Unrun};
use boomdrive_control::fake::Fake;
use boomdrive_control::{
    ActSort, Acted, Answer, At, ControlPlane, Id, Lease, Outcome, Report, Want,
};
use fakes::{act, fixture, offer, place_of, Bench, FakeFlume};
use serde_json::Value;
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};

fn retry() -> ActSort {
    ActSort::Retry {
        stage: Some("code".into()),
        words: Some("use the second option".into()),
        submission: false,
    }
}

/// The work-into-flume spec's "A retry not made twice": a pass retries a
/// place and its report then fails, and the runner starts again from its
/// saved report with the same act pulled.
#[test]
fn a_retry_not_made_twice() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("madswan/12", "a"))]);
    bench.pass(&mut plane);
    plane.set_wants(vec![
        Want::Offer(offer("madswan/12", "a")),
        act("madswan/20", "madswan/12", retry()),
    ]);
    plane.refuse_reports("the state repository cannot be pushed");
    bench.pass(&mut plane);
    assert_eq!(bench.flume.acts().len(), 1);
    assert_eq!(
        bench.report().acts[&Id::new("madswan/20")].outcome,
        Outcome::Done
    );
    plane.take_reports();
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(bench.flume.acts().len(), 1, "no second retry");
    assert_eq!(lines, [] as [String; 0]);
    let reported = plane.reports().last().unwrap();
    assert_eq!(reported.acts[&Id::new("madswan/20")].outcome, Outcome::Done);
}

/// A Flume that reads the report file at every act it is asked, so a test
/// sees what the file held when each act ran.
struct Watching<'a> {
    flume: &'a FakeFlume,
    file: PathBuf,
    held: RefCell<Vec<Report>>,
}

impl Flume for Watching<'_> {
    fn launch(&self, args: &LaunchArgs, brief_dir: &Path) -> Result<Reply, Unrun> {
        self.flume.launch(args, brief_dir)
    }

    fn act(&self, place: &str, act: &Act) -> Result<Reply, Unrun> {
        let held = boomdrive::report::read(&self.file).unwrap().unwrap();
        self.held.borrow_mut().push(held);
        self.flume.act(place, act)
    }

    fn status(&self) -> Result<Reply, Unrun> {
        self.flume.status()
    }

    fn status_of(&self, container_dir: Option<&Path>, place: &str) -> Result<Reply, Unrun> {
        self.flume.status_of(container_dir, place)
    }
}

/// The report is saved after each act, before the next is carried out.
#[test]
fn the_report_is_saved_after_each_act() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    plane.set_wants(vec![
        Want::Offer(offer("a", "a")),
        act("a/1", "a", retry()),
        act("a/2", "a", ActSort::Priority(3)),
    ]);
    let watching = Watching {
        flume: &bench.flume,
        file: bench.report_file(),
        held: RefCell::new(Vec::new()),
    };
    bench.pass_over(&watching, &mut plane);
    let held = watching.held.borrow();
    assert_eq!(held.len(), 2);
    assert!(!held[0].acts.contains_key(&Id::new("a/1")));
    assert_eq!(held[1].acts[&Id::new("a/1")].outcome, Outcome::Done);
    let names: Vec<_> = fs::read_dir(bench.control_dir())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| name != "briefs")
        .collect();
    assert_eq!(names, ["report.json"], "no partial file is left");
}

/// The work-into-flume spec's "A launch found by its mark from before the
/// rename", over Flume 0.4.1's own listing, read before the rename (record
/// 0016): `flywheel:scratch/4` marks `fix--tabs`, and `flywheel:scratch/1`
/// the record of `fix--parser`, tidied with an end pending, which is found
/// at the place it stood. `a_launch_whose_answer_was_lost` finds a launch
/// by the mark `boomdrive:<id>`.
#[test]
fn a_launch_found_by_its_mark() {
    let bench = Bench::new();
    bench
        .flume
        .listing(Ok(Reply::Given(fixture("status.json"))));
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("scratch/4", "tabs")),
        Want::Offer(offer("scratch/1", "parser")),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    let listing: Value = serde_json::from_str(&fixture("status.json")).unwrap();
    let tabs = facts::marked(&listing, "flywheel:scratch/4")
        .and_then(facts::location)
        .unwrap();
    let parser = facts::marked(&listing, "flywheel:scratch/1")
        .and_then(facts::location)
        .unwrap();
    let (parser, at) = parser.rsplit_once('@').unwrap();
    assert!(at.bytes().all(|byte| byte.is_ascii_digit()));
    let report = bench.report();
    for (id, place) in [("scratch/4", tabs), ("scratch/1", parser)] {
        let answer = &report.launches[&Id::new(id)].answer;
        assert_eq!(
            (answer.outcome, answer.place_dir.as_deref()),
            (Outcome::Launched, Some(place))
        );
    }
    assert_eq!(bench.flume.launches(), []);
    assert_eq!(
        lines[0],
        format!("scratch/4: launched at {tabs}, as its mark flywheel:scratch/4 shows")
    );
}

/// The report file lost, and a launch's mark in Flume's listing: it is
/// answered launched at its place, and launched no second time.
#[test]
fn a_launch_whose_answer_was_lost() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    plane.refuse_reports("offline");
    bench.pass(&mut plane);
    fs::remove_file(bench.report_file()).unwrap();
    plane.take_reports();
    bench.pass(&mut plane);
    assert_eq!(bench.flume.launches().len(), 1);
    let answer = &bench.report().launches[&Id::new("a")].answer;
    assert_eq!(answer.place_dir.as_deref(), Some(place_of("a").as_str()));
}

/// Task notes 4.4's third case: the report file missing and the pull's
/// report holding an act, which is not carried out again.
#[test]
fn an_act_the_control_plane_holds_is_not_made_twice() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        act("a/retry", "a", retry()),
    ]);
    bench.pass(&mut plane);
    assert_eq!(bench.flume.acts().len(), 1);
    fs::remove_file(bench.report_file()).unwrap();
    bench.pass(&mut plane);
    assert_eq!(bench.flume.acts().len(), 1, "no second retry");
    assert_eq!(bench.flume.launches().len(), 1, "no second launch");
    let report = bench.report();
    assert_eq!(report.acts[&Id::new("a/retry")].outcome, Outcome::Done);
    assert_eq!(Some(&report), plane.held());
}

/// A report file that does not read as a report is named, and nothing is
/// carried out against it, so no act is made twice.
#[test]
fn a_report_file_that_does_not_read() {
    let bench = Bench::new();
    fs::create_dir_all(bench.control_dir()).unwrap();
    fs::write(bench.report_file(), "{ not a report").unwrap();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(!passed.reported);
    assert_eq!(lines.len(), 1);
    assert!(
        lines[0].ends_with(": nothing carried this pass"),
        "{lines:?}"
    );
    assert_eq!(bench.flume.calls(), []);
    assert_eq!(plane.hellos(), []);
}

/// A report holding the launch `launch` at its place, and the retry
/// `retry` of it done, each where it is named.
fn reported(launch: &str, retry: Option<&str>) -> Report {
    let at = At::parse("2026-11-07T10:00:00Z").unwrap();
    let mut report = Report::default();
    report.answer(
        &Id::new(launch),
        Lease {
            number: 1,
            expires: None,
        },
        Answer {
            outcome: Outcome::Launched,
            place_dir: Some(place_of(launch.rsplit('/').next().unwrap())),
            words: None,
            at,
        },
    );
    if let Some(retry) = retry {
        report.acts.insert(
            Id::new(retry),
            Acted {
                launch: Id::new(launch),
                lease: 1,
                outcome: Outcome::Done,
                words: None,
                at,
            },
        );
    }
    report
}

/// Whether `id` is under `<label>/`.
fn of(label: &str) -> impl Fn(&Id) -> bool + '_ {
    move |id| id.as_str().starts_with(&format!("{label}/"))
}

/// The labels the report file names in `parts`, none where it names
/// none.
fn parts_named(bench: &Bench) -> Option<Vec<String>> {
    let text = fs::read_to_string(bench.report_file()).unwrap();
    let file: Value = serde_json::from_str(&text).unwrap();
    let parts = file.get("parts")?.as_array()?;
    Some(
        parts
            .iter()
            .map(|label| label.as_str().unwrap().to_string())
            .collect(),
    )
}

/// The ids of every entry the report file holds.
fn held_ids(bench: &Bench) -> Vec<String> {
    let report = bench.report();
    let mut ids: Vec<String> = report
        .launches
        .keys()
        .chain(report.acts.keys())
        .map(Id::to_string)
        .collect();
    ids.sort();
    ids
}

/// The part `label`'s control plane, wanting the launch `<label>/1` and,
/// where `retry`, its retry `<label>/2`.
fn part(label: &str, retry: bool) -> Fake {
    let mut plane = Fake::owning(&format!("{label}/"));
    let launch = format!("{label}/1");
    let mut wants = vec![Want::Offer(offer(&launch, label))];
    if retry {
        wants.push(act(&format!("{label}/2"), &launch, self::retry()));
    }
    plane.set_wants(wants);
    plane
}

/// The work-into-flume spec's "A part's save keeps every other part's
/// share": the file holds `home`'s launch `home/1` and `work`'s act
/// `work/2`, and `home` saves its share holding `home/1` and `home/3`. The
/// file holds all three, and names `home` and `work` in `parts`.
#[test]
fn a_parts_save_keeps_every_other_parts_share() {
    let bench = Bench::new();
    let file = &bench.report_file;
    file.save_share("home", &of("home"), &reported("home/1", None))
        .unwrap();
    file.save_share("work", &of("work"), &reported("work/1", Some("work/2")))
        .unwrap();
    let mut home = Fake::owning("home/");
    home.set_wants(vec![
        Want::Offer(offer("home/1", "home")),
        Want::Offer(offer("home/3", "three")),
    ]);

    let (_, lines) = bench.pass_of("home", &bench.flume, &mut home, &|| false);

    assert_eq!(
        lines,
        [format!("home/3: launched at {}", place_of("three"))]
    );
    assert_eq!(held_ids(&bench), ["home/1", "home/3", "work/1", "work/2"]);
    assert_eq!(
        parts_named(&bench),
        Some(vec!["home".into(), "work".into()])
    );
    assert_eq!(home.reports().len(), 1);
    let sent: Vec<String> = home.reports()[0]
        .launches
        .keys()
        .map(Id::to_string)
        .collect();
    assert_eq!(sent, ["home/1", "home/3"], "home reports its own share");
}

/// The work-into-flume spec's "A file from the release before": the file
/// names no `parts` and holds `work`'s retry `work/2` done, and `work`
/// pulls `work/2` again. It is answered from the file, and no `flume`
/// command is run for it.
#[test]
fn a_file_from_the_release_before() {
    let bench = Bench::new();
    let mut file = reported("work/1", Some("work/2"));
    let home = reported("home/1", None);
    file.launches.extend(home.launches);
    boomdrive::report::save(&bench.report_file(), &file).unwrap();
    let mut work = part("work", true);

    let (_, lines) = bench.pass_of("work", &bench.flume, &mut work, &|| false);

    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(bench.flume.acts(), [], "no flume command for work/2");
    assert_eq!(bench.flume.launches(), []);
    assert_eq!(
        bench.report(),
        file,
        "the file holds every part's share still"
    );
    assert_eq!(parts_named(&bench), None);
    let sent = &work.reports()[0];
    assert_eq!(sent.acts[&Id::new("work/2")].outcome, Outcome::Done);
    assert!(!sent.launches.contains_key(&Id::new("home/1")));
}

/// The work-into-flume spec's "A part the file does not hold": the file
/// names only `home` in `parts`, and `work`'s pull gives back a report
/// holding `work/2` done. `work` takes `work/2` from its pull's report,
/// answers it from there, and saves it with `work` named in `parts`.
#[test]
fn a_part_the_file_does_not_hold() {
    let bench = Bench::new();
    bench
        .report_file
        .save_share("home", &of("home"), &reported("home/1", None))
        .unwrap();
    let mut work = part("work", true);
    work.report(&reported("work/1", Some("work/2"))).unwrap();

    let (_, lines) = bench.pass_of("work", &bench.flume, &mut work, &|| false);

    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(bench.flume.acts(), [], "no flume command for work/2");
    assert_eq!(bench.flume.launches(), []);
    assert_eq!(held_ids(&bench), ["home/1", "work/1", "work/2"]);
    assert_eq!(
        parts_named(&bench),
        Some(vec!["home".into(), "work".into()])
    );
}
