//! `Hosted`, the hosted implementation of the interface, over the fake
//! hosted control plane on the loopback address (the hosted-control-plane
//! spec's "A pull gives each partition's wants under its own ids", "A
//! partition's connection carries only its own repositories and work", "A
//! report goes to each partition, answered by its id", "What goes up is
//! version 1's copy, checked before it goes" and "A terminal the control
//! plane opens is closed at once"; design Decisions 7, 8, 9 and 11). Two
//! partitions, `home` and `work`, are served by one fake, each enrolled
//! with a key of the test's own.

mod common;

use common::example;
use common::fake::{kind, Fake, Taken};
use ed25519_dalek::SigningKey;
use flywheel_control::ControlPlane;
use flywheel_control::{
    Answer, At, Capacity, Carried, Hello, HostPart, Id, Launched, Lease, Outcome, Report,
    ToolVersion, Want, INTERFACE_VERSION,
};
use flywheel_hosted::connection::NO_TERMINAL;
use flywheel_hosted::key::{Memory, Store};
use flywheel_hosted::table::Table;
use flywheel_hosted::Hosted;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// The poll interval of these tests, where each backoff's cap stops.
const POLL_INTERVAL: Duration = Duration::from_secs(60);

/// How long a test waits for what it waits on.
const PATIENCE: Duration = Duration::from_secs(10);

/// Each partition's key, fixed bytes of the test's own.
fn key(partition: &str) -> SigningKey {
    match partition {
        "home" => SigningKey::from_bytes(&[5u8; 32]),
        _ => SigningKey::from_bytes(&[6u8; 32]),
    }
}

/// The hosted table naming `fake`: `home`, served `example/app`, and
/// `work`, served `example/service` and `example/app`.
fn table(fake: &Fake) -> Table {
    let text = format!(
        "url = {:?}\n\
         [[partitions]]\nlabel = \"home\"\nrepositories = [\"example/app\"]\n\
         [[partitions]]\nlabel = \"work\"\nrepositories = [\"example/service\", \"example/app\"]\n",
        fake.url()
    );
    Table::read(
        &toml::from_str(&text).expect("a TOML table"),
        Path::new("/h/.config/flywheel/config.toml"),
    )
    .expect("the table read")
}

/// The hosted implementation over `fake`, both partitions enrolled.
struct Plane {
    hosted: Hosted,
    lines: Arc<Mutex<Vec<String>>>,
    wakes: Arc<AtomicUsize>,
}

impl Plane {
    fn open(fake: &Fake) -> Plane {
        let table = table(fake);
        let store = Memory::default();
        for partition in ["home", "work"] {
            fake.enrolled(partition, &key(partition).verifying_key());
            store
                .write(&table.key_name(partition), &key(partition))
                .expect("kept");
        }
        let lines: Arc<Mutex<Vec<String>>> = Arc::default();
        let wakes: Arc<AtomicUsize> = Arc::default();
        let (said, woken) = (lines.clone(), wakes.clone());
        let hosted = Hosted::new(
            &table,
            Arc::new(store),
            POLL_INTERVAL,
            Box::new(move |line| said.lock().expect("the lines").push(line)),
            Box::new(move || {
                woken.fetch_add(1, Ordering::SeqCst);
            }),
        )
        .expect("opened");
        Plane {
            hosted,
            lines,
            wakes,
        }
    }

    fn lines(&self) -> Vec<String> {
        self.lines.lock().expect("the lines").clone()
    }
}

/// The runner's hello: a host keeping `example/app` and `example/service`,
/// with room, holding `report`'s leases.
fn hello(report: &Report) -> Hello {
    Hello {
        interface_version: INTERFACE_VERSION,
        runner_version: "flywheel 0.0.0".to_string(),
        tool_versions: BTreeMap::from([(
            "flume".to_string(),
            ToolVersion::Printed("flume 0.4.1".to_string()),
        )]),
        repositories: vec!["example/app".to_string(), "example/service".to_string()],
        capacity: Capacity::Printed(HostPart {
            running: "0".to_string(),
            bound: "4".to_string(),
            room: "4".to_string(),
            streams:
                r#"[{"name":"default","priority":0,"running":0,"bound":4,"room":4,"queue":[]}]"#
                    .to_string(),
        }),
        leases: report.leases(),
    }
}

/// An offer of version 1 of the launch `id` of `repository`, after the
/// launches `after`.
fn offer(id: &str, repository: &str, after: &[&str]) -> Value {
    json!({ "offer": {
        "id": id,
        "lease": { "number": 1, "expires": null },
        "launch": {
            "repository": repository, "kind": "fix", "name": id, "base": "main",
            "brief": null, "stream": null, "priority": null, "intent": null,
            "recipients": [], "holds": [], "hold_submission": false,
            "after": after, "agent_settings": []
        }
    } })
}

/// The ids of `wants`, as the runner holds them.
fn ids(wants: &[Want]) -> Vec<String> {
    wants.iter().map(|want| want.id().to_string()).collect()
}

/// A report answering each launch of `ids` launched, carrying no status.
fn launched(ids: &[&str]) -> Report {
    let mut report = Report::default();
    for id in ids {
        report.launches.insert(
            Id::new(*id),
            Launched {
                lease: Lease {
                    number: 1,
                    expires: None,
                },
                answer: Answer {
                    outcome: Outcome::Launched,
                    place_dir: Some(format!("/places/{id}")),
                    words: None,
                    at: At::parse("2026-11-01T10:00:00Z").expect("a time"),
                },
                status: None,
                entry: None,
            },
        );
    }
    report
}

/// Each message `partition`'s connections sent, in order.
fn messages(taken: &[Taken], partition: &str) -> Vec<Value> {
    taken
        .iter()
        .filter_map(|taken| match taken {
            Taken::Message {
                partition: sender,
                message,
                ..
            } if sender == partition => Some(message.clone()),
            _ => None,
        })
        .collect()
}

/// Whether `partition` sent a message of `of`.
fn sent(taken: &[Taken], partition: &str, of: &str) -> bool {
    messages(taken, partition)
        .iter()
        .any(|message| kind(message).as_deref() == Some(of))
}

#[test]
fn ids_under_their_partition() {
    let fake = Fake::start();
    fake.set_wants(
        "home",
        json!([
            offer("l-0", "example/app", &[]),
            offer("l-1", "example/app", &["l-0"])
        ]),
    );
    fake.set_wants("work", json!([offer("l-1", "example/app", &[])]));
    let mut plane = Plane::open(&fake);
    let report = Report::default();
    let pulled = plane.hosted.pull(&hello(&report), &report).expect("read");
    assert_eq!(ids(&pulled.wants), ["home/l-0", "home/l-1", "work/l-1"]);
    let Want::Offer(after) = &pulled.wants[1] else {
        panic!("an offer: {:?}", pulled.wants[1]);
    };
    assert_eq!(after.launch.after, [Id::new("home/l-0")]);
    assert!(pulled.terminals.is_empty());
}

#[test]
fn one_partition_unreachable() {
    let fake = Fake::start();
    fake.refuse_key("work", Some("this key is revoked"));
    fake.set_wants("home", json!([offer("l-1", "example/app", &[])]));
    let mut plane = Plane::open(&fake);
    let report = Report::default();
    let pulled = plane
        .hosted
        .pull(&hello(&report), &report)
        .expect("not unread while home is read");
    assert_eq!(ids(&pulled.wants), ["home/l-1"]);
}

#[test]
fn no_partition_read_is_unread_naming_each() {
    let fake = Fake::start();
    fake.refuse_key("home", Some("this key is revoked"));
    fake.refuse_key("work", Some("this key is revoked"));
    let mut plane = Plane::open(&fake);
    let report = Report::default();
    let unread = plane
        .hosted
        .pull(&hello(&report), &report)
        .expect_err("no partition read");
    assert!(
        unread.0.contains("home: ") && unread.0.contains("work: "),
        "{unread}"
    );
}

#[test]
fn woken() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let report = Report::default();
    plane.hosted.pull(&hello(&report), &report).expect("read");
    let deadline = Instant::now() + PATIENCE;
    // Each first `pulled` wakes the runner, so the count settles at two.
    while plane.wakes.load(Ordering::SeqCst) < 2 {
        assert!(Instant::now() < deadline, "each first pulled wakes");
        std::thread::sleep(Duration::from_millis(5));
    }
    let woken = plane.wakes.load(Ordering::SeqCst);
    fake.set_wants("home", json!([offer("l-1", "example/app", &[])]));
    while plane.wakes.load(Ordering::SeqCst) == woken {
        assert!(Instant::now() < deadline, "a pulled between passes wakes");
        std::thread::sleep(Duration::from_millis(5));
    }
    let pulled = plane.hosted.pull(&hello(&report), &report).expect("read");
    assert_eq!(ids(&pulled.wants), ["home/l-1"]);
}

#[test]
fn each_hello_its_own() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let report = launched(&["home/l-1", "work/l-2"]);
    plane.hosted.pull(&hello(&report), &report).expect("read");
    let taken = fake.taken();
    let hello_of = |partition: &str| {
        messages(&taken, partition)
            .into_iter()
            .find(|message| kind(message).as_deref() == Some("hello"))
            .expect("a hello")["body"]["hello"]
            .clone()
    };
    let home = hello_of("home");
    assert_eq!(home["repositories"], json!(["example/app"]));
    assert_eq!(home["leases"], json!({ "l-1": 1 }));
    let work = hello_of("work");
    assert_eq!(
        work["repositories"],
        json!(["example/app", "example/service"])
    );
    assert_eq!(work["leases"], json!({ "l-2": 1 }));
    assert_eq!(home["capacity"]["printed"]["streams"][0].get("queue"), None);
}

#[test]
fn each_report_its_own() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let report = launched(&["home/l-1", "work/l-2", "garden/l-3"]);
    plane.hosted.pull(&hello(&report), &report).expect("read");
    plane.hosted.report(&report).expect("answered");
    let taken = fake.taken();
    let launches = |partition: &str| {
        let report = messages(&taken, partition)
            .into_iter()
            .find(|message| kind(message).as_deref() == Some("report"))
            .expect("a report");
        let ids: Vec<String> = report["body"]["report"]["launches"]
            .as_object()
            .expect("launches")
            .keys()
            .cloned()
            .collect();
        ids
    };
    assert_eq!(launches("home"), ["l-1"]);
    assert_eq!(launches("work"), ["l-2"]);
}

#[test]
fn another_partitions_repository() {
    let fake = Fake::start();
    fake.set_wants(
        "home",
        json!([
            offer("l-9", "example/service", &[]),
            offer("l-1", "example/app", &[])
        ]),
    );
    let mut plane = Plane::open(&fake);
    let report = Report::default();
    for _ in 0..2 {
        let pulled = plane.hosted.pull(&hello(&report), &report).expect("read");
        assert_eq!(ids(&pulled.wants), ["home/l-1"]);
    }
    let named: Vec<String> = plane
        .lines()
        .into_iter()
        .filter(|line| line.contains("home/l-9"))
        .collect();
    assert_eq!(named.len(), 1, "{:#?}", plane.lines());
    assert!(
        named[0].starts_with("home: ") && named[0].contains("example/service"),
        "{named:?}"
    );
}

#[test]
fn a_lease_refused() {
    let fake = Fake::start();
    fake.answer_reports("home", &[], &[("l-1", "the lease is another host's")]);
    let mut plane = Plane::open(&fake);
    let report = launched(&["home/l-1"]);
    plane.hosted.pull(&hello(&report), &report).expect("read");
    let reported = plane.hosted.report(&report).expect("answered");
    assert_eq!(
        reported.refused,
        BTreeMap::from([(
            Id::new("home/l-1"),
            "the lease is another host's".to_string()
        )])
    );
    assert!(reported.let_go.is_empty());
}

#[test]
fn a_report_resent() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let report = launched(&["home/l-1"]);
    plane.hosted.pull(&hello(&report), &report).expect("read");
    fake.withhold_reported("home", true);
    let reported = std::thread::scope(|scope| {
        scope.spawn(|| {
            fake.wait_until("home's report", |taken| sent(taken, "home", "report"));
            fake.withhold_reported("home", false);
            fake.drop_connection("home");
        });
        plane.hosted.report(&report)
    });
    reported.expect("answered once resent");
    let home = messages(&fake.taken(), "home");
    let kinds: Vec<String> = home.iter().filter_map(kind).collect();
    assert_eq!(kinds, ["hello", "report", "hello", "report"], "{home:#?}");
    assert_eq!(
        home[3]["id"], home[1]["id"],
        "the report resent under its id"
    );
    assert_eq!(home[3]["body"], home[1]["body"]);
}

#[test]
fn a_report_taken_back_holds_no_status() {
    let fake = Fake::start();
    let held = example("from-runner", "report-a-place-on-hold.json")["body"]["report"].clone();
    let launch = held["launches"]
        .as_object()
        .and_then(|launches| launches.keys().next().cloned())
        .expect("a launch");
    assert!(held["launches"][&launch]["status"].is_object());
    fake.set_report("home", held);
    let mut plane = Plane::open(&fake);
    let report = Report::default();
    let pulled = plane.hosted.pull(&hello(&report), &report).expect("read");
    let back = pulled
        .report
        .expect("the report home's control plane holds");
    let launched = &back.launches[&Id::new(format!("home/{launch}"))];
    assert_eq!((&launched.status, &launched.entry), (&None, &None));
}

#[test]
fn a_terminal_opened() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let report = launched(&["home/l-1"]);
    plane.hosted.pull(&hello(&report), &report).expect("read");
    let input = json!({ "terminal": "terminal-9", "offset": 0, "sealed": "AAAA" });
    fake.send_body("home", "terminal_input", input)
        .expect("a live connection");
    let mut open = example("to-runner", "terminal_open.json")["body"]["terminal_open"].clone();
    open["launch"] = json!("l-1");
    fake.send_body("home", "terminal_open", open)
        .expect("a live connection");
    let taken = fake.wait_until("the terminal closed", |taken| {
        sent(taken, "home", "terminal_closed")
    });
    let home = messages(&taken, "home");
    let after_hello: Vec<String> = home[1..].iter().filter_map(kind).collect();
    assert_eq!(after_hello, ["terminal_closed"], "{home:#?}");
    let closed = &home[1]["body"]["terminal_closed"];
    assert_eq!(closed["terminal"], json!("terminal-1"));
    assert_eq!(closed["words"], json!(NO_TERMINAL));
    let pulled = plane.hosted.pull(&hello(&report), &report).expect("read");
    assert!(pulled.terminals.is_empty());
}

/// Flume's status of a place in code, as Flume 0.4.1 printed it, but that
/// its code stage's state is `state`.
fn status_in(state: &str) -> String {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../flywheel/tests/fixtures/flume-0.4.1/status-code.json");
    let mut status: Value =
        serde_json::from_str(&std::fs::read_to_string(file).expect("the fixture")).expect("JSON");
    status["status"]["places"][0]["stages"][1]["state"] = json!(state);
    status.to_string()
}

#[test]
fn a_value_flume_adds() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let mut report = launched(&["home/l-1"]);
    report
        .launches
        .get_mut(&Id::new("home/l-1"))
        .expect("l-1")
        .status = Some(Carried {
        text: status_in("dreaming"),
        at: At::parse("2026-11-01T10:01:00Z").expect("a time"),
        listed: true,
    });
    plane.hosted.pull(&hello(&report), &report).expect("read");
    plane.hosted.report(&report).expect("answered");
    plane.hosted.report(&report).expect("answered");
    let taken = fake.taken();
    let reports: Vec<&Taken> = taken
        .iter()
        .filter(|taken| {
            matches!(taken, Taken::Message { partition, message, .. }
            if partition == "home" && kind(message).as_deref() == Some("report"))
        })
        .collect();
    assert_eq!(reports.len(), 2);
    for taken in reports {
        let Taken::Message {
            message, refused, ..
        } = taken
        else {
            unreachable!();
        };
        assert_eq!(refused, &None, "the report passes version 1");
        assert_eq!(
            message["body"]["report"]["launches"]["l-1"]["status"],
            Value::Null
        );
    }
    let lines: Vec<String> = plane
        .lines()
        .into_iter()
        .filter(|line| line.starts_with("home/l-1: "))
        .collect();
    assert_eq!(lines.len(), 1, "{:#?}", plane.lines());
    assert!(lines[0].contains("/stages/1/state"), "{lines:?}");
    assert!(!lines[0].contains("dreaming"), "{lines:?}");
}

#[test]
fn a_status_that_passes_goes_up_as_its_copy() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let mut report = launched(&["home/l-1"]);
    report
        .launches
        .get_mut(&Id::new("home/l-1"))
        .expect("l-1")
        .status = Some(Carried {
        text: status_in("begun"),
        at: At::parse("2026-11-01T10:01:00Z").expect("a time"),
        listed: true,
    });
    plane.hosted.pull(&hello(&report), &report).expect("read");
    plane.hosted.report(&report).expect("answered");
    let taken = fake.taken();
    let report = messages(&taken, "home")
        .into_iter()
        .find(|message| kind(message).as_deref() == Some("report"))
        .expect("a report");
    let status = &report["body"]["report"]["launches"]["l-1"]["status"];
    assert_eq!(status["copy"]["stages"][1]["state"], json!("begun"));
    assert_eq!(status["copy"].get("repository"), None);
    assert_eq!(status["at"], json!("2026-11-01T10:01:00Z"));
}

#[test]
fn a_report_that_fails_version_1_is_not_sent() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let mut report = launched(&["home/l-1", "work/l-2"]);
    report
        .launches
        .get_mut(&Id::new("home/l-1"))
        .expect("l-1")
        .lease
        .number = 0;
    report
        .launches
        .get_mut(&Id::new("work/l-2"))
        .expect("l-2")
        .lease
        .number = 0;
    plane
        .hosted
        .pull(&hello(&Report::default()), &report)
        .expect("read");
    let unsent = plane
        .hosted
        .report(&report)
        .expect_err("no partition answered");
    assert!(
        unsent.0.contains("home: ") && unsent.0.contains("work: "),
        "{unsent}"
    );
    let taken = fake.taken();
    assert!(!sent(&taken, "home", "report") && !sent(&taken, "work", "report"));
    let lines = plane.lines();
    let named = lines
        .iter()
        .find(|line| line.starts_with("home: ") && line.contains("report"))
        .unwrap_or_else(|| panic!("{lines:#?}"));
    assert!(
        named.contains("/body/report/launches/l-1/lease/number"),
        "{named}"
    );
}
