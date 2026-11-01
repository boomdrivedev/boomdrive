//! No connection comes in to the runner (the running spec's "No listener
//! in the code"; design 10; core.3): no crate's sources name a listening
//! socket, the control plane's implementations' among them, and the
//! workspace's lock names no HTTP or WebSocket server.

use std::path::{Path, PathBuf};

/// What names a listening socket, or a socket's `bind`, in Rust source.
const LISTENERS: [&str; 5] = [
    "TcpListener",
    "UnixListener",
    "UdpSocket",
    ".bind(",
    "::bind(",
];

/// The crates that serve HTTP or WebSockets.
const SERVERS: [&str; 8] = [
    "hyper",
    "axum",
    "actix-web",
    "warp",
    "tiny_http",
    "rouille",
    "tide",
    "poem",
];

/// Each `.rs` file under `dir`.
fn sources(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(dir).expect("the sources") {
        let path = entry.expect("an entry").path();
        if path.is_dir() {
            found.extend(sources(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
    found
}

/// Each line of `text` that names a listener.
fn listeners(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|line| LISTENERS.iter().any(|listener| line.contains(listener)))
        .collect()
}

/// The name of each package a `Cargo.lock` holds.
fn packages(lock: &str) -> Vec<&str> {
    lock.lines()
        .filter_map(|line| line.strip_prefix("name = \"")?.strip_suffix('"'))
        .collect()
}

/// The `src` of every crate under `crates/`.
fn crates_sources(crates: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(crates)
        .expect("crates/")
        .map(|entry| entry.expect("an entry").path().join("src"))
        .filter(|src| src.is_dir())
        .collect()
}

#[test]
fn no_listener_in_the_code() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dirs = crates_sources(&crate_dir.join(".."));
    for name in ["flywheel", "flywheel-control", "flywheel-crew"] {
        assert!(
            dirs.iter().any(|dir| dir.ends_with(format!("{name}/src"))),
            "{name}'s sources are not read: {dirs:?}"
        );
    }
    for dir in dirs {
        let files = sources(&dir);
        assert!(!files.is_empty(), "no sources read in {}", dir.display());
        for file in files {
            let text = std::fs::read_to_string(&file).expect("a source");
            assert_eq!(listeners(&text), Vec::<&str>::new(), "{}", file.display());
        }
    }
    let lock = std::fs::read_to_string(crate_dir.join("../../Cargo.lock")).expect("Cargo.lock");
    let names = packages(&lock);
    assert!(names.contains(&"flywheel"), "Cargo.lock is not read");
    let servers: Vec<&str> = names
        .into_iter()
        .filter(|name| SERVERS.contains(name))
        .collect();
    assert_eq!(servers, Vec::<&str>::new());
}

#[test]
fn a_listener_is_seen() {
    for line in [
        "let listener = std::net::TcpListener::bind(address)?;",
        "let socket = UdpSocket::bind(address)?;",
        "let server = UnixListener::bind(path)?;",
        "socket.bind(&address.into())?;",
    ] {
        assert_eq!(listeners(line), [line]);
    }
    assert_eq!(
        packages("[[package]]\nname = \"hyper\"\nversion = \"1.8.1\"\n"),
        ["hyper"]
    );
}
