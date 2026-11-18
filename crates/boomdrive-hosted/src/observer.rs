//! The hosted implementation's observers (design Decisions 2, 6 and 8 of
//! a-runner-reports-to-an-observer-it-takes-no-work-from; record 0048;
//! core.19): one for each partition of `[observer]`, each over one
//! connection dialed in the observing role, all on a runtime of their own,
//! apart from the hosted parts'. An observer is handed each control pass
//! of the part whose label is its partition's, and sends its observing
//! hello, naming only the repositories its partition names of those kept,
//! and an observed report of the part's launches, each cut to version 3's
//! copies as a partition's report is. It waits on nothing and drops every
//! answer, so nothing it is sent reaches the runner's work, and no end is
//! confirmed on it (core.49, core.54).

use crate::check;
use crate::connection::{self, Connection, Dial};
use crate::key::{self, Store};
use crate::plane::{self, cut_launch, Failing, Lacking, Say, Shared};
use crate::protocol::v3::{self, ObservingHello};
use crate::table::Table;
use boomdrive_control::{Hello, Observed, Observer, Report};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::Path;
use std::sync::{mpsc, Arc};
use std::time::Duration;

/// How long dropping an observer waits for its socket to close.
const CLOSE_WAIT: Duration = Duration::from_secs(1);

/// Opens the observers of the observer table, `[observer]`, `table`, of
/// the configuration file `file`: one for each partition, in the table's
/// order, its connection trying again on a backoff whose cap stops at
/// `poll_interval` and saying its lines to the say `say_for` makes for its
/// label. Each partition's key is read from the host's own credential
/// store at every attempt; a host with none says so for each partition,
/// once, and opens nonetheless. No connection is attempted before an
/// observer is handed its first pass.
pub fn open(
    table: &toml::Table,
    file: &Path,
    poll_interval: Duration,
    say_for: &dyn Fn(&str) -> Say,
) -> Result<Vec<Observed>, String> {
    let table = Table::read_observer(table, file)?;
    let store: Arc<dyn Store> = match key::store_for_host() {
        Ok(store) => Arc::from(store),
        Err(words) => Arc::new(Lacking(words)),
    };
    observers(&table, store, poll_interval, say_for)
}

/// The observers of the observer table `table`, read: one for each
/// partition, in its order, labelled by it, its key read from `store`. They
/// share one runtime, stopped once the last of them is dropped.
pub fn observers(
    table: &Table,
    store: Arc<dyn Store>,
    poll_interval: Duration,
    say_for: &dyn Fn(&str) -> Say,
) -> Result<Vec<Observed>, String> {
    let runtime = Arc::new(Shared(Some(plane::runtime()?)));
    check::prepare_observing();
    let observed = table
        .partitions
        .iter()
        .map(|partition| {
            let label = &partition.label;
            let say = plane::shared_say(say_for(label));
            let dial = Dial::observing(table, label, store.clone(), poll_interval, say.clone());
            let observing = Observing {
                label: label.clone(),
                repositories: partition.repositories.iter().cloned().collect(),
                connection: Connection::open(runtime.handle(), dial),
                say,
                unhanded: None,
                unsent: None,
                failing: Failing::default(),
                runtime: runtime.clone(),
            };
            Observed {
                label: label.clone(),
                observer: Box::new(observing),
            }
        })
        .collect();
    Ok(observed)
}

/// One partition's observer, over its observing connection.
pub struct Observing {
    label: String,
    /// The repositories the table names for its partition, `owner/name`.
    repositories: BTreeSet<String>,
    connection: Connection,
    say: connection::Say,
    /// Why its hello was not handed at the last pass, where it was not,
    /// said once while it lasts.
    unhanded: Option<String>,
    /// Why its report was not handed at the last pass, where it was not,
    /// said once while it lasts.
    unsent: Option<String>,
    /// What of the last report's copies failed, each said once while it
    /// lasts.
    failing: Failing,
    /// The runtime its connection runs on, shared with every observer
    /// opened beside it.
    runtime: Arc<Shared>,
}

impl Observing {
    /// The runner's hello as the observer is told it: the hello's versions
    /// and tool versions, and only the repositories its partition names of
    /// those kept, with no capacity, since it is offered no room, and no
    /// lease, since it gives none (record 0048, points 1 and 4; core.26).
    fn hello(&self, hello: &Hello) -> ObservingHello {
        ObservingHello {
            observing: v3::Observing,
            interface_version: hello.interface_version,
            runner_version: hello.runner_version.clone(),
            tool_versions: hello.tool_versions.clone(),
            repositories: hello
                .repositories
                .iter()
                .filter(|repository| self.repositories.contains(*repository))
                .cloned()
                .collect(),
        }
    }

    /// The part's `share` as the observer is told it: each launch, by its
    /// id as the runner holds it, with its answer, and its status and entry
    /// cut to version 3's copies, and no lease, act, command, inbox or
    /// preview (design Decision 2).
    fn report(&mut self, share: &Report) -> Value {
        let was = std::mem::take(&mut self.failing);
        let mut now = Failing::default();
        let mut launches = serde_json::Map::new();
        for (id, launched) in &share.launches {
            let (status, entry) = cut_launch(id, launched, &was, &mut now, &self.say);
            launches.insert(
                id.as_str().to_string(),
                json!({
                    "answer": launched.answer,
                    "status": status,
                    "entry": entry,
                }),
            );
        }
        self.failing = now;
        json!({ "observing": true, "launches": launches })
    }
}

impl Observer for Observing {
    /// Hands the connection the observing hello and the observed report,
    /// each checked against its form, and returns: the connection sends
    /// them from its runtime, the report at once on a live connection and
    /// after the hello of its next one, so nothing here waits on a socket
    /// or an answer (design Decision 8). Every answer come since is
    /// dropped: a `reported` has ended its report's resending, and its
    /// let-go and refused ids change nothing (core.10, core.54).
    fn observe(&mut self, hello: &Hello, share: &Report) {
        drop(self.connection.answers());
        let own = self.hello(hello);
        let handed = self.connection.hand(&own);
        said_once(&self.say, &self.label, &mut self.unhanded, handed);
        let body = self.report(share);
        let sent = self.connection.report(&body).map(drop);
        said_once(&self.say, &self.label, &mut self.unsent, sent);
    }
}

/// Why `result` failed, said by `say` naming `label` once while `kept`
/// holds the same words, and kept; nothing kept where it did not fail.
fn said_once(
    say: &connection::Say,
    label: &str,
    kept: &mut Option<String>,
    result: Result<(), String>,
) {
    match result {
        Ok(()) => *kept = None,
        Err(words) => {
            if kept.as_ref() != Some(&words) {
                say(format!("{label}: {words}"));
            }
            *kept = Some(words);
        }
    }
}

impl Drop for Observing {
    /// Closes its socket with a normal close and waits at most a second
    /// for it, as a hosted part's drop does. The runtime stops once the
    /// last observer holding it is dropped.
    fn drop(&mut self) {
        let Some(task) = self.connection.close() else {
            return;
        };
        let (closed, waited) = mpsc::channel();
        self.runtime.handle().spawn(async move {
            let _ = task.await;
            let _ = closed.send(());
        });
        let _ = waited.recv_timeout(CLOSE_WAIT);
    }
}
