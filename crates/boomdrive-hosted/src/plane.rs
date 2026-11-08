//! `Hosted`, the hosted control plane's implementation of the interface
//! (design Decisions 1, 7, 8, 9 and 11; core.2, core.21, core.26,
//! core.27): one connection for each partition of the hosted table, on a
//! runtime of its own, which the runner's loop reaches through `pull` and
//! `report` alone. Every id a partition's control plane gives is handed to
//! the runner as `<label>/<id>`, and has its prefix taken off as it goes
//! up, so a partition's connection carries only its own repositories,
//! leases and work. What goes up of each tool's answer is version 1's copy
//! of it, cut and checked.

use crate::connection::{self, Answered, Connection, Dial};
use crate::copy;
use crate::key::{self, KeyName, Store};
use crate::protocol::v1::{self, MessageId};
use crate::table::Table;
use boomdrive_control::{
    Capacity, Carried, CommandSort, ControlPlane, Hello, Id, Launched, Pulled, Report, Reported,
    Unread, Unsent, Wake, Want,
};
use ed25519_dalek::SigningKey;
use serde_json::value::RawValue;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;

/// Where the implementation says what it did and could not do, one line
/// each.
pub type Say = Box<dyn FnMut(String) + Send>;

/// How long a pull waits for a partition being attempted, and a report
/// for each partition's `reported` (design Decision 7).
pub const PATIENCE: Duration = Duration::from_secs(10);

/// How long dropping the implementation waits for its sockets to close.
const CLOSE_WAIT: Duration = Duration::from_secs(1);

/// Opens the hosted implementation from its table, `table`, of the
/// configuration file `file`: one connection for each partition, each
/// trying again on a backoff whose cap stops at `poll_interval`, saying
/// its lines to `say` and waking the runner by `wake` (design Decision 1).
/// Each partition's key is read from the host's own credential store at
/// every attempt; a host with none says so for each partition, once, and
/// opens nonetheless.
pub fn open(
    table: &toml::Table,
    file: &Path,
    poll_interval: Duration,
    say: Say,
    wake: Wake,
) -> Result<Hosted, String> {
    let table = Table::read(table, file)?;
    let store: Arc<dyn Store> = match key::store_for_host() {
        Ok(store) => Arc::from(store),
        Err(words) => Arc::new(Lacking(words)),
    };
    Hosted::new(&table, store, poll_interval, say, wake)
}

/// A host with no credential store: every key is unread, with the words
/// saying what the host lacks (core.32).
struct Lacking(String);

impl Store for Lacking {
    fn read(&self, _: &KeyName) -> Result<Option<SigningKey>, String> {
        Err(self.0.clone())
    }

    fn write(&self, _: &KeyName, _: &SigningKey) -> Result<(), String> {
        Err(self.0.clone())
    }
}

/// The hosted control plane, through each partition's connection.
pub struct Hosted {
    partitions: Vec<Partition>,
    say: connection::Say,
    /// Whether the host part's copy failed at the last pull, said once
    /// while it fails.
    host_part_failing: bool,
    runtime: Option<Runtime>,
}

/// One partition, as the implementation holds it.
struct Partition {
    label: String,
    /// The repositories the table names for it, `owner/name`.
    repositories: BTreeSet<String>,
    connection: Connection,
    /// Why its hello was not handed at the last pull, where it was not,
    /// said once while it lasts.
    unhanded: Option<String>,
    /// Its part of the last report its control plane answered with
    /// `reported`, as the runner handed it, its ids under the partition's
    /// prefix, but for what it let go or refused: the report as its
    /// control plane holds it.
    held: Option<Report>,
    /// The last report sent, by its message's id, with the part it holds,
    /// until its answer comes.
    sent: Option<(MessageId, Report)>,
    /// Each want withheld at the last pull for naming a repository the
    /// partition is not served, said once while it is offered.
    withheld: BTreeSet<Id>,
    /// Each launch's carried answer whose copy failed at the last report,
    /// said once while it fails.
    failing: BTreeSet<(Id, &'static str)>,
}

impl Hosted {
    /// The hosted implementation of `table`, each partition's key read
    /// from `store`.
    pub fn new(
        table: &Table,
        store: Arc<dyn Store>,
        poll_interval: Duration,
        say: Say,
        wake: Wake,
    ) -> Result<Hosted, String> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("boomdrive-hosted")
            .enable_all()
            .build()
            .map_err(|e| format!("the hosted implementation's runtime could not be made: {e}"))?;
        let say = Mutex::new(say);
        let say: connection::Say = Arc::new(move |line| {
            let mut say = say.lock().unwrap_or_else(|e| e.into_inner());
            say(line);
        });
        let wake: connection::Wake = Arc::from(wake);
        let partitions = table
            .partitions
            .iter()
            .map(|partition| {
                let dial = Dial::new(
                    table,
                    &partition.label,
                    store.clone(),
                    poll_interval,
                    say.clone(),
                    wake.clone(),
                );
                Partition {
                    label: partition.label.clone(),
                    repositories: partition.repositories.iter().cloned().collect(),
                    connection: Connection::open(runtime.handle(), dial),
                    unhanded: None,
                    held: None,
                    sent: None,
                    withheld: BTreeSet::new(),
                    failing: BTreeSet::new(),
                }
            })
            .collect();
        Ok(Hosted {
            partitions,
            say,
            host_part_failing: false,
            runtime: Some(runtime),
        })
    }

    /// The host's capacity as version 1 carries it: Flume's host part as
    /// its copy, or `missing` with words naming the path where the copy
    /// fails, said once while it fails (design Decision 9).
    fn capacity(&mut self, capacity: &Capacity) -> v1::Capacity {
        let part = match capacity {
            Capacity::Missing(words) => return v1::Capacity::Missing(words.clone()),
            Capacity::Printed(part) => part,
        };
        match copy::host_part(part) {
            Ok(copy) => {
                self.host_part_failing = false;
                v1::Capacity::Printed(v1::HostPart {
                    running: raw(&copy["running"]),
                    bound: raw(&copy["bound"]),
                    room: raw(&copy["room"]),
                    streams: raw(&copy["streams"]),
                })
            }
            Err(words) => {
                let words = format!("the host part's copy fails version 1's check: {words}");
                if !std::mem::replace(&mut self.host_part_failing, true) {
                    (self.say)(format!("hosted: {words}, so it goes up missing"));
                }
                v1::Capacity::Missing(words)
            }
        }
    }
}

impl ControlPlane for Hosted {
    fn pull(&mut self, hello: &Hello, _report: &Report) -> Result<Pulled, Unread> {
        let capacity = self.capacity(&hello.capacity);
        for partition in &mut self.partitions {
            let own = partition.hello(hello, capacity.clone());
            match partition.connection.hand(&own) {
                Ok(()) => partition.unhanded = None,
                Err(words) => {
                    if partition.unhanded.as_ref() != Some(&words) {
                        (self.say)(format!("{}: {words}", partition.label));
                    }
                    partition.unhanded = Some(words);
                }
            }
        }

        let deadline = Instant::now() + PATIENCE;
        let mut wants = Vec::new();
        let mut held: Option<Report> = None;
        let mut unread = Vec::new();
        let mut read = 0;
        for partition in &mut self.partitions {
            if let Some(words) = &partition.unhanded {
                unread.push(format!("{}: {words}", partition.label));
                continue;
            }
            let patience = deadline.saturating_duration_since(Instant::now());
            let taken = match partition.connection.read(patience) {
                Ok(taken) => taken,
                Err(why) => {
                    unread.push(format!("{}: {why}", partition.label));
                    continue;
                }
            };
            read += 1;
            wants.extend(partition.own_wants(taken.wants, &self.say));
            let report = match &partition.held {
                Some(report) => Some(report.clone()),
                None => taken.report.map(|report| partition.taken_back(report)),
            };
            if let Some(report) = report {
                merge(held.get_or_insert_with(Report::default), report);
            }
        }
        if read == 0 {
            return Err(Unread(unread.join("; ")));
        }
        Ok(Pulled {
            wants,
            terminals: Vec::new(),
            report: held,
        })
    }

    fn report(&mut self, report: &Report) -> Result<Reported, Unsent> {
        let mut reported = Reported::default();
        for partition in &mut self.partitions {
            for (id, answer) in partition.connection.answers() {
                partition.answered(&id, answer, &mut reported);
            }
        }

        let mut waiting = Vec::new();
        let mut unanswered = Vec::new();
        for (at, partition) in self.partitions.iter_mut().enumerate() {
            if !partition.connection.live() {
                unanswered.push(format!("{}: it is not reached", partition.label));
                continue;
            }
            let part = partition.part(report);
            let body = partition.upward(&part, &self.say);
            match partition.connection.report(&body) {
                Ok(id) => {
                    partition.sent = Some((id.clone(), part));
                    waiting.push((at, id));
                }
                Err(words) => {
                    (self.say)(format!("{}: {words}", partition.label));
                    unanswered.push(format!("{}: {words}", partition.label));
                }
            }
        }

        let deadline = Instant::now() + PATIENCE;
        let mut answered = 0;
        for (at, id) in waiting {
            let partition = &mut self.partitions[at];
            let patience = deadline.saturating_duration_since(Instant::now());
            match partition.connection.answer(&id, patience) {
                Some(answer @ Answered::Reported(_)) => {
                    answered += 1;
                    partition.answered(&id, answer, &mut reported);
                }
                Some(Answered::Refused(words)) => {
                    unanswered.push(format!(
                        "{}: the control plane refused it: {words}",
                        partition.label
                    ));
                }
                None => {
                    let words = format!(
                        "the control plane did not answer the report within {}s",
                        PATIENCE.as_secs()
                    );
                    (self.say)(format!("{}: {words}", partition.label));
                    unanswered.push(format!("{}: {words}", partition.label));
                }
            }
        }
        if answered == 0 {
            return Err(Unsent(unanswered.join("; ")));
        }
        Ok(reported)
    }
}

impl Drop for Hosted {
    /// Closes each socket with a normal close, waits at most a second for
    /// them, and stops the runtime.
    fn drop(&mut self) {
        let Some(runtime) = self.runtime.take() else {
            return;
        };
        let tasks: Vec<_> = self
            .partitions
            .iter_mut()
            .filter_map(|partition| partition.connection.close())
            .collect();
        let (closed, waited) = mpsc::channel();
        runtime.spawn(async move {
            for task in tasks {
                let _ = task.await;
            }
            let _ = closed.send(());
        });
        let _ = waited.recv_timeout(CLOSE_WAIT);
        runtime.shutdown_background();
    }
}

impl Partition {
    /// The partition's prefix, which every id its control plane gives
    /// carries in the runner.
    fn prefix(&self) -> String {
        format!("{}/", self.label)
    }

    /// `id` under the partition's prefix.
    fn prefixed(&self, id: &Id) -> Id {
        Id::new(format!("{}/{id}", self.label))
    }

    /// `id` with the partition's prefix taken off, or none where it
    /// carries another's.
    fn unprefixed(&self, id: &Id) -> Option<Id> {
        id.as_str().strip_prefix(&self.prefix()).map(Id::new)
    }

    /// The runner's `hello` as this partition's control plane is told it
    /// (design Decision 8): only the repositories its table names, of
    /// those kept, and none the runner drains, which version 1 cannot name
    /// (record 0021), and only its own leases, their prefixes taken off;
    /// the host's tools and capacity as every partition's.
    fn hello(&self, hello: &Hello, capacity: v1::Capacity) -> v1::Hello {
        v1::Hello {
            interface_version: hello.interface_version,
            runner_version: hello.runner_version.clone(),
            tool_versions: hello.tool_versions.clone(),
            repositories: hello
                .repositories
                .iter()
                .filter(|repository| self.repositories.contains(*repository))
                .cloned()
                .collect(),
            capacity,
            leases: hello
                .leases
                .iter()
                .filter_map(|(id, number)| Some((self.unprefixed(id)?, *number)))
                .collect(),
        }
    }

    /// The wants of the partition's control plane as the runner takes
    /// them, their ids under its prefix, but for a want naming a
    /// repository the partition is not served, which is said once while
    /// it is offered (design Decision 8). Version 1 names no preview, so
    /// no message holding one passes its check; one that reached here
    /// would be withheld as an inbox is (core.26, core.42).
    fn own_wants(&mut self, wants: Vec<Want>, say: &connection::Say) -> Vec<Want> {
        let mut withheld = BTreeSet::new();
        let mut own = Vec::new();
        for want in wants {
            let named = match &want {
                Want::Offer(offer) => Some(("offer", &offer.launch.repository)),
                Want::Command(command) => match &command.what {
                    CommandSort::Branch { repository, .. } => Some(("command", repository)),
                    CommandSort::Other { .. } => None,
                },
                Want::Inbox(inbox) => Some(("inbox", &inbox.repository)),
                Want::Preview(preview) => Some(("preview", &preview.repository)),
                Want::Act(_) => None,
            };
            match named {
                Some((kind, repository)) if !self.repositories.contains(repository) => {
                    let id = self.prefixed(want.id());
                    if !self.withheld.contains(&id) {
                        say(format!(
                            "{}: the {kind} {id} names {repository}, which `{}` is not served, \
                             so it is not given to the runner",
                            self.label, self.label
                        ));
                    }
                    withheld.insert(id);
                }
                _ => own.push(self.prefixed_want(want)),
            }
        }
        self.withheld = withheld;
        own
    }

    /// `want` with each id it holds under the partition's prefix.
    fn prefixed_want(&self, want: Want) -> Want {
        match want {
            Want::Offer(mut offer) => {
                offer.id = self.prefixed(&offer.id);
                offer.launch.after = offer
                    .launch
                    .after
                    .iter()
                    .map(|id| self.prefixed(id))
                    .collect();
                Want::Offer(offer)
            }
            Want::Act(mut act) => {
                act.id = self.prefixed(&act.id);
                act.launch = self.prefixed(&act.launch);
                Want::Act(act)
            }
            Want::Command(mut command) => {
                command.id = self.prefixed(&command.id);
                Want::Command(command)
            }
            Want::Inbox(mut inbox) => {
                inbox.id = self.prefixed(&inbox.id);
                Want::Inbox(inbox)
            }
            Want::Preview(mut preview) => {
                preview.id = self.prefixed(&preview.id);
                Want::Preview(preview)
            }
        }
    }

    /// A report taken back from the partition's control plane, its ids
    /// under the partition's prefix: each launch's lease and answer, each
    /// act, command and inbox, and no launch's status or entry, which the
    /// control plane holds as their copies and not as the tools' answers
    /// (design Decision 7). Version 1's report holds no preview, so none
    /// is taken back (core.42).
    fn taken_back(&self, report: v1::Report) -> Report {
        Report {
            launches: report
                .launches
                .into_iter()
                .map(|(id, launched)| {
                    let launched = Launched {
                        lease: launched.lease,
                        repository: None,
                        answer: launched.answer,
                        status: None,
                        entry: None,
                    };
                    (self.prefixed(&id), launched)
                })
                .collect(),
            acts: report
                .acts
                .into_iter()
                .map(|(id, mut acted)| {
                    acted.launch = self.prefixed(&acted.launch);
                    (self.prefixed(&id), acted)
                })
                .collect(),
            commands: report
                .commands
                .into_iter()
                .map(|(id, commanded)| (self.prefixed(&id), commanded))
                .collect(),
            inboxes: report
                .inboxes
                .into_iter()
                .map(|(id, inbox)| (self.prefixed(&id), inbox))
                .collect(),
            previews: BTreeMap::new(),
        }
    }

    /// The partition's own part of the runner's `report`: the entries
    /// whose ids carry its prefix, still under it.
    fn part(&self, report: &Report) -> Report {
        let prefix = self.prefix();
        fn own<T: Clone>(entries: &BTreeMap<Id, T>, prefix: &str) -> BTreeMap<Id, T> {
            entries
                .iter()
                .filter(|(id, _)| id.as_str().starts_with(prefix))
                .map(|(id, entry)| (id.clone(), entry.clone()))
                .collect()
        }
        Report {
            launches: own(&report.launches, &prefix),
            acts: own(&report.acts, &prefix),
            commands: own(&report.commands, &prefix),
            inboxes: own(&report.inboxes, &prefix),
            previews: own(&report.previews, &prefix),
        }
    }

    /// `part` as version 1's `report` carries it: each id with its prefix
    /// taken off, and each launch's status and entry as its copy, or
    /// `null` where the copy fails, which is said once while it fails
    /// (design Decision 9). Its four parts alone go up: version 1 names no
    /// previews, so a report's never do (core.42).
    fn upward(&mut self, part: &Report, say: &connection::Say) -> Value {
        let mut failing = BTreeSet::new();
        let mut launches = serde_json::Map::new();
        for (id, launched) in &part.launches {
            let mut carried = |what: &'static str, carried: &Option<Carried>| {
                let carried = carried.as_ref()?;
                let copied = match what {
                    "status" => copy::place_status(&carried.text),
                    _ => copy::submission_entry(&carried.text),
                };
                match copied {
                    Ok(copy) => Some(json!({
                        "copy": copy,
                        "at": carried.at,
                        "listed": carried.listed,
                    })),
                    Err(words) => {
                        if !self.failing.contains(&(id.clone(), what)) {
                            say(format!(
                                "{id}: its {what}'s copy fails version 1's check, so it goes up \
                                 null: {words}"
                            ));
                        }
                        failing.insert((id.clone(), what));
                        None
                    }
                }
            };
            let status = carried("status", &launched.status);
            let entry = carried("entry", &launched.entry);
            launches.insert(
                self.bare(id),
                json!({
                    "lease": launched.lease,
                    "answer": launched.answer,
                    "status": status,
                    "entry": entry,
                }),
            );
        }
        self.failing = failing;
        let acts: serde_json::Map<String, Value> = part
            .acts
            .iter()
            .map(|(id, acted)| {
                let mut acted = acted.clone();
                acted.launch = Id::new(self.bare(&acted.launch));
                (self.bare(id), json!(acted))
            })
            .collect();
        let commands: serde_json::Map<String, Value> = part
            .commands
            .iter()
            .map(|(id, commanded)| (self.bare(id), json!(commanded)))
            .collect();
        let inboxes: serde_json::Map<String, Value> = part
            .inboxes
            .iter()
            .map(|(id, inbox)| (self.bare(id), json!(inbox)))
            .collect();
        json!({
            "launches": launches,
            "acts": acts,
            "commands": commands,
            "inboxes": inboxes,
        })
    }

    /// `id` with the partition's prefix taken off, where it carries it.
    fn bare(&self, id: &Id) -> String {
        self.unprefixed(id)
            .map(|id| id.as_str().to_string())
            .unwrap_or_else(|| id.as_str().to_string())
    }

    /// The control plane's `answer` to the report `id`: a `reported` adds
    /// its let-go and refused ids, under the partition's prefix, to
    /// `reported`, and where it answers the last report sent, that report
    /// is what the control plane holds.
    fn answered(&mut self, id: &MessageId, answer: Answered, reported: &mut Reported) {
        let last = self.sent.as_ref().is_some_and(|(sent, _)| sent == id);
        let part = match last {
            true => self.sent.take().map(|(_, part)| part),
            false => None,
        };
        let Answered::Reported(answer) = answer else {
            return;
        };
        let let_go: Vec<Id> = answer.let_go.iter().map(|id| self.prefixed(id)).collect();
        let refused: BTreeMap<Id, String> = answer
            .refused
            .iter()
            .map(|(id, words)| (self.prefixed(id), words.clone()))
            .collect();
        if let Some(mut part) = part {
            for id in let_go.iter().chain(refused.keys()) {
                part.launches.remove(id);
                part.acts.remove(id);
                part.commands.remove(id);
                part.inboxes.remove(id);
                part.previews.remove(id);
            }
            self.held = Some(part);
        }
        reported.let_go.extend(let_go);
        reported.refused.extend(refused);
    }
}

/// `part`'s entries added to `into`.
fn merge(into: &mut Report, part: Report) {
    into.launches.extend(part.launches);
    into.acts.extend(part.acts);
    into.commands.extend(part.commands);
    into.inboxes.extend(part.inboxes);
    into.previews.extend(part.previews);
}

/// `value` as raw JSON, as version 1's host part carries each value.
fn raw(value: &Value) -> Box<RawValue> {
    RawValue::from_string(value.to_string()).unwrap_or_else(|_| RawValue::NULL.to_owned())
}
