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
    ActSort, Answer, At, Capacity, Carried, Hello, HostPart, Id, KeptRepository, Launched, Lease,
    Outcome, Part, Previewed, Report, ToolVersion, Want, INTERFACE_VERSION,
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
        drives: BTreeMap::new(),
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

/// An act `id` on the launch `launch`, under its lease, a confirm as
/// `recipient` of `deliveries`, as a control plane of version 2 asks it.
fn confirm(id: &str, launch: &str, recipient: &str, deliveries: &[&str]) -> Value {
    json!({ "act": {
        "id": id, "launch": launch, "lease": 1,
        "what": { "confirm": { "recipient": recipient, "deliveries": deliveries } }
    } })
}

/// `home`'s part's wants at its pull of `fake`, which asks `wants`, and
/// whether `home` sent a `message_refused`.
fn home_pulls(fake: &Fake, wants: Value) -> (Vec<Want>, bool) {
    fake.set_wants("home", wants);
    let mut parts = Parts::open(fake);
    let report = Report::default();
    let pulled = parts
        .part("home")
        .pull(&hello(&report), &report)
        .expect("read");
    let refused = sent(&fake.taken(), "home", "message_refused");
    (pulled.wants, refused)
}

/// The hosted-control-plane spec's "A confirm carried" (design Decision 8
/// of the-hosted-protocol-carries-every-end-of-a-place): a confirm the
/// control plane asks is given to the runner as an act under the
/// partition's prefix, its recipient and keys as given.
#[test]
fn a_confirm_carried() {
    let fake = Fake::start();
    let key = "change--a/0a1b2c3d/stage/code/1/delivered";
    let (wants, refused) = home_pulls(
        &fake,
        json!([
            offer("l-1", "example/app", &[]),
            confirm("c-1", "l-1", "scratch/planner", &[key])
        ]),
    );
    assert!(!refused, "no message_refused: {:#?}", fake.taken());
    assert_eq!(ids(&wants), ["home/l-1", "home/c-1"]);
    let Want::Act(act) = &wants[1] else {
        panic!("an act: {:?}", wants[1]);
    };
    assert_eq!(act.launch, Id::new("home/l-1"));
    assert_eq!(act.lease, 1);
    assert_eq!(
        act.what,
        ActSort::Confirm {
            recipient: "scratch/planner".to_string(),
            deliveries: vec![key.to_string()],
        }
    );
}

/// The hosted-control-plane spec's "A confirm the runner refuses": a
/// confirm of no key reaches the runner as an act, the hosted
/// implementation refusing nothing of it, and it is the runner's to
/// refuse (core.56).
#[test]
fn a_confirm_the_runner_refuses() {
    let fake = Fake::start();
    let (wants, refused) = home_pulls(
        &fake,
        json!([
            offer("l-1", "example/app", &[]),
            confirm("c-2", "l-1", "scratch/planner", &[])
        ]),
    );
    assert!(!refused, "no message_refused: {:#?}", fake.taken());
    assert_eq!(ids(&wants), ["home/l-1", "home/c-2"]);
    let Want::Act(act) = &wants[1] else {
        panic!("an act: {:?}", wants[1]);
    };
    assert_eq!(act.launch, Id::new("home/l-1"));
    assert_eq!(
        act.what,
        ActSort::Confirm {
            recipient: "scratch/planner".to_string(),
            deliveries: vec![],
        }
    );
}

/// The hosted-control-plane spec's "A land carried" (design Decision 6): a
/// land the control plane asks is given to the runner as an act under the
/// partition's prefix, its launch under it too, and the hosted
/// implementation reads nothing else of it (core.53).
#[test]
fn a_land_carried() {
    let fake = Fake::start();
    let (wants, refused) = home_pulls(
        &fake,
        json!([
            offer("l-1", "example/app", &[]),
            { "act": { "id": "land-1", "launch": "l-1", "lease": 1, "what": "land" } }
        ]),
    );
    assert!(!refused, "no message_refused: {:#?}", fake.taken());
    assert_eq!(ids(&wants), ["home/l-1", "home/land-1"]);
    let Want::Act(act) = &wants[1] else {
        panic!("an act: {:?}", wants[1]);
    };
    assert_eq!(act.launch, Id::new("home/l-1"));
    assert_eq!(act.lease, 1);
    assert_eq!(act.what, ActSort::Land);
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

/// The hosted-control-plane spec's "No drive or folder goes up": the
/// runner's hello names the drive `family` with `example/app` at its
/// clone's folder, and `home`'s hello names `example/app` and neither the
/// drive nor the folder, since version 2 carries neither (design Decision
/// 6 of the-runner-names-each-repository-it-keeps; core.27, core.42).
#[test]
fn no_drive_or_folder_goes_up() {
    let fake = Fake::start();
    let mut parts = Parts::open(&fake);
    let report = Report::default();
    let mut hello = hello(&report);
    hello.drives = BTreeMap::from([(
        "family".to_string(),
        vec![KeptRepository {
            address: "example/app".to_string(),
            container_dir: "/data/repositories/example/app".to_string(),
        }],
    )]);
    parts.part("home").pull(&hello, &report).expect("read");
    let taken = fake.taken();
    let first = messages(&taken, "home")
        .into_iter()
        .next()
        .expect("a message from home");
    assert_eq!(kind(&first).as_deref(), Some("hello"), "{first}");
    assert_eq!(
        first["body"]["hello"]["repositories"],
        json!(["example/app"])
    );
    let text = first.to_string();
    assert!(!text.contains("family"), "{text}");
    assert!(!text.contains("/data/repositories"), "{text}");
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

/// A preview `id` of `main` in `repository` at `revision`, done or not,
/// under a lease of its own, as a control plane of version 2 asks it.
fn preview(id: &str, repository: &str, revision: &str, done: bool) -> Value {
    json!({ "preview": {
        "id": id,
        "lease": { "number": 1, "expires": "2026-11-01T11:00:00Z" },
        "repository": repository, "branch": "main", "revision": revision, "done": done
    } })
}

/// A full revision of the tests' own.
const REVISION: &str = "0123456789abcdef0123456789abcdef01234567";

/// The hosted-control-plane spec's "A preview carried" (design Decision
/// 6): a preview and its done the control plane asks are given to the
/// runner in its order, each id under the partition's prefix and every
/// other field as given (core.55).
#[test]
fn a_preview_carried() {
    let fake = Fake::start();
    let (wants, refused) = home_pulls(
        &fake,
        json!([
            preview("preview-1", "example/app", REVISION, false),
            preview("preview-2", "example/app", REVISION, true)
        ]),
    );
    assert!(!refused, "no message_refused: {:#?}", fake.taken());
    assert_eq!(ids(&wants), ["home/preview-1", "home/preview-2"]);
    for (want, done) in wants.iter().zip([false, true]) {
        let Want::Preview(preview) = want else {
            panic!("a preview: {want:?}");
        };
        assert_eq!(
            preview.lease,
            Lease {
                number: 1,
                expires: Some(At::parse("2026-11-01T11:00:00Z").expect("a time")),
            }
        );
        assert_eq!(preview.repository, "example/app");
        assert_eq!(preview.branch, "main");
        assert_eq!(preview.revision, REVISION);
        assert_eq!(preview.done, done);
    }
}

/// The hosted-control-plane spec's "A preview of a revision that is not
/// a commit" (design Decision 2): a preview at the revision `R1` is given
/// as given, sluice's to answer.
#[test]
fn a_preview_of_a_revision_that_is_not_a_commit() {
    let fake = Fake::start();
    let (wants, refused) = home_pulls(
        &fake,
        json!([preview("preview-1", "example/app", "R1", false)]),
    );
    assert!(!refused, "no message_refused: {:#?}", fake.taken());
    let [Want::Preview(preview)] = &wants[..] else {
        panic!("one preview: {wants:?}");
    };
    assert_eq!(preview.id, Id::new("home/preview-1"));
    assert_eq!(preview.revision, "R1");
}

/// The hosted-control-plane spec's "A preview of a repository not
/// served", beside `another_partitions_repository`: a preview of
/// `example/service`, which `home` is not served, is given at neither of
/// two pulls, and one line names it, `home` and `example/service`
/// (core.26).
#[test]
fn a_preview_of_a_repository_not_served() {
    let fake = Fake::start();
    fake.set_wants(
        "home",
        json!([
            preview("preview-9", "example/service", REVISION, false),
            preview("preview-1", "example/app", REVISION, false)
        ]),
    );
    let mut plane = Plane::open(&fake);
    let report = Report::default();
    for _ in 0..2 {
        let pulled = plane.hosted.pull(&hello(&report), &report).expect("read");
        assert_eq!(ids(&pulled.wants), ["home/preview-1"]);
    }
    let named: Vec<String> = plane
        .lines()
        .into_iter()
        .filter(|line| line.contains("home/preview-9"))
        .collect();
    assert_eq!(named.len(), 1, "{:#?}", plane.lines());
    assert!(
        named[0].starts_with("home: ") && named[0].contains("example/service"),
        "{named:?}"
    );
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
    assert!(
        lines[0].starts_with("home/l-1: the launch's status's copy")
            && lines[0].contains("/stages/1/state"),
        "{lines:?}"
    );
    assert!(!lines[0].contains("dreaming"), "{lines:?}");
}

/// The report of `home/l-1` carrying the status `text`.
fn carrying(text: String) -> Report {
    let mut report = launched(&["home/l-1"]);
    report
        .launches
        .get_mut(&Id::new("home/l-1"))
        .expect("l-1")
        .status = Some(Carried {
        text,
        at: At::parse("2026-11-01T10:01:00Z").expect("a time"),
        listed: true,
    });
    report
}

/// Flume's status of a place in code, as [`status_in`] gives it with its
/// code stage `begun`, but that its ends are `ends`.
fn status_with_ends(ends: Value) -> String {
    let mut status: Value = serde_json::from_str(&status_in("begun")).expect("JSON");
    status["status"]["places"][0]["ends"] = ends;
    status.to_string()
}

/// Four ends of a place: one whose `of` Flume 0.21.0 does not have, a
/// stage's whose `state` it does not have, one with no key, as Flume
/// 0.4.1 printed every end, and a submission's that passes.
fn ends_flume_adds() -> Value {
    let status: Value = serde_json::from_str(&status_in("begun")).expect("JSON");
    let construct = status["status"]["places"][0]["ends"][0].clone();
    let mut moved = construct.clone();
    moved["of"] = json!("moved");
    let mut dreaming = construct.clone();
    dreaming["state"] = json!("dreaming");
    dreaming["key"] = json!("stage/construct/1/dreaming");
    let mut keyless = construct.clone();
    keyless.as_object_mut().expect("an end").remove("key");
    let mut submission = construct;
    for field in ["stage", "start", "state", "end"] {
        submission.as_object_mut().expect("an end").remove(field);
    }
    submission["of"] = json!("submission");
    submission["submit"] = json!(1);
    submission["verdict"] = json!({
        "landed": {
            "commit": "fdda3c771caf870178226852bb0bf8e9c8676da3",
            "place_removed": false,
            "branch_removed": false
        }
    });
    submission["key"] = json!("submission/1/landed");
    json!([moved, dreaming, keyless, submission])
}

/// The hosted-control-plane spec's "An end Flume adds" (design Decision 2
/// of the-hosted-protocol-carries-every-end-of-a-place): each end is
/// copied on its own, the rest of the status goes up as its copy, and
/// one line names the launch and each end's path that went up by its
/// delivery alone or was left out, once while it lasts, and no value.
#[test]
fn an_end_flume_adds() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let report = carrying(status_with_ends(ends_flume_adds()));
    plane.hosted.pull(&hello(&report), &report).expect("read");
    for _ in 0..2 {
        plane.hosted.report(&report).expect("answered");
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
        let copy = &message["body"]["report"]["launches"]["l-1"]["status"]["copy"];
        assert_eq!(
            copy["stages"][1]["state"],
            json!("begun"),
            "the rest goes up"
        );
        let ends = copy["ends"].as_array().expect("its ends");
        assert_eq!(ends.len(), 3, "{ends:#?}");
        assert_eq!(ends[0]["of"], json!("moved"));
        assert_eq!(ends[1]["key"], json!("stage/construct/1/dreaming"));
        for alone in &ends[..2] {
            assert!(alone.get("deliveries").is_none(), "by its delivery alone");
            assert!(alone["delivery"].is_string(), "{alone}");
        }
        assert_eq!(ends[2]["of"], json!("submission"));
        assert!(ends[2].get("verdict").is_some(), "whole");
    }
    let lines: Vec<String> = plane
        .lines()
        .into_iter()
        .filter(|line| line.starts_with("home/l-1: "))
        .collect();
    assert_eq!(lines.len(), 1, "{:#?}", plane.lines());
    assert_eq!(
        lines[0],
        "home/l-1: its status goes up with ends version 2 does not name: /ends/0 by its \
         delivery alone, /ends/1 by its delivery alone, /ends/2 left out"
    );
    for value in ["moved", "dreaming", "construct", "scratch/planner"] {
        assert!(!lines[0].contains(value), "{lines:?}");
    }
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

/// The hosted-control-plane spec's "A status with an end by its delivery
/// alone" (design Decision 3 of
/// the-hosted-protocol-carries-every-end-of-a-place): the status goes up
/// with that end as its delivery alone, and the report's answer names no
/// status of that launch taken, whatever the control plane answers.
#[test]
fn a_status_with_an_end_by_its_delivery_alone() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let status: Value = serde_json::from_str(&status_in("begun")).expect("JSON");
    let passing = status["status"]["places"][0]["ends"][0].clone();
    let mut moved = passing.clone();
    moved["of"] = json!("moved");
    let report = carrying(status_with_ends(json!([moved, passing])));
    plane.hosted.pull(&hello(&report), &report).expect("read");
    for _ in 0..2 {
        let reported = plane.hosted.report(&report).expect("answered");
        assert!(
            reported.statuses_taken.is_empty(),
            "a status with an end by its delivery alone is not taken: {reported:?}"
        );
    }
    let taken = fake.taken();
    let report = messages(&taken, "home")
        .into_iter()
        .find(|message| kind(message).as_deref() == Some("report"))
        .expect("a report");
    let ends = &report["body"]["report"]["launches"]["l-1"]["status"]["copy"]["ends"];
    assert_eq!(ends[0]["of"], json!("moved"));
    assert!(ends[0].get("deliveries").is_none(), "by its delivery alone");
    assert!(ends[1].get("deliveries").is_some(), "whole");
}

/// The hosted-control-plane spec's "A drop held for the runner" (design
/// Decisions 3 and 9 of the-hosted-protocol-carries-every-end-of-a-place):
/// Flume 0.21.0's tidied place, dropped, goes up with its drop's end and
/// the delivery's key Flume gave it, and the report's answer names that
/// launch's status taken.
#[test]
fn a_drop_held_for_the_runner() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../boomdrive/tests/fixtures/flume-0.21.0/status-dropped-held.json");
    let text = std::fs::read_to_string(file).expect("the fixture");
    let printed: Value = serde_json::from_str(&text).expect("JSON");
    let flumes = &printed["status"]["places"][0]["ends"][0];
    let report = carrying(text.clone());
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
    let end = &report["body"]["report"]["launches"]["l-1"]["status"]["copy"]["ends"][0];
    assert_eq!(end["of"], json!("over"));
    assert_eq!(end["over"], json!("dropped"));
    assert_eq!(end["key"], json!("over/dropped"));
    assert_eq!(end["delivery"], flumes["delivery"]);
    assert!(end["delivery"].is_string());
    assert_eq!(end["owed"], flumes["owed"]);
    let lines = plane.lines();
    assert!(
        lines.iter().all(|line| !line.starts_with("home/l-1: ")),
        "{lines:#?}"
    );
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

/// sluice 0.74.3's answer `name`, the binary's fixture, as sluice printed
/// it.
fn sluice_answer(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../boomdrive/tests/fixtures/sluice-0.74.3")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).expect("sluice's JSON")
}

/// The fields the runner carries of a preview, and of a release.
const PREVIEW_FIELDS: &[&str] = &["name", "branch", "revision", "state", "made", "addresses"];
const RELEASE_FIELDS: &[&str] = &["preview", "released"];

/// What the runner carries of `answer`, a preview or a release as sluice
/// printed it: a JSON object of `fields`, each where sluice printed it.
fn carried_of(answer: &Value, fields: &[&str]) -> String {
    let kept: serde_json::Map<String, Value> = fields
        .iter()
        .filter_map(|field| Some((field.to_string(), answer.get(*field)?.clone())))
        .collect();
    Value::Object(kept).to_string()
}

/// A preview of `example/app` at sluice's revision the runner answered
/// with `outcome`, `text` and `words`, done or not, its listing `status`.
fn previewed(
    done: bool,
    outcome: Outcome,
    text: Option<String>,
    words: Option<&str>,
    status: Option<String>,
) -> Previewed {
    Previewed {
        lease: Lease {
            number: 1,
            expires: None,
        },
        repository: "example/app".to_string(),
        revision: "c2b72605d2ad8471a98dfae00551852c527d4dcf".to_string(),
        done,
        outcome,
        text,
        words: words.map(str::to_string),
        status: status.map(|text| Carried {
            text,
            at: At::parse("2026-11-01T10:01:00Z").expect("a time"),
            listed: true,
        }),
        at: At::parse("2026-11-01T10:00:30Z").expect("a time"),
    }
}

/// sluice's listing's row of the preview it brought up, `up`.
fn the_up_row() -> Value {
    sluice_answer("previews.json")
        .as_array()
        .and_then(|rows| rows.iter().find(|row| row["state"] == "up").cloned())
        .expect("a row up")
}

/// Each report `home` sent of `fake`'s, and whether the fake refused it.
fn reports_sent(fake: &Fake) -> Vec<(Value, Option<String>)> {
    fake.taken()
        .into_iter()
        .filter_map(|taken| match taken {
            Taken::Message {
                partition,
                message,
                refused,
            } if partition == "home" && kind(&message).as_deref() == Some("report") => {
                Some((message, refused))
            }
            _ => None,
        })
        .collect()
}

/// The keys of `value`, an object.
fn keys(value: &Value) -> BTreeSet<&str> {
    value
        .as_object()
        .map(|fields| fields.keys().map(String::as_str).collect())
        .unwrap_or_default()
}

/// The hosted-control-plane spec's "Previews go up as their copies"
/// (design Decisions 3 and 5): a preview brought up, its done answered
/// with sluice's release, and a preview refused with sluice's words go up
/// under their bare ids, sluice's answer and listing as their copies.
#[test]
fn previews_go_up_as_their_copies() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let mut report = Report::default();
    report.previews.insert(
        Id::new("home/preview-1"),
        previewed(
            false,
            Outcome::Done,
            Some(carried_of(
                &sluice_answer("preview-starting.json"),
                PREVIEW_FIELDS,
            )),
            None,
            Some(carried_of(&the_up_row(), PREVIEW_FIELDS)),
        ),
    );
    report.previews.insert(
        Id::new("home/preview-2"),
        previewed(
            true,
            Outcome::Done,
            Some(carried_of(&sluice_answer("release.json"), RELEASE_FIELDS)),
            None,
            None,
        ),
    );
    let words = sluice_answer("preview-no-table.json")["error"]["message"]
        .as_str()
        .expect("sluice's words")
        .to_string();
    report.previews.insert(
        Id::new("home/preview-3"),
        previewed(false, Outcome::Refused, None, Some(&words), None),
    );
    plane.hosted.pull(&hello(&report), &report).expect("read");
    plane.hosted.report(&report).expect("answered");

    let sent = reports_sent(&fake);
    assert_eq!(sent.len(), 1, "{:#?}", fake.taken());
    let (message, refused) = &sent[0];
    assert_eq!(refused, &None, "the report passes version 2");
    let previews = &message["body"]["report"]["previews"];
    assert_eq!(
        keys(previews),
        BTreeSet::from(["preview-1", "preview-2", "preview-3"])
    );
    let brought_up = &previews["preview-1"];
    let six = BTreeSet::from(["name", "branch", "revision", "state", "made", "addresses"]);
    assert_eq!(keys(&brought_up["copy"]), six);
    assert_eq!(brought_up["copy"]["state"], "starting");
    assert_eq!(keys(&brought_up["status"]["copy"]), six);
    assert_eq!(brought_up["status"]["copy"]["state"], "up");
    assert_eq!(brought_up["status"]["listed"], true);
    assert_eq!(brought_up["outcome"], "done");
    let done = &previews["preview-2"];
    assert_eq!(keys(&done["copy"]), BTreeSet::from(["preview", "released"]));
    assert_eq!(done["copy"]["released"], true);
    assert_eq!(done["done"], true);
    let refused = &previews["preview-3"];
    assert_eq!(
        (&refused["copy"], &refused["status"]),
        (&Value::Null, &Value::Null)
    );
    assert_eq!(refused["outcome"], "refused");
    assert_eq!(refused["words"], words.as_str());
    assert!(
        plane.lines().iter().all(|line| !line.contains("preview-")),
        "{:#?}",
        plane.lines()
    );
}

/// The hosted-control-plane spec's "A preview's copy that fails" (design
/// Decision 5): a preview whose listing holds a `made` that is a number
/// goes up with its `status` `null` and its `copy` as before, the report
/// passing and sent, and one line names the preview and the path, and not
/// the value, once while it fails (core.11).
#[test]
fn a_previews_copy_that_fails() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let mut row = the_up_row();
    row["made"] = json!(1793750400);
    let mut report = Report::default();
    report.previews.insert(
        Id::new("home/preview-1"),
        previewed(
            false,
            Outcome::Done,
            Some(carried_of(
                &sluice_answer("preview-starting.json"),
                PREVIEW_FIELDS,
            )),
            None,
            Some(carried_of(&row, PREVIEW_FIELDS)),
        ),
    );
    plane.hosted.pull(&hello(&report), &report).expect("read");
    for _ in 0..2 {
        plane.hosted.report(&report).expect("answered");
    }
    let sent = reports_sent(&fake);
    assert_eq!(sent.len(), 2, "{:#?}", fake.taken());
    for (message, refused) in &sent {
        assert_eq!(refused, &None, "the report passes version 2");
        let previewed = &message["body"]["report"]["previews"]["preview-1"];
        assert_eq!(previewed["status"], Value::Null);
        assert_eq!(previewed["copy"]["state"], "starting");
    }
    let lines: Vec<String> = plane
        .lines()
        .into_iter()
        .filter(|line| line.contains("home/preview-1"))
        .collect();
    assert_eq!(lines.len(), 1, "{:#?}", plane.lines());
    assert!(
        lines[0].starts_with("home/preview-1: the preview's status's copy")
            && lines[0].contains("/made"),
        "{lines:?}"
    );
    assert!(!lines[0].contains("1793750400"), "{lines:?}");
}

/// Beside `a_value_flume_adds` and `a_previews_copy_that_fails`: a launch
/// and a preview whose ids are equal, each with a status whose copy fails,
/// each go up with its `status` `null` at every report, and each is said
/// once while it fails, its line naming its kind, since the record of
/// failing copies is keyed by the kind and the id, never the bare id.
#[test]
fn a_launch_and_a_preview_of_one_id_both_failing() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let mut report = carrying(status_in("dreaming"));
    report
        .previews
        .insert(Id::new("home/l-1"), a_preview_whose_status_fails());
    plane.hosted.pull(&hello(&report), &report).expect("read");
    for _ in 0..2 {
        let reported = plane.hosted.report(&report).expect("answered");
        assert!(
            reported.statuses_taken.is_empty(),
            "a status that went up null is not taken: {reported:?}"
        );
    }
    let sent = reports_sent(&fake);
    assert_eq!(sent.len(), 2, "{:#?}", fake.taken());
    for (message, refused) in &sent {
        assert_eq!(refused, &None, "the report passes version 2");
        let report = &message["body"]["report"];
        assert_eq!(report["launches"]["l-1"]["status"], Value::Null);
        assert_eq!(report["previews"]["l-1"]["status"], Value::Null);
        assert_eq!(report["previews"]["l-1"]["copy"]["state"], "starting");
    }
    let lines: Vec<String> = plane
        .lines()
        .into_iter()
        .filter(|line| line.starts_with("home/l-1: "))
        .collect();
    assert_eq!(lines.len(), 2, "{:#?}", plane.lines());
    let launch: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("home/l-1: the launch's status's copy"))
        .collect();
    let preview: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("home/l-1: the preview's status's copy"))
        .collect();
    assert_eq!((launch.len(), preview.len()), (1, 1), "{lines:#?}");
    assert!(launch[0].contains("/stages/1/state"), "{launch:?}");
    assert!(preview[0].contains("/made"), "{preview:?}");
    assert!(
        lines
            .iter()
            .all(|line| !line.contains("dreaming") && !line.contains("1793750400")),
        "{lines:?}"
    );
}

/// Beside `a_launch_and_a_preview_of_one_id_both_failing`: a preview's
/// status whose copy fails keeps no launch's status of the same id from
/// being taken, where the launch's passes (core.54).
#[test]
fn a_previews_failing_copy_keeps_no_launchs_status_of_its_id_from_being_taken() {
    let fake = Fake::start();
    let mut plane = Plane::open(&fake);
    let mut report = carrying(status_in("begun"));
    report
        .previews
        .insert(Id::new("home/l-1"), a_preview_whose_status_fails());
    plane.hosted.pull(&hello(&report), &report).expect("read");
    let reported = plane.hosted.report(&report).expect("answered");
    assert_eq!(
        reported.statuses_taken,
        BTreeSet::from([Id::new("home/l-1")])
    );
    let lines: Vec<String> = plane
        .lines()
        .into_iter()
        .filter(|line| line.starts_with("home/l-1: "))
        .collect();
    assert_eq!(lines.len(), 1, "{:#?}", plane.lines());
    assert!(
        lines[0].starts_with("home/l-1: the preview's status's copy"),
        "{lines:?}"
    );
}

/// A preview the runner answered done with sluice's answer, whose listing
/// holds a `made` that is a number, so its status's copy fails.
fn a_preview_whose_status_fails() -> Previewed {
    let mut row = the_up_row();
    row["made"] = json!(1793750400);
    previewed(
        false,
        Outcome::Done,
        Some(carried_of(
            &sluice_answer("preview-starting.json"),
            PREVIEW_FIELDS,
        )),
        None,
        Some(carried_of(&row, PREVIEW_FIELDS)),
    )
}

/// The hosted-control-plane spec's "A report taken back holds its
/// previews" (design Decision 7), beside
/// `a_report_taken_back_holds_no_status`: a preview of the report the
/// control plane hands back is given to the runner under the partition's
/// prefix, as held, its copy's JSON as its text and no status.
#[test]
fn a_report_taken_back_holds_its_previews() {
    let fake = Fake::start();
    let held = example("from-runner", "report-a-preview-brought-up.json")["body"]["report"].clone();
    let (id, kept) = held["previews"]
        .as_object()
        .and_then(|previews| previews.iter().next())
        .map(|(id, kept)| (id.clone(), kept.clone()))
        .expect("a preview");
    fake.set_report("home", held);
    let mut plane = Plane::open(&fake);
    let report = Report::default();
    let pulled = plane.hosted.pull(&hello(&report), &report).expect("read");
    let back = pulled
        .report
        .expect("the report home's control plane holds");
    let previewed = &back.previews[&Id::new(format!("home/{id}"))];
    let as_held: Lease = serde_json::from_value(kept["lease"].clone()).expect("a lease");
    assert_eq!(previewed.lease, as_held);
    assert_eq!(previewed.repository, kept["repository"]);
    assert_eq!(previewed.revision, kept["revision"]);
    assert_eq!(previewed.done, kept["done"]);
    assert_eq!(json!(previewed.outcome), kept["outcome"]);
    assert_eq!(json!(previewed.words), kept["words"]);
    assert_eq!(json!(previewed.at), kept["at"]);
    let text: Value = serde_json::from_str(previewed.text.as_deref().expect("its text"))
        .expect("its text is JSON");
    assert!(text.is_object(), "{text}");
    assert_eq!(text["name"], kept["copy"]["name"]);
    assert_eq!(previewed.status, None);
}
