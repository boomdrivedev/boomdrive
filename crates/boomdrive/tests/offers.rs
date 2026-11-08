//! Offers taken, left or refused for their repository and clone, and
//! launched into one `flume launch` whose answer is Flume's (design
//! Decisions 5 to 7 and 9; the work-into-flume spec's hello, repository,
//! launch and answer requirements; the control-plane-interface spec's
//! "A pull that cannot be read", "An offer's lease kept with its answer"
//! and "A report not sent"; Task notes 4.1).

mod fakes;

use boomdrive::flume::{Cli, Reply};
use boomdrive_control::fake::Fake;
use boomdrive_control::{AgentSetting, Capacity, Id, Lease, Outcome, Report, ToolVersion, Want};
use fakes::{fixture, offer, Bench, Call, Stub, KEPT};
use std::fs;

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
    bench.clones.not_yet("afterthought/crew");
    let mut plane = Fake::new();
    bench.pass(&mut plane);
    assert_eq!(plane.hellos()[0].repositories, [KEPT]);
}

/// The work-into-flume spec's "A tool missing".
#[test]
fn a_tool_missing() {
    let bench = Bench::new();
    bench.tools.missing("herdr");
    let mut plane = Fake::holding(vec![Want::Offer(offer("madswan/1", "a"))]);
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
    assert_eq!(outcome(&bench, "madswan/1"), Some(Outcome::Launched));
}

/// The work-into-flume spec's "Streams as Flume printed them".
#[test]
fn streams_as_flume_printed_them() {
    let bench = Bench::new();
    let streams = r#"[ {"name":"wide", "priority":5,"running":2,"bound":3,"room":1,"queue":[]},{"name":"work","priority":0,"running":1,"bound":1,"room":0,"queue":[]} ]"#;
    let listing = format!(
        r#"{{"status":{{"places":[],"host":{{"running": 3,"bound":null,"room": null,"queue":[],"streams":{streams},"pools":[{{"name":"madswan","accounts":["madswan-1"]}}],"addresses":[],"bases":[],"terminal_kept":false}}}}}}"#
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
    assert!(!text.contains("madswan"), "{text}");
}

/// The work-into-flume spec's "A repository not kept".
#[test]
fn a_repository_not_kept() {
    let bench = Bench::new();
    let mut sluice = offer("madswan/1", "a");
    sluice.launch.repository = "agentplot/sluice".into();
    let mut plane = Fake::holding(vec![Want::Offer(sluice)]);
    let (_, lines) = bench.pass(&mut plane);
    let answer = &bench.report().launches[&id("madswan/1")].answer;
    assert_eq!(answer.outcome, Outcome::Refused);
    assert_eq!(
        answer.words.as_deref(),
        Some("this host's configuration does not keep agentplot/sluice")
    );
    assert_eq!(bench.flume.commands(), []);
    assert_eq!(
        lines,
        ["madswan/1: refused: this host's configuration does not keep agentplot/sluice"]
    );
}

/// The work-into-flume spec's "A clone not yet made".
#[test]
fn a_clone_not_yet_made() {
    let bench = Bench::new();
    bench.clones.not_yet("afterthought/crew");
    let mut crew = offer("madswan/1", "a");
    crew.launch.repository = "afterthought/crew".into();
    let mut plane = Fake::holding(vec![Want::Offer(crew)]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "madswan/1"), None);
    assert_eq!(bench.flume.commands(), []);
    assert_eq!(lines, [] as [String; 0]);
    let container = bench.container("afterthought/crew");
    bench.clones.ready("afterthought/crew", &container);
    bench.pass(&mut plane);
    assert_eq!(outcome(&bench, "madswan/1"), Some(Outcome::Launched));
    let launches = bench.flume.launches();
    assert_eq!(launches.len(), 1);
    assert_eq!(launches[0].container_dir, container);
}

/// The work-into-flume spec's "A unit's launch", over Flume's own command
/// line.
#[test]
fn a_units_launch() {
    let bench = Bench::new();
    let container = bench.container("afterthought/crew");
    bench.clones.ready("afterthought/crew", &container);
    let stub = Stub::new();
    stub.answers("launch", "launch.json", 0);
    let mut unit = offer("madswan/12", "a");
    unit.launch.repository = "afterthought/crew".into();
    unit.launch.base = Some("bolt/x".into());
    unit.launch.stream = Some("madswan".into());
    unit.launch.priority = Some(4);
    unit.launch.intent = Some("madswan/x".into());
    unit.launch.recipients = vec!["madswan/planner".into()];
    unit.launch.brief = Some("Parse the input.".into());
    let mut plane = Fake::holding(vec![Want::Offer(unit)]);
    bench.pass_over(&Cli::new(stub.program()), &mut plane);
    let brief_file = bench
        .control_dir()
        .join("briefs/boomdrive%3Amadswan%2F12.md");
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
            "madswan",
            "--priority",
            "4",
            "--intent",
            "madswan/x",
            "--recipient",
            "madswan/planner",
            "--brief-file",
            brief_file.to_str().unwrap(),
            "--mark",
            "boomdrive:madswan/12",
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
    let mut fix = offer("madswan/13", "b");
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

/// The work-into-flume spec's "A kind named by its identity".
#[test]
fn a_kind_named_by_its_identity() {
    let bench = Bench::new();
    let container = bench.container("afterthought/madswan-blueprints");
    bench
        .clones
        .ready("afterthought/madswan-blueprints", &container);
    let stub = Stub::new();
    stub.answers("launch", "launch.json", 0);
    let mut planner = offer("madswan/planner/1", "madswan");
    planner.launch.repository = "afterthought/madswan-blueprints".into();
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
    let mut briefed = offer("madswan/14", "c");
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
    let mut nowhere = offer("madswan/15", "nowhere");
    nowhere.launch.stream = Some("nosuch".into());
    let mut plane = Fake::holding(vec![Want::Offer(nowhere)]);
    let (_, lines) = bench.pass_over(&Cli::new(stub.program()), &mut plane);
    let reason =
        "the stream nosuch the launch names is not declared by the host's configuration (143)";
    let answer = &bench.report().launches[&id("madswan/15")].answer;
    assert_eq!(
        (answer.outcome, answer.words.as_deref(), &answer.place_dir),
        (Outcome::Refused, Some(reason), &None)
    );
    assert!(fixture("launch-refused.json").contains(reason));
    assert_eq!(lines, [format!("madswan/15: refused by Flume: {reason}")]);
}

/// The work-into-flume spec's "Flume is not there".
#[test]
fn flume_is_not_there() {
    let bench = Bench::new();
    let stub = Stub::absent();
    let mut plane = Fake::holding(vec![Want::Offer(offer("madswan/16", "d"))]);
    let flume = Cli::new(stub.program());
    let (passed, lines) = bench.pass_over(&flume, &mut plane);
    assert!(passed.reported);
    assert_eq!(outcome(&bench, "madswan/16"), None);
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
    assert_eq!(outcome(&bench, "madswan/16"), Some(Outcome::Launched));
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
    let mut plane = Fake::holding(vec![Want::Offer(offer("madswan/17", "e"))]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(
        lines,
        ["madswan/17: not launched: flume was ended: signal: 9"]
    );
    assert!(boomdrive::report::read(&bench.report_file())
        .unwrap()
        .is_none());
}

/// The control-plane-interface spec's "A pull that cannot be read".
#[test]
fn a_pull_that_cannot_be_read() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("madswan/1", "a"))]);
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
