//! An observer's connection, and the hosted observer over it (the
//! hosted-control-plane spec's "An observer is told the observing hello
//! and the places the part follows, with no lease", "What an observer
//! sends of work is refused and said", "An observer's answers change
//! nothing and wake nothing", "Nothing waits on an observer" and "An
//! observer's reach is said once each way"; design Decision 6 of
//! a-runner-reports-to-an-observer-it-takes-no-work-from; record 0048),
//! over the fake hosted control plane as the observer, on the loopback
//! address. The outage runs on tokio's paused clock, whose time the fake's
//! checks never read.

mod common;

use boomdrive_control::{
    Acted, Answer, At, Capacity, Carried, Hello, HostPart, Id, Launched, Lease, Observed, Outcome,
    Report, ToolVersion, INTERFACE_VERSION,
};
use boomdrive_hosted::connection::{Connection, Dial, Wake, OBSERVES};
use boomdrive_hosted::key::{Memory, Store};
use boomdrive_hosted::observer;
use boomdrive_hosted::table::Table;
use boomdrive_hosted::Hosted;
use common::example;
use common::fake::{kind, Fake, Taken, SUBPROTOCOL};
use common::waits::{advance_until, Hold, Lines};
use ed25519_dalek::SigningKey;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::runtime::Handle;

/// The poll interval of these tests, where the backoff's cap stops.
const POLL_INTERVAL: Duration = Duration::from_secs(60);

/// The observer's key for `home`, fixed bytes of the test's own.
fn home_key() -> SigningKey {
    SigningKey::from_bytes(&[7u8; 32])
}

/// The observer table naming `fake`, with the partition `home`, which
/// names `example/app`.
fn table(fake: &Fake) -> Table {
    let text = format!(
        "url = {:?}\n[[partitions]]\nlabel = \"home\"\nrepositories = [\"example/app\"]\n",
        fake.url()
    );
    Table::read_observer(
        &toml::from_str(&text).expect("a TOML table"),
        Path::new("/h/.config/boomdrive/config.toml"),
    )
    .expect("the observer table read")
}

/// A store in memory holding `key` as the observer's `home`'s, under
/// `observer.home`.
fn store(fake: &Fake, key: &SigningKey) -> Arc<dyn Store> {
    let memory = Memory::default();
    memory
        .write(&table(fake).key_name("home"), key)
        .expect("kept");
    Arc::new(memory)
}

/// The fake, with `home`'s observer key enrolled.
fn observer() -> Fake {
    let fake = Fake::start();
    fake.enrolled("home", &home_key().verifying_key());
    fake
}

/// Version 4's example of an observing hello's body.
fn observing_hello() -> Value {
    example("from-runner", "hello-observing.json")["body"]["hello"].clone()
}

/// `home`'s observing connection to `fake`, its lines said to `lines`,
/// and every call of its wake counted in `wakes`, though its role calls
/// none.
fn dial(fake: &Fake, lines: &Lines, wakes: &Arc<AtomicUsize>) -> Dial {
    let mut dial = Dial::observing(
        &table(fake),
        "home",
        store(fake, &home_key()),
        POLL_INTERVAL,
        lines.say(),
    );
    let wakes = wakes.clone();
    let wake: Wake = Arc::new(move || {
        wakes.fetch_add(1, Ordering::SeqCst);
    });
    dial.wake = wake;
    dial
}

/// A runtime of the test's own, as the hosted implementation's is.
fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("a runtime")
}

/// Each message `home`'s connections sent, in order.
fn messages(taken: &[Taken]) -> Vec<&Value> {
    taken
        .iter()
        .filter_map(|t| match t {
            Taken::Message {
                partition, message, ..
            } if partition == "home" => Some(message),
            _ => None,
        })
        .collect()
}

/// How many of `home`'s messages were of `of`.
fn sent(taken: &[Taken], of: &str) -> usize {
    messages(taken)
        .into_iter()
        .filter(|message| kind(message).as_deref() == Some(of))
        .count()
}

/// Each `message_refused` `home`'s connections sent, its body.
fn refusals(taken: &[Taken]) -> Vec<Value> {
    messages(taken)
        .into_iter()
        .filter(|message| kind(message).as_deref() == Some("message_refused"))
        .map(|message| message["body"]["message_refused"].clone())
        .collect()
}

/// How many upgrades the fake took with `status`.
fn upgraded(taken: &[Taken], with: u16) -> usize {
    taken
        .iter()
        .filter(|t| matches!(t, Taken::Upgrade { status, .. } if *status == with))
        .count()
}

fn unanswered(taken: &[Taken]) -> usize {
    taken
        .iter()
        .filter(|t| matches!(t, Taken::Unanswered))
        .count()
}

/// The line an observer's refusal of the message `id`, of `kind`, says.
fn refused_line(kind: &str, id: &str) -> String {
    format!(
        "home: the observer sent {kind} {id}, which an observer gives none of, so it is refused"
    )
}

/// `home`'s observing connection opened on `runtime` and handed its
/// hello, once the observer has answered it.
fn reached(fake: &Fake, runtime: &tokio::runtime::Runtime, lines: &Lines) -> Connection {
    let wakes = Arc::new(AtomicUsize::new(0));
    let connection = Connection::open(runtime.handle(), dial(fake, lines, &wakes));
    connection.hand(&observing_hello()).expect("handed");
    lines.wait_until("the observer reached", |lines| {
        lines.contains(&"home: the observer is reached".to_string())
    });
    connection
}

/// The hosted-control-plane spec's "The observing hello", the
/// connection's half: proved with the observer's key on `boomdrive.v4`,
/// its first message at every connection is the observing hello whole,
/// and a working hello or report is not handed it.
#[test]
fn the_observing_hello_first() {
    let fake = observer();
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let runtime = runtime();
    let connection = Connection::open(runtime.handle(), dial(&fake, &lines, &wakes));
    let working = example("from-runner", "hello.json")["body"]["hello"].clone();
    assert!(
        connection.hand(&working).is_err(),
        "a working hello is handed an observer"
    );
    connection.hand(&observing_hello()).expect("handed");

    fake.wait_until("home's first hello", |t| sent(t, "hello") == 1);
    lines.wait_until("the observer reached", |lines| lines.len() == 1);
    fake.drop_connection("home");
    let taken = fake.wait_until("home's second hello", |t| sent(t, "hello") == 2);

    let mut connections = 0;
    let mut first = false;
    for t in &taken {
        match t {
            Taken::Upgrade {
                partition,
                protocols,
                status: 101,
                ..
            } => {
                assert_eq!(partition.as_deref(), Some("home"), "the key proved");
                assert_eq!(protocols, &[SUBPROTOCOL.to_string()]);
                connections += 1;
                first = true;
            }
            Taken::Message {
                message, refused, ..
            } if first => {
                assert_eq!(kind(message).as_deref(), Some("hello"), "{taken:#?}");
                assert_eq!(message["body"]["hello"], observing_hello());
                assert_eq!(message["body"]["hello"]["observing"], json!(true));
                assert_eq!(refused, &None);
                first = false;
            }
            _ => {}
        }
    }
    assert_eq!(connections, 2, "{taken:#?}");

    let working = example("from-runner", "report.json")["body"]["report"].clone();
    assert!(
        connection.report(&working).is_err(),
        "a working report is handed an observer"
    );
    let observed = example("from-runner", "report-observed.json")["body"]["report"].clone();
    connection
        .report(&observed)
        .expect("an observed report handed");
    fake.wait_until("the observed report", |t| sent(t, "report") == 1);
    assert_eq!(
        wakes.load(Ordering::SeqCst),
        0,
        "an observer woke the runner"
    );
}

/// The hosted-control-plane spec's "An offer from an observer": a
/// `pulled` holding an offer is answered with `message_refused` naming
/// it, its words saying the connection observes, one line names it, and
/// nothing wakes the runner.
#[test]
fn an_offer_from_an_observer() {
    let fake = observer();
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let runtime = runtime();
    let connection = Connection::open(runtime.handle(), dial(&fake, &lines, &wakes));
    connection.hand(&observing_hello()).expect("handed");
    lines.wait_until("the observer reached", |lines| lines.len() == 1);

    let offer = example("to-runner", "pulled.json")["body"]["pulled"]["wants"][0].clone();
    let id = fake
        .send_body(
            "home",
            "pulled",
            json!({ "wants": [offer], "report": null }),
        )
        .expect("sent");

    let taken = fake.wait_until("a message_refused", |t| sent(t, "message_refused") == 1);
    assert_eq!(
        refusals(&taken),
        [json!({ "message": id, "words": OBSERVES })]
    );
    let lines = lines.wait_until("the refusal", |lines| lines.len() == 2);
    assert_eq!(
        lines,
        [
            "home: the observer is reached".to_string(),
            refused_line("pulled", &id)
        ]
    );
    assert!(
        connection
            .read(Duration::ZERO)
            .expect("live")
            .wants
            .is_empty(),
        "nothing of the offer is kept"
    );
    assert_eq!(
        wakes.load(Ordering::SeqCst),
        0,
        "an observer woke the runner"
    );
}

/// The hosted-control-plane spec's "What an observer sends of work is
/// refused and said": a `pulled` holding a report is refused as an offer
/// is, and nothing of it is kept.
#[test]
fn a_report_from_an_observer() {
    let fake = observer();
    let lines = Lines::new();
    let runtime = runtime();
    let connection = reached(&fake, &runtime, &lines);

    let report = example("from-runner", "report.json")["body"]["report"].clone();
    let id = fake
        .send_body("home", "pulled", json!({ "wants": [], "report": report }))
        .expect("sent");

    let taken = fake.wait_until("a message_refused", |t| sent(t, "message_refused") == 1);
    assert_eq!(
        refusals(&taken),
        [json!({ "message": id, "words": OBSERVES })]
    );
    lines.wait_until("the refusal", |lines| {
        lines.contains(&refused_line("pulled", &id))
    });
    assert!(
        connection
            .read(Duration::ZERO)
            .expect("live")
            .report
            .is_none(),
        "nothing of the report is kept"
    );
}

/// The hosted-control-plane spec's "A terminal from an observer": each
/// message of a terminal is answered with `message_refused` naming it,
/// said in one line, and no `terminal_closed` is sent.
#[test]
fn a_terminal_from_an_observer() {
    let fake = observer();
    let lines = Lines::new();
    let runtime = runtime();
    let _connection = reached(&fake, &runtime, &lines);

    let mut ids = Vec::new();
    for terminal in [
        "terminal_open",
        "terminal_input",
        "terminal_resume",
        "terminal_close",
    ] {
        let body = example("to-runner", &format!("{terminal}.json"))["body"][terminal].clone();
        let id = fake.send_body("home", terminal, body).expect("sent");
        ids.push((terminal, id));
    }

    let taken = fake.wait_until("four refusals", |t| sent(t, "message_refused") == 4);
    let expected: Vec<Value> = ids
        .iter()
        .map(|(_, id)| json!({ "message": id, "words": OBSERVES }))
        .collect();
    assert_eq!(refusals(&taken), expected);
    let lines = lines.wait_until("the four refusals", |lines| lines.len() == 5);
    for (terminal, id) in &ids {
        assert!(lines.contains(&refused_line(terminal, id)), "{lines:#?}");
    }
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(
        sent(&fake.taken(), "terminal_closed"),
        0,
        "a terminal_closed was sent"
    );
}

/// The hosted-control-plane spec's "An observer's outage": reached, then
/// unanswered for five attempts, then answering, `home` says the observer
/// is reached, once that it is unreachable, and once that it is reached
/// again (core.23).
#[tokio::test(start_paused = true)]
async fn an_observers_outage() {
    let fake = observer();
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let hold = Hold::new();
    let connection = Connection::open(&Handle::current(), dial(&fake, &lines, &wakes));
    connection.hand(&observing_hello()).expect("handed");
    lines.until("home reached", |lines| lines.len() == 1).await;

    fake.down(true);
    fake.drop_connection("home");
    lines
        .until("home unreachable", |lines| lines.len() == 2)
        .await;
    // No connection is live while the fake is down, so the clock runs on
    // through the backoff's waits by itself.
    drop(hold);
    fake.until("five attempts unanswered", |t| unanswered(t) == 5)
        .await;
    fake.down(false);
    let _hold = Hold::new();
    advance_until(&fake, "a sixth attempt", |t| upgraded(t, 101) == 2).await;
    let lines = lines
        .until("home reached again", |lines| {
            lines
                .iter()
                .filter(|line| *line == "home: the observer is reached")
                .count()
                == 2
        })
        .await;

    assert_eq!(lines.len(), 3, "{lines:#?}");
    assert_eq!(lines[0], "home: the observer is reached");
    assert!(
        lines[1].starts_with("home: the observer is unreachable: "),
        "{lines:#?}"
    );
    assert_eq!(lines[2], "home: the observer is reached");
    assert_eq!(
        wakes.load(Ordering::SeqCst),
        0,
        "an observer woke the runner"
    );
}

/// The runner's hello: a host keeping `example/app` and `example/service`,
/// with room, holding the lease of `home/1`.
fn runner_hello() -> Hello {
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
        leases: BTreeMap::from([(Id::new("home/1"), 1)]),
    }
}

/// The copy of a place's status version 4's example of a live agent
/// carries.
fn status_copy() -> Value {
    example("from-runner", "report-a-live-agent-with-its-pane.json")["body"]["report"]["launches"]
        ["offer-7"]["status"]["copy"]
        .clone()
}

/// `home`'s share of the runner's report: the launch `home/1` launched,
/// carrying Flume's status of its place, and each of `more` launched with
/// none; and the act `home/2` done.
fn share(more: &[&str]) -> Report {
    let answer = Answer {
        outcome: Outcome::Launched,
        place_dir: Some("/places/change--parse-words".to_string()),
        words: None,
        at: At::parse("2026-11-01T10:00:00Z").expect("a time"),
    };
    let launched = |status: Option<Carried>| Launched {
        lease: Lease {
            number: 1,
            expires: None,
        },
        repository: Some("example/app".to_string()),
        answer: answer.clone(),
        status,
        entry: None,
    };
    let mut report = Report::default();
    report.launches.insert(
        Id::new("home/1"),
        launched(Some(Carried {
            text: json!({ "status": { "places": [status_copy()] } }).to_string(),
            at: At::parse("2026-11-01T10:01:00Z").expect("a time"),
            listed: true,
        })),
    );
    for id in more {
        report.launches.insert(Id::new(*id), launched(None));
    }
    report.acts.insert(
        Id::new("home/2"),
        Acted {
            launch: Id::new("home/1"),
            lease: 1,
            outcome: Outcome::Done,
            words: None,
            at: At::parse("2026-11-01T10:02:00Z").expect("a time"),
        },
    );
    report
}

/// The observers of the observer table naming `fake`, `home`'s key kept,
/// each saying its lines to `lines`.
fn observers(fake: &Fake, lines: &Lines) -> Vec<Observed> {
    let say = lines.say();
    observer::observers(
        &table(fake),
        store(fake, &home_key()),
        POLL_INTERVAL,
        &|_| {
            let say = say.clone();
            Box::new(move |line| say(line))
        },
    )
    .expect("opened")
}

/// `home`'s one observer over `fake`.
fn home(fake: &Fake, lines: &Lines) -> Observed {
    let mut observed = observers(fake, lines);
    assert_eq!(observed.len(), 1);
    observed.remove(0)
}

/// Each report `home`'s observer sent, its body.
fn reports(taken: &[Taken]) -> Vec<Value> {
    messages(taken)
        .into_iter()
        .filter(|message| kind(message).as_deref() == Some("report"))
        .map(|message| message["body"]["report"].clone())
        .collect()
}

/// The hosted-control-plane spec's "The places observed": the observed
/// report holds `home/1` by its id as the runner holds it, with its answer
/// and its status's copy, and no lease, and no act (design Decision 2).
#[test]
fn the_places_observed() {
    let fake = observer();
    let lines = Lines::new();
    let mut home = home(&fake, &lines);

    home.observer.observe(&runner_hello(), &share(&[]));

    let taken = fake.wait_until("an observed report", |t| sent(t, "report") == 1);
    let report = &reports(&taken)[0];
    assert_eq!(
        *report,
        json!({
            "observing": true,
            "launches": {
                "home/1": {
                    "answer": {
                        "outcome": "launched",
                        "place_dir": "/places/change--parse-words",
                        "words": null,
                        "at": "2026-11-01T10:00:00Z"
                    },
                    "status": {
                        "copy": status_copy(),
                        "at": "2026-11-01T10:01:00Z",
                        "listed": true
                    },
                    "entry": null
                }
            }
        })
    );
    for taken in &taken {
        if let Taken::Message { refused, .. } = taken {
            assert_eq!(refused, &None, "each message passes version 4");
        }
    }
}

/// The hosted-control-plane spec's "The observing hello": handed a hello
/// keeping `example/app` and `example/service`, with a capacity and a
/// lease, `home`'s observer first sends a `hello` with `observing` true,
/// naming `example/app` alone, and no capacity and no lease (core.26).
#[test]
fn the_observing_hello_names_the_partitions_repositories() {
    let fake = observer();
    let lines = Lines::new();
    let mut home = home(&fake, &lines);

    home.observer.observe(&runner_hello(), &share(&[]));

    let taken = fake.wait_until("the observing hello", |t| sent(t, "hello") == 1);
    let first = messages(&taken)[0];
    assert_eq!(kind(first).as_deref(), Some("hello"), "the hello is first");
    assert_eq!(
        first["body"]["hello"],
        json!({
            "observing": true,
            "interface_version": INTERFACE_VERSION,
            "runner_version": "boomdrive 0.0.0",
            "tool_versions": { "flume": { "printed": "flume 0.4.1" } },
            "repositories": ["example/app"]
        })
    );
}

/// The hosted-control-plane spec's "A report's answer that lets go": the
/// observer answers the report letting go `home/1`, so it is not sent
/// again at the next connection, and the next observed report still holds
/// `home/1` (core.10).
#[test]
fn a_report_answered_and_let_go() {
    let fake = observer();
    fake.answer_reports("home", &["home/1"], &[]);
    let lines = Lines::new();
    let mut home = home(&fake, &lines);

    home.observer.observe(&runner_hello(), &share(&[]));
    fake.wait_until("the reported sent", |t| sent(t, "report") == 1);
    // The `reported` crosses the loopback after the report is taken.
    std::thread::sleep(Duration::from_millis(200));
    fake.drop_connection("home");
    fake.wait_until("a second connection's hello", |t| sent(t, "hello") == 2);
    std::thread::sleep(Duration::from_millis(200));
    assert_eq!(sent(&fake.taken(), "report"), 1, "the report sent again");

    home.observer.observe(&runner_hello(), &share(&[]));
    let taken = fake.wait_until("the next report", |t| sent(t, "report") == 2);
    assert!(
        reports(&taken)[1]["launches"].get("home/1").is_some(),
        "the next report still holds home/1"
    );
}

/// The hosted-control-plane spec's "A pulled wakes nothing": the observer
/// of `home` sent three `pulled`, each holding no want, the wake of the
/// part `home` it was opened beside is never called.
#[test]
fn a_pulled_wakes_nothing() {
    let (plane, fake) = (Fake::start(), observer());
    let plane_key = SigningKey::from_bytes(&[5u8; 32]);
    plane.enrolled("home", &plane_key.verifying_key());
    let hosted_table = Table::read(
        &toml::from_str(&format!(
            "url = {:?}\n[[partitions]]\nlabel = \"home\"\nrepositories = [\"example/app\"]\n",
            plane.url()
        ))
        .expect("a TOML table"),
        Path::new("/h/.config/boomdrive/config.toml"),
    )
    .expect("the table read");
    let memory = Memory::default();
    memory
        .write(&hosted_table.key_name("home"), &plane_key)
        .expect("kept");
    let wakes = Arc::new(AtomicUsize::new(0));
    let woken = wakes.clone();
    let mut parts = Hosted::parts(
        &hosted_table,
        Arc::new(memory),
        POLL_INTERVAL,
        &|_| Box::new(|_| {}),
        &|_| {
            let woken = woken.clone();
            Box::new(move || {
                woken.fetch_add(1, Ordering::SeqCst);
            })
        },
    )
    .expect("opened");
    let lines = Lines::new();
    let mut home = home(&fake, &lines);
    let report = share(&[]);
    parts[0]
        .plane
        .pull(&runner_hello(), &report)
        .expect("home read");
    home.observer.observe(&runner_hello(), &report);
    lines.wait_until("the observer reached", |lines| {
        lines.contains(&"home: the observer is reached".to_string())
    });
    let before = wakes.load(Ordering::SeqCst);

    for _ in 0..3 {
        fake.send_body("home", "pulled", json!({ "wants": [], "report": null }))
            .expect("sent");
    }
    // A message of work after them, refused once the three are read.
    fake.send_body(
        "home",
        "terminal_close",
        json!({ "terminal": "terminal-1" }),
    )
    .expect("sent");
    fake.wait_until("the refusal after them", |t| {
        sent(t, "message_refused") == 1
    });

    assert_eq!(
        wakes.load(Ordering::SeqCst),
        before,
        "a pulled woke the runner"
    );
    assert_eq!(
        lines
            .all()
            .iter()
            .filter(|line| line.contains("is reached"))
            .count(),
        1,
        "{:#?}",
        lines.all()
    );
}

/// The hosted-control-plane spec's "An observer down": the observer down,
/// each of three handings of a hello and a report returns within 100
/// milliseconds (design Decision 8; core.49).
#[test]
fn an_observer_down_holds_up_nothing() {
    let fake = observer();
    fake.down(true);
    let lines = Lines::new();
    let mut home = home(&fake, &lines);

    for n in 0..3 {
        let began = Instant::now();
        home.observer.observe(&runner_hello(), &share(&[]));
        let took = began.elapsed();
        assert!(
            took < Duration::from_millis(100),
            "handing {n} took {took:?}"
        );
        if n == 0 {
            fake.wait_until("an attempt unanswered", |t| unanswered(t) >= 1);
        }
    }
}

/// The hosted-control-plane spec's "Back again": two reports handed while
/// the observer is down, then back, the second alone is sent, after the
/// observing hello (core.21).
#[test]
fn back_again_the_newest_alone() {
    let fake = observer();
    fake.down(true);
    let lines = Lines::new();
    let mut home = home(&fake, &lines);

    home.observer.observe(&runner_hello(), &share(&[]));
    fake.wait_until("an attempt unanswered", |t| unanswered(t) >= 1);
    home.observer.observe(&runner_hello(), &share(&["home/3"]));
    fake.down(false);

    let taken = fake.wait_until("a report", |t| sent(t, "report") == 1);
    std::thread::sleep(Duration::from_millis(200));
    let taken_now = fake.taken();
    let kinds: Vec<Option<String>> = messages(&taken_now).into_iter().map(kind).collect();
    assert_eq!(
        kinds,
        [Some("hello".to_string()), Some("report".to_string())],
        "{taken:#?}"
    );
    let launches = reports(&taken_now)[0]["launches"].clone();
    let ids: Vec<&String> = launches.as_object().expect("launches").keys().collect();
    assert_eq!(ids, ["home/1", "home/3"], "the second report alone is sent");
}

/// The control-plane-interface spec's "The hosted observers": `[observer]`
/// naming `home` and `work` gives two observers, `home` then `work`.
#[test]
fn the_hosted_observers() {
    let text = "url = \"http://localhost:8787\"\n\
                [[partitions]]\nlabel = \"home\"\nrepositories = [\"example/app\"]\n\
                [[partitions]]\nlabel = \"work\"\nrepositories = [\"example/service\"]\n";
    let table = Table::read_observer(
        &toml::from_str(text).expect("a TOML table"),
        Path::new("/h/.config/boomdrive/config.toml"),
    )
    .expect("the observer table read");
    let observed = observer::observers(&table, Arc::new(Memory::default()), POLL_INTERVAL, &|_| {
        Box::new(|_| {})
    })
    .expect("opened");
    let labels: Vec<&str> = observed.iter().map(|o| o.label.as_str()).collect();
    assert_eq!(labels, ["home", "work"]);
}
