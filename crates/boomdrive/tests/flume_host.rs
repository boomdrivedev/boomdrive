//! flume host's socket as the runner asks it (the change
//! the-runner-hears-flume-by-subscription): a status ask, a subscription,
//! another protocol, an error line and no flume host, each against a
//! flume host of the test's own on a scratch socket. It lives here, and not
//! beside the module, so that no source of the runner's names a listener
//! (core.3).

use boomdrive::flume::{Reply, Unrun};
use boomdrive::flume_host::{status, Subscription, Where};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// A flume host on `socket` that answers one connection with `lines`,
/// after reading the hello and the ask, which it gives back; it closes
/// the connection once `close` is sent, or dropped.
fn host_until(
    socket: &Path,
    lines: Vec<String>,
    close: std::sync::mpsc::Receiver<()>,
) -> std::thread::JoinHandle<Vec<String>> {
    let listener = UnixListener::bind(socket).unwrap();
    std::thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = stream;
        let mut asked = Vec::new();
        for (n, line) in lines.iter().enumerate() {
            let mut read = String::new();
            if n < 2 {
                reader.read_line(&mut read).unwrap();
                asked.push(read.trim_end().to_string());
            }
            writer.write_all(format!("{line}\n").as_bytes()).unwrap();
        }
        let _ = close.recv();
        asked
    })
}

/// As [`host_until`], closing once its lines are written.
fn host(socket: &Path, lines: Vec<String>) -> std::thread::JoinHandle<Vec<String>> {
    let (close, closed) = std::sync::mpsc::channel();
    drop(close);
    host_until(socket, lines, closed)
}

#[test]
fn a_status_ask() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("host.sock");
    let document = r#"{"status":{"places":[] , "host":{}}}"#;
    let served = host(
        &socket,
        vec![
            r#"{"version":"0.36.0","protocol":1,"pid":1,"since":"x"}"#.to_string(),
            format!(r#"{{"document":{document}}}"#),
        ],
    );
    assert_eq!(status(&socket), Ok(Reply::Given(document.to_string())));
    let asked = served.join().unwrap();
    assert_eq!(
        asked,
        [
            format!(r#"{{"hello":"boomdrive/{}"}}"#, env!("CARGO_PKG_VERSION")),
            r#"{"ask":"status"}"#.to_string()
        ]
    );
}

/// The running spec's "Another protocol".
#[test]
fn another_protocol() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("host.sock");
    let listener = UnixListener::bind(&socket).unwrap();
    let served = std::thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut hello = String::new();
        reader.read_line(&mut hello).unwrap();
        let mut writer = stream;
        writer
            .write_all(b"{\"version\":\"9\",\"protocol\":2,\"pid\":1,\"since\":\"x\"}\n")
            .unwrap();
        let mut more = String::new();
        reader.read_line(&mut more).unwrap();
        more
    });
    let Err(Unrun(words)) = status(&socket) else {
        panic!("a status from protocol 2");
    };
    assert!(words.contains("speaks protocol 2"), "{words}");
    assert_eq!(
        served.join().unwrap(),
        "",
        "the runner asked on the connection"
    );
}

#[test]
fn no_flume_host() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("host.sock");
    let Err(Unrun(words)) = status(&socket) else {
        panic!("a status with no flume host");
    };
    assert!(words.starts_with("no flume host answers on "), "{words}");
}

#[test]
fn an_error_line() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("host.sock");
    let _served = host(
        &socket,
        vec![
            r#"{"version":"0.36.0","protocol":1,"pid":1,"since":"x"}"#.to_string(),
            r#"{"error":"unknown variant `x`"}"#.to_string(),
        ],
    );
    assert_eq!(
        status(&socket),
        Err(Unrun(
            "flume host refused the ask: unknown variant `x`".to_string()
        ))
    );
}

/// The subscription keeps the latest document, wakes only on a change,
/// says when flume host stops answering, and ends at the stop.
#[test]
fn a_subscription() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("host.sock");
    let one = r#"{"status":{"places":[],"host":{"room":1}}}"#;
    let two = r#"{"status":{"places":[],"host":{"room":2}}}"#;
    let (close, closed) = std::sync::mpsc::channel();
    let served = host_until(
        &socket,
        vec![
            r#"{"version":"0.36.0","protocol":1,"pid":1,"since":"x"}"#.to_string(),
            format!(r#"{{"document":{one}}}"#),
            format!(r#"{{"document":{one}}}"#),
            format!(r#"{{"document":{two}}}"#),
        ],
        closed,
    );
    let subscription = Subscription::new(Where::At(socket.clone()));
    let stop = AtomicBool::new(false);
    let wakes = std::sync::atomic::AtomicUsize::new(0);
    let said = Mutex::new(Vec::new());
    let (seen, down) = std::thread::scope(|scope| {
        scope.spawn(|| {
            subscription.hold(
                &stop,
                &|| {
                    wakes.fetch_add(1, Ordering::SeqCst);
                },
                &|line| said.lock().unwrap().push(line),
            )
        });
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut seen = false;
        while Instant::now() < deadline && !seen {
            seen = subscription.latest() == Ok(Reply::Given(two.to_string()));
            std::thread::sleep(Duration::from_millis(10));
        }
        close.send(()).unwrap();
        let mut down = false;
        while Instant::now() < deadline && !down {
            down = !said.lock().unwrap().is_empty();
            std::thread::sleep(Duration::from_millis(10));
        }
        stop.store(true, Ordering::SeqCst);
        (seen, down)
    });
    assert!(seen, "the second document");
    assert!(down, "the line of the end");
    assert_eq!(
        served.join().unwrap(),
        [
            format!(r#"{{"hello":"boomdrive/{}"}}"#, env!("CARGO_PKG_VERSION")),
            r#"{"ask":"subscribe"}"#.to_string()
        ]
    );
    assert_eq!(wakes.load(Ordering::SeqCst), 2);
    let said = said.into_inner().unwrap();
    assert_eq!(said.len(), 1, "{said:?}");
    assert!(
        said[0].starts_with("boomdrive: flume host does not answer: "),
        "{said:?}"
    );
    assert!(matches!(subscription.latest(), Err(Unrun(_))));
}
