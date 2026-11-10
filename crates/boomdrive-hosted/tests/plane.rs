//! `Hosted`, the hosted implementation of the interface, over the fake
//! hosted control plane on the loopback address (the hosted-control-plane
//! spec's "A pull gives each partition's wants under its own ids", "A
//! partition's connection carries only its own repositories and work", "A
//! report goes to each partition, answered by its id", "What goes up is
//! version 2's copy, checked before it goes" and "A terminal the control
//! plane opens is closed at once"; the control-plane-interface spec's "An
//! implementation serves its control plane as parts" and "A part says
//! which ids are its own"; design Decisions 7, 8, 9 and 11, and Decision
//! 8 of the change each-partition-the-runner-serves-runs-on-its-own). Two
//! partitions, `home` and `work`, are served by one fake, each enrolled
//! with a key of the test's own, as parts or by one `Hosted` serving
//! both.

mod common;

use boomdrive_control::ControlPlane;
use boomdrive_control::{
    Answer, At, Capacity, Carried, Hello, HostPart, Id, Launched, Lease, Outcome, Part, Report,
    ToolVersion, Want, INTERFACE_VERSION,
};
use boomdrive_hosted::connection::NO_TERMINAL;
use boomdrive_hosted::key::{Memory, Store};
use boomdrive_hosted::table::Table;
use boomdrive_hosted::Hosted;
use common::example;
use common::fake::{kind, Fake, Taken};
use ed25519_dalek::SigningKey;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
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
        Path::new("/h/.config/boomdrive/config.toml"),
    )
    .expect("the table read")
}

/// The hosted implementation over `fake`, both partitions enrolled.
struct Plane {
    hosted: Hosted,
    lines: Arc<Mutex<Vec<String>>>,
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
        let said = lines.clone();
        let hosted = Hosted::new(
            &table,
            Arc::new(store),
            POLL_INTERVAL,
            Box::new(move |line| said.lock().expect("the lines").push(line)),
            Box::new(|| {}),
        )
        .expect("opened");
        Plane { hosted, lines }
    }

    fn lines(&self) -> Vec<String> {
        self.lines.lock().expect("the lines").clone()
    }
}

/// The hosted implementation over `fake` as parts, both partitions
/// enrolled, each saying its lines and woken through its own.
struct Parts {
    parts: Vec<Part>,
    lines: BTreeMap<String, Arc<Mutex<Vec<String>>>>,
    wakes: BTreeMap<String, Arc<AtomicUsize>>,
}

impl Parts {
    fn open(fake: &Fake) -> Parts {
        let table = table(fake);
        let store = Memory::default();
        let mut lines = BTreeMap::new();
        let mut wakes = BTreeMap::new();
        for partition in ["home", "work"] {
            fake.enrolled(partition, &key(partition).verifying_key());
            store
                .write(&table.key_name(partition), &key(partition))
                .expect("kept");
            lines.insert(partition.to_string(), Arc::default());
            wakes.insert(partition.to_string(), Arc::default());
        }
        let parts = Hosted::parts(
            &table,
            Arc::new(store),
            POLL_INTERVAL,
            &|label| {
                let said: Arc<Mutex<Vec<String>>> = Arc::clone(&lines[label]);
                Box::new(move |line| said.lock().expect("the lines").push(line))
            },
            &|label| {
                let woken: Arc<AtomicUsize> = Arc::clone(&wakes[label]);
                Box::new(move || {
                    woken.fetch_add(1, Ordering::SeqCst);
                })
            },
        )
        .expect("opened");
        Parts {
            parts,
            lines,
            wakes,
        }
    }

    /// The part `label`'s control plane.
    fn part(&mut self, label: &str) -> &mut (dyn ControlPlane + Send) {
        let part = self
            .parts
            .iter_mut()
            .find(|part| part.label == label)
            .expect("the part");
        part.plane.as_mut()
    }

    /// The lines the part `label` said.
    fn lines(&self, label: &str) -> Vec<String> {
        self.lines[label].lock().expect("the lines").clone()
    }

    /// How many times the part `label` was woken.
    fn woken(&self, label: &str) -> usize {
        self.wakes[label].load(Ordering::SeqCst)
    }
}

/// The runner's hello: a host keeping `example/app` and `example/service`,
/// with room, holding `report`'s leases.
fn hello(report: &Report) -> Hello {
    Hello {
        interface_version: INTERFACE_VERSION,
        runner_version: "boomdrive 0.0.0".to_string(),
        tool_versions: BTreeMap::from([(
            "flume".to_string(),
            ToolVersion::Printed("flume 0.4.1".to_string()),
        )]),
        repositories: vec!["example/app".to_string(), "example/service".to_string()],
        draining: vec![],
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

/// An offer of version 2 of the launch `id` of `repository`, after the
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
                repository: None,
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

/// The control-plane-interface spec's "The hosted partitions as parts":
/// a table naming `home` and `work` opens two parts, `home` then `work`.
#[test]
fn the_hosted_partitions_as_parts() {
    let fake = Fake::start();
    let parts = Parts::open(&fake);
    let labels: Vec<&str> = parts.parts.iter().map(|part| part.label.as_str()).collect();
    assert_eq!(labels, ["home", "work"]);
}

/// The control-plane-interface spec's "The hosted ids by partition":
/// each part owns its own launch `l-1`, and not the other's.
#[test]
fn the_hosted_ids_by_partition() {
    let fake = Fake::start();
    fake.set_wants("home", json!([offer("l-1", "example/app", &[])]));
    fake.set_wants("work", json!([offer("l-1", "example/app", &[])]));
    let mut parts = Parts::open(&fake);
    let report = Report::default();
    for label in ["home", "work"] {
        let pulled = parts
            .part(label)
            .pull(&hello(&report), &report)
            .expect("read");
        assert_eq!(ids(&pulled.wants), [format!("{label}/l-1")]);
    }
    let (home, work) = (Id::new("home/l-1"), Id::new("work/l-1"));
    assert!(parts.part("home").owns(&home) && !parts.part("home").owns(&work));
    assert!(parts.part("work").owns(&work) && !parts.part("work").owns(&home));
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
    let mut parts = Parts::open(&fake);
    let report = Report::default();
    let home = parts
        .part("home")
        .pull(&hello(&report), &report)
        .expect("read");
    assert_eq!(ids(&home.wants), ["home/l-0", "home/l-1"]);
    let Want::Offer(after) = &home.wants[1] else {
        panic!("an offer: {:?}", home.wants[1]);
    };
    assert_eq!(after.launch.after, [Id::new("home/l-0")]);
    assert!(home.terminals.is_empty());
    let work = parts
        .part("work")
        .pull(&hello(&report), &report)
        .expect("read");
    assert_eq!(ids(&work.wants), ["work/l-1"]);
}

/// The hosted-control-plane spec's "One partition unreachable": `home`'s
/// part gives its offer without waiting on `work`'s connection, and
/// `work`'s part is unread, its words not repeating its label.
#[test]
fn one_partition_unreachable() {
    let fake = Fake::start();
    fake.refuse_key("work", Some("this key is revoked"));
    fake.set_wants("home", json!([offer("l-1", "example/app", &[])]));
    let mut parts = Parts::open(&fake);
    let report = Report::default();
    let began = Instant::now();
    let pulled = parts
        .part("home")
        .pull(&hello(&report), &report)
        .expect("home's part read");
    assert_eq!(ids(&pulled.wants), ["home/l-1"]);
    assert!(
        began.elapsed() < Duration::from_secs(5),
        "{:?}",
        began.elapsed()
    );
    let unread = parts
        .part("work")
        .pull(&hello(&report), &report)
        .expect_err("work's part unread");
    assert!(!unread.0.starts_with("work"), "{unread}");
    let home = parts.lines("home");
    assert!(home.iter().all(|line| !line.contains("work")), "{home:#?}");
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

/// The hosted-control-plane spec's "Woken": a `pulled` between passes on
/// `home`'s connection calls the wake handed for `home`, and not the one
/// handed for `work`.
#[test]
fn woken() {
    let fake = Fake::start();
    let mut parts = Parts::open(&fake);
    let report = Report::default();
    for label in ["home", "work"] {
        parts
            .part(label)
            .pull(&hello(&report), &report)
            .expect("read");
    }
    let deadline = Instant::now() + PATIENCE;
    // Each first `pulled` wakes its own part.
    while parts.woken("home") < 1 || parts.woken("work") < 1 {
        assert!(Instant::now() < deadline, "each first pulled wakes");
        std::thread::sleep(Duration::from_millis(5));
    }
    let (home, work) = (parts.woken("home"), parts.woken("work"));
    fake.set_wants("home", json!([offer("l-1", "example/app", &[])]));
    while parts.woken("home") == home {
        assert!(Instant::now() < deadline, "a pulled between passes wakes");
        std::thread::sleep(Duration::from_millis(5));
    }
    std::thread::sleep(Duration::from_millis(200));
    assert_eq!(parts.woken("work"), work, "work's wake is not called");
    let pulled = parts
        .part("home")
        .pull(&hello(&report), &report)
        .expect("read");
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

/// The hosted-control-plane spec's "A repository drained goes up in no
/// hello": version 2 names no repository drained, nor the repository a
/// launch was answered in (record 0021; core.42).
#[test]
fn a_repository_draining_goes_up_in_no_hello() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let mut report = launched(&["home/l-1"]);
    for launched in report.launches.values_mut() {
        launched.repository = Some("example/app".to_string());
    }
    let mut hello = hello(&report);
    hello.repositories = vec!["example/service".to_string()];
    hello.draining = vec!["example/app".to_string()];
    plane.hosted.pull(&hello, &report).expect("read");
    plane.hosted.report(&report).expect("answered");
    let taken = fake.taken();
    let body = |partition: &str, of: &str| {
        messages(&taken, partition)
            .into_iter()
            .find(|message| kind(message).as_deref() == Some(of))
            .unwrap_or_else(|| panic!("a {of} from {partition}"))["body"][of]
            .clone()
    };
    let home = body("home", "hello");
    assert_eq!(home["repositories"], json!([]));
    assert_eq!(home.get("draining"), None);
    assert_eq!(home["leases"], json!({ "l-1": 1 }));
    assert_eq!(
        body("work", "hello")["repositories"],
        json!(["example/service"])
    );
    let l_1 = &body("home", "report")["launches"]["l-1"];
    assert!(l_1.is_object(), "{l_1}");
    assert_eq!(l_1.get("repository"), None, "{l_1}");
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
    let mut parts = Parts::open(&fake);
    let report = launched(&["home/l-1"]);
    let home = parts.part("home");
    home.pull(&hello(&report), &report).expect("read");
    let reported = home.report(&report).expect("answered");
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
    let mut parts = Parts::open(&fake);
    let report = launched(&["home/l-1"]);
    let home = parts.part("home");
    home.pull(&hello(&report), &report).expect("read");
    fake.withhold_reported("home", true);
    let reported = std::thread::scope(|scope| {
        scope.spawn(|| {
            fake.wait_until("home's report", |taken| sent(taken, "home", "report"));
            fake.withhold_reported("home", false);
            fake.drop_connection("home");
        });
        home.report(&report)
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

/// The hosted-control-plane spec's "A report unanswered by another
/// partition": `work`'s control plane takes a `report` and never answers
/// it, and `home`'s part's report is answered at once, while `work`'s is
/// unsent after 10 seconds.
#[test]
fn a_report_unanswered_by_another_partition() {
    let fake = Fake::start();
    let mut parts = Parts::open(&fake);
    let report = launched(&["home/l-1", "work/l-2"]);
    for label in ["home", "work"] {
        parts
            .part(label)
            .pull(&hello(&report), &report)
            .expect("read");
    }
    fake.withhold_reported("work", true);
    let mut work = parts.parts.pop().expect("work's part");
    assert_eq!(work.label, "work");
    let began = Instant::now();
    let (home, unsent) = std::thread::scope(|scope| {
        let unsent = scope.spawn(|| {
            let unsent = work.plane.report(&report);
            (unsent, began.elapsed())
        });
        std::thread::sleep(Duration::from_millis(100));
        let home = parts.part("home").report(&report);
        let home = (home, began.elapsed());
        (home, unsent.join().expect("work's report ends"))
    });
    home.0.expect("home's report answered");
    assert!(home.1 < Duration::from_secs(5), "{:?}", home.1);
    let words = unsent.0.expect_err("work's report unsent");
    assert!(words.0.contains("did not answer"), "{words}");
    assert!(unsent.1 >= Duration::from_secs(10), "{:?}", unsent.1);
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

/// Each end of `place`, as Flume 0.4.1 printed it, given the fields
/// Flume 0.21.0 prints of it: its key, its delivery's key and expiry, the
/// readers it is owed to, and those that confirmed it (design Decision 9
/// of the-hosted-protocol-carries-every-end-of-a-place).
fn ends_as_flume_0_21_0_prints_them(place: &mut Value) {
    let short = format!(
        "{}--{}",
        place["kind"].as_str().expect("a kind"),
        place["name"].as_str().expect("a name")
    );
    for end in place["ends"].as_array_mut().expect("ends") {
        let key = match end["of"].as_str().expect("an of") {
            "stage" => format!(
                "stage/{}/{}/{}",
                end["stage"].as_str().expect("a stage"),
                end["start"],
                end["state"].as_str().expect("a state")
            ),
            of => panic!("an end of {of} in the fixture"),
        };
        let pending: Vec<Value> = end["pending_for"].as_array().expect("pending").clone();
        end["delivery"] = json!(format!("{short}/0a1b2c3d/{key}"));
        end["expires_at"] = json!(end["read_at"].as_u64().expect("read") + 604_800);
        end["owed"] = pending
            .iter()
            .map(|recipient| json!({ "recipient": recipient, "expired": false, "known": true }))
            .collect();
        end["confirmed"] = json!([]);
        end["key"] = json!(key);
    }
}

/// Flume's status of a place in code, as Flume 0.4.1 printed it, its ends
/// as Flume 0.21.0 prints them, but that its code stage's state is
/// `state`.
fn status_in(state: &str) -> String {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../boomdrive/tests/fixtures/flume-0.4.1/status-code.json");
    let mut status: Value =
        serde_json::from_str(&std::fs::read_to_string(file).expect("the fixture")).expect("JSON");
    status["status"]["places"][0]["stages"][1]["state"] = json!(state);
    ends_as_flume_0_21_0_prints_them(&mut status["status"]["places"][0]);
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
    for _ in 0..2 {
        let reported = plane.hosted.report(&report).expect("answered");
        assert!(
            reported.statuses_taken.is_empty(),
            "a status that went up null is not taken: {reported:?}"
        );
    }
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
        assert_eq!(refused, &None, "the report passes version 2");
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
    let reported = plane.hosted.report(&report).expect("answered");
    assert_eq!(
        reported.statuses_taken,
        BTreeSet::from([Id::new("home/l-1")])
    );
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
fn a_report_that_fails_version_2_is_not_sent() {
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
