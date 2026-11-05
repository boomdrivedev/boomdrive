//! The fake hosted control plane does what the tests of the connection
//! rest on (Task notes 3.4): it takes an upgrade only on a statement that
//! proves an enrolled key, over its own origin, at a time within its
//! window and with a nonce not seen; it checks every message against
//! version 1; and it answers, withholds, drops and sends as a test asks.
//! A client of the test's own dials it, on the loopback address.

mod common;

use common::example;
use common::fake::{kind, Fake, Taken, SUBPROTOCOL};
use ed25519_dalek::SigningKey;
use flywheel_hosted::key;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::error::{Error, ProtocolError, SubProtocolError};
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message as Frame;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// `future`, run to its end on a runtime of the test's own.
fn block_on<F: std::future::Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("a runtime")
        .block_on(future)
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("after 1970")
        .as_secs()
}

/// A key of the test's own, fixed bytes.
fn home_key() -> SigningKey {
    SigningKey::from_bytes(&[3u8; 32])
}

/// Dials `fake` with `statement` as the upgrade's `Authorization`, asking
/// for version 1's subprotocol.
async fn dial(fake: &Fake, statement: &str) -> Result<Socket, Error> {
    let mut request = fake
        .connect_address()
        .into_client_request()
        .expect("a request");
    let headers = request.headers_mut();
    headers.insert(
        "sec-websocket-protocol",
        HeaderValue::from_static(SUBPROTOCOL),
    );
    headers.insert(
        "authorization",
        HeaderValue::from_str(statement).expect("a header"),
    );
    tokio_tungstenite::connect_async(request)
        .await
        .map(|(socket, _)| socket)
}

/// A fresh statement of `key`'s, for `fake`.
fn statement(fake: &Fake, key: &SigningKey) -> String {
    key::statement(key, &fake.origin(), now(), &key::nonce().expect("a nonce"))
}

/// The next message `socket` is sent, read as JSON, within five seconds.
async fn next(socket: &mut Socket) -> Option<Value> {
    loop {
        let frame = tokio::time::timeout(Duration::from_secs(5), socket.next())
            .await
            .ok()??
            .ok()?;
        if let Frame::Text(text) = frame {
            return serde_json::from_str(text.as_str()).ok();
        }
    }
}

/// `body` of `kind` as a runner's message under `id`.
fn message(id: &str, kind: &str, body: Value) -> Frame {
    Frame::text(json!({ "protocol_version": 1, "id": id, "body": { kind: body } }).to_string())
}

/// The example hello's body.
fn hello() -> Value {
    example("from-runner", "hello.json")["body"]["hello"].clone()
}

/// The status of the refusal `result` holds.
fn refused_with(result: Result<Socket, Error>) -> (u16, String) {
    match result {
        Err(Error::Http(response)) => (
            response.status().as_u16(),
            response
                .body()
                .as_deref()
                .map(|body| String::from_utf8_lossy(body).into_owned())
                .unwrap_or_default(),
        ),
        Err(e) => panic!("refused with no status: {e}"),
        Ok(_) => panic!("taken"),
    }
}

#[test]
fn a_connection_proved_is_taken_and_its_hello_answered() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    fake.set_wants("home", json!([]));
    block_on(async {
        let mut socket = dial(&fake, &statement(&fake, &key)).await.expect("taken");
        socket
            .send(message("runner:1", "hello", hello()))
            .await
            .expect("sent");
        let pulled = next(&mut socket).await.expect("an answer");
        assert_eq!(kind(&pulled).as_deref(), Some("pulled"));
        assert_eq!(
            pulled["body"]["pulled"],
            json!({ "wants": [], "report": null })
        );
    });
    let taken = fake.taken();
    assert!(
        matches!(
            &taken[0],
            Taken::Upgrade { partition: Some(partition), status: 101, protocols, .. }
                if partition == "home" && protocols == &[SUBPROTOCOL.to_string()]
        ),
        "{taken:#?}"
    );
    assert!(
        matches!(
            &taken[1],
            Taken::Message { partition, refused: None, .. } if partition == "home"
        ),
        "{taken:#?}"
    );
}

#[test]
fn a_statement_for_another_control_plane() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let elsewhere = key::statement(
        &key,
        "https://control.example",
        now(),
        &key::nonce().unwrap(),
    );
    let (status, words) = refused_with(block_on(dial(&fake, &elsewhere)));
    assert_eq!(status, 401);
    assert_eq!(words, "the statement's signature does not check");
}

#[test]
fn a_statement_out_of_time_or_seen() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    let old = key::statement(&key, &fake.origin(), now() - 301, &key::nonce().unwrap());
    let (status, words) = refused_with(block_on(dial(&fake, &old)));
    assert_eq!(status, 401);
    assert!(words.contains("300 seconds"), "{words}");

    let once = statement(&fake, &key);
    block_on(dial(&fake, &once)).expect("taken once");
    let (status, words) = refused_with(block_on(dial(&fake, &once)));
    assert_eq!(status, 401);
    assert_eq!(words, "the statement's nonce was seen");
}

#[test]
fn a_key_not_enrolled_or_refused() {
    let fake = Fake::start();
    let key = home_key();
    let (status, words) = refused_with(block_on(dial(&fake, &statement(&fake, &key))));
    assert_eq!(
        (status, words.as_str()),
        (401, "this key is enrolled to no host")
    );

    fake.enrolled("home", &key.verifying_key());
    fake.refuse_key("home", Some("this host's key was revoked"));
    let (status, words) = refused_with(block_on(dial(&fake, &statement(&fake, &key))));
    assert_eq!(
        (status, words.as_str()),
        (401, "this host's key was revoked")
    );

    fake.refuse_key("home", None);
    block_on(dial(&fake, &statement(&fake, &key))).expect("taken again");
}

#[test]
fn the_subprotocol_refused() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    fake.refuse_subprotocol(true);
    let result = block_on(dial(&fake, &statement(&fake, &key)));
    assert!(
        matches!(
            result,
            Err(Error::Protocol(
                ProtocolError::SecWebSocketSubProtocolError(SubProtocolError::NoSubProtocol)
            ))
        ),
        "{result:?}"
    );
}

#[test]
fn a_message_failing_version_1_is_refused() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    block_on(async {
        let mut socket = dial(&fake, &statement(&fake, &key)).await.expect("taken");
        socket
            .send(message(
                "runner:9",
                "pulled",
                json!({ "wants": [], "report": null }),
            ))
            .await
            .expect("sent");
        let refused = next(&mut socket).await.expect("an answer");
        assert_eq!(kind(&refused).as_deref(), Some("message_refused"));
        assert_eq!(
            refused["body"]["message_refused"]["message"],
            json!("runner:9")
        );
    });
    let taken = fake.taken();
    assert!(
        matches!(
            &taken[1],
            Taken::Message {
                refused: Some(_),
                ..
            }
        ),
        "{taken:#?}"
    );
}

#[test]
fn a_report_answered_withheld_and_dropped() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    fake.answer_reports("home", &["l-0"], &[("l-1", "the lease is another host's")]);
    let report = example("from-runner", "report.json")["body"]["report"].clone();
    block_on(async {
        let mut socket = dial(&fake, &statement(&fake, &key)).await.expect("taken");
        socket
            .send(message("runner:2", "report", report.clone()))
            .await
            .expect("sent");
        let reported = next(&mut socket).await.expect("an answer");
        assert_eq!(
            reported["body"]["reported"],
            json!({
                "report": "runner:2",
                "let_go": ["l-0"],
                "refused": { "l-1": "the lease is another host's" }
            })
        );

        fake.withhold_reported("home", true);
        socket
            .send(message("runner:3", "report", report.clone()))
            .await
            .expect("sent");
        fake.wait_until("the report withheld", |taken| {
            taken
                .iter()
                .any(|t| matches!(t, Taken::Message { message, .. } if message["id"] == "runner:3"))
        });
        let sent = fake
            .send_body("home", "terminal_close", json!({ "terminal": "t-1" }))
            .expect("a live connection");
        let close = next(&mut socket).await.expect("the fake's message");
        assert_eq!(close["id"], json!(sent), "the report was not answered");

        fake.drop_connection("home");
        assert_eq!(next(&mut socket).await, None, "the connection dropped");
    });
    assert!(!fake.connected("home"));
}

#[test]
fn the_wants_changed_are_sent_again() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    block_on(async {
        let mut socket = dial(&fake, &statement(&fake, &key)).await.expect("taken");
        socket
            .send(message("runner:1", "hello", hello()))
            .await
            .expect("sent");
        next(&mut socket).await.expect("the first pulled");
        fake.set_wants("home", json!([]));
        let again = next(&mut socket).await.expect("a pulled again");
        assert_eq!(again["body"]["pulled"]["report"], Value::Null);
    });
}

#[test]
fn pings_are_taken() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    block_on(async {
        let mut socket = dial(&fake, &statement(&fake, &key)).await.expect("taken");
        socket
            .send(Frame::Ping(vec![1, 2].into()))
            .await
            .expect("sent");
        let pong = tokio::time::timeout(Duration::from_secs(5), socket.next())
            .await
            .expect("an answer in time");
        assert!(matches!(pong, Some(Ok(Frame::Pong(_)))), "{pong:?}");
    });
    fake.wait_until("a ping", |taken| {
        taken
            .iter()
            .any(|t| matches!(t, Taken::Ping { partition } if partition == "home"))
    });
}

#[test]
fn a_fake_down_takes_nothing() {
    let fake = Fake::start();
    let key = home_key();
    fake.enrolled("home", &key.verifying_key());
    fake.down(true);
    assert!(block_on(dial(&fake, &statement(&fake, &key))).is_err());
    assert!(fake.taken().is_empty());
}
