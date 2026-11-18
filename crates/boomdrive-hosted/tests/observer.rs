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

use boomdrive_hosted::connection::{Connection, Dial, Wake, OBSERVES};
use boomdrive_hosted::key::{Memory, Store};
use boomdrive_hosted::table::Table;
use common::example;
use common::fake::{kind, Fake, Taken, SUBPROTOCOL};
use common::waits::{advance_until, Hold, Lines};
use ed25519_dalek::SigningKey;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
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

/// Version 3's example of an observing hello's body.
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
/// connection's half: proved with the observer's key on `boomdrive.v3`,
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
