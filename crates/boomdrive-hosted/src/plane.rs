//! `Hosted`, the hosted control plane's implementation of the interface
//! (design Decisions 1, 7, 8, 9 and 11; core.2, core.21, core.26,
//! core.27): one connection for each partition of the hosted table, on a
//! runtime of its own, which the runner reaches through `pull`, `report`
//! and `owns` alone. [`Hosted::parts`] gives a part for each partition,
//! an account's enrollment and its one connection, each pulled, answered
//! and reported on its own and woken alone, all on one runtime the parts
//! share (record 0026; core.49). Every id a partition's control plane gives is handed to
//! the runner as `<label>/<id>`, and has its prefix taken off as it goes
//! up, so a partition's connection carries only its own repositories,
//! leases and work. What goes up of each tool's answer is version 2's copy
//! of it, cut and checked.

use crate::connection::{self, Answered, Connection, Dial};
use crate::copy;
use crate::key::{self, KeyName, Store};
use crate::protocol::v2::{self, MessageId};
use crate::table::{self, Table};
use boomdrive_control::{
    Capacity, Carried, CommandSort, ControlPlane, Hello, Id, Launched, Part, Pulled, Report,
    Reported, Unread, Unsent, Wake, Want,
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
/// configuration file `file`, as its parts: one for each partition, its
/// connection trying again on a backoff whose cap stops at
/// `poll_interval`, saying its lines to the say `say_for` makes for its
/// label and waking the runner for it by the wake `wake_for` makes for it
/// (design Decision 1; record 0026). Each partition's key is read from
/// the host's own credential store at every attempt; a host with none
/// says so for each partition, once, and opens nonetheless.
pub fn open(
    table: &toml::Table,
    file: &Path,
    poll_interval: Duration,
    say_for: &dyn Fn(&str) -> Say,
    wake_for: &dyn Fn(&str) -> Wake,
) -> Result<Vec<Part>, String> {
    let table = Table::read(table, file)?;
    let store: Arc<dyn Store> = match key::store_for_host() {
        Ok(store) => Arc::from(store),
        Err(words) => Arc::new(Lacking(words)),
    };
    Hosted::parts(&table, store, poll_interval, say_for, wake_for)
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

/// The hosted control plane, through each partition's connection: as a
/// part, one partition's.
pub struct Hosted {
    partitions: Vec<Partition>,
    say: connection::Say,
    /// Whether the host part's copy failed at the last pull, said once
    /// while it fails.
    host_part_failing: bool,
    /// The runtime its connections run on, shared with every part opened
    /// beside it, and stopped once the last of them is dropped.
    runtime: Arc<Shared>,
}

/// The runtime every part opened together shares, stopped once the last
/// part holding it is dropped.
struct Shared(Option<Runtime>);

impl Shared {
    fn handle(&self) -> &tokio::runtime::Handle {
        self.0
            .as_ref()
            .expect("the runtime is held until it is dropped")
            .handle()
    }
}

impl Drop for Shared {
    fn drop(&mut self) {
        if let Some(runtime) = self.0.take() {
            runtime.shutdown_background();
        }
    }
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
    /// The last report sent, until its answer comes.
    sent: Option<Sent>,
    /// Each want withheld at the last pull for naming a repository the
    /// partition is not served, said once while it is offered.
    withheld: BTreeSet<Id>,
    /// Each launch's carried answer whose copy failed at the last report,
    /// said once while it fails.
    failing: BTreeSet<(Id, &'static str)>,
    /// Each launch whose status went up at the last report with an end
    /// carried by its delivery alone or left out, said once while it does
    /// (design Decisions 2 and 3 of
    /// the-hosted-protocol-carries-every-end-of-a-place).
    partial: BTreeSet<Id>,
}

impl Hosted {
    /// The hosted implementation of `table`, each partition's key read
    /// from `store`, serving every partition in turn, saying its lines to
    /// `say` and woken by `wake` for any of them.
    pub fn new(
        table: &Table,
        store: Arc<dyn Store>,
        poll_interval: Duration,
        say: Say,
        wake: Wake,
    ) -> Result<Hosted, String> {
        let runtime = Arc::new(Shared(Some(runtime()?)));
        let say = shared_say(say);
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
                Partition::open(partition, &runtime, dial)
            })
            .collect();
        Ok(Hosted {
            partitions,
            say,
            host_part_failing: false,
            runtime,
        })
    }

    /// The hosted implementation of `table` as parts: one for each
    /// partition, in the table's order, labelled by it, each holding the
    /// partition's one connection, its key read from `store`, saying its
    /// lines to the say `say_for` makes for its label and woken by the
    /// wake `wake_for` makes for it, so a `pulled` on one connection wakes
    /// that part alone (record 0026; design Decision 8). The parts share
    /// one runtime, stopped once the last of them is dropped.
    pub fn parts(
        table: &Table,
        store: Arc<dyn Store>,
        poll_interval: Duration,
        say_for: &dyn Fn(&str) -> Say,
        wake_for: &dyn Fn(&str) -> Wake,
    ) -> Result<Vec<Part>, String> {
        let runtime = Arc::new(Shared(Some(runtime()?)));
        let parts = table
            .partitions
            .iter()
            .map(|partition| {
                let label = &partition.label;
                let say = shared_say(say_for(label));
                let dial = Dial::new(
                    table,
                    label,
                    store.clone(),
                    poll_interval,
                    say.clone(),
                    Arc::from(wake_for(label)),
                );
                let hosted = Hosted {
                    partitions: vec![Partition::open(partition, &runtime, dial)],
                    say,
                    host_part_failing: false,
                    runtime: runtime.clone(),
                };
                Part {
                    label: label.clone(),
                    plane: Box::new(hosted),
                }
            })
            .collect();
        Ok(parts)
    }

    /// `words` of the partition `label`, naming it where this holds more
    /// than one partition: a part's own words do not repeat its label,
    /// which the runner's line names.
    fn of(&self, label: &str, words: &str) -> String {
        match self.partitions.len() {
            1 => words.to_string(),
            _ => format!("{label}: {words}"),
        }
    }

    /// The host's capacity as version 2 carries it: Flume's host part as
    /// its copy, or `missing` with words naming the path where the copy
    /// fails, said once while it fails (design Decision 9).
    fn capacity(&mut self, capacity: &Capacity) -> v2::Capacity {
        let part = match capacity {
            Capacity::Missing(words) => return v2::Capacity::Missing(words.clone()),
            Capacity::Printed(part) => part,
        };
        match copy::host_part(part) {
            Ok(copy) => {
                self.host_part_failing = false;
                v2::Capacity::Printed(v2::HostPart {
                    running: raw(&copy["running"]),
                    bound: raw(&copy["bound"]),
                    room: raw(&copy["room"]),
                    streams: raw(&copy["streams"]),
                })
            }
            Err(words) => {
                let words = format!("the host part's copy fails version 2's check: {words}");
                if !std::mem::replace(&mut self.host_part_failing, true) {
                    (self.say)(format!("hosted: {words}, so it goes up missing"));
                }
                v2::Capacity::Missing(words)
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
        for at in 0..self.partitions.len() {
            let partition = &self.partitions[at];
            if let Some(words) = &partition.unhanded {
                unread.push(self.of(&partition.label, words));
                continue;
            }
            let patience = deadline.saturating_duration_since(Instant::now());
            let taken = match partition.connection.read(patience) {
                Ok(taken) => taken,
                Err(why) => {
                    unread.push(self.of(&partition.label, &why));
                    continue;
                }
            };
            let partition = &mut self.partitions[at];
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
            // An answer come since the last report names no status taken:
            // the statuses it took may have changed since.
            for (id, answer) in partition.connection.answers() {
                partition.answered(&id, answer, &mut reported);
            }
        }

        let mut waiting = Vec::new();
        let mut unanswered = Vec::new();
        for at in 0..self.partitions.len() {
            let label = self.partitions[at].label.clone();
            if !self.partitions[at].connection.live() {
                unanswered.push(self.of(&label, "it is not reached"));
                continue;
            }
            let say = self.say.clone();
            let partition = &mut self.partitions[at];
            let part = partition.part(report);
            let body = partition.upward(&part, &say);
            let statuses = partition.statuses_up(&part);
            match partition.connection.report(&body) {
                Ok(id) => {
                    partition.sent = Some(Sent {
                        id: id.clone(),
                        part,
                        statuses,
                    });
                    waiting.push((at, id));
                }
                Err(words) => {
                    (self.say)(format!("{label}: {words}"));
                    unanswered.push(self.of(&label, &words));
                }
            }
        }

        let deadline = Instant::now() + PATIENCE;
        let mut answered = 0;
        for (at, id) in waiting {
            let label = self.partitions[at].label.clone();
            let patience = deadline.saturating_duration_since(Instant::now());
            match self.partitions[at].connection.answer(&id, patience) {
                Some(answer @ Answered::Reported(_)) => {
                    answered += 1;
                    let taken = self.partitions[at].answered(&id, answer, &mut reported);
                    reported.statuses_taken.extend(taken);
                }
                Some(Answered::Refused(words)) => {
                    let words = format!("the control plane refused it: {words}");
                    unanswered.push(self.of(&label, &words));
                }
                None => {
                    let words = format!(
                        "the control plane did not answer the report within {}s",
                        PATIENCE.as_secs()
                    );
                    (self.say)(format!("{label}: {words}"));
                    unanswered.push(self.of(&label, &words));
                }
            }
        }
        if answered == 0 {
            return Err(Unsent(unanswered.join("; ")));
        }
        Ok(reported)
    }

    /// An id under one of its partitions' prefixes, `<label>/`.
    fn owns(&self, id: &Id) -> bool {
        self.partitions
            .iter()
            .any(|partition| partition.unprefixed(id).is_some())
    }
}

impl Drop for Hosted {
    /// Closes each of its sockets with a normal close and waits at most a
    /// second for them. The runtime stops once the last part holding it
    /// is dropped.
    fn drop(&mut self) {
        let tasks: Vec<_> = self
            .partitions
            .iter_mut()
            .filter_map(|partition| partition.connection.close())
            .collect();
        let (closed, waited) = mpsc::channel();
        self.runtime.handle().spawn(async move {
            for task in tasks {
                let _ = task.await;
            }
            let _ = closed.send(());
        });
        let _ = waited.recv_timeout(CLOSE_WAIT);
    }
}

/// The runtime the connections run on.
fn runtime() -> Result<Runtime, String> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("boomdrive-hosted")
        .enable_all()
        .build()
        .map_err(|e| format!("the hosted implementation's runtime could not be made: {e}"))
}

/// `say` as the connections say their lines, from the runtime's threads.
fn shared_say(say: Say) -> connection::Say {
    let say = Mutex::new(say);
    Arc::new(move |line| {
        let mut say = say.lock().unwrap_or_else(|e| e.into_inner());
        say(line);
    })
}

impl Partition {
    /// The partition `partition` of the table, its connection opened on
    /// `runtime` by `dial`.
    fn open(partition: &table::Partition, runtime: &Shared, dial: Dial) -> Partition {
        Partition {
            label: partition.label.clone(),
            repositories: partition.repositories.iter().cloned().collect(),
            connection: Connection::open(runtime.handle(), dial),
            unhanded: None,
            held: None,
            sent: None,
            withheld: BTreeSet::new(),
            failing: BTreeSet::new(),
            partial: BTreeSet::new(),
        }
    }

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
    /// those kept, and none the runner drains, which version 2 cannot name
    /// (record 0021), and only its own leases, their prefixes taken off;
    /// the host's tools and capacity as every partition's.
    fn hello(&self, hello: &Hello, capacity: v2::Capacity) -> v2::Hello {
        v2::Hello {
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
    /// it is offered (design Decision 8). Version 2 carries a land among
    /// the acts and a preview among the wants: a land is given as any act,
    /// and a preview of a repository the partition is not served is
    /// withheld as an offer is, every other field of it as given (design
    /// Decision 6 of the-hosted-protocol-carries-land-and-preview; core.26,
    /// core.53, core.55).
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
    /// (design Decision 7). Version 2's report holds no preview, so none
    /// is taken back (core.42).
    fn taken_back(&self, report: v2::Report) -> Report {
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

    /// `part` as version 2's `report` carries it: each id with its prefix
    /// taken off, and each launch's status and entry as its copy, or
    /// `null` where the copy fails, which is said once while it fails
    /// (design Decision 9). A status goes up with each end of the place
    /// copied on its own, and one with an end carried by its delivery
    /// alone or left out is said once while it does, naming each end's
    /// path and never a value (design Decision 2 of
    /// the-hosted-protocol-carries-every-end-of-a-place; core.11). Its
    /// four parts alone go up: version 2 names no previews, so a report's
    /// never do (core.42).
    fn upward(&mut self, part: &Report, say: &connection::Say) -> Value {
        let mut failing = BTreeSet::new();
        let mut partial = BTreeSet::new();
        let mut launches = serde_json::Map::new();
        for (id, launched) in &part.launches {
            let mut carried = |what: &'static str, carried: &Option<Carried>| {
                let carried = carried.as_ref()?;
                let copied = match what {
                    "status" => copy::place_status(&carried.text).map(|placed| {
                        if !placed.whole() {
                            if !self.partial.contains(id) {
                                say(partial_words(id, &placed));
                            }
                            partial.insert(id.clone());
                        }
                        placed.copy
                    }),
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
                                "{id}: its {what}'s copy fails version 2's check, so it goes up \
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
        self.partial = partial;
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

    /// The launches of `part` whose status went up whole as its copy at
    /// the last [`Partition::upward`]: each with a status, but those whose
    /// copy failed and went up `null` (design Decision 2), and those that
    /// went up with an end carried by its delivery alone or left out, so
    /// no end the control plane holds only the delivery of is confirmed
    /// (design Decision 3 of
    /// the-hosted-protocol-carries-every-end-of-a-place; core.54).
    fn statuses_up(&self, part: &Report) -> BTreeSet<Id> {
        part.launches
            .iter()
            .filter(|(id, launched)| {
                launched.status.is_some()
                    && !self.failing.contains(&((*id).clone(), "status"))
                    && !self.partial.contains(*id)
            })
            .map(|(id, _)| id.clone())
            .collect()
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
    /// is what the control plane holds, and it gives each launch whose
    /// status went up in it as its copy, but those let go or refused: the
    /// statuses taken, which the caller names where the answer is the one
    /// it waited for (design Decision 2).
    fn answered(
        &mut self,
        id: &MessageId,
        answer: Answered,
        reported: &mut Reported,
    ) -> BTreeSet<Id> {
        let last = self.sent.as_ref().is_some_and(|sent| &sent.id == id);
        let sent = match last {
            true => self.sent.take(),
            false => None,
        };
        let Answered::Reported(answer) = answer else {
            return BTreeSet::new();
        };
        let let_go: Vec<Id> = answer.let_go.iter().map(|id| self.prefixed(id)).collect();
        let refused: BTreeMap<Id, String> = answer
            .refused
            .iter()
            .map(|(id, words)| (self.prefixed(id), words.clone()))
            .collect();
        let taken = if let Some(Sent {
            mut part, statuses, ..
        }) = sent
        {
            for id in let_go.iter().chain(refused.keys()) {
                part.launches.remove(id);
                part.acts.remove(id);
                part.commands.remove(id);
                part.inboxes.remove(id);
                part.previews.remove(id);
            }
            let taken = statuses
                .into_iter()
                .filter(|id| part.launches.contains_key(id))
                .collect();
            self.held = Some(part);
            taken
        } else {
            BTreeSet::new()
        };
        reported.let_go.extend(let_go);
        reported.refused.extend(refused);
        taken
    }
}

/// A report a partition sent, until its answer comes.
struct Sent {
    /// Its message's id.
    id: MessageId,
    /// The part it holds, its ids under the partition's prefix.
    part: Report,
    /// Each launch of `part` whose status went up as its copy.
    statuses: BTreeSet<Id>,
}

/// `part`'s entries added to `into`.
fn merge(into: &mut Report, part: Report) {
    into.launches.extend(part.launches);
    into.acts.extend(part.acts);
    into.commands.extend(part.commands);
    into.inboxes.extend(part.inboxes);
    into.previews.extend(part.previews);
}

/// The line said of the launch `id` whose status went up as `placed`,
/// with an end carried by its delivery alone or left out: the launch and
/// each such end's path in Flume's status, and never a value (core.11).
fn partial_words(id: &Id, placed: &copy::PlaceCopy) -> String {
    let ends: Vec<String> = placed
        .by_delivery
        .iter()
        .map(|n| format!("/ends/{n} by its delivery alone"))
        .chain(
            placed
                .left_out
                .iter()
                .map(|n| format!("/ends/{n} left out")),
        )
        .collect();
    format!(
        "{id}: its status goes up with ends version 2 does not name: {}",
        ends.join(", ")
    )
}

/// `value` as raw JSON, as version 2's host part carries each value.
fn raw(value: &Value) -> Box<RawValue> {
    RawValue::from_string(value.to_string()).unwrap_or_else(|_| RawValue::NULL.to_owned())
}
