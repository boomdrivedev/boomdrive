//! The control pass (design Decision 5). It takes the runner's report,
//! reads the host from Flume's status, hands the hello to the pull,
//! answers each want in the control plane's order, but carries out an
//! act on a place only once every launch the pull offers is taken, and
//! none but the first once the next pass is due (record 0002; core.50),
//! carries each place's status against Flume's listing as it stands once
//! those answers are made (record 0027), opens each terminal pulled, and
//! sends the whole report. Once its control plane has taken a report
//! holding a place's status, it confirms to Flume, as the runner, each end
//! that status gives owed to it, so no end leaves Flume before its control
//! plane holds it (record 0030 as amended on 2026-11-09; core.54). An act
//! on a place the listing read before the pull does not hold reads Flume's
//! status of the place first, and takes Flume's refusal of it as its
//! answer (record 0010; core.51). The runner takes only what it was
//! offered, launches only through Flume's command line, and carries each
//! answer as Flume gave it (core.1, core.9, core.10). It reaches its
//! control plane only through `ControlPlane` (core.2), and acts on no
//! place it did not launch. Work under a lease that has lapsed by the
//! pass's clock waits for a live one (core.24). A preview its control plane asks for is brought up in a
//! repository the runner keeps by sluice's own preview command, after
//! every launch the pull offers, its ask awaited no longer than the
//! runner's patience; sluice's listing of it is carried until sluice lists
//! it no more; and on its done, only what the runner asked sluice for is
//! put away, by sluice's own release, with no output of a command carried
//! (core.55). A pass is over one part of the control plane (record 0026):
//! it reads and saves that part's share of the report file alone, and
//! each of its lines names the part's label after `boomdrive:`. Each act
//! is said on standard error, one line each, prefixed with its id, which
//! is the part's own, and a pass that changes nothing says nothing.

use crate::config::{Address, Config};
use crate::flume::{self, Flume, LaunchArgs, Reply, Unrun};
use crate::git::Git;
use crate::hello::{self, Tools};
use crate::locks::Locks;
use crate::report::{self, ReportFile};
use crate::run::Notices;
use crate::sluice::{self, Asked, Sluice};
use crate::terminal::Relays;
use crate::{clone, facts, inbox};
use boomdrive_control::{
    Act, ActSort, Acted, Answer, At, Carrying, Command, CommandSort, Commanded, ControlPlane, Id,
    Inbox, InboxCarried, InboxFile, KeptRepository, Launched, Lease, Offer, Outcome, Preview,
    Previewed, Report, Reported, ToolVersion, Unread, Unsent, Want,
};
use serde_json::Value;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// How the runner keeps the repository a launch names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keeping {
    /// The configuration names it, and its clone is made in this
    /// container.
    Ready(PathBuf),
    /// The configuration names it, and its clone is not yet made.
    NotYet,
    /// The configuration does not name it, and no clone of it is made.
    NotNamed,
    /// The configuration does not name it, and its clone stands in this
    /// container, dropped: the keeper fetches it no more, and keeps it only
    /// while something holds it (core.5).
    Dropped(PathBuf),
}

/// The repositories the runner keeps, as a control pass reads them.
pub trait Clones {
    /// How the runner keeps the repository at `address`, `owner/name`.
    fn keeping(&self, address: &str) -> Keeping;
    /// Each address the configuration names whose clone is made, in the
    /// configuration's order and as it writes each.
    fn made(&self) -> Vec<String>;
    /// Each address whose clone stands while the configuration does not
    /// name it, as its clone is found.
    fn dropped(&self) -> Vec<String>;
    /// Each drive of the configuration, by its name, with each address it
    /// names whose clone is made, in the configuration's order and as it
    /// writes each, at its clone's container. A drive none of whose
    /// clones is made is given with none, and an address no drive names
    /// is given in none (design Decision 2).
    fn drives(&self) -> BTreeMap<String, Vec<KeptRepository>>;
}

/// The runner's clones against its configuration: `Clones` on a host.
#[derive(Clone, Debug)]
pub struct Configured {
    named: Vec<Address>,
    /// Each drive of the configuration, by its name, with its addresses
    /// in the file's order.
    drives: Vec<(String, Vec<Address>)>,
    clones: Vec<(Address, PathBuf)>,
}

impl Configured {
    /// `clones`, as `clone::find` gives them, against `config`.
    pub fn new(config: &Config, clones: Vec<clone::Found>) -> Configured {
        let drives: Vec<(String, Vec<Address>)> = config
            .drives()
            .map(|drive| (drive.name.clone(), drive.repositories.clone()))
            .collect();
        Configured {
            named: drives
                .iter()
                .flat_map(|(_, repositories)| repositories.iter().cloned())
                .collect(),
            drives,
            clones: clones
                .into_iter()
                .map(|found| (found.address, found.container))
                .collect(),
        }
    }

    fn container(&self, address: &Address) -> Option<&PathBuf> {
        self.clones
            .iter()
            .find(|(made, _)| made == address)
            .map(|(_, container)| container)
    }
}

impl Clones for Configured {
    fn keeping(&self, address: &str) -> Keeping {
        let Ok(address) = Address::parse(address) else {
            return Keeping::NotNamed;
        };
        if !self.named.contains(&address) {
            return match self.container(&address) {
                Some(container) => Keeping::Dropped(container.clone()),
                None => Keeping::NotNamed,
            };
        }
        match self.container(&address) {
            Some(container) => Keeping::Ready(container.clone()),
            None => Keeping::NotYet,
        }
    }

    fn made(&self) -> Vec<String> {
        self.named
            .iter()
            .filter(|address| self.container(address).is_some())
            .map(Address::to_string)
            .collect()
    }

    fn dropped(&self) -> Vec<String> {
        self.clones
            .iter()
            .filter(|(address, _)| !self.named.contains(address))
            .map(|(address, _)| address.to_string())
            .collect()
    }

    fn drives(&self) -> BTreeMap<String, Vec<KeptRepository>> {
        self.drives
            .iter()
            .map(|(name, repositories)| {
                let kept = repositories
                    .iter()
                    .filter_map(|address| {
                        self.container(address).map(|container| KeptRepository {
                            address: address.to_string(),
                            container_dir: container.to_string_lossy().into_owned(),
                        })
                    })
                    .collect();
                (name.clone(), kept)
            })
            .collect()
    }
}

/// What a control pass of one part runs over.
pub struct Carrier<'a> {
    /// The part's label, which each of the pass's own lines names.
    pub label: &'a str,
    pub flume: &'a dyn Flume,
    pub sluice: &'a dyn Sluice,
    /// git, for an inbox read from its clone.
    pub git: &'a Git,
    pub clones: &'a dyn Clones,
    pub tools: &'a dyn Tools,
    /// Told of each landing the pass reads, so its repository is kept at
    /// once.
    pub notices: &'a Notices,
    /// The runner's control directory, `<data>/control`: the folder its
    /// briefs pass through.
    pub control_dir: &'a Path,
    /// The one report file, of which the pass reads and saves its part's
    /// share alone (design Decision 5).
    pub report: &'a ReportFile,
    /// Each clone's lock, which a launch into the clone holds shared
    /// (design Decision 4).
    pub locks: &'a Locks,
    /// The terminals relaying, kept across passes: the pass opens each
    /// terminal pulled, and closes those on a launch it lets go.
    pub relays: &'a Relays,
    /// Whether the last pull was read, kept across passes.
    pub reach: &'a Reach,
    /// What was last said of Flume's version, kept across passes.
    pub least: &'a Least,
    /// Whether the next control pass is due, by the control timer or
    /// because the control plane woke the runner. Once it is, the pass
    /// carries out no further act on a place, after its first, and
    /// leaves the rest to that pass, whose launches go ahead of them
    /// (core.50).
    pub due: &'a dyn Fn() -> bool,
}

/// Whether the control plane's last pull was read, kept across control
/// passes, so its reach is said once each way: when a pull first cannot
/// be read, with the pull's words, and when one is next read, and at no
/// pass between (design Decision 13; core.23). The first pull is said
/// unread as one after a pull that was read.
#[derive(Debug, Default)]
pub struct Reach {
    unread: Cell<bool>,
}

impl Reach {
    /// Takes a pull of the part `label` that could not be read, with the
    /// pull's `words`.
    fn unread(&self, label: &str, words: &str, say: &mut dyn FnMut(String)) {
        if !self.unread.replace(true) {
            say(format!(
                "boomdrive: {label}: the control plane could not be read: {words}"
            ));
        }
    }

    /// Takes a pull of the part `label` that was read.
    fn read(&self, label: &str, say: &mut dyn FnMut(String)) {
        if self.unread.replace(false) {
            say(format!(
                "boomdrive: {label}: the control plane is read again"
            ));
        }
    }
}

/// What the runner last said of Flume's version, kept across control
/// passes beside its reach, so a Flume older than the least the runner
/// launches with is said once while it lasts: at the first pass that reads
/// it so, and again only once what Flume says changes (design Decision 5;
/// core.38).
#[derive(Debug, Default)]
pub struct Least {
    said: RefCell<Option<String>>,
}

impl Least {
    /// Takes Flume's `version` as the part `label`'s hello read it, and
    /// gives why no launch is made, where Flume printed a version older
    /// than [`flume::LEAST`] or none; else none, where it runs with the
    /// least, or is missing, whose launch is tried and named as it fails.
    fn read(
        &self,
        label: &str,
        version: Option<&ToolVersion>,
        say: &mut dyn FnMut(String),
    ) -> Option<String> {
        let Some(ToolVersion::Printed(printed)) = version else {
            return None;
        };
        let Err(words) = flume::runs_with(printed) else {
            self.said.replace(None);
            return None;
        };
        if self.said.borrow().as_deref() != Some(words.as_str()) {
            say(format!("boomdrive: {label}: {words}"));
            self.said.replace(Some(words.clone()));
        }
        Some(words)
    }
}

/// What a control pass leaves for the loop.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Passed {
    /// The earliest expiry of a lease the report holds not yet lapsed at
    /// the pass's end, where one has an expiry.
    pub expires: Option<At>,
    /// Whether the pass pulled and the control plane took its report,
    /// which is what `boomdrive run --once` asks of a control pass. A tool
    /// that could not run is named, and does not count against it.
    pub reported: bool,
}

impl Passed {
    /// When the control pass after this one, which began at `began` and
    /// ended at `ended`, is due, each `poll_interval`, by the clock as it
    /// reads now: [`next_due`], with the time left of the earliest lease
    /// not yet lapsed.
    pub fn next_due(&self, began: Instant, ended: Instant, poll_interval: Duration) -> Instant {
        let left = self.expires.and_then(|expires| left(expires, At::now()));
        next_due(began, ended, poll_interval, left, self.reported)
    }
}

impl Carrier<'_> {
    /// One of the pass's own lines, naming the part's label.
    fn line(&self, words: &str) -> String {
        format!("boomdrive: {}: {words}", self.label)
    }

    /// One control pass of the part whose control plane is `plane`, each
    /// line said to `say` as it is said. Its report is the part's share of
    /// the report file, where the file holds it, else what the pull gives
    /// back (design Decision 5).
    pub fn pass(&self, plane: &mut dyn ControlPlane, say: &mut dyn FnMut(String)) -> Passed {
        // The pass's clock, read once, by which the work under each lease
        // is judged (core.24).
        let now = At::now();
        let kept = match self.report.share(self.label, &|id| plane.owns(id)) {
            Ok(kept) => kept,
            Err(why) => {
                say(self.line(&format!("{why}: nothing carried this pass")));
                return Passed {
                    expires: None,
                    reported: false,
                };
            }
        };
        let saved = kept.clone().unwrap_or_default();
        let status = self.flume.status();
        let listing = listing_of(&status, self.label, say);
        let mut hello = hello::hello(
            self.tools,
            self.clones.made(),
            hello::capacity(&status),
            &saved,
        );
        hello.draining = hello::draining(self.clones.dropped(), &saved);
        hello.drives = self.clones.drives();
        let pulled = match plane.pull(&hello, &saved) {
            Ok(pulled) => pulled,
            Err(Unread(words)) => {
                self.reach.unread(self.label, &words, say);
                return Passed {
                    expires: expires(&saved, At::now()),
                    reported: false,
                };
            }
        };
        self.reach.read(self.label, say);
        let below_least = self
            .least
            .read(self.label, hello.tool_versions.get("flume"), say);
        // A share the report file does not hold, the file lost with the
        // data directory among it, is taken from the control plane, which
        // holds every answer the runner reported.
        let report = kept.or(pulled.report).unwrap_or_default();
        let mut work = Work {
            carrier: self,
            wants: &pulled.wants,
            saved,
            report,
            room: Room::read(listing.as_ref()),
            listing,
            below_least,
            ran: false,
            now,
            round: Round::Wants,
            acts_run: 0,
            left: BTreeSet::new(),
            waiting: Vec::new(),
            previews_run: 0,
            previews_left: BTreeSet::new(),
        };
        'carrying: for round in [Round::Wants, Round::OnPlaces] {
            work.round = round;
            if round == Round::OnPlaces {
                for (offer, container, address) in std::mem::take(&mut work.waiting) {
                    work.launch_once_kept(&offer, &container, &address, say);
                    if !work.save(&*plane, say) {
                        say(self.line("nothing more is carried out this pass"));
                        break 'carrying;
                    }
                }
            }
            for want in &pulled.wants {
                work.want(want, say);
                if !work.save(&*plane, say) {
                    say(self.line("nothing more is carried out this pass"));
                    break 'carrying;
                }
            }
        }
        match work.left.len() {
            0 => {}
            1 => say(self.line("the next control pass is due, so 1 act on a place waits for it")),
            n => say(self.line(&format!(
                "the next control pass is due, so {n} acts on places wait for it"
            ))),
        }
        match work.previews_left.len() {
            0 => {}
            1 => say(
                "boomdrive: the next control pass is due, so 1 preview waits for it".to_string(),
            ),
            n => say(format!(
                "boomdrive: the next control pass is due, so {n} previews wait for it"
            )),
        }
        // The commands the wants ran may have taken minutes, while Flume
        // moved places on, tidied some and let their records go: the
        // statuses are carried against its listing as it stands now
        // (record 0027).
        if work.ran {
            work.listing = listing_of(&self.flume.status(), self.label, say);
        }
        work.carry_statuses(say);
        work.carry_entries(say);
        work.carry_previews(say);
        work.save(&*plane, say);
        for terminal in pulled.terminals {
            self.relays.open(terminal, &work.report);
        }
        let sent = work.send(plane, say);
        // An end Flume owes the runner is confirmed only once its control
        // plane has taken the status that gives it, in a pass whose share
        // the file holds after the answer, and never below the least Flume
        // (core.25, core.38, core.54; record 0030 as amended on 2026-11-09).
        if let Some((answer, true)) = &sent {
            if work.below_least.is_none() {
                work.confirm(answer, say);
            }
        }
        Passed {
            expires: expires(&work.report, At::now()),
            reported: sent.is_some(),
        }
    }
}

/// One pass's work: the report as it stands, and what was read of the
/// host.
struct Work<'p> {
    carrier: &'p Carrier<'p>,
    /// What the pull gave, in the control plane's order.
    wants: &'p [Want],
    /// The part's share as the file holds it.
    saved: Report,
    /// The part's share as it stands.
    report: Report,
    /// Flume's listing, `flume --json status`, where Flume gave it: the
    /// pass's own, read before the pull, and read again before the
    /// statuses are carried where answering the wants ran a launch or an
    /// act of Flume's.
    listing: Option<Value>,
    room: Room,
    /// Why no offer is launched this pass, where Flume is older than the
    /// least the runner launches with (core.38).
    below_least: Option<String>,
    /// Whether answering the wants ran a tool's command: a launch or an
    /// act of Flume's. A command runs none (record 0027).
    ran: bool,
    /// The pass's clock, read once at its start.
    now: At,
    round: Round,
    /// How many acts the pass has run Flume's command for on their places.
    acts_run: usize,
    /// The acts on places left for the next pass, which is due.
    left: BTreeSet<Id>,
    /// Each offer whose clone the keeper held when its turn came, with
    /// its clone's container and address, launched once the keep is done,
    /// its room counted at its turn (design Decision 4).
    waiting: Vec<(Offer, PathBuf, Address)>,
    /// How many previews the pass has run sluice's command for, counted
    /// apart from the acts' (design Decision 7).
    previews_run: usize,
    /// The previews left for the next pass, which is due.
    previews_left: BTreeSet<Id>,
}

/// The pass's two rounds over the pull's wants, each in the control
/// plane's order, so that no launch waits behind an act on a place
/// (record 0002; core.50). An act on a place moves no room and lifts no
/// hold within the pass, which read both from its listing before the pull,
/// so taking it second changes no answer. The second round ends its acts on
/// places once the next pass is due, after the first, so no launch the next
/// pull offers waits behind them either. An act Flume refuses because its
/// place no longer stands costs the pass the reading of Flume's refusal
/// alone (record 0010; core.51). A launch whose clone the keeper holds at
/// its turn is made before the second round, once that keep is done, its
/// room counted at its turn, so no other launch waits on another
/// repository's keep (record 0026; design Decision 4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Round {
    /// Every want: each offer, inbox and command, and each act and preview
    /// answered from the report or refused, running no command. An act
    /// that runs Flume's command on its launch's place, and a preview that
    /// runs sluice's, is left for the second round.
    Wants,
    /// Each act left by the first, as Flume's command on its place, each
    /// act whose launch the first round took, and each preview left by the
    /// first, as sluice's command (core.55).
    OnPlaces,
}

impl Work<'_> {
    fn want(&mut self, want: &Want, say: &mut dyn FnMut(String)) {
        if self.round == Round::OnPlaces {
            match want {
                Want::Act(act) => self.act(act, say),
                Want::Preview(preview) => self.preview_command(preview, say),
                Want::Offer(_) | Want::Inbox(_) | Want::Command(_) => {}
            }
            return;
        }
        match want {
            Want::Offer(offer) => self.offer(offer, say),
            Want::Act(act) => self.act(act, say),
            Want::Inbox(inbox) => self.inbox(inbox, say),
            Want::Command(command) => self.command(command, say),
            Want::Preview(preview) => self.preview(preview, say),
        }
    }

    /// Takes, leaves or refuses `offer` (design Decision 7). An offer
    /// whose lease has lapsed is left unanswered, and one already answered
    /// keeps its answer, until a pull gives it under a live lease (design
    /// Decision 13; core.24). A launch is taken only for a repository the
    /// runner keeps (record 0021; core.5): an offer for one it neither
    /// keeps nor drains is refused before any mark is read, and one in a
    /// repository it drains is answered `launched` only where Flume's
    /// listing holds its place under its mark, so a launch made just before
    /// a stop is not made twice (core.10), and refused otherwise.
    fn offer(&mut self, offer: &Offer, say: &mut dyn FnMut(String)) {
        let id = &offer.id;
        let repository = &offer.launch.repository;
        if let Some(launched) = self.report.launches.get_mut(id) {
            // The lease is the one the control plane gives now.
            launched.lease = offer.lease;
            // A launch a report of before holds with no repository takes
            // the offer's.
            if launched.repository.is_none() {
                launched.repository = Some(repository.clone());
            }
            if launched.answer.outcome != Outcome::Held {
                return;
            }
        }
        if lapsed(offer.lease, self.now) {
            return;
        }
        // Without Flume's listing no mark can be read, so nothing is
        // launched.
        let Some(listing) = &self.listing else {
            return;
        };
        let keeping = self.carrier.clones.keeping(repository);
        let drained = matches!(keeping, Keeping::Dropped(_)) && self.drains(repository);
        let refuse = |work: &mut Work, say: &mut dyn FnMut(String)| {
            let words = format!("this host's configuration does not keep {repository}");
            say(format!("{id}: refused: {words}"));
            work.answer(offer, Outcome::Refused, None, Some(words));
        };
        if keeping == Keeping::NotNamed || (matches!(keeping, Keeping::Dropped(_)) && !drained) {
            return refuse(self, say);
        }
        let mark = mark(id);
        if let Some(place) = facts::marked(listing, &mark).and_then(place_of) {
            say(format!(
                "{id}: launched at {place}, as its mark {mark} shows"
            ));
            self.answer(offer, Outcome::Launched, Some(place), None);
            return;
        }
        // Nothing more is launched in a repository the runner drains.
        let container = match keeping {
            Keeping::Ready(container) => container,
            Keeping::NotYet => return,
            Keeping::NotNamed | Keeping::Dropped(_) => return refuse(self, say),
        };
        if let Some(words) = self.waits(&offer.launch.after) {
            if self.answer(offer, Outcome::Held, None, Some(words.clone())) {
                say(format!("{id}: held: {words}"));
            }
            return;
        }
        let stream = offer.launch.stream.as_deref();
        if let Some(words) = self.room.lacks(stream) {
            // A new offer with no room is left for the control plane to
            // give where it will; one already held stays held.
            if self.report.launches.contains_key(id)
                && self.answer(offer, Outcome::Held, None, Some(words.clone()))
            {
                say(format!("{id}: held: {words}"));
            }
            return;
        }
        // Below the least Flume nothing is launched: a new offer is left
        // unanswered, and one held stays held (core.38; record 0030).
        if self.below_least.is_some() {
            return;
        }
        let Ok(address) = Address::parse(repository) else {
            return self.launch(offer, &container, say);
        };
        match self.carrier.locks.try_launch(&address) {
            Some(held) => {
                self.launch(offer, &container, say);
                drop(held);
            }
            None => {
                // Its room is counted now, at its turn, so the launch is
                // the one the control plane's order makes.
                self.room.take(stream);
                self.waiting.push((offer.clone(), container, address));
            }
        }
    }

    /// Launches `offer`, its room already counted, in the clone at
    /// `container` once the keeper's keep of `address` is done, holding
    /// the clone's lock shared until `flume launch` ends.
    fn launch_once_kept(
        &mut self,
        offer: &Offer,
        container: &Path,
        address: &Address,
        say: &mut dyn FnMut(String),
    ) {
        let held = self.carrier.locks.launch(address);
        if container.is_dir() {
            self.made(offer, container, say);
        }
        drop(held);
    }

    /// Launches `offer` in the clone at `container`, holding its lock,
    /// where the clone is still there: one the keeper let go meanwhile is
    /// one not yet made, and the offer is left unanswered (design
    /// Decision 4; core.14).
    fn launch(&mut self, offer: &Offer, container: &Path, say: &mut dyn FnMut(String)) {
        if !container.is_dir() {
            return;
        }
        self.room.take(offer.launch.stream.as_deref());
        self.made(offer, container, say);
    }

    /// Runs `flume launch` for `offer` in the clone at `container`, its
    /// room counted (design Decision 9).
    fn made(&mut self, offer: &Offer, container: &Path, say: &mut dyn FnMut(String)) {
        let id = &offer.id;
        let args = launch_args(offer, container, self.priority(offer));
        let briefs = self.carrier.control_dir.join("briefs");
        self.ran = true;
        match self.carrier.flume.launch(&args, &briefs) {
            Ok(Reply::Given(text)) => {
                let answer = serde_json::from_str::<Value>(&text).ok();
                let place = answer
                    .as_ref()
                    .and_then(facts::launched)
                    .and_then(facts::location)
                    .map(str::to_string);
                match &place {
                    Some(place) => say(format!("{id}: launched at {place}")),
                    None => say(format!(
                        "{id}: launched, though Flume's answer names no place"
                    )),
                }
                self.answer(offer, Outcome::Launched, place, None);
            }
            Ok(Reply::Refused(words)) => {
                say(format!("{id}: refused by Flume: {words}"));
                self.answer(offer, Outcome::Refused, None, Some(words));
            }
            Err(Unrun(words)) => {
                say(format!("{id}: not launched: {words}"));
            }
        }
    }

    /// What `after` still waits on, each launch with where it stands, or
    /// none where every one's place has read landed. The runner reads only
    /// whether Flume says a place landed (core.1).
    fn waits(&self, after: &[Id]) -> Option<String> {
        let waiting: Vec<String> = after
            .iter()
            .filter_map(|launch| match self.standing(launch) {
                Standing::Landed => None,
                standing => Some(format!("{launch} ({})", standing.words())),
            })
            .collect();
        (!waiting.is_empty()).then(|| format!("waits on {}", waiting.join(", ")))
    }

    /// Where launch `id` stands, as the report holds it and Flume's status
    /// reads its place: the pass's listing where it holds the place, else
    /// the status last carried.
    fn standing(&self, id: &Id) -> Standing {
        let Some(launched) = self.report.launches.get(id) else {
            return Standing::Unlaunched;
        };
        let answer = &launched.answer;
        match answer.outcome {
            Outcome::Held => Standing::Unlaunched,
            Outcome::Refused => Standing::Refused(answer.words.clone()),
            Outcome::Done => Standing::Dropped,
            Outcome::Launched => match &answer.place_dir {
                Some(place) if self.landed(launched, place) => Standing::Landed,
                place => Standing::Launched(place.clone()),
            },
        }
    }

    /// Whether the place at `place` reads landed in the pass's listing,
    /// where it holds the place, else in the status `launched` carries.
    fn landed(&self, launched: &Launched, place: &str) -> bool {
        let listed = self
            .listing
            .as_ref()
            .and_then(|listing| facts::entry(listing, place).cloned());
        let entry = listed.or_else(|| carried_entry(launched, place));
        reads_landed(entry.as_ref())
    }

    /// The priority `offer` is launched with: the newest an act on it set
    /// while it was not yet launched, in the control plane's order, else
    /// its own (crew's teams-cross Decision 6).
    fn priority(&self, offer: &Offer) -> Option<i64> {
        let mut set = self.wants.iter().filter_map(|want| match want {
            Want::Act(act) if act.launch == offer.id => match act.what {
                ActSort::Priority(priority) if self.done(&act.id) => Some(priority),
                _ => None,
            },
            _ => None,
        });
        set.next_back().or(offer.launch.priority)
    }

    /// Whether the report holds act `id` done.
    fn done(&self, id: &Id) -> bool {
        self.report
            .acts
            .get(id)
            .is_some_and(|acted| acted.outcome == Outcome::Done)
    }

    /// Whether the runner drains `repository`: a launch of the report
    /// holds it, at a place that has not ended (record 0021). Only a
    /// repository whose clone stands dropped is drained.
    fn drains(&self, repository: &str) -> bool {
        Address::parse(repository)
            .is_ok_and(|address| !report::holding(&self.report, &address).is_empty())
    }

    /// Whether the pull offers launch `id`.
    fn offered(&self, id: &Id) -> bool {
        self.wants
            .iter()
            .any(|want| matches!(want, Want::Offer(offer) if offer.id == *id))
    }

    /// Carries out or answers `act` on its launch (design Decision 9). An
    /// act the report holds is left as it stands, and one whose launch's
    /// lease in the report has lapsed is left untaken (design Decision 13;
    /// core.24). A confirm as the runner, or of no key, is refused at once,
    /// running nothing (core.56). One on its launch's place is carried out
    /// in the pass's second round alone, after every launch the pull offers
    /// (core.50).
    fn act(&mut self, act: &Act, say: &mut dyn FnMut(String)) {
        if self.report.acts.contains_key(&act.id) {
            return;
        }
        let Some(launched) = self.report.launches.get(&act.launch) else {
            // A launch the pull offers and the runner has not yet taken is
            // the control plane's to give: its act waits with it.
            if !self.offered(&act.launch) {
                let words = format!("{} is no launch of this runner's", act.launch);
                self.acted(act, Outcome::Refused, Some(words), None, say);
            }
            return;
        };
        if lapsed(launched.lease, self.now) {
            return;
        }
        if let Some(words) = refused_confirm(&act.what) {
            self.acted(act, Outcome::Refused, Some(words), None, say);
            return;
        }
        if launched.answer.outcome != Outcome::Launched {
            self.act_unlaunched(act, say);
            return;
        }
        match launched.answer.place_dir.clone() {
            Some(place) if self.round == Round::OnPlaces => self.act_on(act, &place, say),
            Some(_) => {}
            None => {
                let words = format!("{} was launched at no place Flume named", act.launch);
                self.acted(act, Outcome::Refused, Some(words), None, say);
            }
        }
    }

    /// Carries out `act` on the place at `place`, its answer Flume's. A
    /// place the pass's listing does not hold is first asked of Flume's
    /// status, which settles nothing: where Flume refuses it, that refusal
    /// is the act's answer, as Flume gave it, and the act's command is not
    /// run (core.51). A confirm naming a key that is the delivery of no end
    /// of the place, as the listing or that status gives it, is refused in
    /// the runner's words, naming the key, and runs nothing (core.56).
    /// Otherwise the act is Flume's own command on the place, never forced;
    /// once the next pass is due, the pass runs none but its first, and
    /// leaves the act for that pass (core.50). Where Flume could not be
    /// run, nothing is kept, and the next pass carries it out.
    fn act_on(&mut self, act: &Act, place: &str, say: &mut dyn FnMut(String)) {
        let unrun = |words: String, say: &mut dyn FnMut(String)| {
            let what = sort(&act.what);
            say(format!(
                "{}: {what} of {} at {place}: not carried out: {words}",
                act.id, act.launch
            ));
        };
        let status = if self.listed(place) {
            None
        } else {
            match self.carrier.flume.status_of(None, place) {
                Ok(Reply::Given(status)) => Some(status),
                Ok(Reply::Refused(words)) => {
                    self.acted(act, Outcome::Refused, Some(words), Some(place), say);
                    return;
                }
                Err(Unrun(words)) => return unrun(words, say),
            }
        };
        if let ActSort::Confirm { deliveries, .. } = &act.what {
            if let Some(key) = self.unheld(deliveries, place, status.as_deref()) {
                let words = format!("{key} is the delivery of no end of {place} as Flume gives it");
                self.acted(act, Outcome::Refused, Some(words), None, say);
                return;
            }
        }
        if self.acts_run > 0 && (self.carrier.due)() {
            self.left.insert(act.id.clone());
            return;
        }
        self.acts_run += 1;
        self.ran = true;
        match self.carrier.flume.act(place, &flume_act(&act.what)) {
            Ok(Reply::Given(_)) => self.acted(act, Outcome::Done, None, Some(place), say),
            Ok(Reply::Refused(words)) => {
                self.acted(act, Outcome::Refused, Some(words), Some(place), say)
            }
            Err(Unrun(words)) => unrun(words, say),
        }
    }

    /// The first of `keys` that is the delivery of no end of the place at
    /// `place`, as the pass's listing gives its entry, else `status`,
    /// Flume's answer to the place's status, where the listing does not
    /// hold it (core.56). The runner reads Flume's own field, and judges
    /// nothing of the end: whom it is owed and whether it is owed are
    /// Flume's to answer (Flume's 43).
    fn unheld<'k>(&self, keys: &'k [String], place: &str, status: Option<&str>) -> Option<&'k str> {
        let answer = status.and_then(|status| serde_json::from_str::<Value>(status).ok());
        let entry = match &answer {
            Some(answer) => facts::entry(answer, place),
            None => self
                .listing
                .as_ref()
                .and_then(|listing| facts::entry(listing, place)),
        };
        let held = entry.map(facts::deliveries).unwrap_or_default();
        keys.iter()
            .map(String::as_str)
            .find(|key| !held.contains(key))
    }

    /// Whether the pass's listing holds the place at `place`, standing or
    /// as the record Flume keeps of it tidied.
    fn listed(&self, place: &str) -> bool {
        self.listing
            .as_ref()
            .is_some_and(|listing| facts::entry(listing, place).is_some())
    }

    /// Answers `act` on a launch the runner holds not yet launched, from
    /// the report, running no command. A priority is done, and the launch
    /// carries it when made; a drop is done, and the launch is never made;
    /// a retry, a land, a confirm or a done is refused, naming where the
    /// launch stands, and a land so refused changes nothing of the launch
    /// when it is made (core.53, core.56).
    fn act_unlaunched(&mut self, act: &Act, say: &mut dyn FnMut(String)) {
        let launched = &self.report.launches[&act.launch];
        let (lease, answer) = (launched.lease, launched.answer.clone());
        match act.what {
            ActSort::Priority(_) => self.acted(act, Outcome::Done, None, None, say),
            ActSort::Drop => {
                self.acted(act, Outcome::Done, None, None, say);
                if answer.outcome == Outcome::Held {
                    let words = format!("dropped by {} before it was launched", act.id);
                    say(format!("{}: done: {words}", act.launch));
                    let dropped = Answer {
                        outcome: Outcome::Done,
                        place_dir: None,
                        words: Some(words),
                        at: At::now(),
                    };
                    self.report.answer(&act.launch, lease, dropped);
                }
            }
            ActSort::Retry { .. } | ActSort::Land | ActSort::Confirm { .. } | ActSort::Done => {
                let outcome = match answer.outcome {
                    Outcome::Held => "held",
                    Outcome::Done => "done",
                    Outcome::Refused | Outcome::Launched => "refused",
                };
                let why = answer
                    .words
                    .map_or(String::new(), |words| format!(": {words}"));
                let words = format!("{} is {outcome}, not launched{why}", act.launch);
                self.acted(act, Outcome::Refused, Some(words), None, say);
            }
        }
    }

    /// Records `act` answered `outcome`, with `words`, and says so: as
    /// Flume answered it on the place at `place`, where it ran there, else
    /// as the runner answered it from the report.
    fn acted(
        &mut self,
        act: &Act,
        outcome: Outcome,
        words: Option<String>,
        place: Option<&str>,
        say: &mut dyn FnMut(String),
    ) {
        let what = sort(&act.what);
        let at = place.map_or(String::new(), |place| format!(" at {place}"));
        let by = if place.is_some() { " by Flume" } else { "" };
        match (&outcome, &words) {
            (Outcome::Refused, Some(words)) => say(format!(
                "{}: {what} of {}{at}: refused{by}: {words}",
                act.id, act.launch
            )),
            _ => say(format!("{}: {what} of {}{at}: done", act.id, act.launch)),
        }
        let acted = Acted {
            launch: act.launch.clone(),
            lease: act.lease,
            outcome,
            words,
            at: At::now(),
        };
        self.report.acts.insert(act.id.clone(), acted);
    }

    /// Gives `offer` the answer `outcome`, at `place_dir`, with `words`.
    /// The first answer records the repository the offer names, which no
    /// later answer changes.
    fn answer(
        &mut self,
        offer: &Offer,
        outcome: Outcome,
        place_dir: Option<String>,
        words: Option<String>,
    ) -> bool {
        let answer = Answer {
            outcome,
            place_dir,
            words,
            at: At::now(),
        };
        let changed = self.report.answer(&offer.id, offer.lease, answer);
        let recorded = match self.report.launches.get_mut(&offer.id) {
            Some(launched) if launched.repository.is_none() => {
                launched.repository = Some(offer.launch.repository.clone());
                true
            }
            _ => false,
        };
        changed || recorded
    }

    /// Answers `command` (core.16, core.52). A command the report holds
    /// is left as it stands, under the lease the control plane gives now
    /// (core.10). One whose lease has lapsed is left untaken (design
    /// Decision 13; core.24). Every other is refused, naming its tool, a
    /// branch command's being sluice: this binary carries no tool's
    /// command, so it reads neither the command's repository nor its
    /// clone, and runs nothing.
    fn command(&mut self, command: &Command, say: &mut dyn FnMut(String)) {
        if let Some(commanded) = self.report.commands.get_mut(&command.id) {
            commanded.lease = command.lease;
            return;
        }
        if lapsed(command.lease, self.now) {
            return;
        }
        let tool = match &command.what {
            CommandSort::Branch { .. } => "sluice",
            CommandSort::Other { tool, .. } => tool,
        };
        let words = format!("this binary does not carry {tool}'s commands");
        say(format!("{}: refused: {words}", command.id));
        self.commanded(command, Outcome::Refused, Some(words));
    }

    /// Records `command` answered `outcome`, with `words`: no tool ran
    /// for it, so it has no text and no origin commit.
    fn commanded(&mut self, command: &Command, outcome: Outcome, words: Option<String>) {
        let commanded = Commanded {
            lease: command.lease,
            outcome,
            text: None,
            words,
            origin_commit: None,
            at: At::now(),
        };
        self.report.commands.insert(command.id.clone(), commanded);
    }

    /// Answers `preview` in the pass's first round, running no command
    /// (core.55). A preview the report holds `done` or `refused` keeps its
    /// answer, under the lease the control plane gives now (core.10), and
    /// one whose lease has lapsed is left untaken, one held not asked again
    /// (core.24). A bring-up in a repository the runner neither keeps nor
    /// drains, or one it drains, is refused, since nothing new is made in
    /// one, and one whose clone is not yet made waits (core.5). A done
    /// whose repository has no clone is refused, since its checkout went
    /// with the clone. Every other is left for the second round, after
    /// every launch the pull offers (core.50).
    fn preview(&mut self, preview: &Preview, say: &mut dyn FnMut(String)) {
        if let Some(previewed) = self.report.previews.get_mut(&preview.id) {
            previewed.lease = preview.lease;
            if previewed.outcome != Outcome::Held {
                return;
            }
        }
        if lapsed(preview.lease, self.now) {
            return;
        }
        let repository = &preview.repository;
        match (preview.done, self.carrier.clones.keeping(repository)) {
            (false, Keeping::NotNamed | Keeping::Dropped(_)) => {
                let words = format!("this host's configuration does not keep {repository}");
                self.preview_refused(preview, words, say);
            }
            (true, Keeping::NotYet | Keeping::NotNamed) => {
                let words = format!("no clone of {repository} stands on this host");
                self.preview_refused(preview, words, say);
            }
            _ => {}
        }
    }

    /// Carries out `preview` in the pass's second round, after every launch
    /// the pull offers, by sluice's own command (core.50, core.55). A
    /// preview the report holds answered, or one whose lease has lapsed, is
    /// left as it stands.
    fn preview_command(&mut self, preview: &Preview, say: &mut dyn FnMut(String)) {
        let held = match self.report.previews.get(&preview.id) {
            Some(previewed) if previewed.outcome != Outcome::Held => return,
            Some(_) => true,
            None => false,
        };
        if lapsed(preview.lease, self.now) {
            return;
        }
        match preview.done {
            false => self.bring_up(preview, held, say),
            true => self.put_away(preview, say),
        }
    }

    /// Brings `preview` up by `sluice preview <branch> <revision> --json`,
    /// in its repository's clone, once (design Decisions 2 and 3). A
    /// preview held whose done the pull gives is not asked again: the done
    /// puts away what sluice holds of it. Once the next pass is due, the
    /// pass runs no preview's command but its first (design Decision 7).
    /// sluice's answer is the preview's: `done` with what is carried of
    /// it, or `refused` with sluice's words. Where sluice was interrupted
    /// at the runner's patience, waiting at the host's bound, the preview
    /// is held, said once, and asked again at the next pass. Where sluice
    /// could not be run, nothing is kept, and the next pass asks again.
    fn bring_up(&mut self, preview: &Preview, held: bool, say: &mut dyn FnMut(String)) {
        let Keeping::Ready(container) = self.carrier.clones.keeping(&preview.repository) else {
            return;
        };
        if held && self.done_given(preview) {
            return;
        }
        if !self.preview_may_run(&preview.id) {
            return;
        }
        let what = what_preview(preview);
        let asked = self
            .carrier
            .sluice
            .preview(&container, &preview.branch, &preview.revision);
        match asked {
            Ok(Asked::Replied(sluice::Reply::Given(answer))) => {
                let Some(text) = sluice::preview_carried(&answer) else {
                    let words = "sluice's answer is no preview".to_string();
                    return self.preview_refused(preview, words, say);
                };
                match sluice::text_of(&text, "state") {
                    Some(state) => say(format!("{what}: brought up, {state}")),
                    None => say(format!("{what}: brought up")),
                }
                self.previewed(preview, Outcome::Done, Some(text), None);
                // A listing that held it no more while it waited was of a
                // preview sluice no longer kept: the one brought up is read
                // anew.
                if let Some(previewed) = self.report.previews.get_mut(&preview.id) {
                    if previewed
                        .status
                        .as_ref()
                        .is_some_and(|status| !status.listed)
                    {
                        previewed.status = None;
                    }
                }
            }
            Ok(Asked::Replied(sluice::Reply::Refused(words))) => {
                say(format!("{what}: refused by sluice: {words}"));
                self.previewed(preview, Outcome::Refused, None, Some(words));
            }
            Ok(Asked::Waiting) => {
                if !held {
                    say(format!(
                        "{what}: waits at sluice's bound, asked again at the next pass"
                    ));
                }
                self.previewed(preview, Outcome::Held, None, None);
            }
            Err(sluice::Unrun(words)) => say(format!("{what}: not carried out: {words}")),
        }
    }

    /// Puts away, on `preview`'s done, what the report holds of its
    /// repository and revision asked of sluice, brought up or held, by one
    /// `sluice release <name> --json` in its repository's clone, named or
    /// dropped (design Decision 6). The name is the one sluice gave the
    /// first such preview, in its answer or else its listing, else the
    /// revision. Where the report holds none, the done is refused and
    /// sluice is not run, so a person's own preview is never put away.
    /// sluice's answer is the done's. A preview held so put away is
    /// refused, and asked of sluice no more; one brought up keeps its
    /// answer, and its listing reads it gone.
    fn put_away(&mut self, preview: &Preview, say: &mut dyn FnMut(String)) {
        let repository = &preview.repository;
        let container = match self.carrier.clones.keeping(repository) {
            Keeping::Ready(container) | Keeping::Dropped(container) => container,
            Keeping::NotYet | Keeping::NotNamed => {
                let words = format!("no clone of {repository} stands on this host");
                return self.preview_refused(preview, words, say);
            }
        };
        let asked: Vec<Id> = self
            .report
            .previews
            .iter()
            .filter(|(_, previewed)| {
                !previewed.done
                    && matches!(previewed.outcome, Outcome::Done | Outcome::Held)
                    && previewed.revision == preview.revision
                    && same_repository(&previewed.repository, repository)
            })
            .map(|(id, _)| id.clone())
            .collect();
        let Some(first) = asked.first().map(|id| &self.report.previews[id]) else {
            let words = format!(
                "no preview of {} in {repository} was brought up here at the control plane's asking",
                preview.revision
            );
            return self.preview_refused(preview, words, say);
        };
        let name = first
            .text
            .as_deref()
            .and_then(|text| sluice::text_of(text, "name"))
            .or_else(|| {
                let status = first.status.as_ref()?;
                sluice::text_of(&status.text, "name")
            })
            .unwrap_or_else(|| preview.revision.clone());
        if !self.preview_may_run(&preview.id) {
            return;
        }
        let what = what_preview(preview);
        match self.carrier.sluice.release(&container, &name) {
            Ok(sluice::Reply::Given(answer)) => {
                let Some(text) = sluice::release_carried(&answer) else {
                    let words = "sluice's answer is no release".to_string();
                    return self.preview_refused(preview, words, say);
                };
                say(format!("{what}: put away"));
                self.previewed(preview, Outcome::Done, Some(text), None);
                for id in asked {
                    let Some(previewed) = self.report.previews.get_mut(&id) else {
                        continue;
                    };
                    if previewed.outcome != Outcome::Held {
                        continue;
                    }
                    let words = "put away before sluice brought it up".to_string();
                    say(format!(
                        "{id}: preview of {} at {}: refused: {words}",
                        preview.branch, previewed.revision
                    ));
                    previewed.outcome = Outcome::Refused;
                    previewed.words = Some(words);
                    previewed.at = At::now();
                }
            }
            Ok(sluice::Reply::Refused(words)) => {
                say(format!("{what}: refused by sluice: {words}"));
                self.previewed(preview, Outcome::Refused, None, Some(words));
            }
            Err(sluice::Unrun(words)) => say(format!("{what}: not carried out: {words}")),
        }
    }

    /// Whether the pull gives, under a live lease, a done of `preview`'s
    /// repository and revision the report has not answered.
    fn done_given(&self, preview: &Preview) -> bool {
        self.wants.iter().any(|want| match want {
            Want::Preview(done) => {
                done.done
                    && done.revision == preview.revision
                    && same_repository(&done.repository, &preview.repository)
                    && !lapsed(done.lease, self.now)
                    && !self.report.previews.contains_key(&done.id)
            }
            _ => false,
        })
    }

    /// Whether the pass may run sluice's command for preview `id`: once the
    /// next pass is due, none but its first, the rest left to that pass
    /// (design Decision 7). Counts the one it may run.
    fn preview_may_run(&mut self, id: &Id) -> bool {
        if self.previews_run > 0 && (self.carrier.due)() {
            self.previews_left.insert(id.clone());
            return false;
        }
        self.previews_run += 1;
        true
    }

    /// Answers `preview` refused by the runner, with `words`, running
    /// nothing, and says so.
    fn preview_refused(&mut self, preview: &Preview, words: String, say: &mut dyn FnMut(String)) {
        say(format!("{}: refused: {words}", what_preview(preview)));
        self.previewed(preview, Outcome::Refused, None, Some(words));
    }

    /// Gives `preview` the answer `outcome`, with `text` and `words`,
    /// under the lease the control plane gives now. Its `at` moves only
    /// where the outcome, text or words do, and the status carried stands.
    /// Says whether the report changed.
    fn previewed(
        &mut self,
        preview: &Preview,
        outcome: Outcome,
        text: Option<String>,
        words: Option<String>,
    ) -> bool {
        if let Some(held) = self.report.previews.get_mut(&preview.id) {
            let renewed = held.lease != preview.lease;
            held.lease = preview.lease;
            if held.outcome == outcome && held.text == text && held.words == words {
                return renewed;
            }
            held.outcome = outcome;
            held.text = text;
            held.words = words;
            held.at = At::now();
            return true;
        }
        let previewed = Previewed {
            lease: preview.lease,
            repository: preview.repository.clone(),
            revision: preview.revision.clone(),
            done: preview.done,
            outcome,
            text,
            words,
            status: None,
            at: At::now(),
        };
        self.report.previews.insert(preview.id.clone(), previewed);
        true
    }

    /// Carries Flume's status of each launch's place (design Decision 12;
    /// core.11): `flume --json status --repository <container> <place>`'s
    /// whole answer, as the one line Flume printed. A place is followed
    /// only through its repository's clone, named or dropped, which the
    /// keeper drains while the place has not ended; where no clone stands,
    /// Flume is asked nothing, and the status last carried stands (record
    /// 0021). No old name of a repository is matched to a new one. Flume's
    /// listing as it stands once the wants are
    /// answered decides when it is read again (record 0027): only where the
    /// place's entry there, the place standing or else its tidied record
    /// `<location>@<at>`, differs, as JSON, from its entry in the answer
    /// carried. A place the listing no longer holds is
    /// asked for once a pass, and where Flume refuses, the last status
    /// carried stands, not listed. The first status carried that reads the
    /// place landed tells the keeper, where the configuration names the
    /// repository, so it is fetched at once; a dropped repository is
    /// fetched no more. Only the places the report holds are read, and
    /// only for launches the pull gives, whose repositories it names.
    fn carry_statuses(&mut self, say: &mut dyn FnMut(String)) {
        let Some(listing) = self.listing.take() else {
            return;
        };
        for offer in offers(self.wants) {
            let id = &offer.id;
            let Some(launched) = self.report.launches.get(id) else {
                continue;
            };
            let (Outcome::Launched, Some(place)) =
                (launched.answer.outcome, launched.answer.place_dir.clone())
            else {
                continue;
            };
            let (container, named) = match self.carrier.clones.keeping(&offer.launch.repository) {
                Keeping::Ready(container) => (container, true),
                Keeping::Dropped(container) => (container, false),
                Keeping::NotYet | Keeping::NotNamed => continue,
            };
            let was = carried_entry(launched, &place);
            let listed = facts::entry(&listing, &place);
            if listed.is_some() && listed == was.as_ref() {
                self.relist(id, Carrying::Status, true);
                continue;
            }
            match self.carrier.flume.status_of(Some(&container), &place) {
                Ok(Reply::Given(text)) => {
                    let line = text.strip_suffix('\n').unwrap_or(&text);
                    self.report
                        .carry(id, Carrying::Status, line, listed.is_some(), At::now());
                }
                Ok(Reply::Refused(_)) => {
                    self.relist(id, Carrying::Status, listed.is_some());
                }
                Err(Unrun(words)) => {
                    say(format!(
                        "{id}: Flume's status of {place} could not be read: {words}"
                    ));
                }
            }
            let now = carried_entry(&self.report.launches[id], &place);
            if named && !reads_landed(was.as_ref()) && reads_landed(now.as_ref()) {
                if let Ok(address) = Address::parse(&offer.launch.repository) {
                    self.carrier.notices.landed(address);
                }
            }
        }
        self.listing = Some(listing);
    }

    /// Carries sluice's entry of each launch's submission (design
    /// Decision 13; core.11): of `sluice status <id> --json`, run in the
    /// clone of the launch's repository, named or dropped, the fields the
    /// cloud's copy of an entry holds, each
    /// as sluice printed it, and no log. The entry is the one the status
    /// carried for the launch names; a launch whose status names none runs
    /// no sluice. An entry is read again at each pass until its state is
    /// final, and carried anew only when what is carried of it changes.
    /// Where sluice refuses, the entry last carried stands, not listed.
    /// Only the launches the pull gives are read, whose repositories it
    /// names.
    fn carry_entries(&mut self, say: &mut dyn FnMut(String)) {
        for offer in offers(self.wants) {
            let id = &offer.id;
            let Some(launched) = self.report.launches.get(id) else {
                continue;
            };
            let (Outcome::Launched, Some(place)) = (
                launched.answer.outcome,
                launched.answer.place_dir.as_deref(),
            ) else {
                continue;
            };
            let entry = carried_entry(launched, place);
            let Some(submission) = entry.as_ref().and_then(facts::submission_id) else {
                continue;
            };
            let held = launched.entry.as_ref().map(|carried| carried.text.as_str());
            if held.is_some_and(|text| {
                sluice::id_of(text).as_deref() == Some(submission) && sluice::is_final(text)
            }) {
                continue;
            }
            let submission = submission.to_string();
            let container = match self.carrier.clones.keeping(&offer.launch.repository) {
                Keeping::Ready(container) | Keeping::Dropped(container) => container,
                Keeping::NotYet | Keeping::NotNamed => continue,
            };
            let refused = match self.carrier.sluice.entry(&container, &submission) {
                Ok(sluice::Reply::Given(answer)) => match sluice::carried(&answer) {
                    Some(text) => {
                        self.report
                            .carry(id, Carrying::Entry, &text, true, At::now());
                        continue;
                    }
                    None => "sluice's answer is no submission".to_string(),
                },
                Ok(sluice::Reply::Refused(words)) => words,
                Err(sluice::Unrun(words)) => {
                    say(format!(
                        "{id}: sluice's entry {submission} could not be read: {words}"
                    ));
                    continue;
                }
            };
            if self.relist(id, Carrying::Entry, false) {
                say(format!(
                    "{id}: sluice's entry {submission} is refused: {refused}"
                ));
            }
        }
    }

    /// Confirms to Flume, as the runner, each end of a place it owes the
    /// runner, once the control plane's `answer` names the status that
    /// gives it taken (design Decision 3; core.54; record 0030 as amended on
    /// 2026-11-09): for each launch the pull gives, in its order, that the
    /// report, after the answer, holds answered `launched` at a place in a
    /// repository whose clone stands, named or dropped, whose carried status
    /// is listed and named taken, the delivery key of each end the status
    /// gives pending for [`flume::RECIPIENT`], in the status's order, by one
    /// `flume confirm` for the launch. Nothing of Flume's answer is carried
    /// or kept (core.11). A confirmation Flume refuses, or could not be run
    /// for, is said in one line and made again at a later pass whose report
    /// is taken, whose carried status gives the ends pending still. A
    /// launch the report no longer holds, let go or its lease refused, has
    /// nothing confirmed (core.10).
    fn confirm(&self, answer: &Reported, say: &mut dyn FnMut(String)) {
        for offer in offers(self.wants) {
            let id = &offer.id;
            if !answer.statuses_taken.contains(id) {
                continue;
            }
            let Some(launched) = self.report.launches.get(id) else {
                continue;
            };
            let (Outcome::Launched, Some(place)) = (
                launched.answer.outcome,
                launched.answer.place_dir.as_deref(),
            ) else {
                continue;
            };
            if matches!(
                self.carrier.clones.keeping(&offer.launch.repository),
                Keeping::NotYet | Keeping::NotNamed
            ) {
                continue;
            }
            if !launched
                .status
                .as_ref()
                .is_some_and(|carried| carried.listed)
            {
                continue;
            }
            let Some(entry) = carried_entry(launched, place) else {
                continue;
            };
            let keys = facts::owed(&entry, flume::RECIPIENT);
            if keys.is_empty() {
                continue;
            }
            let named = facts::location(&entry).unwrap_or(place);
            let recipient = flume::RECIPIENT;
            match self.carrier.flume.confirm(recipient, &keys) {
                Ok(Reply::Given(_)) => {}
                Ok(Reply::Refused(words)) => say(format!(
                    "{id}: Flume refused to confirm the ends of {named} as {recipient}: {words}"
                )),
                Err(Unrun(why)) => say(format!(
                    "{id}: the ends of {named} could not be confirmed: {why}"
                )),
            }
        }
    }

    /// Carries sluice's listing of each preview the runner holds up (design
    /// Decision 5; core.11, core.55): of `sluice previews --json`, read once
    /// a pass in the clone, named or dropped, of each repository of a
    /// preview the pull gives, not marked done, that the report holds held
    /// or brought up, and whose status is not carried as unlisted, the row
    /// whose `name` is the one sluice's answer gave, or, for a preview held,
    /// the row whose `revision` begins with the revision it names, with no
    /// output of its command. A preview the listing holds no more keeps its
    /// status, not listed, and is read no more. A listing that cannot be
    /// read leaves each status as it was, said in one line. sluice's
    /// listing releases each preview nobody can use before it lists them,
    /// which is sluice's (sluice's 101 and 123). None of it is Flume's, so
    /// it sets no `ran` (record 0027).
    fn carry_previews(&mut self, say: &mut dyn FnMut(String)) {
        let mut repositories: Vec<(&str, Vec<&Id>)> = Vec::new();
        for want in self.wants {
            let Want::Preview(preview) = want else {
                continue;
            };
            let Some(previewed) = self.report.previews.get(&preview.id) else {
                continue;
            };
            let up = matches!(previewed.outcome, Outcome::Done | Outcome::Held);
            let unlisted = previewed
                .status
                .as_ref()
                .is_some_and(|status| !status.listed);
            if preview.done || !up || unlisted {
                continue;
            }
            let repository = preview.repository.as_str();
            match repositories
                .iter_mut()
                .find(|(named, _)| same_repository(named, repository))
            {
                Some((_, ids)) => ids.push(&preview.id),
                None => repositories.push((repository, vec![&preview.id])),
            }
        }
        for (repository, ids) in repositories {
            let container = match self.carrier.clones.keeping(repository) {
                Keeping::Ready(container) | Keeping::Dropped(container) => container,
                Keeping::NotYet | Keeping::NotNamed => continue,
            };
            let listing = match self.carrier.sluice.previews(&container) {
                Ok(sluice::Reply::Given(answer)) => {
                    sluice::listed(&answer).ok_or("sluice's answer is no listing".to_string())
                }
                Ok(sluice::Reply::Refused(words)) | Err(sluice::Unrun(words)) => Err(words),
            };
            let listing = match listing {
                Ok(listing) => listing,
                Err(words) => {
                    say(format!(
                        "boomdrive: sluice's previews of {repository} could not be read: {words}"
                    ));
                    continue;
                }
            };
            for id in ids {
                let previewed = &self.report.previews[id];
                let name = previewed
                    .text
                    .as_deref()
                    .and_then(|text| sluice::text_of(text, "name"));
                let row = listing.iter().find(|row| match &name {
                    Some(name) => row.name.as_ref() == Some(name),
                    None => row
                        .revision
                        .as_deref()
                        .is_some_and(|revision| revision.starts_with(&previewed.revision)),
                });
                let row = row.map(|row| row.carried.clone());
                let Some(previewed) = self.report.previews.get_mut(id) else {
                    continue;
                };
                match (row, &mut previewed.status) {
                    (Some(text), Some(status)) if status.text == text => status.listed = true,
                    (Some(text), status) => {
                        *status = Some(boomdrive_control::Carried {
                            text,
                            at: At::now(),
                            listed: true,
                        });
                    }
                    (None, Some(status)) => status.listed = false,
                    (None, None) => {}
                }
            }
        }
    }

    /// Carries the inbox `inbox` names (design Decision 14; core.5,
    /// core.11). An inbox in a repository the configuration does not keep,
    /// one the runner drains among them, is answered with words naming it,
    /// and nothing of it read; one whose
    /// clone is not yet made waits; one on a branch origin does not hold
    /// is answered with words naming the branch. Otherwise its branch's
    /// tip in the clone is read, and, where it moved since the tip last
    /// carried, its files: where weir takes them they are carried, each
    /// brief by its digest, and where weir refuses them the tip is carried
    /// with weir's words, and the files carried before stand.
    fn inbox(&mut self, inbox: &Inbox, say: &mut dyn FnMut(String)) {
        let (id, repository, branch) = (&inbox.id, &inbox.repository, &inbox.branch);
        let container = match self.carrier.clones.keeping(repository) {
            Keeping::Ready(container) => container,
            Keeping::NotYet => return,
            Keeping::NotNamed | Keeping::Dropped(_) => {
                let words = format!("this host's configuration does not keep {repository}");
                if self.carry_inbox(id, None, BTreeMap::new(), Some(words.clone())) {
                    say(format!("{id}: not carried: {words}"));
                }
                return;
            }
        };
        let held = self.report.inboxes.get(id);
        let tip = held.and_then(|carried| carried.tip.clone());
        let files = held
            .map(|carried| carried.files.clone())
            .unwrap_or_default();
        let bare = clone::bare(&container);
        match inbox::read(self.carrier.git, &bare, branch, tip.as_deref()) {
            Ok(inbox::Read::Unmoved) => {}
            Ok(inbox::Read::NoBranch) => {
                let words = format!("origin holds no branch {branch} of {repository}");
                if self.carry_inbox(id, None, BTreeMap::new(), Some(words.clone())) {
                    say(format!("{id}: not carried: {words}"));
                }
            }
            Ok(inbox::Read::Taken { tip, files }) => {
                let line = format!("{id}: the inbox on {branch} of {repository} carried at {tip}");
                if self.carry_inbox(id, Some(tip), files, None) {
                    say(line);
                }
            }
            Ok(inbox::Read::Refused { tip, words }) => {
                let line = format!(
                    "{id}: the inbox on {branch} of {repository} at {tip} is refused by weir: {words}"
                );
                if self.carry_inbox(id, Some(tip), files, Some(words)) {
                    say(line);
                }
            }
            Err(words) => {
                say(format!(
                    "{id}: the inbox on {branch} of {repository} could not be read: {words}"
                ));
            }
        }
    }

    /// Carries inbox `id` as read at `tip`, holding `files`, with `words`,
    /// its `at` moving only where one of them changes, and says whether
    /// the report changed.
    fn carry_inbox(
        &mut self,
        id: &Id,
        tip: Option<String>,
        files: BTreeMap<String, InboxFile>,
        words: Option<String>,
    ) -> bool {
        let held = self.report.inboxes.get(id);
        if held.is_some_and(|held| held.tip == tip && held.files == files && held.words == words) {
            return false;
        }
        let carried = InboxCarried {
            tip,
            files,
            words,
            at: At::now(),
        };
        self.report.inboxes.insert(id.clone(), carried);
        true
    }

    /// Marks the status or entry carried for launch `id` as its tool's
    /// last listing holds it or not, its text and time as they stand, and
    /// says whether the report changed.
    fn relist(&mut self, id: &Id, what: Carrying, listed: bool) -> bool {
        let launched = self.report.launches.get(id);
        let carried = launched.and_then(|launched| match what {
            Carrying::Status => launched.status.as_ref(),
            Carrying::Entry => launched.entry.as_ref(),
        });
        match carried.map(|carried| carried.text.clone()) {
            Some(text) => self.report.carry(id, what, &text, listed, At::now()),
            None => false,
        }
    }

    /// Saves the part's share, whose entries `plane` owns, where it
    /// changed since the file was last written, every other part's share
    /// kept as it stands, and says whether the file holds it.
    fn save(&mut self, plane: &dyn ControlPlane, say: &mut dyn FnMut(String)) -> bool {
        if self.report == self.saved {
            return true;
        }
        let (report, label) = (self.carrier.report, self.carrier.label);
        match report.save_share(label, &|id| plane.owns(id), &self.report) {
            Ok(()) => {
                self.saved = self.report.clone();
                true
            }
            Err(why) => {
                say(self.carrier.line(&why));
                false
            }
        }
    }

    /// Sends the part's share, which renews every lease it names, and
    /// gives the control plane's answer where it took it, with whether the
    /// file holds the share once the answer is taken; one not sent gives
    /// none, and is kept in the file, to be sent at the next pass.
    fn send(
        &mut self,
        plane: &mut dyn ControlPlane,
        say: &mut dyn FnMut(String),
    ) -> Option<(Reported, bool)> {
        self.save(&*plane, say);
        match plane.report(&self.report) {
            Ok(reported) => {
                self.reported(&reported, say);
                let saved = self.save(&*plane, say);
                Some((reported, saved))
            }
            Err(Unsent(words)) => {
                say(self.carrier.line(&format!(
                    "the report could not be sent: {words}: it is sent at the next pass"
                )));
                None
            }
        }
    }

    /// Takes the control plane's answer to a report (design Decision 11).
    /// What it carries no more leaves the report. What it refused, its
    /// lease no longer the runner's, leaves the report too, so the runner
    /// neither acts on it nor reports it again; its place is left on the
    /// host as Flume holds it, and named, and nothing of it is removed
    /// (core.10).
    fn reported(&mut self, reported: &Reported, say: &mut dyn FnMut(String)) {
        for id in &reported.let_go {
            if self.forget(id) {
                say(format!("{id}: let go by the control plane"));
            }
        }
        for (id, words) in &reported.refused {
            let place = self
                .report
                .launches
                .get(id)
                .and_then(|launched| launched.answer.place_dir.clone());
            let preview = self.report.previews.get(id).is_some_and(|p| !p.done);
            if !self.forget(id) {
                continue;
            }
            match place {
                Some(place) => say(format!(
                    "{id}: let go, its lease no longer this runner's: {words}: \
                     its place {place} is left on this host as Flume holds it"
                )),
                None if preview => say(format!(
                    "{id}: let go, its lease no longer this runner's: {words}: \
                     its preview is left on this host as sluice holds it"
                )),
                None => say(format!(
                    "{id}: let go, its lease no longer this runner's: {words}"
                )),
            }
        }
    }

    /// Drops `id` from the report, closing every terminal on it, and says
    /// whether the report held it.
    fn forget(&mut self, id: &Id) -> bool {
        self.carrier.relays.let_go(id);
        let report = &mut self.report;
        let launch = report.launches.remove(id).is_some();
        let act = report.acts.remove(id).is_some();
        let command = report.commands.remove(id).is_some();
        let inbox = report.inboxes.remove(id).is_some();
        let preview = report.previews.remove(id).is_some();
        launch || act || command || inbox || preview
    }
}

/// Flume's listing in its answer to `flume --json status`, where it gave
/// one, else none, its words said on a line naming the part `label`.
fn listing_of(
    status: &Result<Reply, Unrun>,
    label: &str,
    say: &mut dyn FnMut(String),
) -> Option<Value> {
    match status {
        Ok(Reply::Given(text)) => serde_json::from_str::<Value>(text).ok(),
        Ok(Reply::Refused(words)) | Err(Unrun(words)) => {
            say(format!(
                "boomdrive: {label}: Flume's status could not be read: {words}"
            ));
            None
        }
    }
}

/// The offers among `wants`, in the control plane's order.
fn offers(wants: &[Want]) -> impl Iterator<Item = &Offer> {
    wants.iter().filter_map(|want| match want {
        Want::Offer(offer) => Some(offer),
        _ => None,
    })
}

/// `preview` as its lines name it: `<id>: preview of <branch> at
/// <revision>`, or, for a done, `<id>: done of the preview of <branch> at
/// <revision>`.
fn what_preview(preview: &Preview) -> String {
    let (id, branch, revision) = (&preview.id, &preview.branch, &preview.revision);
    match preview.done {
        false => format!("{id}: preview of {branch} at {revision}"),
        true => format!("{id}: done of the preview of {branch} at {revision}"),
    }
}

/// Whether `one` and `other` address one repository: as addresses, which
/// compare without case, where both are, else as written.
fn same_repository(one: &str, other: &str) -> bool {
    match (Address::parse(one), Address::parse(other)) {
        (Ok(one), Ok(other)) => one == other,
        _ => one == other,
    }
}

/// The entry of the place at `place` in the status `launched` carries.
fn carried_entry(launched: &Launched, place: &str) -> Option<Value> {
    let text = &launched.status.as_ref()?.text;
    let answer = serde_json::from_str::<Value>(text).ok()?;
    facts::entry(&answer, place).cloned()
}

/// Whether a place's entry, where there is one, reads landed.
fn reads_landed(entry: Option<&Value>) -> bool {
    matches!(entry.and_then(facts::over), Some(facts::Over::Landed(_)))
}

/// Where a launch stands, for what waits on it.
enum Standing {
    /// Not answered, or held: not yet launched.
    Unlaunched,
    /// Launched, at the place Flume named where it named one, and not
    /// landed.
    Launched(Option<String>),
    Landed,
    Refused(Option<String>),
    /// Dropped before it was launched.
    Dropped,
}

impl Standing {
    /// Where it stands, in the runner's own words.
    fn words(&self) -> String {
        match self {
            Standing::Unlaunched => "not yet launched".to_string(),
            Standing::Launched(Some(place)) => format!("launched at {place}, not landed"),
            Standing::Launched(None) => "launched, not landed".to_string(),
            Standing::Landed => "landed".to_string(),
            Standing::Refused(Some(words)) => format!("refused: {words}"),
            Standing::Refused(None) => "refused".to_string(),
            Standing::Dropped => "dropped, so its place will never land".to_string(),
        }
    }
}

/// `what` as Flume's own command of its name.
fn flume_act(what: &ActSort) -> flume::Act {
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
        ActSort::Drop => flume::Act::Drop,
        ActSort::Priority(priority) => flume::Act::Priority(*priority),
        ActSort::Done => flume::Act::Done,
    }
}

/// The runner's words refusing `what`, where it is a confirm the runner
/// carries out for no control plane: one made as the runner's own name,
/// whose ends the runner carries and confirms itself (core.54), or one
/// naming no key, which confirms nothing (core.56).
fn refused_confirm(what: &ActSort) -> Option<String> {
    let ActSort::Confirm {
        recipient,
        deliveries,
    } = what
    else {
        return None;
    };
    if recipient == flume::RECIPIENT {
        Some(format!(
            "{} is the runner's own name, whose ends the runner confirms itself",
            flume::RECIPIENT
        ))
    } else if deliveries.is_empty() {
        Some("a confirm naming no delivery key confirms nothing".to_string())
    } else {
        None
    }
}

/// An act's sort, as its lines name it.
fn sort(what: &ActSort) -> String {
    match what {
        ActSort::Retry { .. } => "retry".to_string(),
        ActSort::Land => "land".to_string(),
        ActSort::Confirm { recipient, .. } => format!("confirm as {recipient}"),
        ActSort::Drop => "drop".to_string(),
        ActSort::Priority(priority) => format!("priority {priority}"),
        ActSort::Done => "done".to_string(),
    }
}

/// The location of the place whose entry is `entry`: its `place`, or,
/// for the record Flume keeps of a place tidied while an end of it is
/// pending, `<location>@<at>`, the location it stood at.
fn place_of(entry: &Value) -> Option<String> {
    let named = facts::location(entry)?;
    let tidied = entry.pointer("/phase/tidied").is_some();
    let stood = named
        .rsplit_once('@')
        .filter(|(_, at)| tidied && !at.is_empty() && at.bytes().all(|byte| byte.is_ascii_digit()));
    Some(stood.map_or(named, |(location, _)| location).to_string())
}

/// Room on the host and in each of its streams, as the pass's one
/// `flume --json status` gives it, counted down as the pass launches
/// (design Decision 8; core.9). Room is Flume's: the runner reads it and
/// adds no bound of its own.
struct Room {
    host: Option<facts::Room>,
    streams: Vec<facts::StreamRoom>,
}

impl Room {
    fn read(listing: Option<&Value>) -> Room {
        Room {
            host: listing.and_then(facts::host_room),
            streams: listing.map(facts::streams).unwrap_or_default(),
        }
    }

    /// Why a launch running in `stream`, or naming none, has no room, or
    /// none where it has room. The host has room while its room is
    /// unbounded or above zero. A stream Flume lists has room while the
    /// host has, its own room is unbounded or above zero, and no place
    /// waits in its queue. A stream Flume does not list is no room
    /// question: Flume's refusal of it is the launch's answer. A launch
    /// naming no stream runs in the one Flume's host configuration gives
    /// it, which its status does not say, so it is judged by the host's
    /// room alone.
    fn lacks(&self, stream: Option<&str>) -> Option<String> {
        if !open(self.host) {
            return Some("waits for room on this host".to_string());
        }
        let listed = stream.and_then(|name| self.streams.iter().find(|s| s.name == name))?;
        if open(listed.room) && listed.queue_empty == Some(true) {
            return None;
        }
        Some(format!("waits for room in the stream {}", listed.name))
    }

    /// Counts one launch against the host's room and `stream`'s, where
    /// each is a number.
    fn take(&mut self, stream: Option<&str>) {
        count_down(&mut self.host);
        if let Some(listed) =
            stream.and_then(|name| self.streams.iter_mut().find(|s| s.name == name))
        {
            count_down(&mut listed.room);
        }
    }
}

/// Whether `room`, as Flume gives it, is unbounded or above zero. A room
/// Flume's status does not give as either is none.
fn open(room: Option<facts::Room>) -> bool {
    match room {
        Some(facts::Room::Unbounded) => true,
        Some(facts::Room::Count(count)) => count > 0,
        None => false,
    }
}

fn count_down(room: &mut Option<facts::Room>) {
    if let Some(facts::Room::Count(count)) = room {
        *count -= 1;
    }
}

/// The runner's mark on the place it launches for `id`, `boomdrive:<id>`
/// (design Decision 11).
pub fn mark(id: &Id) -> String {
    format!("boomdrive:{id}")
}

/// What `flume launch` is given for `offer` in the clone at `container`:
/// every field as the launch gives it, its priority `priority`, and the
/// runner's mark. It names no stream, priority or intent of the runner's
/// own (core.9), and no kinds file (core.5).
fn launch_args(offer: &Offer, container: &Path, priority: Option<i64>) -> LaunchArgs {
    let launch = &offer.launch;
    LaunchArgs {
        container_dir: container.to_path_buf(),
        kind: launch.kind.clone(),
        name: launch.name.clone(),
        base: launch.base.clone(),
        stream: launch.stream.clone(),
        priority,
        intent: launch.intent.clone(),
        recipients: launch.recipients.clone(),
        holds: launch
            .holds
            .iter()
            .map(|hold| (hold.stage.clone(), hold.words.clone()))
            .collect(),
        hold_submission: launch.hold_submission,
        brief: launch.brief.clone(),
        agent_settings: launch
            .agent_settings
            .iter()
            .map(|setting| (setting.field.clone(), setting.value.clone()))
            .collect(),
        mark: mark(&offer.id),
    }
}

/// When the control pass after one that began at `began` and ended at
/// `ended` is due (design Decisions 5 and 13; core.22): `poll_interval`
/// after `began`, or, after a pass that `reported`, sooner, by the
/// midpoint between `ended` and the earliest expiry of a lease the runner
/// holds not yet lapsed, `left` after `ended`, so the lease is renewed
/// before it lapses; and never before `ended`. A lapsed lease, or a pass
/// that did not report, hastens none: while the control plane cannot be
/// reached, a pass sooner renews nothing.
pub fn next_due(
    began: Instant,
    ended: Instant,
    poll_interval: Duration,
    left: Option<Duration>,
    reported: bool,
) -> Instant {
    let polled = began + poll_interval;
    let due = match left {
        Some(left) if reported => polled.min(ended + left / 2),
        _ => polled,
    };
    due.max(ended)
}

/// How long after `now` a lease expiring at `expires` has left, or none
/// where it has lapsed by `now`.
pub fn left(expires: At, now: At) -> Option<Duration> {
    Duration::try_from(expires.time() - now.time())
        .ok()
        .filter(|left| !left.is_zero())
}

/// Whether `lease` has lapsed by `now`: it has an expiry, and `now` is
/// not before it. `At` drops a second's fraction, so an expiry `now`
/// equals has passed.
fn lapsed(lease: Lease, now: At) -> bool {
    lease.expires.is_some_and(|expires| expires <= now)
}

/// The earliest expiry of a lease `report` holds not yet lapsed at `now`.
fn expires(report: &Report, now: At) -> Option<At> {
    let launches = report.launches.values().map(|launched| launched.lease);
    let commands = report.commands.values().map(|commanded| commanded.lease);
    let previews = report.previews.values().map(|previewed| previewed.lease);
    launches
        .chain(commands)
        .chain(previews)
        .filter(|lease| !lapsed(*lease, now))
        .filter_map(|lease| lease.expires)
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Account, Drive};
    use std::path::Path;

    fn at(text: &str) -> At {
        At::parse(text).unwrap()
    }

    /// The work-into-flume spec's "Renewed before it lapses": a lease
    /// expiring 40 seconds after the end of the pass that took it, with
    /// `poll_interval` one minute, has the next pass due no later than 20
    /// seconds after that pass ended.
    #[test]
    fn renewed_before_it_lapses() {
        let began = Instant::now();
        let ended = began + Duration::from_secs(5);
        let minute = Duration::from_secs(60);
        let due = next_due(began, ended, minute, Some(Duration::from_secs(40)), true);
        assert_eq!(due, ended + Duration::from_secs(20));
        assert_eq!(next_due(began, ended, minute, None, true), began + minute);
        let far = Some(Duration::from_secs(600));
        assert_eq!(next_due(began, ended, minute, far, true), began + minute);
        let slow = began + Duration::from_secs(90);
        assert_eq!(next_due(began, slow, minute, None, true), slow);
    }

    /// The running spec's "A lapsed lease": with `poll_interval` one
    /// minute, a lease that lapsed before a pass that began at 10:00:00,
    /// and the control plane unreachable, the next pass begins at
    /// 10:01:00, and none between. A lapsed lease has no time left, so it
    /// hastens no pass, whether the pass reported or not.
    #[test]
    fn a_lapsed_lease() {
        let ten = at("2026-11-01T10:00:00Z");
        let lapsed = left(at("2026-11-01T09:59:00Z"), ten);
        assert_eq!(lapsed, None);
        assert_eq!(left(ten, ten), None, "an expiry of now has passed");
        let began = Instant::now();
        let ended = began + Duration::from_secs(5);
        let minute = Duration::from_secs(60);
        assert_eq!(
            next_due(began, ended, minute, lapsed, false),
            began + minute
        );
        assert_eq!(next_due(began, ended, minute, lapsed, true), began + minute);
    }

    /// The running spec's "A pass that did not report": with
    /// `poll_interval` one minute, a lease expiring 40 seconds after a
    /// pass that began at 10:00:00, and that pass's report not sent, the
    /// next pass begins at 10:01:00.
    #[test]
    fn a_pass_that_did_not_report() {
        let ten = at("2026-11-01T10:00:00Z");
        let left = left(at("2026-11-01T10:00:40Z"), ten);
        assert_eq!(left, Some(Duration::from_secs(40)));
        let began = Instant::now();
        let ended = began + Duration::from_secs(5);
        let minute = Duration::from_secs(60);
        assert_eq!(next_due(began, ended, minute, left, false), began + minute);
    }

    /// A lease has lapsed once the clock reads its expiry, and a lease
    /// with no expiry never lapses; only leases not yet lapsed give the
    /// earliest expiry.
    #[test]
    fn a_lease_lapsed_by_the_clock() {
        let ten = at("2026-11-01T10:00:00Z");
        let lease = |expires: Option<&str>| Lease {
            number: 1,
            expires: expires.map(at),
        };
        assert!(lapsed(lease(Some("2026-11-01T09:59:00Z")), ten));
        assert!(lapsed(lease(Some("2026-11-01T10:00:00Z")), ten));
        assert!(!lapsed(lease(Some("2026-11-01T10:00:01Z")), ten));
        assert!(!lapsed(lease(None), ten));
        let mut report = Report::default();
        let answer = Answer {
            outcome: Outcome::Launched,
            place_dir: None,
            words: None,
            at: ten,
        };
        for (id, expires) in [
            ("a", Some("2026-11-01T09:59:00Z")),
            ("b", Some("2026-11-01T10:05:00Z")),
            ("c", None),
        ] {
            report.answer(&Id::new(id), lease(expires), answer.clone());
        }
        assert_eq!(expires(&report, ten), Some(at("2026-11-01T10:05:00Z")));
    }

    fn config(repositories: &[&str]) -> Config {
        Config {
            repository_root: PathBuf::from("/code"),
            fetch_interval: std::time::Duration::from_secs(120),
            code_workspaces: false,
            accounts: vec![Account {
                name: "own".to_string(),
                drives: vec![Drive {
                    name: "home".to_string(),
                    repositories: repositories
                        .iter()
                        .map(|address| Address::parse(address).unwrap())
                        .collect(),
                }],
            }],
            control_plane: None,
            local: crate::config::LocalLayer::NoFile,
        }
    }

    /// The clone of `address` in `container`, as `clone::find` finds one
    /// that records no drive.
    fn found(address: Address, container: &str) -> clone::Found {
        clone::Found {
            address,
            container: PathBuf::from(container),
            drive: None,
        }
    }

    #[test]
    fn a_repository_is_kept_as_the_configuration_and_the_clones_have_it() {
        let flume = Address::parse("agentplot/flume").unwrap();
        let clones = Configured::new(
            &config(&["example/crew", "agentplot/flume"]),
            vec![found(flume, "/d/repositories/agentplot/flume")],
        );
        assert_eq!(
            clones.keeping("Agentplot/Flume"),
            Keeping::Ready(Path::new("/d/repositories/agentplot/flume").into())
        );
        assert_eq!(clones.keeping("example/crew"), Keeping::NotYet);
        assert_eq!(clones.keeping("agentplot/sluice"), Keeping::NotNamed);
        assert_eq!(clones.keeping("not an address"), Keeping::NotNamed);
        assert_eq!(clones.made(), ["agentplot/flume"]);
    }

    /// With task 2.2, the work-into-flume spec's "Each drive's
    /// repositories with their folders": each drive is given with each
    /// address it names whose clone is made, at its container, a drive
    /// with none made is given with none, and a clone no drive names is
    /// in none (design Decision 2).
    #[test]
    fn each_drive_names_its_repositories_whose_clones_are_made() {
        let address = |text: &str| Address::parse(text).unwrap();
        let drive = |name: &str, repositories: &[&str]| Drive {
            name: name.to_string(),
            repositories: repositories.iter().copied().map(address).collect(),
        };
        let config = Config {
            accounts: vec![Account {
                name: "own".to_string(),
                drives: vec![
                    drive("home", &["agentplot/flume", "example/crew"]),
                    drive("work", &["example/app"]),
                    drive("idle", &["example/later"]),
                ],
            }],
            ..config(&[])
        };
        let clones = Configured::new(
            &config,
            vec![
                found(
                    address("agentplot/flume"),
                    "/d/repositories/agentplot/flume",
                ),
                found(address("example/app"), "/d/repositories/example/app"),
                found(address("example/kit"), "/d/repositories/example/kit"),
            ],
        );
        let kept = |address: &str, container: &str| KeptRepository {
            address: address.to_string(),
            container_dir: container.to_string(),
        };
        let drives = clones.drives();
        assert_eq!(
            drives,
            BTreeMap::from([
                (
                    "home".to_string(),
                    vec![kept("agentplot/flume", "/d/repositories/agentplot/flume")],
                ),
                (
                    "work".to_string(),
                    vec![kept("example/app", "/d/repositories/example/app")],
                ),
                ("idle".to_string(), vec![]),
            ])
        );
        assert!(drives
            .values()
            .flatten()
            .all(|kept| kept.address != "example/kit"));
    }

    #[test]
    fn a_clone_the_configuration_drops_is_dropped() {
        let flume = Address::parse("agentplot/flume").unwrap();
        let crew = Address::parse("example/crew").unwrap();
        let clones = Configured::new(
            &config(&["example/crew"]),
            vec![
                found(flume, "/d/repositories/agentplot/flume"),
                found(crew, "/d/repositories/example/crew"),
            ],
        );
        assert_eq!(
            clones.keeping("Agentplot/Flume"),
            Keeping::Dropped(Path::new("/d/repositories/agentplot/flume").into())
        );
        assert_eq!(clones.keeping("agentplot/sluice"), Keeping::NotNamed);
        assert_eq!(clones.made(), ["example/crew"]);
        assert_eq!(clones.dropped(), ["agentplot/flume"]);
    }
}
