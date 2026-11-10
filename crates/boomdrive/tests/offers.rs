//! Offers taken, left or refused for their repository and clone, and
//! launched into one `flume launch` whose answer is Flume's (design
//! Decisions 5 to 7 and 9; the work-into-flume spec's hello, repository,
//! launch and answer requirements; the control-plane-interface spec's
//! "A pull that cannot be read", "An offer's lease kept with its answer"
//! and "A report not sent"; the clones spec's "A clone's keep and a
//! launch into it take turns on the clone's lock" and "A launch finds its
//! clone again under the lock"; Task notes 4.1). A repository the
//! configuration drops while a place the runner launched there has not
//! ended is drained, and nothing more is launched there (record 0021).

mod fakes;

use boomdrive::config::Address;
use boomdrive::flume::{Cli, Reply};
use boomdrive_control::fake::Fake;
use boomdrive_control::{
    AgentSetting, Answer, At, Capacity, Command, CommandSort, Id, Inbox, Launched, Lease, Offer,
    Outcome, Report, ToolVersion, Want,
};
use fakes::{fixture, offer, place_of, Bench, Call, Stub, KEPT};
use serde_json::json;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs;
use std::rc::Rc;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

fn id(text: &str) -> Id {
    Id::new(text)
}

/// The answer the report file holds for `launch`.
fn outcome(bench: &Bench, launch: &str) -> Option<Outcome> {
    let report = boomdrive::report::read(&bench.report_file()).expect("the file reads")?;
    Some(report.launches.get(&id(launch))?.answer.outcome)
}

/// `words` as a call's arguments.
fn argv(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

/// The work-into-flume spec's "Repositories kept".
#[test]
fn repositories_kept() {
    let bench = Bench::new();
    bench.clones.not_yet("example/crew");
    let mut plane = Fake::new();
    bench.pass(&mut plane);
    assert_eq!(plane.hellos()[0].repositories, [KEPT]);
}

/// The work-into-flume spec's "A tool missing".
#[test]
fn a_tool_missing() {
    let bench = Bench::new();
    bench.tools.missing("herdr");
    let mut plane = Fake::holding(vec![Want::Offer(offer("home/1", "a"))]);
    bench.pass(&mut plane);
    let tools = &plane.hellos()[0].tool_versions;
    assert!(
        matches!(tools["herdr"], ToolVersion::Missing(_)),
        "{tools:?}"
    );
    assert_eq!(tools["flume"], ToolVersion::Printed("flume 1.0.0".into()));
    assert_eq!(
        tools.keys().collect::<Vec<_>>(),
        ["flume", "git", "herdr", "sluice"]
    );
    assert_eq!(outcome(&bench, "home/1"), Some(Outcome::Launched));
}

/// The work-into-flume spec's "Streams as Flume printed them".
#[test]
fn streams_as_flume_printed_them() {
    let bench = Bench::new();
    let streams = r#"[ {"name":"wide", "priority":5,"running":2,"bound":3,"room":1,"queue":[]},{"name":"work","priority":0,"running":1,"bound":1,"room":0,"queue":[]} ]"#;
    let listing = format!(
        r#"{{"status":{{"places":[],"host":{{"running": 3,"bound":null,"room": null,"queue":[],"streams":{streams},"pools":[{{"name":"pool-a","accounts":["pool-a-1"]}}],"addresses":[],"bases":[],"terminal_kept":false}}}}}}"#
    ) + "\n";
    bench.flume.listing(Ok(Reply::Given(listing)));
    let mut plane = Fake::new();
    bench.pass(&mut plane);
    let hello = &plane.hellos()[0];
    let Capacity::Printed(part) = &hello.capacity else {
        panic!("no capacity: {:?}", hello.capacity);
    };
    assert_eq!(
        (&*part.running, &*part.bound, &*part.room, &*part.streams),
        ("3", "null", "null", streams)
    );
    let text = serde_json::to_string(hello).unwrap();
    assert!(!text.contains("pool-a"), "{text}");
}

/// The work-into-flume spec's "A repository not kept".
#[test]
fn a_repository_not_kept() {
    let bench = Bench::new();
    let mut sluice = offer("home/1", "a");
    sluice.launch.repository = "agentplot/sluice".into();
    let mut plane = Fake::holding(vec![Want::Offer(sluice)]);
    let (_, lines) = bench.pass(&mut plane);
    let answer = &bench.report().launches[&id("home/1")].answer;
    assert_eq!(answer.outcome, Outcome::Refused);
    assert_eq!(
        answer.words.as_deref(),
        Some("this host's configuration does not keep agentplot/sluice")
    );
    assert_eq!(bench.flume.commands(), []);
    assert_eq!(
        lines,
        ["home/1: refused: this host's configuration does not keep agentplot/sluice"]
    );
}

/// The work-into-flume spec's "A clone not yet made".
#[test]
fn a_clone_not_yet_made() {
    let bench = Bench::new();
    bench.clones.not_yet("example/crew");
    let mut crew = offer("home/1", "a");
    crew.launch.repository = "example/crew".into();
    let mut plane = Fake::holding(vec![Want::Offer(crew)]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), None);
    assert_eq!(bench.flume.commands(), []);
    assert_eq!(lines, [] as [String; 0]);
    let container = bench.container("example/crew");
    bench.clones.ready("example/crew", &container);
    bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "home/1"), Some(Outcome::Launched));
    let launches = bench.flume.launches();
    assert_eq!(launches.len(), 1);
    assert_eq!(launches[0].container_dir, container);
}

/// The work-into-flume spec's "A unit's launch", over Flume's own command
/// line.
#[test]
fn a_units_launch() {
    let bench = Bench::new();
    let container = bench.container("example/crew");
    bench.clones.ready("example/crew", &container);
    let stub = Stub::new();
    stub.answers("launch", "launch.json", 0);
    let mut unit = offer("home/12", "a");
    unit.launch.repository = "example/crew".into();
    unit.launch.base = Some("bolt/x".into());
    unit.launch.stream = Some("home".into());
    unit.launch.priority = Some(4);
    unit.launch.intent = Some("home/x".into());
    unit.launch.recipients = vec!["home/planner".into()];
    unit.launch.brief = Some("Parse the input.".into());
    let mut plane = Fake::holding(vec![Want::Offer(unit)]);
    bench.pass_over(&Cli::new(stub.program()), &mut plane);
    let brief_file = bench.control_dir().join("briefs/boomdrive%3Ahome%2F12.md");
    let launches: Vec<_> = stub
        .calls()
        .into_iter()
        .filter(|call| call[1] == "launch")
        .collect();
    assert_eq!(
        launches,
        [argv(&[
            "--json",
            "launch",
            "--repository",
            container.to_str().unwrap(),
            "--kind",
            "change",
            "--name",
            "a",
            "--base",
            "bolt/x",
            "--stream",
            "home",
            "--priority",
            "4",
            "--intent",
            "home/x",
            "--recipient",
            "home/planner",
            "--recipient",
            "boomdrive",
            "--brief-file",
            brief_file.to_str().unwrap(),
            "--mark",
            "boomdrive:home/12",
        ])]
    );
    let n = stub.call_of("launch").unwrap();
    assert_eq!(
        stub.read(&format!("brief-{n}")).as_deref(),
        Some("Parse the input.")
    );
}

/// The work-into-flume spec's "A launch naming no stream, priority or
/// intent".
#[test]
fn a_launch_naming_no_stream_priority_or_intent() {
    let bench = Bench::new();
    let stub = Stub::new();
    stub.answers("launch", "launch.json", 0);
    let mut fix = offer("home/13", "b");
    fix.launch.kind = "fix".into();
    let mut plane = Fake::holding(vec![Want::Offer(fix)]);
    bench.pass_over(&Cli::new(stub.program()), &mut plane);
    let n = stub.call_of("launch").expect("a launch");
    let call = &stub.calls()[n - 1];
    for flag in ["--stream", "--priority", "--intent"] {
        assert!(!call.iter().any(|arg| arg == flag), "{flag} in {call:?}");
    }
    assert_eq!(
        call[..8],
        argv(&[
            "--json",
            "launch",
            "--repository",
            bench.container(KEPT).to_str().unwrap(),
            "--kind",
            "fix",
            "--name",
            "b"
        ])
    );
}

/// The work-into-flume spec's "The runner is one more recipient": the
/// pass's launch of an offer naming `home/planner` names the runner after
/// it, and its launch of an offer naming no recipient names none, so
/// Flume's default is its recipient (core.15, core.54).
#[test]
fn the_runner_is_one_more_recipient() {
    let bench = Bench::new();
    let stub = Stub::new();
    stub.answers("launch", "launch.json", 0);
    let mut named = offer("home/12", "a");
    named.launch.recipients = vec!["home/planner".into()];
    let none = offer("home/13", "b");
    let mut plane = Fake::holding(vec![Want::Offer(named), Want::Offer(none)]);
    bench.pass_over(&Cli::new(stub.program()), &mut plane);
    let recipients: Vec<Vec<String>> = stub
        .calls()
        .into_iter()
        .filter(|call| call[1] == "launch")
        .map(|call| {
            call.windows(2)
                .filter(|pair| pair[0] == "--recipient")
                .map(|pair| pair[1].clone())
                .collect()
        })
        .collect();
    assert_eq!(
        recipients,
        [argv(&["home/planner", "boomdrive"]), Vec::<String>::new()]
    );
}

/// The words said, once, of a Flume that says `flume 0.20.0` for
/// `--version` (design Decision 7).
const BELOW_THE_LEAST: &str = "boomdrive: home: flume --version says \"flume 0.20.0\": \
     boomdrive launches with Flume 0.21.0 or later, \
     the first whose ends a recipient confirms by key";

/// The work-into-flume spec's "A Flume older than the least": an offer
/// pulled with room for it is not launched, and has no answer in the
/// report (core.38).
#[test]
fn a_flume_older_than_the_least() {
    let bench = Bench::new();
    bench.tools.prints("flume", "flume 0.20.0");
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(bench.flume.launches(), []);
    assert_eq!(outcome(&bench, "a"), None);
    assert_eq!(lines, [BELOW_THE_LEAST]);
}

/// The work-into-flume spec's "A Flume at the least": the same offer,
/// pulled again once `flume --version` prints the least, is launched.
#[test]
fn a_flume_at_the_least() {
    let bench = Bench::new();
    bench.tools.prints("flume", "flume 0.20.0");
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "a"), None);
    bench.tools.prints("flume", "flume 0.21.0");
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "a"), Some(Outcome::Launched));
    let names: Vec<_> = bench.flume.launches().into_iter().map(|a| a.name).collect();
    assert_eq!(names, ["a"]);
    assert_eq!(lines, [format!("a: launched at {}", place_of("a"))]);
}

/// The work-into-flume spec's "Said once while it lasts": two passes in
/// turn read `flume 0.20.0`, an offer pulled at each, and only the first
/// says so (Task notes 6.1).
#[test]
fn said_once_while_it_lasts() {
    let bench = Bench::new();
    bench.tools.prints("flume", "flume 0.20.0");
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [BELOW_THE_LEAST]);
    plane.set_wants(vec![
        Want::Offer(offer("a", "a")),
        Want::Offer(offer("b", "b")),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(bench.flume.launches(), []);
}

/// The work-into-flume spec's "A kind named by its identity".
#[test]
fn a_kind_named_by_its_identity() {
    let bench = Bench::new();
    let container = bench.container("example/home-blueprints");
    bench.clones.ready("example/home-blueprints", &container);
    let stub = Stub::new();
    stub.answers("launch", "launch.json", 0);
    let mut planner = offer("home/planner/1", "home");
    planner.launch.repository = "example/home-blueprints".into();
    planner.launch.kind = "crew/crew-planner".into();
    planner.launch.agent_settings = vec![AgentSetting {
        field: "model".into(),
        value: "opus".into(),
    }];
    let mut plane = Fake::holding(vec![Want::Offer(planner)]);
    bench.pass_over(&Cli::new(stub.program()), &mut plane);
    let n = stub.call_of("launch").expect("a launch");
    let call = &stub.calls()[n - 1];
    let pairs: Vec<_> = call.windows(2).map(|w| (&*w[0], &*w[1])).collect();
    assert!(pairs.contains(&("--kind", "crew/crew-planner")), "{call:?}");
    assert!(pairs.contains(&("--agent", "model=opus")), "{call:?}");
    assert!(!call.iter().any(|arg| arg == "--kinds"), "{call:?}");
}

/// The work-into-flume spec's "A launch's environment and brief".
#[test]
fn a_launchs_environment_and_brief() {
    let bench = Bench::new();
    let stub = Stub::new();
    stub.answers("launch", "launch.json", 0);
    let mut briefed = offer("home/14", "c");
    briefed.launch.brief = Some("Read the words.\n\nAll of them.".into());
    let mut plane = Fake::holding(vec![Want::Offer(briefed)]);
    std::env::set_var("FLUME_RECIPIENT", "someone/else");
    bench.pass_over(&Cli::new(stub.program()), &mut plane);
    std::env::remove_var("FLUME_RECIPIENT");
    let n = stub.call_of("launch").expect("a launch");
    for m in 1..=stub.calls().len() {
        assert_eq!(
            stub.read(&format!("recipient-{m}")).as_deref(),
            Some("unset\n")
        );
    }
    assert_eq!(
        stub.read(&format!("brief-{n}")).as_deref(),
        Some("Read the words.\n\nAll of them.")
    );
    let briefs = bench.control_dir().join("briefs");
    let left: Vec<_> = fs::read_dir(&briefs)
        .map(|entries| entries.map(|entry| entry.unwrap().path()).collect())
        .unwrap_or_default();
    assert_eq!(left, [] as [std::path::PathBuf; 0]);
}

/// The work-into-flume spec's "Flume refuses", over Flume 0.4.1's own
/// refusal of a stream it does not declare.
#[test]
fn flume_refuses() {
    let bench = Bench::new();
    let stub = Stub::new();
    stub.answers("launch", "launch-refused.json", 1);
    let mut nowhere = offer("home/15", "nowhere");
    nowhere.launch.stream = Some("nosuch".into());
    let mut plane = Fake::holding(vec![Want::Offer(nowhere)]);
    let (_, lines) = bench.pass_over(&Cli::new(stub.program()), &mut plane);
    let reason =
        "the stream nosuch the launch names is not declared by the host's configuration (143)";
    let answer = &bench.report().launches[&id("home/15")].answer;
    assert_eq!(
        (answer.outcome, answer.words.as_deref(), &answer.place_dir),
        (Outcome::Refused, Some(reason), &None)
    );
    assert!(fixture("launch-refused.json").contains(reason));
    assert_eq!(lines, [format!("home/15: refused by Flume: {reason}")]);
}

/// The work-into-flume spec's "Flume is not there".
#[test]
fn flume_is_not_there() {
    let bench = Bench::new();
    let stub = Stub::absent();
    let mut plane = Fake::holding(vec![Want::Offer(offer("home/16", "d"))]);
    let flume = Cli::new(stub.program());
    let (passed, lines) = bench.pass_over(&flume, &mut plane);
    assert!(passed.reported);
    assert_eq!(outcome(&bench, "home/16"), None);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(
        lines[0]
            .starts_with("boomdrive: home: Flume's status could not be read: flume could not run:"),
        "{lines:?}"
    );
    assert!(matches!(plane.hellos()[0].capacity, Capacity::Missing(_)));
    stub.write();
    stub.answers("status", "status.json", 0);
    stub.answers("launch", "launch.json", 0);
    let (passed, _) = bench.pass_over(&flume, &mut plane);
    assert!(passed.reported);
    assert_eq!(outcome(&bench, "home/16"), Some(Outcome::Launched));
}

/// A launch Flume could not be run for is kept unanswered, named, and
/// launched at the next pass.
#[test]
fn a_launch_flume_could_not_run() {
    let bench = Bench::new();
    bench.flume.launch_answers(
        "e",
        Err(boomdrive::flume::Unrun("flume was ended: signal: 9".into())),
    );
    let mut plane = Fake::holding(vec![Want::Offer(offer("home/17", "e"))]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(lines, ["home/17: not launched: flume was ended: signal: 9"]);
    assert!(boomdrive::report::read(&bench.report_file())
        .unwrap()
        .is_none());
}

/// The control-plane-interface spec's "A pull that cannot be read".
#[test]
fn a_pull_that_cannot_be_read() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("home/1", "a"))]);
    plane.refuse_pulls("no network");
    let (passed, lines) = bench.pass(&mut plane);
    assert!(!passed.reported);
    assert_eq!(
        lines,
        ["boomdrive: home: the control plane could not be read: no network"]
    );
    assert_eq!(bench.flume.calls(), [Call::Status]);
    assert_eq!(plane.reports(), [] as [Report; 0]);
}

/// The control-plane-interface spec's "An offer's lease kept with its
/// answer".
#[test]
fn an_offers_lease_kept_with_its_answer() {
    let bench = Bench::new();
    let mut a = offer("a", "a");
    a.lease = Lease {
        number: 3,
        expires: None,
    };
    let mut plane = Fake::holding(vec![Want::Offer(a)]);
    bench.pass(&mut plane);
    assert_eq!(bench.report().launches[&id("a")].lease.number, 3);
    assert_eq!(plane.reports()[0].launches[&id("a")].lease.number, 3);
    bench.pass(&mut plane);
    assert_eq!(plane.hellos()[1].leases[&id("a")], 3);
    assert_eq!(plane.reports()[1].launches[&id("a")].lease.number, 3);
}

/// The control-plane-interface spec's "A report not sent".
#[test]
fn a_report_not_sent() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    plane.refuse_reports("offline");
    let (passed, lines) = bench.pass(&mut plane);
    assert!(!passed.reported);
    assert_eq!(plane.reports(), [] as [Report; 0]);
    assert!(lines.contains(
        &"boomdrive: home: the report could not be sent: offline: it is sent at the next pass"
            .to_string()
    ));
    let held = bench.report();
    assert_eq!(held.launches[&id("a")].answer.outcome, Outcome::Launched);
    plane.take_reports();
    bench.pass(&mut plane);
    let reported = &plane.reports()[0].launches[&id("a")];
    assert_eq!(reported.answer, held.launches[&id("a")].answer);
    assert_eq!(bench.flume.launches().len(), 1);
}

/// How long the keeper of these tests holds a clone's lock.
const KEEPING: Duration = Duration::from_millis(300);

/// The keeper, on a thread of its own, holding `address`'s lock among
/// `bench`'s for a while, doing `meanwhile` before it lets it go; given
/// once it holds it.
fn keeping(
    bench: &Bench,
    address: &str,
    meanwhile: impl FnOnce() + Send + 'static,
) -> thread::JoinHandle<()> {
    let locks = Arc::clone(&bench.locks);
    let address = Address::parse(address).unwrap();
    let (held, holding) = mpsc::channel();
    let keeper = thread::spawn(move || {
        let kept = locks.keep(&address);
        held.send(()).unwrap();
        thread::sleep(KEEPING);
        meanwhile();
        drop(kept);
    });
    holding.recv().expect("the keeper holds the lock");
    keeper
}

/// The clones spec's "A launch waits for its own repository's fetch
/// alone": the keeper holds `agentplot/flume`'s lock for its fetch, and a
/// pass offers a launch into `agentplot/flume`, then one into
/// `agentplot/sluice`. The launch into `agentplot/sluice` is made at once,
/// and the one into `agentplot/flume` once the fetch is done.
#[test]
fn a_launch_waits_for_its_own_repositorys_fetch_alone() {
    let bench = Bench::new();
    let container = bench.container("agentplot/sluice");
    bench.clones.ready("agentplot/sluice", &container);
    let mut sluice = offer("home/2", "b");
    sluice.launch.repository = "agentplot/sluice".into();
    let mut plane = Fake::holding(vec![Want::Offer(offer("home/1", "a")), Want::Offer(sluice)]);
    let keeper = keeping(&bench, KEPT, || {});

    let began = std::time::Instant::now();
    bench.pass(&mut plane);
    let took = began.elapsed();
    keeper.join().unwrap();

    let launched: Vec<String> = bench
        .flume
        .launches()
        .into_iter()
        .map(|args| args.name)
        .collect();
    assert_eq!(launched, ["b", "a"], "sluice's at once, flume's once kept");
    assert!(took >= KEEPING / 2, "flume's launch waited for the keep");
    assert_eq!(outcome(&bench, "home/1"), Some(Outcome::Launched));
    assert_eq!(outcome(&bench, "home/2"), Some(Outcome::Launched));
}

/// The clones spec's "Two parts launch into one clone at once": while
/// `home`'s pass is launching into `agentplot/flume`, `work`'s pass offers
/// a launch into it, which is made without waiting for `home`'s to end.
#[test]
fn two_parts_launch_into_one_clone_at_once() {
    let home = Bench::new();
    let mut work = Bench::new();
    work.locks = Arc::clone(&home.locks);
    let mut home_plane = Fake::owning("home/");
    home_plane.set_wants(vec![Want::Offer(offer("home/1", "a"))]);
    let mut work_plane = Fake::owning("work/");
    work_plane.set_wants(vec![Want::Offer(offer("work/1", "b"))]);
    let during: Rc<RefCell<Option<Vec<String>>>> = Rc::default();
    let said = Rc::clone(&during);
    home.flume.meanwhile(move |_| {
        // home's `flume launch` is running, and holds the clone's lock.
        let (_, lines) = work.pass_of("work", &work.flume, &mut work_plane, &|| false);
        *said.borrow_mut() = Some(lines);
    });

    let (_, lines) = home.pass_of("home", &home.flume, &mut home_plane, &|| false);

    let during = during
        .borrow()
        .clone()
        .expect("work's pass ran during home's launch");
    assert_eq!(
        during,
        [format!("work/1: launched at {}", fakes::place_of("b"))]
    );
    assert_eq!(
        lines,
        [format!("home/1: launched at {}", fakes::place_of("a"))]
    );
}

/// The clones spec's "A clone let go before the launch": a pass finds
/// `agentplot/flume`'s clone made, and the keeper removes it, as dropped,
/// before the pass takes its lock. The offer is left unanswered, and no
/// `flume launch` is run for it.
#[test]
fn a_clone_let_go_before_the_launch() {
    let bench = Bench::new();
    let container = bench.container(KEPT);
    let mut plane = Fake::holding(vec![Want::Offer(offer("home/1", "a"))]);
    let keeper = keeping(&bench, KEPT, move || {
        fs::remove_dir_all(&container).expect("the clone removed");
    });

    let (_, lines) = bench.pass(&mut plane);
    keeper.join().unwrap();

    assert_eq!(outcome(&bench, "home/1"), None);
    assert!(bench.flume.launches().is_empty());
    assert_eq!(lines, [] as [String; 0]);
}

/// The repository a test drains.
const KIT: &str = "example/kit";

/// An offer `id` of a `change` named `name` in `repository`.
fn offer_in(id: &str, name: &str, repository: &str) -> Offer {
    let mut offer = offer(id, name);
    offer.launch.repository = repository.to_string();
    offer
}

/// A bench that launched `a` at its place in [`KIT`], whose carried status
/// reads it live, and whose configuration then dropped [`KIT`], its clone
/// standing: the runner drains [`KIT`] for `a`.
fn draining() -> (Bench, Fake) {
    let bench = Bench::new();
    bench.clones.ready(KIT, &bench.container(KIT));
    let mut plane = Fake::holding(vec![Want::Offer(offer_in("a", "a", KIT))]);
    bench.pass(&mut plane);
    let a = &bench.report().launches[&id("a")];
    assert_eq!(a.answer.outcome, Outcome::Launched);
    assert!(a.status.as_ref().is_some_and(|status| status.listed));
    bench.clones.drop_named(KIT);
    (bench, plane)
}

/// The work-into-flume spec's "A launch's repository kept with its
/// answer".
#[test]
fn a_launchs_repository_kept_with_its_answer() {
    let bench = Bench::new();
    let before = Launched {
        lease: Lease {
            number: 1,
            expires: None,
        },
        repository: None,
        answer: Answer {
            outcome: Outcome::Launched,
            place_dir: Some(place_of("b")),
            words: None,
            at: At::parse("2026-11-01T10:00:00Z").unwrap(),
        },
        status: None,
        entry: None,
    };
    let of_before = Report {
        launches: BTreeMap::from([(id("b"), before)]),
        ..Report::default()
    };
    boomdrive::report::save(&bench.report_file(), &of_before).unwrap();
    let written = fs::read_to_string(bench.report_file()).unwrap();
    assert!(!written.contains("\"repository\""), "{written}");
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    let report = bench.report();
    assert_eq!(report.launches[&id("a")].repository.as_deref(), Some(KEPT));
    assert_eq!(report.launches[&id("b")].repository, None);
    let file: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(bench.report_file()).unwrap()).unwrap();
    assert_eq!(file["launches"]["a"]["repository"], json!(KEPT));

    let mut b = offer("b", "b");
    b.launch.repository = "Agentplot/Flume".into();
    plane.set_wants(vec![Want::Offer(offer("a", "a")), Want::Offer(b)]);
    bench.pass(&mut plane);
    let report = bench.report();
    assert_eq!(
        report.launches[&id("b")].repository.as_deref(),
        Some("Agentplot/Flume")
    );
    assert_eq!(report.launches[&id("b")].answer.outcome, Outcome::Launched);
    assert_eq!(bench.flume.launches().len(), 1, "b is not launched again");

    let mut renamed = offer("a", "a");
    renamed.launch.repository = KIT.into();
    plane.set_wants(vec![Want::Offer(renamed)]);
    bench.pass(&mut plane);
    assert_eq!(
        bench.report().launches[&id("a")].repository.as_deref(),
        Some(KEPT),
        "no later pull changes it"
    );
}

/// The work-into-flume spec's "A repository draining in the hello".
#[test]
fn a_repository_draining_in_the_hello() {
    let (bench, mut plane) = draining();
    assert_eq!(plane.hellos()[0].draining, [] as [String; 0]);
    bench.pass(&mut plane);
    let hello = &plane.hellos()[1];
    assert_eq!(hello.draining, [KIT]);
    assert_eq!(hello.repositories, [KEPT]);
}

/// The work-into-flume spec's "A mark in a repository not kept", in a
/// repository with no clone and in one whose clone stands dropped with
/// nothing launched there.
#[test]
fn a_mark_in_a_repository_not_kept() {
    let bench = Bench::new();
    bench.clones.ready(KIT, &bench.container(KIT));
    bench.clones.drop_named(KIT);
    for (id, name) in [("a", "a"), ("b", "b")] {
        bench.flume.stand(json!({
            "place": place_of(name),
            "mark": format!("boomdrive:{id}"),
            "phase": "live",
        }));
    }
    let mut plane = Fake::holding(vec![
        Want::Offer(offer_in("a", "a", "example/old-kit")),
        Want::Offer(offer_in("b", "b", KIT)),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    let report = bench.report();
    for (launch, repository) in [("a", "example/old-kit"), ("b", KIT)] {
        let answer = &report.launches[&id(launch)].answer;
        let words = format!("this host's configuration does not keep {repository}");
        assert_eq!(
            (answer.outcome, answer.words.as_deref(), &answer.place_dir),
            (Outcome::Refused, Some(words.as_str()), &None)
        );
    }
    assert_eq!(bench.flume.commands(), []);
    assert_eq!(
        lines,
        [
            "a: refused: this host's configuration does not keep example/old-kit",
            "b: refused: this host's configuration does not keep example/kit",
        ]
    );
    assert_eq!(plane.hellos()[0].draining, [] as [String; 0]);
}

/// The work-into-flume spec's "A repository drained": a new offer there is
/// refused, but one whose place Flume's listing holds under its mark is
/// answered launched at it, and an inbox there is answered as one in a
/// repository not kept. A branch command there is refused naming sluice,
/// as every command is (core.52), and nothing of the repository is read
/// for it.
#[test]
fn a_repository_drained() {
    let (bench, mut plane) = draining();
    bench.flume.stand(json!({
        "place": place_of("c"),
        "mark": "boomdrive:c",
        "phase": "live",
    }));
    let branch = Want::Command(Command {
        id: id("branch/1"),
        lease: Lease {
            number: 1,
            expires: None,
        },
        what: CommandSort::Branch {
            repository: KIT.to_string(),
            branch: "bolt/x".to_string(),
            from: "main".to_string(),
        },
    });
    let inbox = Want::Inbox(Inbox {
        id: id("inbox/1"),
        repository: KIT.to_string(),
        branch: "inbox".to_string(),
    });
    plane.set_wants(vec![
        Want::Offer(offer_in("a", "a", KIT)),
        Want::Offer(offer_in("b", "b", KIT)),
        Want::Offer(offer_in("c", "c", KIT)),
        branch,
        inbox,
    ]);
    bench.flume.take_calls();
    let (_, lines) = bench.pass(&mut plane);
    let words = "this host's configuration does not keep example/kit";
    let report = bench.report();
    let b = &report.launches[&id("b")].answer;
    assert_eq!(
        (b.outcome, b.words.as_deref()),
        (Outcome::Refused, Some(words))
    );
    let c = &report.launches[&id("c")].answer;
    assert_eq!(
        (c.outcome, c.place_dir.clone()),
        (Outcome::Launched, Some(place_of("c")))
    );
    assert_eq!(report.launches[&id("c")].repository.as_deref(), Some(KIT));
    let refused = "this binary does not carry sluice's commands";
    let command = &report.commands[&id("branch/1")];
    assert_eq!(
        (command.outcome, command.words.as_deref(), &command.text),
        (Outcome::Refused, Some(refused), &None)
    );
    let carried = &report.inboxes[&id("inbox/1")];
    assert_eq!(
        (carried.words.as_deref(), &carried.tip),
        (Some(words), &None)
    );
    assert!(bench.flume.launches().is_empty(), "nothing is launched");
    assert!(lines.contains(&format!("b: refused: {words}")), "{lines:?}");
    assert!(
        lines.contains(&format!("branch/1: refused: {refused}")),
        "{lines:?}"
    );
    assert!(
        lines.contains(&format!("inbox/1: not carried: {words}")),
        "{lines:?}"
    );
    assert_eq!(bench.sluice.calls(), [] as [Vec<String>; 0]);
}
