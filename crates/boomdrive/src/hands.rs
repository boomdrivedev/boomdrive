//! The threads a part carries its wants' commands on, and the channel
//! their answers come back over (record 0002 as amended on 2026-11-19;
//! core.50). A launch, an act on a place, a preview and a release each run
//! their tool's command on a thread of their own as soon as the pass gives
//! it one, so no want waits on another want's command; each thread sends
//! its answer back to its part over one channel the moment its command
//! returns, and wakes the part. The part alone reads the report, answers
//! the control plane and says each line: a thread runs a command and
//! judges nothing of its answer (core.1).
//!
//! What a part has in flight is kept across its passes in [`Underway`],
//! so a want being carried is never taken again from a later pull
//! (core.10), and the part's next send carries each answer once it is
//! made, waiting on no command and on no pass's end.

use crate::config::Address;
use crate::facts;
use crate::flume::{self, Flume, LaunchArgs, Reply, Unrun};
use crate::locks::Locks;
use crate::sluice::{self, Asked, Sluice};
use boomdrive_control::{ActSort, Id, Want};
use serde_json::Value;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread::{self, Scope};

/// A tool's command a want runs, with all it needs to run it on a thread
/// of its own.
#[derive(Clone, Debug)]
pub enum Job {
    /// `flume launch`, in the clone at `container`, under the clone's lock
    /// held shared where `address` names its repository, so it waits for
    /// that repository's keep alone (record 0026; core.14), and only where
    /// the clone is still there once the lock is held.
    Launch {
        args: Box<LaunchArgs>,
        briefs: PathBuf,
        address: Option<Address>,
        container: PathBuf,
    },
    /// Flume's own command for an act on the place at `place`, its entry
    /// `listed` where the pass's listing holds it. A place the listing does
    /// not hold is first asked of Flume's status, whose refusal is the
    /// act's answer (core.51); a confirm naming a key that is the delivery
    /// of no end of the place runs nothing (core.56).
    Act {
        place: String,
        listed: Option<Value>,
        what: ActSort,
    },
    /// `sluice preview <branch> <revision>`, in the clone at `container`.
    Preview {
        container: PathBuf,
        branch: String,
        revision: String,
    },
    /// `sluice release <name>`, in the clone at `container`.
    Release { container: PathBuf, name: String },
}

/// What a want's command gave, as its tool gave it.
#[derive(Clone, Debug)]
pub enum Ran {
    /// Flume's answer to the launch, or none where the clone was let go
    /// before the launch held its lock, so the offer is left unanswered,
    /// as one whose clone is not yet made.
    Launch(Option<Result<Reply, Unrun>>),
    Act(ActRan),
    Preview(Result<Asked, sluice::Unrun>),
    Release(Result<sluice::Reply, sluice::Unrun>),
}

/// What an act's command gave.
#[derive(Clone, Debug)]
pub enum ActRan {
    /// Flume refused the status of a place the listing did not hold: the
    /// act's answer, with Flume's words.
    StatusRefused(String),
    /// Flume could not be run for that status, so nothing is kept.
    StatusUnrun(String),
    /// A confirm's key that is the delivery of no end of the place, as
    /// the listing or the status gave it: refused in the runner's words.
    Unheld(String),
    /// Flume's answer to its command on the place.
    Acted(Result<Reply, Unrun>),
}

/// A want's command that has returned: the want as the pull gave it, the
/// order it was started in, and what its command gave.
#[derive(Clone, Debug)]
pub struct Done {
    pub seq: u64,
    pub want: Want,
    pub ran: Ran,
}

/// Runs a want's command on a thread of its own.
pub trait Hands {
    /// Starts `job` for `want`, the `seq`th the part started, and returns
    /// at once; its [`Done`] comes back over the part's channel.
    fn start(&self, seq: u64, want: Want, job: Job);
}

/// Hands that run each command on a thread of `scope`, so every one
/// started has ended before the scope does.
pub struct Scoped<'scope, 'env> {
    scope: &'scope Scope<'scope, 'env>,
    flume: &'env dyn Flume,
    sluice: &'env dyn Sluice,
    locks: &'env Locks,
    answers: Sender<Done>,
    /// Told once each answer is sent, so the part takes it at once.
    wake: &'env (dyn Fn() + Sync),
}

impl Hands for Scoped<'_, '_> {
    fn start(&self, seq: u64, want: Want, job: Job) {
        let (flume, sluice, locks, wake) = (self.flume, self.sluice, self.locks, self.wake);
        let answers = self.answers.clone();
        self.scope.spawn(move || {
            let ran = run(flume, sluice, locks, job);
            // A part that has stopped taking answers takes none.
            let _ = answers.send(Done { seq, want, ran });
            wake();
        });
    }
}

/// Runs `job` with these tools, on the thread it is called on.
fn run(flume: &dyn Flume, sluice: &dyn Sluice, locks: &Locks, job: Job) -> Ran {
    match job {
        Job::Launch {
            args,
            briefs,
            address,
            container,
        } => {
            let held = address.as_ref().map(|address| locks.launch(address));
            let reply = container.is_dir().then(|| flume.launch(&args, &briefs));
            drop(held);
            Ran::Launch(reply)
        }
        Job::Act {
            place,
            listed,
            what,
        } => Ran::Act(act(flume, &place, listed, &what)),
        Job::Preview {
            container,
            branch,
            revision,
        } => Ran::Preview(sluice.preview(&container, &branch, &revision)),
        Job::Release { container, name } => Ran::Release(sluice.release(&container, &name)),
    }
}

/// Carries out `what` on the place at `place`, its entry `listed` where
/// the pass's listing held it (core.51, core.56).
fn act(flume: &dyn Flume, place: &str, listed: Option<Value>, what: &ActSort) -> ActRan {
    let entry = match listed {
        Some(entry) => Some(entry),
        None => match flume.status_of(None, place) {
            Ok(Reply::Given(status)) => serde_json::from_str::<Value>(&status)
                .ok()
                .and_then(|answer| facts::entry(&answer, place).cloned()),
            Ok(Reply::Refused(words)) => return ActRan::StatusRefused(words),
            Err(Unrun(words)) => return ActRan::StatusUnrun(words),
        },
    };
    if let ActSort::Confirm { deliveries, .. } = what {
        let held = entry.as_ref().map(facts::deliveries).unwrap_or_default();
        if let Some(key) = deliveries.iter().find(|key| !held.contains(&key.as_str())) {
            return ActRan::Unheld(key.clone());
        }
    }
    ActRan::Acted(flume.act(place, &flume_act(what)))
}

/// `what` as Flume's own command of its name.
pub fn flume_act(what: &ActSort) -> flume::Act {
    match what {
        ActSort::Retry {
            stage,
            words,
            submission,
        } => flume::Act::Retry {
            stage: stage.clone(),
            words: words.clone(),
            submission: *submission,
        },
        // A land is Flume's retry of the submission, naming no stage and
        // no words, whatever the launch's kind (core.53).
        ActSort::Land => flume::Act::Retry {
            stage: None,
            words: None,
            submission: true,
        },
        // A confirm is Flume's confirm of exactly the keys it names, in
        // its order, as the recipient it names (core.56).
        ActSort::Confirm {
            recipient,
            deliveries,
        } => flume::Act::Confirm {
            recipient: recipient.clone(),
            keys: deliveries.clone(),
        },
        // A hold is Flume's hold of the move the place waits on its caller
        // for, with exactly the words and the bound it names (core.59).
        ActSort::Hold {
            words,
            bound_interval,
        } => flume::Act::Hold {
            words: words.clone(),
            bound_interval: *bound_interval,
        },
        ActSort::Drop => flume::Act::Drop,
        ActSort::Priority(priority) => flume::Act::Priority(*priority),
        ActSort::Done => flume::Act::Done,
    }
}

/// A want whose command runs.
#[derive(Clone, Debug)]
pub struct Flight {
    seq: u64,
    /// The launch an act acts on.
    on: Option<Id>,
    /// A launch's stream and mark, which its room is counted by until
    /// Flume's listing holds its place.
    launch: Option<(Option<String>, String)>,
    /// The branch a launch's place works on, which a launch based on it
    /// waits for.
    branch: Option<String>,
    /// A preview's repository and revision.
    preview: Option<(String, String)>,
}

impl Flight {
    /// A launch running in `stream`, or naming none, marked `mark`, whose
    /// place works on `branch`.
    pub fn launch(stream: Option<String>, mark: String, branch: String) -> Flight {
        Flight {
            seq: 0,
            on: None,
            launch: Some((stream, mark)),
            branch: Some(branch),
            preview: None,
        }
    }

    /// An act on launch `launch`.
    pub fn act(launch: Id) -> Flight {
        Flight {
            seq: 0,
            on: Some(launch),
            launch: None,
            branch: None,
            preview: None,
        }
    }

    /// A preview's bring-up or release of `revision` in `repository`.
    pub fn preview(repository: String, revision: String) -> Flight {
        Flight {
            seq: 0,
            on: None,
            launch: None,
            branch: None,
            preview: Some((repository, revision)),
        }
    }
}

/// What a part carries across its passes: its hands, the answers coming
/// back, each want in flight, and what the last pass read, so a part takes
/// answers between its passes (record 0002 as amended on 2026-11-19).
pub struct Underway<'h> {
    hands: &'h dyn Hands,
    answers: Receiver<Done>,
    flights: RefCell<BTreeMap<Id, Flight>>,
    next: Cell<u64>,
    /// What the control plane let go or refused while a command of it, or
    /// of an act on it, ran: its answer is not reported (core.10).
    gone: RefCell<BTreeSet<Id>>,
    /// Each want started since the part's pass began: one whose command
    /// returned with no answer, its tool not run, waits for the next pass.
    started: RefCell<BTreeSet<Id>>,
    /// The last pull's wants, in the control plane's order.
    wants: RefCell<Vec<Want>>,
    /// Flume's listing as the last pass last read it.
    listing: RefCell<Option<Value>>,
}

impl<'h> Underway<'h> {
    pub fn new(hands: &'h dyn Hands, answers: Receiver<Done>) -> Underway<'h> {
        Underway {
            hands,
            answers,
            flights: RefCell::new(BTreeMap::new()),
            next: Cell::new(0),
            gone: RefCell::new(BTreeSet::new()),
            started: RefCell::new(BTreeSet::new()),
            wants: RefCell::new(Vec::new()),
            listing: RefCell::new(None),
        }
    }

    /// Starts `job` for `want`, whose command is now in flight.
    pub fn start(&self, want: &Want, job: Job, mut flight: Flight) {
        let seq = self.next.get();
        self.next.set(seq + 1);
        flight.seq = seq;
        self.started.borrow_mut().insert(want.id().clone());
        self.flights.borrow_mut().insert(want.id().clone(), flight);
        self.hands.start(seq, want.clone(), job);
    }

    /// Whether want `id` was started since the part's pass began, so it is
    /// not started again before the next.
    pub fn started(&self, id: &Id) -> bool {
        self.started.borrow().contains(id)
    }

    /// Whether want `id`'s command is in flight.
    pub fn flying(&self, id: &Id) -> bool {
        self.flights.borrow().contains_key(id)
    }

    /// Whether nothing is in flight.
    pub fn idle(&self) -> bool {
        self.flights.borrow().is_empty()
    }

    /// The stream and mark of each launch in flight.
    pub fn launches(&self) -> Vec<(Option<String>, String)> {
        self.flights
            .borrow()
            .values()
            .filter_map(|flight| flight.launch.clone())
            .collect()
    }

    /// Whether a launch in flight makes the place that works on `branch`.
    pub fn making(&self, branch: &str) -> bool {
        self.flights
            .borrow()
            .values()
            .any(|flight| flight.branch.as_deref() == Some(branch))
    }

    /// Whether a bring-up or release of `revision` in a repository that
    /// `same` reads as `repository` is in flight.
    pub fn previewing(&self, same: impl Fn(&str) -> bool, revision: &str) -> bool {
        self.flights.borrow().values().any(|flight| {
            flight
                .preview
                .as_ref()
                .is_some_and(|(repository, flying)| flying == revision && same(repository))
        })
    }

    /// Each answer that has come back, without waiting, in the order the
    /// answers arrived.
    pub fn arrived(&self) -> Vec<Done> {
        let mut done = Vec::new();
        loop {
            match self.answers.try_recv() {
                Ok(answer) => done.push(self.landed(answer)),
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => return done,
            }
        }
    }

    /// Waits until nothing is in flight, and gives every answer that came
    /// back meanwhile, in the order their commands were started.
    pub fn all(&self) -> Vec<Done> {
        let mut done = self.arrived();
        while !self.idle() {
            match self.answers.recv() {
                Ok(answer) => done.push(self.landed(answer)),
                Err(_) => break,
            }
        }
        done.sort_by_key(|answer| answer.seq);
        done
    }

    /// Takes `answer` off the flights.
    fn landed(&self, answer: Done) -> Done {
        self.flights.borrow_mut().remove(answer.want.id());
        answer
    }

    /// Takes the control plane's letting go or refusal of `id`: where it,
    /// or a launch an act in flight acts on, is in flight, its answer is
    /// not reported.
    pub fn let_go(&self, id: &Id) {
        let flights = self.flights.borrow();
        let held = flights.contains_key(id)
            || flights
                .values()
                .any(|flight| flight.on.as_ref() == Some(id));
        if held {
            self.gone.borrow_mut().insert(id.clone());
        }
    }

    /// Whether the control plane let `id` go, or refused it, while a
    /// command of it ran.
    pub fn gone(&self, id: &Id) -> bool {
        self.gone.borrow().contains(id)
    }

    /// Begins a pass: each want may be started again, and each id let go
    /// whose commands have all returned is forgotten.
    pub fn begin_pass(&self) {
        self.started.borrow_mut().clear();
        self.settle_gone();
    }

    /// Forgets each id let go whose commands have all returned.
    fn settle_gone(&self) {
        let flights = self.flights.borrow();
        self.gone.borrow_mut().retain(|id| {
            flights.contains_key(id)
                || flights
                    .values()
                    .any(|flight| flight.on.as_ref() == Some(id))
        });
    }

    /// Keeps the last pull's `wants` and Flume's `listing` as the pass
    /// read it, for the answers taken before the next pass.
    pub fn keep(&self, wants: &[Want], listing: Option<&Value>) {
        *self.wants.borrow_mut() = wants.to_vec();
        *self.listing.borrow_mut() = listing.cloned();
    }

    /// The last pull's wants.
    pub fn wants(&self) -> Vec<Want> {
        self.wants.borrow().clone()
    }

    /// Flume's listing as the last pass read it.
    pub fn listing(&self) -> Option<Value> {
        self.listing.borrow().clone()
    }
}

/// Runs `body` with a part's [`Underway`], whose commands run on threads
/// of a scope over `flume`, `sluice` and `locks`, each answer sent back
/// over its channel and `wake` told; the scope waits for every command
/// started before it returns.
pub fn carried<R>(
    flume: &dyn Flume,
    sluice: &dyn Sluice,
    locks: &Locks,
    wake: &(dyn Fn() + Sync),
    body: impl FnOnce(&Underway) -> R,
) -> R {
    let (answers, received) = mpsc::channel();
    thread::scope(|scope| {
        let hands = Scoped {
            scope,
            flume,
            sluice,
            locks,
            answers,
            wake,
        };
        let carrying = Underway::new(&hands, received);
        body(&carrying)
    })
}
