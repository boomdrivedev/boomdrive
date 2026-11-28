//! flume host's socket as the runner asks it (the change
//! the-runner-hears-flume-by-subscription): a status ask, a subscription,
//! another protocol, an error line and no flume host, each against a
//! flume host of the test's own on a scratch socket. It lives here, and not
//! beside the module, so that no source of the runner's names a listener
//! (core.3).

use boomdrive::flume::{Reply, Unrun};
use boomdrive::flume_host::{status, Subscription, Where};
use boomdrive::run::Edge;
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
    assert_eq!(status(&socket, &[]), Ok(Reply::Given(document.to_string())));
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
    let Err(Unrun(words)) = status(&socket, &[]) else {
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
    let Err(Unrun(words)) = status(&socket, &[]) else {
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
        status(&socket, &[]),
        Err(Unrun(
            "flume host refused the ask: unknown variant `x`".to_string()
        ))
    );
}

/// The subscription keeps the latest document, wakes only on a change
/// after its first, says when flume host stops answering, and ends at the
/// stop.
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
    // The runner's edge, on real time, whose pause ends at the stop.
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let edge = Edge::of(runtime.handle());
    let told = tokio::sync::Notify::new();
    let wakes = std::sync::atomic::AtomicUsize::new(0);
    let said = Mutex::new(Vec::new());
    let (seen, down) = std::thread::scope(|scope| {
        scope.spawn(|| {
            subscription.hold(
                &stop,
                &|wait| edge.pause(wait, &stop, &told),
                &|| {
                    wakes.fetch_add(1, Ordering::SeqCst);
                },
                &|line| said.lock().unwrap().push(line),
            )
        });
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut seen = false;
        while Instant::now() < deadline && !seen {
            seen = subscription.latest(&[]) == Ok(Reply::Given(two.to_string()));
            std::thread::sleep(Duration::from_millis(10));
        }
        close.send(()).unwrap();
        let mut down = false;
        while Instant::now() < deadline && !down {
            down = !said.lock().unwrap().is_empty();
            std::thread::sleep(Duration::from_millis(10));
        }
        stop.store(true, Ordering::SeqCst);
        told.notify_waiters();
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
    assert_eq!(
        wakes.load(Ordering::SeqCst),
        1,
        "the second document alone wakes, the first being the first read's"
    );
    let said = said.into_inner().unwrap();
    assert_eq!(said.len(), 1, "{said:?}");
    assert!(
        said[0].starts_with("boomdrive: flume host does not answer: "),
        "{said:?}"
    );
    assert!(matches!(subscription.latest(&[]), Err(Unrun(_))));
}

/// The running spec's "The subscription names the kept clones": a status
/// ask names each folder it reads, and a subscription asks first at the
/// first read, naming its folders, and asks again on a new connection
/// when a read names others, a read waiting for the document of its own
/// ask; the first document of each ask wakes no part.
#[test]
fn the_asks_name_the_folders_read() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("host.sock");
    let hello = r#"{"version":"0.37.0","protocol":1,"pid":1,"since":"x"}"#;
    let document = r#"{"status":{"places":[],"host":{}}}"#;
    let served = host(
        &socket,
        vec![hello.to_string(), format!(r#"{{"document":{document}}}"#)],
    );
    assert_eq!(
        status(&socket, &["/r".to_string(), "/s".to_string()]),
        Ok(Reply::Given(document.to_string()))
    );
    assert_eq!(
        served.join().unwrap()[1],
        r#"{"ask":"status","repositories":["/r","/s"]}"#
    );
    std::fs::remove_file(&socket).unwrap();

    // A flume host that answers each connection's ask with a document
    // naming the folders it asked, and keeps each connection open until
    // the next comes.
    let listener = UnixListener::bind(&socket).unwrap();
    let asks = std::sync::Arc::new(Mutex::new(Vec::new()));
    let kept = asks.clone();
    std::thread::spawn(move || {
        let mut open = Vec::new();
        for stream in listener.incoming() {
            let Ok(stream) = stream else { return };
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut writer = stream;
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            writer.write_all(format!("{hello}\n").as_bytes()).unwrap();
            line.clear();
            reader.read_line(&mut line).unwrap();
            let asked: serde_json::Value = serde_json::from_str(line.trim_end()).unwrap();
            kept.lock().unwrap().push(line.trim_end().to_string());
            let named = asked["repositories"].clone();
            writer
                .write_all(
                    format!(
                        r#"{{"document":{{"status":{{"places":[],"host":{{}},"named":{named}}}}}}}"#
                    )
                    .as_bytes(),
                )
                .unwrap();
            writer.write_all(b"\n").unwrap();
            open.push(writer);
        }
    });
    let subscription = Subscription::new(Where::At(socket.clone()));
    let stop = AtomicBool::new(false);
    // The runner's edge, on real time, whose pause ends at the stop.
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let edge = Edge::of(runtime.handle());
    let told = tokio::sync::Notify::new();
    let wakes = std::sync::atomic::AtomicUsize::new(0);
    let (first, second) = std::thread::scope(|scope| {
        scope.spawn(|| {
            subscription.hold(
                &stop,
                &|wait| edge.pause(wait, &stop, &told),
                &|| {
                    wakes.fetch_add(1, Ordering::SeqCst);
                },
                &|_| {},
            )
        });
        let first = subscription.latest(&["/r".to_string()]);
        let second = subscription.latest(&["/r".to_string(), "/s".to_string()]);
        stop.store(true, Ordering::SeqCst);
        told.notify_waiters();
        (first, second)
    });
    assert_eq!(
        first,
        Ok(Reply::Given(
            r#"{"status":{"places":[],"host":{},"named":["/r"]}}"#.to_string()
        ))
    );
    assert_eq!(
        second,
        Ok(Reply::Given(
            r#"{"status":{"places":[],"host":{},"named":["/r","/s"]}}"#.to_string()
        ))
    );
    assert_eq!(
        *asks.lock().unwrap(),
        [
            r#"{"ask":"subscribe","repositories":["/r"]}"#,
            r#"{"ask":"subscribe","repositories":["/r","/s"]}"#,
        ]
    );
    assert_eq!(wakes.load(Ordering::SeqCst), 0);
}
