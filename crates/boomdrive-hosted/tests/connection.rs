//! Each partition's connection (the hosted-control-plane spec's "Every
//! connection proves the host's key", "Each partition keeps one outgoing
//! connection, its whole hello first", "A connection lost is tried again
//! on a bounded backoff with jitter", "A partition's reach is said once
//! each way" and "Every message the runner is sent is checked before it
//! is read"; design Decisions 5, 6 and 10), over the fake hosted control
//! plane on the loopback address. The backoff, the pings and the reach
//! run on tokio's paused clock, whose time the fake's checks never read.

mod common;

use boomdrive_hosted::connection::{Connection, Dial, Say, Wake};
use boomdrive_hosted::key::{Memory, Store};
use boomdrive_hosted::protocol::v2::Hello;
use boomdrive_hosted::table::Table;
use common::example;
use common::fake::{kind, Fake, Taken, SUBPROTOCOL};
use ed25519_dalek::SigningKey;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::sync::watch;
use tokio::time::Instant;

/// How long a test waits for what it waits on, by the host's own clock.
const PATIENCE: Duration = Duration::from_secs(10);

/// The poll interval of these tests, where the backoff's cap stops.
const POLL_INTERVAL: Duration = Duration::from_secs(60);

/// `home`'s key, fixed bytes of the test's own.
fn home_key() -> SigningKey {
    SigningKey::from_bytes(&[5u8; 32])
}

/// The hosted table naming `fake`, with the partition `home`.
fn table(fake: &Fake) -> Table {
    let text = format!(
        "url = {:?}\n[[partitions]]\nlabel = \"home\"\nrepositories = [\"example/app\"]\n",
        fake.url()
    );
    Table::read(
        &toml::from_str(&text).expect("a TOML table"),
        Path::new("/h/.config/boomdrive/config.toml"),
    )
    .expect("the table read")
}

/// A store in memory holding `key` as `home`'s.
fn store(fake: &Fake, key: &SigningKey) -> Arc<dyn Store> {
    let memory = Memory::default();
    memory
        .write(&table(fake).key_name("home"), key)
        .expect("kept");
    Arc::new(memory)
}

/// `home`'s hello, version 2's example of one.
fn hello() -> Hello {
    serde_json::from_value(example("from-runner", "hello.json")["body"]["hello"].clone())
        .expect("the example is a hello")
}

/// The lines a connection says, for a test to read and wait on.
#[derive(Clone)]
struct Lines {
    lines: Arc<Mutex<Vec<String>>>,
    said: Arc<watch::Sender<usize>>,
}

impl Lines {
    fn new() -> Lines {
        Lines {
            lines: Arc::default(),
            said: Arc::new(watch::Sender::new(0)),
        }
    }

    fn say(&self) -> Say {
        let lines = self.clone();
        Arc::new(move |line| {
            lines.lines.lock().expect("the lines").push(line);
            lines.said.send_modify(|count| *count += 1);
        })
    }

    fn all(&self) -> Vec<String> {
        self.lines.lock().expect("the lines").clone()
    }

    /// The lines, once `until` holds of them, waited on without blocking
    /// the test's runtime; or the test fails, naming `what`.
    async fn until(&self, what: &str, until: impl Fn(&[String]) -> bool) -> Vec<String> {
        let deadline = std::time::Instant::now() + PATIENCE;
        let mut changes = self.said.subscribe();
        let said = self.said.clone();
        std::thread::spawn(move || {
            std::thread::sleep(PATIENCE);
            said.send_modify(|_| {});
        });
        loop {
            let lines = self.all();
            if until(&lines) {
                return lines;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "no line said {what}: {lines:#?}"
            );
            let _ = changes.changed().await;
        }
    }
}

/// `home`'s connection to `fake`, its key in `store`, its lines said to
/// `lines` and its wakes counted in `wakes`.
fn dial(fake: &Fake, store: Arc<dyn Store>, lines: &Lines, wakes: &Arc<AtomicUsize>) -> Dial {
    let wakes = wakes.clone();
    let wake: Wake = Arc::new(move || {
        wakes.fetch_add(1, Ordering::SeqCst);
    });
    Dial::new(
        &table(fake),
        "home",
        store,
        POLL_INTERVAL,
        lines.say(),
        wake,
    )
}

/// A runtime of the test's own, as the hosted implementation's is.
fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("a runtime")
}

/// Each upgrade the fake took, with its status.
fn upgrades(taken: &[Taken]) -> Vec<&Taken> {
    taken
        .iter()
        .filter(|t| matches!(t, Taken::Upgrade { .. }))
        .collect()
}

/// How many upgrades the fake took with `status`.
fn upgraded(taken: &[Taken], with: u16) -> usize {
    upgrades(taken)
        .into_iter()
        .filter(|t| matches!(t, Taken::Upgrade { status, .. } if *status == with))
        .count()
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

fn pings(taken: &[Taken]) -> usize {
    taken
        .iter()
        .filter(|t| matches!(t, Taken::Ping { .. }))
        .count()
}

fn unanswered(taken: &[Taken]) -> usize {
    taken
        .iter()
        .filter(|t| matches!(t, Taken::Unanswered))
        .count()
}

/// Holds tokio's paused clock still while it lives: the runtime does not
/// move it on as it idles, so bytes cross the loopback in the host's own
/// time while the test's stands, and the test moves it by
/// `tokio::time::advance`. A test holds it whenever a connection is live,
/// or the clock would run through its pings while a message crosses.
struct Hold {
    _release: std::sync::mpsc::Sender<()>,
}

impl Hold {
    fn new() -> Hold {
        let (release, held) = std::sync::mpsc::channel::<()>();
        // A blocking task stops the paused clock's moving on of its own.
        tokio::task::spawn_blocking(move || {
            let _ = held.recv();
        });
        Hold { _release: release }
    }
}

/// Lets `millis` milliseconds of the host's own time pass, while the
/// paused clock stands, for an answer to cross the loopback.
async fn settle(millis: u64) {
    tokio::task::spawn_blocking(move || std::thread::sleep(Duration::from_millis(millis)))
        .await
        .expect("settled");
}

/// Moves the held clock on a second at a time, letting the host's own
/// time pass between, until `until` holds of what `fake` took; or the
/// test fails, naming `what`.
async fn advance_until(fake: &Fake, what: &str, until: impl Fn(&[Taken]) -> bool) {
    for _ in 0..120 {
        if until(&fake.taken()) {
            return;
        }
        tokio::time::advance(Duration::from_secs(1)).await;
        settle(10).await;
    }
    panic!(
        "the fake did not see {what} in two minutes of the test's clock: {:#?}",
        fake.taken()
    );
}

#[test]
fn a_connection_proved() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let runtime = runtime();
    let connection = Connection::open(
        runtime.handle(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    connection.hand(&hello()).expect("handed");

    fake.wait_until("home's first connection", |t| sent(t, "hello") == 1);
    fake.drop_connection("home");
    let taken = fake.wait_until("home's second connection", |t| sent(t, "hello") == 2);

    let upgrades = upgrades(&taken);
    assert_eq!(upgrades.len(), 2, "{taken:#?}");
    let mut nonces = Vec::new();
    for upgrade in upgrades {
        let Taken::Upgrade {
            partition,
            protocols,
            authorization,
            key: proved,
            nonce,
            status,
        } = upgrade
        else {
            unreachable!()
        };
        assert_eq!(protocols, &[SUBPROTOCOL.to_string()]);
        // The fake names the partition only where the signature checks
        // over the README's context with an enrolled key.
        assert_eq!(partition.as_deref(), Some("home"), "{upgrade:#?}");
        assert_eq!(*status, 101);
        assert_eq!(proved.as_ref(), Some(key.verifying_key().as_bytes()));
        assert!(authorization
            .as_deref()
            .is_some_and(|value| value.starts_with("BoomdriveHost ")));
        nonces.push(nonce.clone().expect("a nonce"));
    }
    assert_ne!(nonces[0], nonces[1]);
}

#[tokio::test(start_paused = true)]
async fn a_key_refused() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    fake.refuse_key("home", Some("this host's key was revoked"));
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let connection = Connection::open(
        &Handle::current(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    connection.hand(&hello()).expect("handed");

    let began = Instant::now();
    fake.until("three refusals", |t| upgraded(t, 401) == 3)
        .await;

    let lines = lines.all();
    assert_eq!(lines.len(), 1, "{lines:#?}");
    assert!(lines[0].starts_with("home: "), "{lines:#?}");
    assert!(
        lines[0].contains("this host's key was revoked"),
        "{lines:#?}"
    );
    assert!(lines[0].contains("`boomdrive enroll home`"), "{lines:#?}");
    // A refused key waits at the backoff's cap.
    assert_eq!(Instant::now() - began, POLL_INTERVAL * 2);
}

#[test]
fn the_hello_first() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let runtime = runtime();
    let connection = Connection::open(
        runtime.handle(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    let handed = hello();
    connection.hand(&handed).expect("handed");

    fake.wait_until("home's first hello", |t| sent(t, "hello") == 1);
    // The fake records a hello before it sends the `pulled` answering
    // it, so home is read before its connection is dropped: a drop on
    // seeing the hello could reach the connection before that `pulled`.
    connection.read(PATIENCE).expect("home is read");
    fake.drop_connection("home");
    let taken = fake.wait_until("home's second hello", |t| sent(t, "hello") == 2);

    let whole = serde_json::to_value(&handed).expect("a hello");
    let mut connections = 0;
    let mut first = false;
    for t in &taken {
        match t {
            Taken::Upgrade { status: 101, .. } => {
                connections += 1;
                first = true;
            }
            Taken::Message {
                message, refused, ..
            } if first => {
                assert_eq!(kind(message).as_deref(), Some("hello"), "{taken:#?}");
                assert_eq!(message["body"]["hello"], whole);
                assert_eq!(refused, &None);
                first = false;
            }
            _ => {}
        }
    }
    assert_eq!(connections, 2, "{taken:#?}");
    connection.read(PATIENCE).expect("home is read again");
    // Each pulled wakes the runner, once the connection has taken it.
    let deadline = std::time::Instant::now() + PATIENCE;
    while wakes.load(Ordering::SeqCst) < 2 {
        assert!(
            std::time::Instant::now() < deadline,
            "a pulled woke nothing"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn the_hello_again_when_it_changes() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let runtime = runtime();
    let connection = Connection::open(
        runtime.handle(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    let first = hello();
    connection.hand(&first).expect("handed");
    connection.read(PATIENCE).expect("home is read");

    let mut second = hello();
    second.repositories = vec!["example/app".to_string()];
    connection.hand(&second).expect("handed");
    connection.hand(&second).expect("handed again");
    fake.wait_until("home's second hello", |t| sent(t, "hello") == 2);
    let mut third = hello();
    third.repositories = vec![];
    connection.hand(&third).expect("handed");

    let taken = fake.wait_until("home's third hello", |t| sent(t, "hello") == 3);
    let hellos: Vec<&Value> = messages(&taken)
        .into_iter()
        .filter(|message| kind(message).as_deref() == Some("hello"))
        .map(|message| &message["body"]["hello"])
        .collect();
    let handed: Vec<Value> = [first, second, third]
        .iter()
        .map(|hello| serde_json::to_value(hello).expect("a hello"))
        .collect();
    assert_eq!(hellos, handed.iter().collect::<Vec<_>>());
    assert_eq!(upgraded(&taken, 101), 1, "one connection: {taken:#?}");
}

#[tokio::test(start_paused = true)]
async fn no_heartbeat_message() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let _hold = Hold::new();
    let connection = Connection::open(
        &Handle::current(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    connection.hand(&hello()).expect("handed");
    lines.until("home reached", |lines| lines.len() == 1).await;

    // Two minutes, a ping each half minute, each answered before the
    // next.
    for n in 1..=4 {
        tokio::time::advance(Duration::from_secs(30)).await;
        fake.until(&format!("ping {n}"), |t| pings(t) == n).await;
        settle(200).await;
    }

    let taken = fake.taken();
    assert_eq!(pings(&taken), 4, "{taken:#?}");
    assert_eq!(messages(&taken).len(), 1, "the hello alone: {taken:#?}");
    assert_eq!(upgrades(&taken).len(), 1, "{taken:#?}");
    assert!(
        !taken.iter().any(|t| matches!(t, Taken::Closed { .. })),
        "{taken:#?}"
    );
    assert_eq!(lines.all(), ["home: the control plane is reached"]);
}

#[tokio::test(start_paused = true)]
async fn a_connection_whose_pong_does_not_come_is_lost() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let _hold = Hold::new();
    let connection = Connection::open(
        &Handle::current(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    connection.hand(&hello()).expect("handed");
    lines.until("home reached", |lines| lines.len() == 1).await;

    fake.deaf("home", true);
    settle(50).await;
    tokio::time::advance(Duration::from_secs(30)).await;
    settle(50).await;
    tokio::time::advance(Duration::from_secs(30)).await;
    lines
        .until("home unreachable", |lines| lines.len() == 2)
        .await;
    // The cap is back at a second once the connection took its pulled.
    tokio::time::advance(Duration::from_secs(1)).await;
    let lines = lines
        .until("home reached again", |lines| lines.len() == 3)
        .await;

    assert_eq!(
        lines,
        [
            "home: the control plane is reached",
            "home: the control plane is unreachable: no pong came within 30s of a ping",
            "home: the control plane is reached",
        ]
    );
    assert_eq!(upgraded(&fake.taken(), 101), 2);
}

#[tokio::test(start_paused = true)]
async fn never_back_to_back_never_past_the_poll_interval() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    fake.down(true);
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let connection = Connection::open(
        &Handle::current(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    connection.hand(&hello()).expect("handed");

    let mut attempts = Vec::new();
    for n in 1..=10 {
        fake.until(&format!("attempt {n}"), |t| unanswered(t) == n)
            .await;
        attempts.push(Instant::now());
    }

    let waits: Vec<Duration> = attempts.windows(2).map(|two| two[1] - two[0]).collect();
    for (n, wait) in waits.iter().enumerate() {
        assert!(!wait.is_zero(), "attempts {n} and {} at one instant", n + 1);
        assert!(*wait <= POLL_INTERVAL, "a wait of {wait:?}");
        // The cap starts at a second and doubles.
        let cap = Duration::from_secs(1 << n).min(POLL_INTERVAL);
        assert!(*wait <= cap, "wait {n}, {wait:?}, is past its cap {cap:?}");
    }
    assert_eq!(lines.all().len(), 1, "{:#?}", lines.all());
}

#[tokio::test(start_paused = true)]
async fn an_outage() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let hold = Hold::new();
    let connection = Connection::open(
        &Handle::current(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    connection.hand(&hello()).expect("handed");
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
                .filter(|line| *line == "home: the control plane is reached")
                .count()
                == 2
        })
        .await;

    assert_eq!(lines.len(), 3, "{lines:#?}");
    assert_eq!(lines[0], "home: the control plane is reached");
    assert!(
        lines[1].starts_with("home: the control plane is unreachable: "),
        "{lines:#?}"
    );
    assert_eq!(lines[2], "home: the control plane is reached");
}

#[test]
fn a_pulled_with_a_field_unnamed() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let mut offer = example("to-runner", "pulled.json")["body"]["pulled"]["wants"][0].clone();
    offer["offer"]["id"] = json!("offer-9");
    fake.set_wants("home", json!([offer]));
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let runtime = runtime();
    let connection = Connection::open(
        runtime.handle(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    connection.hand(&hello()).expect("handed");
    let read = connection.read(PATIENCE).expect("home is read");
    assert_eq!(read.wants.len(), 1);

    let mut pulled = example("to-runner", "pulled.json");
    pulled["id"] = json!("plane:unnamed");
    pulled["body"]["pulled"]["wants"][0]["offer"]["launch"]["tint"] = json!("n0t-sh0wn");
    assert!(fake.send("home", pulled));

    let taken = fake.wait_until("a message_refused", |t| sent(t, "message_refused") == 1);
    let refusal = messages(&taken)
        .into_iter()
        .find(|message| kind(message).as_deref() == Some("message_refused"))
        .expect("a refusal");
    assert_eq!(
        refusal["body"]["message_refused"]["message"],
        "plane:unnamed"
    );
    let words = refusal["body"]["message_refused"]["words"]
        .as_str()
        .expect("words");
    assert!(
        words.starts_with("it fails version 2's check: /"),
        "{words}"
    );
    assert!(!words.contains("n0t-sh0wn"), "{words}");
    assert!(
        taken.iter().all(|t| !matches!(
            t,
            Taken::Message {
                refused: Some(_),
                ..
            }
        )),
        "every message the runner sent passes version 2: {taken:#?}"
    );

    let read = connection.read(PATIENCE).expect("home is read");
    let ids: Vec<&str> = read.wants.iter().map(|want| want.id().as_str()).collect();
    assert_eq!(ids, ["offer-9"], "none of the refused pulled's wants");
    let lines = lines.all();
    assert!(
        lines
            .iter()
            .any(|line| line.contains("plane:unnamed") && !line.contains("n0t-sh0wn")),
        "{lines:#?}"
    );
}

#[test]
fn a_message_whose_id_cannot_be_read_closes_the_connection() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let runtime = runtime();
    let connection = Connection::open(
        runtime.handle(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    connection.hand(&hello()).expect("handed");
    connection.read(PATIENCE).expect("home is read");

    assert!(fake.send(
        "home",
        json!({ "body": { "pulled": { "wants": [], "report": null } } })
    ));

    let taken = fake.wait_until("home closed with 1007 and dialed again", |t| {
        t.iter().any(|t| {
            matches!(
                t,
                Taken::Closed {
                    code: Some(1007),
                    ..
                }
            )
        }) && sent(t, "hello") == 2
    });
    assert_eq!(sent(&taken, "message_refused"), 0, "{taken:#?}");
    let lines = lines.all();
    assert!(
        lines.iter().any(
            |line| line.starts_with("home: the control plane is unreachable: ")
                && line.contains("no id that can be read")
        ),
        "{lines:#?}"
    );
}

#[test]
fn an_enrollments_answer_on_the_socket_is_refused() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let runtime = runtime();
    let connection = Connection::open(
        runtime.handle(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    connection.hand(&hello()).expect("handed");
    connection.read(PATIENCE).expect("home is read");

    let id = fake
        .send_body("home", "enrolled", json!({ "host": "h-1" }))
        .expect("sent");

    let taken = fake.wait_until("a message_refused", |t| sent(t, "message_refused") == 1);
    let refusal = messages(&taken)
        .into_iter()
        .find(|message| kind(message).as_deref() == Some("message_refused"))
        .expect("a refusal");
    assert_eq!(refusal["body"]["message_refused"]["message"], json!(id));
    assert!(refusal["body"]["message_refused"]["words"]
        .as_str()
        .is_some_and(|words| words.contains("runner/enroll")));
}

#[test]
fn a_hello_the_control_plane_refuses_is_said_by_its_kind() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let (lines, wakes) = (Lines::new(), Arc::new(AtomicUsize::new(0)));
    let runtime = runtime();
    let connection = Connection::open(
        runtime.handle(),
        dial(&fake, store(&fake, &key), &lines, &wakes),
    );
    connection.hand(&hello()).expect("handed");
    connection.read(PATIENCE).expect("home is read");
    let taken = fake.taken();
    let hello_id = messages(&taken)[0]["id"]
        .as_str()
        .expect("an id")
        .to_string();

    fake.send_body(
        "home",
        "message_refused",
        json!({ "message": hello_id, "words": "the hello names\na tool it may not" }),
    )
    .expect("sent");

    let deadline = std::time::Instant::now() + PATIENCE;
    let expected = format!(
        "home: the control plane refused the runner's hello {hello_id}: the hello names a tool \
         it may not"
    );
    while !lines.all().contains(&expected) {
        assert!(std::time::Instant::now() < deadline, "{:#?}", lines.all());
        std::thread::sleep(Duration::from_millis(10));
    }
}
