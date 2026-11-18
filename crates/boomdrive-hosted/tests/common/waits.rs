//! What a test of a connection waits with: the lines it says, read and
//! waited on, and tokio's paused clock held still while bytes cross the
//! loopback, as `connection.rs`'s tests wait.

use super::fake::{Fake, Taken};
use boomdrive_hosted::connection::Say;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::watch;

/// How long a test waits for what it waits on, by the host's own clock.
pub const PATIENCE: Duration = Duration::from_secs(10);

/// The lines a connection says, for a test to read and wait on.
#[derive(Clone)]
pub struct Lines {
    lines: Arc<Mutex<Vec<String>>>,
    said: Arc<watch::Sender<usize>>,
}

impl Default for Lines {
    fn default() -> Lines {
        Lines::new()
    }
}

impl Lines {
    pub fn new() -> Lines {
        Lines {
            lines: Arc::default(),
            said: Arc::new(watch::Sender::new(0)),
        }
    }

    /// Where the lines are said.
    pub fn say(&self) -> Say {
        let lines = self.clone();
        Arc::new(move |line| {
            lines.lines.lock().expect("the lines").push(line);
            lines.said.send_modify(|count| *count += 1);
        })
    }

    pub fn all(&self) -> Vec<String> {
        self.lines.lock().expect("the lines").clone()
    }

    /// The lines, once `until` holds of them, waited on without blocking
    /// the test's runtime; or the test fails, naming `what`.
    pub async fn until(&self, what: &str, until: impl Fn(&[String]) -> bool) -> Vec<String> {
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

    /// The lines, once `until` holds of them, waited on by blocking the
    /// test's thread; or the test fails, naming `what`.
    pub fn wait_until(&self, what: &str, until: impl Fn(&[String]) -> bool) -> Vec<String> {
        let deadline = std::time::Instant::now() + PATIENCE;
        loop {
            let lines = self.all();
            if until(&lines) {
                return lines;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "no line said {what}: {lines:#?}"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

/// Holds tokio's paused clock still while it lives: the runtime does not
/// move it on as it idles, so bytes cross the loopback in the host's own
/// time while the test's stands, and the test moves it by
/// `tokio::time::advance`.
pub struct Hold {
    _release: std::sync::mpsc::Sender<()>,
}

impl Hold {
    pub fn new() -> Hold {
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
pub async fn settle(millis: u64) {
    tokio::task::spawn_blocking(move || std::thread::sleep(Duration::from_millis(millis)))
        .await
        .expect("settled");
}

/// Moves the held clock on a second at a time, letting the host's own
/// time pass between, until `until` holds of what `fake` took; or the
/// test fails, naming `what`.
pub async fn advance_until(fake: &Fake, what: &str, until: impl Fn(&[Taken]) -> bool) {
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
